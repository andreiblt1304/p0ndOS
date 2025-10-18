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
    use p0nd_os::memory::active_level_4_table;
    use x86_64::VirtAddr;

    println!("HELLO from the p0nd OS!");
    p0nd_os::init();

    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { active_level_4_table(physical_memory_offset) };

    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 entry {}: {:?}", i, entry);
        }
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
