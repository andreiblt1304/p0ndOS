#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(p0nd_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use p0nd_os::println;

// type-checked way to define the function as the kernel entry point
entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use p0nd_os::memory::translate_address_wrapper;
    use x86_64::VirtAddr;

    println!("HELLO from the p0nd OS!");
    p0nd_os::init();

    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];
    for &address in &addresses {
        let virtual_address = VirtAddr::new(address);
        let physical_address =
            unsafe { translate_address_wrapper(virtual_address, physical_memory_offset) };

        println!("{:?} -> {:?}", virtual_address, physical_address);
    }

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
