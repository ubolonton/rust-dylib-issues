extern crate libc;
extern crate libloading;

use std::{thread, time};
use libloading::{Library, Symbol};

const LIB_PATH: &'static str = "../module/target/debug/libmodule.dylib";
// const LIB_PATH: &'static str = "../module/target/libmodule-c.so";

extern "C" {
    fn _dyld_image_count() -> u32;

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
    // println!("use_lib in thread {:?}", thread::current().id());

    unsafe {
        _tlv_atexit(on_thread_exit, std::ptr::null_mut());
    }

    let lib = Library::new(LIB_PATH)
        .expect("Library not found");

    println!("image count after loading  : {}", image_count());

    unsafe {
        let init: Symbol<unsafe extern "C" fn()> =
            lib.get(b"init").expect("Symbol not found");
        init()
    };

    drop(lib);

    println!("image count after unloading: {}", image_count());
}

fn main() {
    unsafe {
        _dyld_register_func_for_add_image(image_added);
        _dyld_register_func_for_remove_image(image_removed);
    }

    // println!("main thread {:?}", thread::current().id());
    loop {
        thread::sleep(time::Duration::from_secs(1));
        thread::spawn(use_lib).join().unwrap();
    }
}
