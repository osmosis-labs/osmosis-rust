use std::ffi::CString;
use crate::GoString;

/// conversion from &CString to GoString
impl From<&CString> for GoString {
    fn from(c_str: &CString) -> Self {
        let ptr_c_str = c_str.as_ptr();

        GoString {
            p: ptr_c_str,
            n: c_str.as_bytes().len() as isize,
        }
    }
}
