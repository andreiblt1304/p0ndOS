#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
// have to rename the test runner generated function because it's called "main"
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod vga_buffer;

#[unsafe(no_mangle)] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("HELLO from the p0nd OS!");

    #[cfg(test)]
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());

    for test in tests {
        test();
    }
}

#[test_case]
fn test_assertion() {
    print!("some test assertion...");
    let a = true;
    let b = true;
    assert_eq!(a, b);
    println!("[OK]");
}
