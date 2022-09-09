use cosmrs::proto::cosmos::base::abci::v1beta1::{GasInfo, TxMsgData};
use cosmrs::proto::tendermint::abci::ResponseDeliverTx;
use cosmwasm_std::{Attribute, Event};
use prost::Message;
use std::str::Utf8Error;
use thiserror::Error;

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
    type Error = DecodeError;

    fn try_from(res: ResponseDeliverTx) -> Result<Self, Self::Error> {
        let tx_msg_data = TxMsgData::decode(res.data.as_slice())?;
        let msg_data = &tx_msg_data.data[0]; // since this tx contains only 1 msg, panic if not the case

        let data = R::decode(msg_data.data.as_slice())?;

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

#[derive(Error, Debug)]
pub enum RunnerError {
    #[error("unable to encode request")]
    EncodeError(#[from] EncodeError),

    #[error("unable to decode response")]
    DecodeError(#[from] DecodeError),
}

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("invalid utf8 bytes")]
    Utf8Error(#[from] Utf8Error),

    #[error("invalid protobuf")]
    ProtoDecodeError(#[from] prost::DecodeError),

    #[error("invalid json")]
    JsonDecodeError(#[from] serde_json::Error),

    #[error("invalid base64")]
    Base64DecodeError(#[from] base64::DecodeError),

    #[error("invalid signing key")]
    SigningKeyDecodeError { msg: String },
}

#[derive(Error, Debug)]
pub enum EncodeError {
    #[error("invalid protobuf")]
    ProtoEncodeError(#[from] prost::EncodeError),

    #[error("unable to encode json")]
    JsonEncodeError(#[from] serde_json::Error),
}
