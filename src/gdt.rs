use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use lazy_static::lazy_static;

// TSS (Task State Segment) stuff to create our Interrupt Stack Table
pub const DOUBLE_FAULT_IST_INDEX: u16 = 0; // explicitly define the first stack in our Interrupt Stack Table to correspond to a Double Fault.

lazy_static! { // use lazy static initialization
	static ref TSS: TaskStateSegment = { // TSS will be a reference to the tss object created in this block
		let mut tss = TaskStateSegment::new();
		tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = { // set the value at that index to be:
			const STACK_SIZE: usize = 4096 * 5; // Define the stack size
			static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE]; // create our stack of that size and fill it with 0s

			let stack_start = VirtAddr::from_ptr(&raw const STACK);
			let stack_end = stack_start + STACK_SIZE;
			stack_end
		};
		tss
	};
}

// GDT (Global Descriptor Table) stuff to get our stack allocated in memory;

lazy_static! {
	static ref GDT: (GlobalDescriptorTable, Selectors) = { // Create and sync up with the Global Descriptor Table
		let mut gdt = GlobalDescriptorTable::new();
		let code_selector = gdt.add_entry(Descriptor::kernel_code_segment()); 
		let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS)); // Add entry pointing to our TSS
		(gdt, Selectors { code_selector, tss_selector })
	};
}

struct Selectors {
	code_selector: SegmentSelector,
	tss_selector: SegmentSelector,
}

pub fn init() { // An init method to be called by the crate lib on program init
	use x86_64::instructions::segmentation::{CS, Segment};
	use x86_64::instructions::tables::load_tss;


	GDT.0.load();
	unsafe {
		CS::set_reg(GDT.1.code_selector);
		load_tss(GDT.1.tss_selector);
	}
}
