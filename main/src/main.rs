extern crate libloading;

use std::os::raw;
use std::{thread, time};
use libloading::{Library, Symbol};

const LIB_PATH: &'static str = "../app/target/debug/libapp.dylib";

extern "C" {
    fn _dyld_image_count() -> u32;
}

fn main() {
    loop {
        unsafe {
            let i = _dyld_image_count();
            println!("{}", i);
        }

        thread::sleep(time::Duration::from_secs(1));

        let lib = Library::new(LIB_PATH)
            .expect("Library not found");

        unsafe {
            let init: Symbol<unsafe extern "C" fn()> =
                lib.get(b"init").expect("Symbol not found");
            init()
        };
    }
}
