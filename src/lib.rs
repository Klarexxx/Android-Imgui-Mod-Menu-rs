#![allow(unsafe_op_in_unsafe_fn)]
use jni::{
    self, JavaVM,
    sys::{JNI_VERSION_1_6, jint},
};
use kittymemory::prelude::*;
use log::LevelFilter;
use std::{mem::transmute, os::raw::c_void, sync::OnceLock, thread::sleep};
mod menu;

#[unsafe(no_mangle)]
pub extern "system" fn JNI_OnLoad(vm: JavaVM, _: *mut c_void) -> jint {
    let _env = vm.get_env().expect("Cannot get reference to the JNIEnv");

    std::hint::black_box(env!("CARGO_PKG_AUTHORS"));
    android_logger::init_once(
        android_logger::Config::default()
            .with_tag("ModMenu")
            .with_max_level(LevelFilter::Debug),
    );

    unsafe {
        std::thread::spawn(|| hack_thread());
    }
    JNI_VERSION_1_6
}

unsafe fn hack_thread() {
    sleep(std::time::Duration::from_millis(200));
    menu::setup();

    let libil2cpp: usize;
    loop {
        if let Some(lib) = ElfScanner::find("libil2cpp.so") {
            libil2cpp = lib.base();
            break;
        }
        sleep(std::time::Duration::from_secs(1));
    }

    // patch, hook etc
    // https://docs.rs/dobby-rs/0.1.0/dobby_rs/
    // https://docs.rs/crate/kittymemory-rs/latest

    let _ = GET_MAIN.set(transmute(libil2cpp + 0x27DC3B4));
    //log::info!("get_main: {:#X}", get_main())
}

static GET_MAIN: OnceLock<fn() -> usize> = OnceLock::new();
unsafe fn get_main() -> usize {
    GET_MAIN.get().unwrap_unchecked()()
}
