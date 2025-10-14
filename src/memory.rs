use x86_64::{
    structures::paging::{OffsetPageTable, PageTable},
    VirtAddr,
};
pub unsafe fn active_level_table(physical_memory_offset: VirtAddr)->&'static mut PageTable {
    use x86_64::registers::control::Cr3;
    let (level_4_table, _) = Cr3::read();
    let phys = level_4_table.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();
    unsafe {&mut *page_table_ptr}
}

use x86_64::PhysAddr;
pub unsafe fn translate_addr(addr:VirtAddr, physical_memory_offset: VirtAddr)->Option<PhysAddr> {
    translate_addr_inner(addr, physical_memory_offset)
}
pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

 


fn translate_addr_inner(addr:VirtAddr, physical_memory_offset: VirtAddr)->Option<PhysAddr> {
    use x86_64::structures::paging::page_table::FrameError;
    use x86_64::registers::control::Cr3;
    //read the active level 4 frame from the cr3 register
    let (level_4_table_frame, _) = Cr3::read();
    let table_indexes = [addr.p4_index(), addr.p3_index(), addr.p2_index(), addr.p1_index()];
    let mut frame = level_4_table_frame;

    for &index in &table_indexes {
        let virt  = physical_memory_offset + frame.start_address().as_u64();
        let table_ptr: *const PageTable = virt.as_ptr();
        let table = unsafe { &*table_ptr };
        let entry = &table[index];
        frame = match entry.frame() {
            Ok(frame)=>frame,
            Err(FrameError::FrameNotPresent)=>return None,
            Err(FrameError::HugeFrame)=>return None,
        };
    }
    Some(frame.start_address() + addr.page_offset().into())
}


