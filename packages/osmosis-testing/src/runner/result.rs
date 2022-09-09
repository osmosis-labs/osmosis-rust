use crate::runner::error::{DecodeError, RunnerError};
use cosmrs::proto::cosmos::base::abci::v1beta1::{GasInfo, TxMsgData};
use cosmrs::proto::tendermint::abci::ResponseDeliverTx;
use cosmwasm_std::{Attribute, Event};
use prost::Message;
use std::ffi::CString;
use std::str::Utf8Error;

pub type RunnerResult<T> = Result<T, RunnerError>;
pub type RunnerExecuteResult<R> = Result<ExecuteResponse<R>, RunnerError>;

#[derive(Debug, Clone, PartialEq)]
pub struct ExecuteResponse<R>
where
    R: prost::Message + Default,
{
    pub data: R,
    pub raw_data: Vec<u8>,
    pub events: Vec<Event>,
    pub gas_info: GasInfo,
}

impl<R> TryFrom<ResponseDeliverTx> for ExecuteResponse<R>
where
    R: prost::Message + Default,
{
    type Error = RunnerError;

    fn try_from(res: ResponseDeliverTx) -> Result<Self, Self::Error> {
        let tx_msg_data =
            TxMsgData::decode(res.data.as_slice()).map_err(DecodeError::ProtoDecodeError)?;

        let msg_data = &tx_msg_data
            .data
            // since this tx contains exactly 1 msg
            // when getting none of them, that means error
            .get(0)
            .ok_or(RunnerError::AppError { msg: res.log })?;

        let data = R::decode(msg_data.data.as_slice()).map_err(DecodeError::ProtoDecodeError)?;

        let events = res
            .events
            .into_iter()
            .map(|e| -> Result<Event, DecodeError> {
                Ok(Event::new(e.r#type.to_string()).add_attributes(
                    e.attributes
                        .into_iter()
                        .map(|a| -> Result<Attribute, Utf8Error> {
                            Ok(Attribute {
                                key: std::str::from_utf8(a.key.as_slice())?.to_string(),
                                value: std::str::from_utf8(a.value.as_slice())?.to_string(),
                            })
                        })
                        .collect::<Result<Vec<Attribute>, Utf8Error>>()?,
                ))
            })
            .collect::<Result<Vec<Event>, DecodeError>>()?;

        Ok(ExecuteResponse {
            data,
            raw_data: res.data,
            events,
            gas_info: GasInfo {
                gas_wanted: res.gas_wanted as u64,
                gas_used: res.gas_used as u64,
            },
        })
    }
}

/// `RawResult` facilitates type conversions between Go and Rust,
///
/// Since Go struct could not be exposed via cgo due to limitations on
/// its unstable behavior of its memory layout.
/// So, apart from passing primitive types, we need to:
///
///   Go { T -> bytes(T) -> base64 -> *c_char }
///                      ↓
///   Rust { *c_char -> base64 -> bytes(T') -> T' }
///
/// Where T and T' are corresponding data structures, regardless of their encoding
/// in their respective language plus error information.
///
/// Resulted bytes are tagged by prepending 1 byte to byte array
/// before base64 encoded. The prepended byte represents: 0 = Err, 1 = Ok.
pub struct RawResult(Result<Vec<u8>, String>);

impl RawResult {
    /// Convert ptr to AppResult. Check the first byte tag before decoding the rest of the bytes into expected type
    pub(crate) fn from_ptr(ptr: *mut std::os::raw::c_char) -> Option<Self> {
        if ptr.is_null() {
            return None;
        }

        let c_string = unsafe { CString::from_raw(ptr) };
        let base64_bytes = c_string.to_bytes();
        let bytes = base64::decode(base64_bytes).unwrap();

        if bytes[0] == 0 {
            let error = CString::new(&bytes[1..])
                .unwrap()
                .to_str()
                .expect("Go code must encode valid UTF-8 string")
                .to_string();
            Some(Self(Err(error)))
        } else {
            let res = CString::new(&bytes[1..]).unwrap().into_bytes();

            Some(Self(Ok(res)))
        }
    }

    /// Convert ptr to AppResult. Use this function only when it is sure that the
    /// pointer is not a null pointer.
    ///
    /// # Safety
    /// There is a potential null pointer here, need to be extra careful before
    /// calling this function
    pub(crate) unsafe fn from_non_null_ptr(ptr: *mut std::os::raw::c_char) -> Self {
        Self::from_ptr(ptr).expect("Must ensure that the pointer is not null")
    }

    pub(crate) fn into_result(self) -> Result<Vec<u8>, String> {
        self.0
    }
}
