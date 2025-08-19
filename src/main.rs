#![no_std] // don't link the Rust standard lib
#![no_main] // disable all Rust-level entry points (don't look for a main function)

mod vga_buffer;

use core::panic::PanicInfo;

/// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	/// The _info parameter contains info about the panic's cause.
	loop {}
}

#[unsafe(no_mangle)] // don't mangle the name of this function when compiling, literally name it _start
pub extern "C" fn _start() -> ! {
	// this function is the entry point, since the linker looks for a function named _start by default
	use core::fmt::Write;
	vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
	write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();
	
	loop {}
}
