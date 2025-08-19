use x86_64::structures::idt::InterruptDescriptorTable;
use crate::println;

pub fn init_idt() {
	let mut idt = InterruptDescriptorTable::new();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
	println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}
