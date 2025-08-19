
use spin::Mutex;
use core::fmt;
use volatile::Volatile;

#[allow(dead_code)] 
#[derive(Debug, Clone, Copy, PartialEq, Eq)] // Try to automatically ensure that our custom enum will have all of these traits: be copyable, comparable, and debuggable.
#[repr(u8)] // Tells the compiler to represent this enum's values as an unsigned 8 bit integer.
pub enum Color { // Basic enum of VGA buffer compatible colors.
	Black = 0,
	Blue = 1, 
	Green = 2,
	Cyan = 3, 
	Red = 4, 
	Magenta = 5, 
	Brown = 6, 
	LightGray = 7,
	DarkGray = 8,
	LightBlue = 9,
	LightGreen = 10,
	LightCyan = 11,
	LightRed = 12,
	Pink = 13,
	Yellow = 14,
	White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] // Instruct the compiler to inherit the size of its datatype (u8 in this case)
struct ColorCode(u8); // Declare the ColorCode type

impl ColorCode { // Actually define the ColorCode's implementation
	fn new(foreground: Color, background: Color) -> ColorCode { // Constructor for the ColorCode struct
		ColorCode((background as u8) << 4 | (foreground as u8))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)] // Tells the compiler to represent this the way C's compiler would (shortcut for compatibility)
struct ScreenChar { // Defines how a character within the VGA buffer is defined:
	ascii_character: u8, // An 8-bit unsigned int ASCII character value...
	color_code: ColorCode, // ... and the color code for said character, being a background and foreground.
}

const BUFFER_HEIGHT: usize = 25; // VGA buffer is 25 rows 
const BUFFER_WIDTH: usize = 80;	// and 80 columns

#[repr(transparent)]
struct Buffer {
	// Defines an abstraction for the VGA buffer. The Volatile wrapper type provides volatile memory access to the buffer directly.
	chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer { // Defines a type with which to write to the VGA buffer, by providing info about its size and the memory address to which it's mapped.
	column_position: usize,
	color_code: ColorCode,
	buffer: &'static mut Buffer, // static, mutable pointer directly to the buffer in memory.
}

impl fmt::Write for Writer { // Implement fmt::Write on the Writer for macro compatibility
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.write_string(s); // Call this struct's usual write_string class
		Ok(()) // Return an OK object, because an fmt::Result is expected.
	}
}

impl Writer {
	// implementation for the Writer struct.
	pub fn write_byte(&mut self, byte: u8) {
		match byte { // Switch statement for the u8 byte to write:
			b'\n' => self.new_line(), // If it's just a newline char, call the newline func
			byte => { // Otherwise...
				if self.column_position >= BUFFER_WIDTH { 
					self.new_line(); // If we're at the end of a line, call newline 
				}
				
				let row = BUFFER_HEIGHT - 1; // Push everything up and set the row to the bottom
				let col = self.column_position;

				let color_code = self.color_code;
				self.buffer.chars[row][col].write(ScreenChar { // Call the write function with a new ScreenChar object!
					ascii_character: byte,
					color_code,
				});
				self.column_position += 1; // Move the cursor one character to the right.
			}
		}
	}

	pub fn write_string(&mut self, s: &str) { // Basically just call the write function over and over.
		for byte in s.bytes(){
			match byte {
				0x20..=0x7e | b'\n' => self.write_byte(byte), // Valid byte
				_ => self.write_byte(0xfe), // Invalid: Write a nothing byte.
			}
		}
	}
	
	pub fn new_line(&mut self) { // New line function
		// Basically, move everything up a row and clear the bottom one for new characters.
		for row in 1..BUFFER_HEIGHT {
			for col in 0..BUFFER_WIDTH {
				let character = self.buffer.chars[row][col].read();
				self.buffer.chars[row - 1][col].write(character);
			}
		}
		self.clear_row(BUFFER_HEIGHT - 1);
		self.column_position = 0;
	 }

	 fn clear_row(&mut self, row: usize) {
	 	// Write ' ' to every single character on a line.
		let blank = ScreenChar {
			ascii_character: b' ',
			color_code: self.color_code,
		};
		for col in 0..BUFFER_WIDTH {
			self.buffer.chars[row][col].write(blank);
		}
	 }
}

use lazy_static::lazy_static; // Necessary to make the compiler wait until everything else has been initialized

lazy_static! {
	// If we don't use lazy_static!, we run into an error because it attempts to initialize WRITER
	// at the very beginning of the program.
	pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
		column_position: 0,
		color_code: ColorCode::new(Color::Pink, Color::Black), // Feel free to change!
		buffer: unsafe { &mut *(0xb8000 as *mut Buffer) } // Use an unsafe block to access memory directly
	});
}

// Define macros to make things easier: println! and print! become shortcuts to write directly to the buffer.
#[macro_export]
macro_rules! print {
	($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
	() => ($crate::print!("\n"));
	($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)] // Hide this from the auto-generated documentation.
pub fn _print(args: fmt::Arguments) {
	use core::fmt::Write;
	WRITER.lock().write_fmt(args).unwrap();
}
