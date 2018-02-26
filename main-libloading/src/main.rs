extern crate libloading;

use std::os::raw;
use std::{thread, time};
use libloading::{Library, Symbol};

// const LIB_PATH: &'static str = "../module/target/debug/libmodule.dylib";
const LIB_PATH: &'static str = "../module/target/libmodule-c.so";

extern "C" {
    fn _dyld_image_count() -> u32;
}

fn image_count() -> u32 {
    unsafe { _dyld_image_count() }
}

fn main() {
    loop {
        thread::sleep(time::Duration::from_secs(1));

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
}
