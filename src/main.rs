#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(p0nd_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::boxed::Box;
use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use p0nd_os::{memory::BootInfoFrameAllocator, println};

// type-checked way to define the function as the kernel entry point
entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use p0nd_os::allocator;
    use p0nd_os::memory;
    use x86_64::VirtAddr;

    println!("HELLO from the p0nd OS!");
    p0nd_os::init();

    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(physical_memory_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap init failed");

    let x = Box::new(1);

    #[cfg(test)]
    test_main();

    println!("Nothing happened");
    p0nd_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    p0nd_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    p0nd_os::test_panic_handler(info)
}
