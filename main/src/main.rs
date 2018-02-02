use std::ffi::CString;
use std::os::raw;
use std::{thread, time};

const RTLD_NOW: raw::c_int = 2;
const LIB_PATH: &'static str = "../app/target/debug/libapp.dylib";

extern "C" {
    fn dlopen(filename: *const raw::c_char, flags: raw::c_int) -> *mut raw::c_void;
    fn dlclose(handle: *mut raw::c_void) -> raw::c_int;
    fn dlsym(handle: *mut raw::c_void, symbol: *const raw::c_char) -> *mut raw::c_void;
    fn dlerror() -> *mut raw::c_char;
}

fn main() {
    loop {
        unsafe {
            thread::sleep(time::Duration::from_secs(1));
            let handle = dlopen(CString::new(LIB_PATH).unwrap().into_raw(), RTLD_NOW);

            let symbol = b"get_message";
            let get_message = dlsym(handle, CString::new(&symbol[..]).unwrap().into_raw());
            if get_message.is_null() {
                println!("Failed to retrieve get_message symbol: {}", CString::from_raw(dlerror()).into_string().expect("Failed to retrieve dlerror"));
                return;
            }

            let func = ::std::mem::transmute::<*mut raw::c_void, unsafe extern fn() -> *mut raw::c_char>(get_message);
            let message = CString::from_raw(func()).into_string().unwrap();
            println!("Message: {}", message);

            if dlclose(handle) != 0 {
                println!("Failed to close handle: {}", CString::from_raw(dlerror()).into_string().expect("Failed to retrieve dlerror"));
                return;
            }
        }
    }
}
