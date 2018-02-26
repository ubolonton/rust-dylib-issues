use std::ffi::{CString, CStr};
use std::os::raw;
use std::{thread, time};
use std::mem;

const RTLD_NOW: raw::c_int = 2;
extern "C" {
    fn _dyld_image_count() -> u32;
    fn dlopen(filename: *const raw::c_char, flags: raw::c_int) -> *mut raw::c_void;
    fn dlclose(handle: *mut raw::c_void) -> raw::c_int;
    fn dlsym(handle: *mut raw::c_void, symbol: *const raw::c_char) -> *mut raw::c_void;
    fn dlerror() -> *mut raw::c_char;
}

fn image_count() -> u32 {
    unsafe { _dyld_image_count() }
}

// const LIB_PATH: &'static str = "../module/target/debug/libmodule.dylib";
const LIB_PATH: &'static str = "../module/target/libmodule-c.so";

fn main() {
    loop {
        thread::sleep(time::Duration::from_secs(1));

        unsafe {
            let handle = dlopen(CString::new(LIB_PATH).unwrap().into_raw(), RTLD_NOW);

            println!("image count after loading  : {}", image_count());

            let symbol = b"init";
            let raw_init = dlsym(handle, CString::new(&symbol[..]).unwrap().into_raw());
            if raw_init.is_null() {
                println!("Failed to retrieve init symbol: {}", CStr::from_ptr(dlerror()).to_string_lossy());
                return;
            }

            let init = mem::transmute::<*mut raw::c_void, unsafe extern fn()>(raw_init);
            init();

            if dlclose(handle) != 0 {
                println!("Failed to close handle: {}", CString::from_raw(dlerror()).into_string().expect("Failed to retrieve dlerror"));
                return;
            }

            println!("image count after unloading: {}", image_count());
        }
    }
}
