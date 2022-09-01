/// For the communication flow between Go and Rust code. Go struct is not supported
/// due to limitations on its unstable behavior of its memory layout.
/// So, apart from privimites, we need to:
///
///   Go { struct -> bytes(proto) -> c_char }
///                      ↓
///   Rust { c_char -> bytes(proto) -> struct }
///
/// Correct proto encoding is expected, so it will panic in case of invalid encoding.
///
use std::ffi::CString;

/// For passing error information, resulted bytes are tagged by prepending 1 byte to byte array
/// before base64 encoded. The prepended byte represents: 0 = Err, 1 = Ok.
pub struct AppResult<T>(Result<T, String>);

impl AppResult<String> {
    /// Convert ptr to AppResult. Check the first byte tag before decoding the rest of the bytes into expected type
    pub fn from_ptr(ptr: *mut std::os::raw::c_char) -> Option<Self> {
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
            let res = CString::new(&bytes[1..])
                .unwrap()
                .to_str()
                .expect("Go code must encode valid UTF-8 string")
                .to_string();

            // TODO: remove the need to base64 encode
            Some(Self(Ok(
                String::from_utf8(base64::decode(res).unwrap()).unwrap()
            )))
        }
    }

    /// Convert ptr to AppResult. Use this function only when it is sure that the
    /// pointer is not a null pointer.
    pub unsafe fn from_non_null_ptr(ptr: *mut std::os::raw::c_char) -> Self {
        Self::from_ptr(ptr).expect("Must ensure that the pointer is not null")
    }

    pub fn into_result(self) -> Result<String, String> {
        self.0
    }
}
