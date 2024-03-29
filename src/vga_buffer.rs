use core::fmt;
use volatile::Volatile;
use core::ptr::Unique;
use spin::Mutex;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Colour {
    Black       = 0,
    Blue        = 1,
    Green       = 2,
    Cyan        = 3,
    Red         = 4,
    Magenta     = 5,
    Brown       = 6,
    LightGrey   = 7,
    DarkGrey    = 8,
    LightBlue   = 9,
    LightGreen  = 10,
    LightCyan   = 11,
    LightRed    = 12,
    Pink        = 13,
    Yellow      = 14,
    White       = 15,
}

#[derive(Debug, Clone, Copy)]
struct ColourCode(u8);

impl ColourCode {
    const fn new(foreground: Colour, background: Colour) -> ColourCode {
        ColourCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    colour_code: ColourCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    colour_code: ColourCode,
    buffer: Unique<Buffer>,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT -1;
                let col = self.column_position;

                let colour_code = self.colour_code;
                self.buffer().chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    colour_code: colour_code,
                });
                self.column_position += 1;
            }
        }
    }

    pub fn write_str(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte)
        }
    }

    fn buffer(&mut self) -> &mut Buffer {
        unsafe{ self.buffer.as_mut() }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let buffer = self.buffer();
                let ch = buffer.chars[row][col].read();
                buffer.chars[row-1][col].write(ch);
            }
        }
        self.clear_row(BUFFER_HEIGHT-1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            colour_code: self.colour_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer().chars[row][col].write(blank);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s);
        Ok(())
    }
}

pub static WRITER: Mutex<Writer> = Mutex::new(Writer {
    column_position: 0,
    colour_code: ColourCode::new(Colour::LightGreen, Colour::Black),
    buffer: unsafe { Unique::new_unchecked(0xb8000 as *mut _) },
});

// Define some useful macros
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::vga_buffer::print(format_args!($($arg)*));
    });
}

pub fn clear_screen() {
    for _ in 0..BUFFER_HEIGHT {
        println!("");
    }
}

pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
