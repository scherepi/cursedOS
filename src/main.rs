#![no_std] // don't link the Rust standard lib
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
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
mod gdt;
mod interrupts;

extern crate alloc;


use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};

// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	// The _info parameter contains info about the panic's cause.
	println!("{}", _info); // Right now, use the VGA buffer println macro to print panic info.
	cursed_os::hlt_loop(); // Pause processes by creating an infinite loop.
}



entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
	use crate::memory::{BootInfoFrameAllocator};
	use x86_64::VirtAddr;
	println!("Hello World{}", "!");
	cursed_os::init();

	let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
	cursed_os::allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap init failed");

	//uncomment to see the test
    /* let addresses = [
        0xb8000,
        0x201008,
        0x0100_0020_1a10,
        boot_info.physical_memory_offset,
    ];
    for &addr  in &addresses{
        let virt = VirtAddr::new(addr);
        let phys = unsafe { memory::translate_addr(virt, phys_mem_offset) };
        println!("{:?} -> {:?}", virt, phys);
    }*/
	// map unused page
	//let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    //memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);
    // test the mapping by writing to it
    //let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    //unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };


	let heap_value = Box::new(41); //this is what gave us the panic
	println!("heap_value: {:p}", heap_value);

	let mut vec = Vec::new();
	for i in 0..500{
		vec.push(i);
	}
	println!("vec at: {:p}", vec.as_slice());

	let ref_counted = Rc::new(vec![1,2,3]);
	let cloned_ref = ref_counted.clone();
	println!("current reference count is {}", Rc::strong_count(&cloned_ref));
    core::mem::drop(cloned_ref);
    println!("reference count is {} now", Rc::strong_count(&ref_counted));

	






	println!("Didn't crash!!"); // Call our println macro to write to the VGA buffer directly.
	//println!("It is so cool that {}", "this works");
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
	cursed_os::hlt_loop(); // infinite loop to satisfy the `!` return type (i forgor i removed this pls dont remove ts)From lib.rs, calls the HLT function to not do anything unless there's an interrupt.
}