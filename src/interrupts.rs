use crate::println;
use crate::print;
use crate::gdt;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

impl InterruptIndex {
	fn as_u8(self) -> u8 {
		self as u8
	}
	
	fn as_usize(self) -> usize {
	usize::from(self.as_u8())
	}
}

lazy_static! {
	static ref IDT: InterruptDescriptorTable = {
		let mut idt = InterruptDescriptorTable::new();
		idt.breakpoint.set_handler_fn(breakpoint_handler);
		unsafe {
			idt.double_fault.set_handler_fn(double_fault_handler) // set double faults to be handled by our function
				.set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX); // set the handler to use corresponding stack
		}
		idt[InterruptIndex::Timer.as_u8()]
			.set_handler_fn(timer_interrupt_handler); 	// set the timer handler function
		idt[InterruptIndex::Keyboard.as_u8()]			// set the key handler function
			.set_handler_fn(keyboard_interrupt_handler);

		idt.page_fault.set_handler_fn(page_fault_handler);

		idt // return the resultant Interrupt Descriptor Table
	};
}

pub fn init_idt() {
	IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
	println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}         

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
	panic!("EXCEPTION: DOUBLE FAULT\n\nOH NO! how to ungay?\n\n{:#?}", stack_frame);
}


// Pic 8259 interrupt controller setup begins here.

use pic8259::ChainedPics; // Struct abstraction of the chained units 
use spin; // necessary to avoid deadlocks?

pub const PIC_1_OFFSET: u8 = 32; // PIC interrupts will begin in the table after 32
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> = 
	spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });


// Define enum to work with PIC ports
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
	Timer = PIC_1_OFFSET, 	// first up is our timer port, at that index
	Keyboard,		// Next, the keyboard!
}

/*impl InterruptIndex {
	fn as_u8(self) -> u8 {
		self as u8
	}

	fn as_usize(self) -> usize {
		usize::from(self.as_u8())
	}
}
*/
// Timer interrupt handling function:

pub static TIMERVAR: spin::Mutex<i32> =
	spin::Mutex::new(1); 

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
	//print!(".");
	//let mut timervar = TIMERVAR.lock();
	//*timervar *= 2;
	//print!("{}", *timervar);
	//removing the number overflow because ts is not in use anymore and it pmo

	unsafe {
		PICS.lock()
			.notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
	}
}

// Keyboard interrupt handling function:
extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
	use x86_64::instructions::port::Port; // Provides an interface for interacting with I/O ports
	use spin::Mutex; // for concurrency bug prevention
	use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1}; // so that i don't have to write the world's LONGEST FUCKING MATCH STATEMENT

	lazy_static! {
		static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = 
			Mutex::new(Keyboard::new(ScancodeSet1::new(), layouts::Us104Key, HandleControl::Ignore));
	}

	let mut keyboard = KEYBOARD.lock();
	let mut port = Port::new(0x60); // Direct route to the PS/2 data port
	let scancode: u8 = unsafe { port.read() }; // Read the key scancode from the data port as a u8

	if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
		if let Some(key) = keyboard.process_keyevent(key_event) {
			match key {
				DecodedKey::Unicode(character) => print!("{}", character),
				DecodedKey::RawKey(key) => print!("{:?}", key),
			}
		}
	}
	
	unsafe {
		PICS.lock()
			.notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
	}
}
// Manual page fault handling
use x86_64::structures::idt::PageFaultErrorCode;
use crate::hlt_loop;

extern "x86-interrupt" fn page_fault_handler(
	stack_frame: InterruptStackFrame,
	error_code: PageFaultErrorCode,
	) {
		use x86_64::registers::control::Cr2;

		println!("EXCEPTION: PAGE FAULT");
		println!("oh god... i lost my page... how am i gonna finish this book if i keep doing that");
		println!("address you tried (and failed, haha) to access: {:?}", Cr2::read());
		println!("error code for your big dumb ass: {:?}", error_code);
		println!("{:#?}", stack_frame);
		hlt_loop();
	}
