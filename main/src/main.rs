extern crate libc;

use std::ffi::{CString, CStr};
use std::{thread, time};
use std::mem;

const LIB_PATH: &'static str = "../module/target/debug/libmodule.dylib";
// const LIB_PATH: &'static str = "../module/target/libmodule-c.so";

const RTLD_NOW: libc::c_int = 2;
extern "C" {
    fn _dyld_image_count() -> u32;
    fn dlopen(filename: *const libc::c_char, flags: libc::c_int) -> *mut libc::c_void;
    fn dlclose(handle: *mut libc::c_void) -> libc::c_int;
    fn dlsym(handle: *mut libc::c_void, symbol: *const libc::c_char) -> *mut libc::c_void;
    fn dlerror() -> *mut libc::c_char;

    fn _dyld_register_func_for_add_image(
        func: unsafe extern "C" fn(mh: *const libc::mach_header, vmaddr_slide: libc::intptr_t)
    );
    fn _dyld_register_func_for_remove_image(
        func: unsafe extern "C" fn(mh: *const libc::mach_header, vmaddr_slide: libc::intptr_t)
    );
    fn _tlv_atexit(dtor: unsafe extern fn(*mut libc::c_void), arg: *mut libc::c_void);
}

fn image_count() -> u32 {
    unsafe { _dyld_image_count() }
}

unsafe extern fn on_thread_exit(obj_addr: *mut libc::c_void) {
    println!("-- thread exiting {:?}", obj_addr);
}

unsafe extern "C" fn image_added(mh: *const libc::mach_header, vmaddr_slide: libc::intptr_t) {
    println!("-- image-added  : {:?} {}", mh, vmaddr_slide);
}

unsafe extern "C" fn image_removed(mh: *const libc::mach_header, vmaddr_slide: libc::intptr_t) {
    println!("-- image-removed: {:?} {}", mh, vmaddr_slide);
}

fn use_lib() {
    println!("use_lib in thread {:?}", thread::current().id());

    unsafe {
        let handle = dlopen(CString::new(LIB_PATH).unwrap().into_raw(), RTLD_NOW);

        _tlv_atexit(on_thread_exit, std::ptr::null_mut());

        println!("image count after loading  : {}", image_count());

        let symbol = b"init";
        let raw_init = dlsym(handle, CString::new(&symbol[..]).unwrap().into_raw());
        if raw_init.is_null() {
            println!("Failed to retrieve init symbol: {}", CStr::from_ptr(dlerror()).to_string_lossy());
            return;
        }

        let init = mem::transmute::<*mut libc::c_void, unsafe extern fn()>(raw_init);
        init();

        if dlclose(handle) != 0 {
            println!("Failed to close handle: {}", CString::from_raw(dlerror()).into_string().expect("Failed to retrieve dlerror"));
            return;
        }

        println!("image count after unloading: {}", image_count());
    }
}

fn main() {
    unsafe {
        _dyld_register_func_for_add_image(image_added);
        _dyld_register_func_for_remove_image(image_removed);
    }

    loop {
        thread::sleep(time::Duration::from_secs(1));
        thread::spawn(use_lib).join().unwrap();
    }
}
