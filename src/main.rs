#![no_std]
#![no_main]

use core::panic::PanicInfo;
<<<<<<< HEAD

// Overriding Windows entry points
#[no_mangle]
pub extern "C" fn mainCRTStartup() -> ! {
    main();
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    loop {}
=======
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
>>>>>>> ubuntu
}

static HELLO: &[u8] = b"Hello World!";

<<<<<<< HEAD
// Overriding Linux entry point
=======
>>>>>>> ubuntu
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
<<<<<<< HEAD
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
=======
            *vga_buffer.offset(i as isize * 2 + 1) = 0xd;
        }
    }

    loop {}
}


>>>>>>> ubuntu
