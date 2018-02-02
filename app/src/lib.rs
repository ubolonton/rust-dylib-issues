use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub fn get_message() -> *mut c_char {
    CString::new(String::from("hello world")).unwrap().into_raw()
}
