#![no_main]
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(unsafe_op_in_unsafe_fn)]

pub mod bindings {
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use core::ffi::{c_char, c_int};
use core::ptr;

pub use bindings::*;

// Main functions
// Error libparted handler
static mut PREV_HANDLER: PedExceptionHandler = None;

pub fn init_exception_handler() {
    unsafe {
        PREV_HANDLER = ped_exception_get_handler();
        ped_exception_set_handler(Some(exception_handler));
    }
}

unsafe extern "C" fn exception_handler(ex: *mut PedException) -> PedExceptionOption {
    error(b"Libparted exception occurred\0".as_ptr().cast());

    let f = PREV_HANDLER.expect("ped_exception_get_handler returned None");
    f(ex)
}

pub fn error(message: *const c_char) {
    unsafe { perror(message) }
}

// Extract block device names
pub fn scan_devices() -> Vec<*mut c_char> {
    unsafe {
        ped_device_probe_all();
        let mut devices: Vec<*mut c_char> = Vec::new();
        let mut dev: *mut PedDevice = ptr::null_mut();

        loop {
            dev = ped_device_get_next(dev);
            if dev.is_null() {
                break;
            }
            devices.push((*dev).path);
        }

        devices
    }
}

// helpers
pub fn print_devices(disk: *const PedDisk) {
    unsafe { ped_disk_print(disk) }
}

pub fn get_device(path: *const c_char) -> *mut PedDevice {
    unsafe { ped_device_get(path) }
}

pub fn terminate(status: c_int) -> ! {
    unsafe { exit(status) }
}
