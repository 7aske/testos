#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Overriding Windows entry points
#[no_mangle]
pub extern "C" fn mainCRTStartup() -> ! {
    main();
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

// Overriding Linux entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}

//Overriding exception handler

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop{}
}