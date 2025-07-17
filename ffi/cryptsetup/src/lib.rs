#![no_std]
#![no_main]
use core::ffi::{c_char, c_int};
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

unsafe extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn _start() {
    printf(b"Hola Mundo\n\0".as_ptr().cast());
    exit(69);
}
