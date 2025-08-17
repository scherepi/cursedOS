#![no_std] // don't link the Rust standard lib
#![no_main] // disable all Rust-level entry points (don't look for a main function)

use core::panic::PanicInfo;

/// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	/// The _info parameter contains info about the panic's cause.
	loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[unsafe(no_mangle)] // don't mangle the name of this function when compiling, literally name it _start
pub extern "C" fn _start() -> ! {
	// this function is the entry point, since the linker looks for a function named _start by default
	let vga_buffer = 0xb8000 as *mut u8;

	for (i, &byte) in HELLO.iter().enumerate() {
		unsafe {
			*vga_buffer.offset(i as isize * 2) = byte;
			*vga_buffer.offset(i as isize * 2 + 1) = 0xb;
		}
	}

	loop {} // dummy comment
}
