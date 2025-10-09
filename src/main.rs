#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(p0nd_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use p0nd_os::println;

#[unsafe(no_mangle)] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("HELLO from the p0nd OS!");

    p0nd_os::init();

    fn stack_overflow() {
        stack_overflow();
    }

    stack_overflow();

    unsafe {
        *(0xdeadbeef as *mut u8) = 42;
    }

    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    println!("Nothing happened");
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    p0nd_os::test_panic_handler(info)
}
