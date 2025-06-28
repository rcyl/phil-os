#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

// no mangling, because we need to tell the name of the entry point
// function to the linker, with mangling with get some cryptic function name
#[unsafe(no_mangle)] 
pub extern "C" fn _start() -> ! {
    // 0xb8000 is the address of the VGA buffer, cast to raw pinter 
    let vga_buffer = 0xb8000 as *mut u8; 

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // 0xb is light cyan
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
