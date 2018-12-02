#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]

use core::panic::PanicInfo;
use haemo::println;

//static HELLO: &[u8] = b"Hello World!";

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    /* let vga_buffer = 0xb8000 as *mut u8;
    
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }*/
    //vga_buffer::print_something();
    //use core::fmt::Write;
    //vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    //write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();
    println!("Hello World{}", "!");

    haemo::gdt::init();
    haemo::interrupts::init_idt();

    unsafe { haemo::interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    // fn stack_overflow() {
    //     stack_overflow();
    // }

    // stack_overflow();

    println!("It did not crash!");
    haemo::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    haemo::hlt_loop();
}

