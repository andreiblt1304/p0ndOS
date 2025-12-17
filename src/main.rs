#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(p0nd_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::vec;
use alloc::{boxed::Box, rc::Rc, vec::Vec};
use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use p0nd_os::task::keyboard::print_keypresses;
use p0nd_os::task::simple_executor::SimpleExecutor;
use p0nd_os::task::task_struct::Task;
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

    let heap_value = Box::new(1);
    println!("heap_value at {:p}", heap_value);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!(
        "current reference count is {}",
        Rc::strong_count(&cloned_reference)
    );
    core::mem::drop(reference_counted);
    println!(
        "reference count is {} now",
        Rc::strong_count(&cloned_reference)
    );

    let mut executor = SimpleExecutor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(print_keypresses()));
    executor.run();

    #[cfg(test)]
    test_main();

    println!("Nothing happened");
    p0nd_os::hlt_loop();
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
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
