#![no_std]
#![no_main]

use core::panic::PanicInfo;

// In Rust 2024, the block itself must be marked unsafe
unsafe extern "C" {
    fn puts(s: *const u8) -> i32;
}

#[unsafe(no_mangle)]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    // Null-terminated string for C
    const HELLO: &[u8] = b"Hello from a tiny M4 Rust binary!\0";
    
    unsafe {
        puts(HELLO.as_ptr());
    }
    
    0
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

