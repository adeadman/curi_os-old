#![feature(lang_items)] // required to define the panic handler
#![feature(const_fn)]   // allow constant functions
#![feature(ptr_internals)]     // allow unique pointers

#![no_std]  // Disable the standard library
#![no_main]  // Disable the usual binary entry points

extern crate rlibc;
extern crate spin;
extern crate volatile;

#[macro_use]
mod vga_buffer;

#[lang = "panic_fmt"]  // On panic, this function should be called
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
    _file: &'static str, _line: u32, _column: u32) -> !
{
    loop {}
}

#[no_mangle]
pub fn _start() -> ! {
    // the linker looks for a function named `_start` by default as the entry

    vga_buffer::clear_screen();
    println!("Hello World{}", "!");
    println!("Hello {} from inside this OS", "Aaron");
    loop {}
}
