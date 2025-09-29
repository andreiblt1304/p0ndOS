#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use p0nd_os::{exit_qemu, serial_print, serial_println};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("[test din not panic]");
    exit_qemu(p0nd_os::QemuExitCode::Failed);

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(p0nd_os::QemuExitCode::Success);

    loop {}
}

fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(true, false);
}

// test runner used to mimic should_panic behavior
// not used because there no known way to pass tests that panic
// pub fn test_runner(tests: &[&dyn Fn()]) {
//     serial_println!("Running {} tests", tests.len());

//     for test in tests {
//         test();
//         serial_println!("[test did not panic]");
//         exit_qemu(p0nd_os::QemuExitCode::Failed);
//     }

//     exit_qemu(p0nd_os::QemuExitCode::Success);
// }
