use x86_64::{
    PhysAddr, VirtAddr,
    structures::paging::{OffsetPageTable, PageTable},
};

// translate virtual address to the corresponding physical one if exists
// it is unsafe because the caller guarantees that the physical memory is mapped to a virtual one
pub unsafe fn translate_address_wrapper(
    address: VirtAddr,
    physical_memory_offset: VirtAddr,
) -> Option<PhysAddr> {
    translate_address(address, physical_memory_offset)
}

// safe function to limit the scope of unsafe
// the wrapper can only be called using unsafe blocks outside the module
fn translate_address(address: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
    use x86_64::registers::control::Cr3;
    use x86_64::structures::paging::page_table::FrameError;

    let (level_4_table_frame, _) = Cr3::read();
    let table_indexes = [
        address.p4_index(),
        address.p3_index(),
        address.p2_index(),
        address.p1_index(),
    ];
    let mut frame = level_4_table_frame;

    for &index in &table_indexes {
        let virtual_address = physical_memory_offset + frame.start_address().as_u64();
        let table_ptr: *const PageTable = virtual_address.as_ptr();
        let table = unsafe { &*table_ptr };

        let entry = &table[index];
        frame = match entry.frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None,
            Err(FrameError::HugeFrame) => panic!("huge pages not supported"),
        }
    }

    Some(frame.start_address() + u64::from(address.page_offset()))
}

// unsafe because the caller must guarantee that
// the complete physical memory is mapped to virtual one at the passed offset
pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    unsafe {
        let level_4_table = active_level_4_table(physical_memory_offset);
        OffsetPageTable::new(level_4_table, physical_memory_offset)
    }
}

unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();
    let physical_address = level_4_table_frame.start_address();
    let virtual_address = physical_memory_offset + physical_address.as_u64();
    let page_table_ptr: *mut PageTable = virtual_address.as_mut_ptr();

    unsafe { &mut *page_table_ptr }
}
