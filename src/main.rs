#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![no_std] // don't link the Rust standard lib
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

use core::panic::PanicInfo;

// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	// The _info parameter contains info about the panic's cause.
	println!("{}", _info); // Right now, use the VGA buffer println macro to print panic info.
	loop {} // Pause processes by creating an infinite loop.
}

#[unsafe(no_mangle)] // don't mangle the name of this function when compiling, literally name it _start
pub extern "C" fn _start() -> ! {
	// this function is the entry point, since the linker looks for a function named _start by default
	println!("Test!!"); // Call our println macro to write to the VGA buffer directly.
	println!("It is so cool that {}", "this works");
	
	loop {}
}


