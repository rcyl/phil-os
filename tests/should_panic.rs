#![no_std]
#![no_main]

use core::panic::PanicInfo;
use phil_os::{QemuExitCode, exit_qemu, serial_print, serial_println};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

// run with cargo test --test should_panic (not sure why --test is needed)
// This only works for a single test function, if multiple test_cases
// can panic only the first function is executed and we cannot continue if
// the panic handler is called
fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}
