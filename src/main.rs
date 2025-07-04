#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(phil_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use phil_os::println;

static HELLO: &[u8] = b"Hello World!";

// no mangling, because we need to tell the name of the entry point
// function to the linker, with mangling with get some cryptic function name
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    
    //{
    // 0xb8000 is the address of the VGA buffer in BIOS, cast to raw pinter
    // let vga_buffer = 0xb8000 as *mut u8;

    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // 0xb is light cyan
    //     }
    // }

    // vga_buffer::print_something();

    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    // write!(vga_buffer::WRITER.lock(), "some numbers: {} {}", 42, 1.337).unwrap();
    // }

    println!("Hello World{}", "!");

    phil_os::init();

    /* invoke a breakpoint exception */
    // x86_64::instructions::interrupts::int3();

    /* trigger a page fault */
    // {
    //     unsafe {
    //         *(0xdeadbeef as *mut u8) = 42;
    //     };
    // }

    /* Trigging stack overflow */
    {
        fn stack_overflow() {
            stack_overflow(); // for each recursion the address is pushed
        }
        stack_overflow();
    }


    #[cfg(test)]
    test_main();

    println!("It did not crash");
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    phil_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

// No panic is considered passing case
#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

// No panic is considered passing case
#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}
