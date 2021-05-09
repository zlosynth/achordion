use std::ffi::CString;

pub fn cstr(string: &str) -> CString {
    CString::new(string).expect("CString::new failed")
}
