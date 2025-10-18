#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(p0nd_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use p0nd_os::println;
use x86_64::structures::paging::Page;

// type-checked way to define the function as the kernel entry point
entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use p0nd_os::memory;
    use x86_64::VirtAddr;

    println!("HELLO from the p0nd OS!");
    p0nd_os::init();

    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(physical_memory_offset) };
    let mut frame_allocator = memory::EmptyFrameAllocator;

    let page = Page::containing_address(VirtAddr::zero());
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe {
        page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e);
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
