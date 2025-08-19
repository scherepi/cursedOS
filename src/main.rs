#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![no_std] // don't link the Rust standard lib
#![no_main] // disable all Rust-level entry points (don't look for a main function)

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
	println!("Running {} tests", tests.len());
	for test in tests {
		test();
	}
}


mod vga_buffer;

use core::panic::PanicInfo;

// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	// The _info parameter contains info about the panic's cause.
	println!("{}", _info);
	loop {}
}

#[unsafe(no_mangle)] // don't mangle the name of this function when compiling, literally name it _start
pub extern "C" fn _start() -> ! {
	// this function is the entry point, since the linker looks for a function named _start by default
	println!("Test!!");
	println!("It is so cool that {}", "this works");
	
	loop {}
}


