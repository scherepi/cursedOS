#![no_std] //IDK WHY IT KEEPS ON UNDERLINING NO_STD BUT ITS WORKING FINE 
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
pub mod gdt;
pub mod interrupts;
pub mod vga_buffer;
pub mod memory;

pub mod users;

pub mod shell;

extern crate alloc;
pub mod allocator;

pub fn init() {
	gdt::init();
	interrupts::init_idt();
	unsafe { interrupts::PICS.lock().initialize() };
	x86_64::instructions::interrupts::enable();
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode { Success = 0x10, Failed = 0x11}
pub fn exit_qemu(exit_code: QemuExitCode) -> ! {
    use x86_64::instructions::port::Port;
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
    unreachable!()
}

pub fn hlt_loop() -> ! {
	loop {
		x86_64::instructions::hlt();
	}
}
