#![no_std] // don't link the Rust standard lib
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![no_main] // disable all Rust-level entry points (don't look for a main function)

#[cfg(test)]
// Used to define our custom testing framework because we can't use Rust's standard one.
pub fn test_runner(tests: &[&dyn Fn()]) {
	println!("Running {} tests", tests.len()); // First, print out how many tests we're running.
	for test in tests {
		test(); // Call each function we found.
	}
}


mod vga_buffer; // Include our module for interacting with the VGA buffer, a memory-mapped hardware abstraction of the terminal.
mod memory;

use core::panic::PanicInfo;
use bootloader::entry_point;

// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	// The _info parameter contains info about the panic's cause.
	println!("{}", _info); // Right now, use the VGA buffer println macro to print panic info.
	cursed_os::hlt_loop(); // Pause processes by creating an infinite loop.
}

use x86_64::registers::control::Cr3;
use bootloader::BootInfo;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
	use crate::memory::active_level_table;
	use x86_64::VirtAddr;
	
	println!("Test!!"); // Call our println macro to write to the VGA buffer directly.
	println!("It is so cool that {}", "this works");
	
	cursed_os::init(); // run our init function to set up interrupt descriptor table, etc.

	/*
	//fn stack_overflow() {
	// how to ungay?
	//	stack_overflow();
	//}
	//stack_overflow();
	
	// trigger a page fault
	unsafe {
		*(0xdeadbeef as *mut u8) = 42;
		*(0x40000000 as *mut u8) = 81;
	}
	
	// invoke a breakpoint exception
	// x86_64::instructions::interrupts::int3();
	*/
	let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
	let l4_table = unsafe { active_level_table(physical_memory_offset) };
	
	for (i, entry) in l4_table.iter().enumerate() {
		if !entry.is_unused() {
			println!("Level 4 Entry {}: {:?}", i, entry);
		}
	}

	let (level_4_page_table, _) = Cr3::read();
	println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

	println!("It did not crash?");
	cursed_os::hlt_loop(); // From lib.rs, calls the HLT function to not do anything unless there's an interrupt.
}