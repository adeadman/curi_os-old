#![feature(lang_items)]  // required to define the panic handler

#![no_std]  // Disable the standard library
#![no_main]  // Disable the usual binary entry points

#[lang = "panic_fmt"]  // On panic, this function should be called
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
    _file: &'static str, _line: u32, _column: u32) -> !
{
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub fn _start() -> ! {
    // the linker looks for a function named `_start` by default as the entry
    let vga_buffer = 0xb8000 as *const u8 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
