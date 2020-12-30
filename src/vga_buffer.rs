use volatile::Volatile;

use core::fmt;

#[allow(dead_code)]
#[derive(Debug,Clone,Copy, PartialEq, Eq)]
#[repr(u8)]
// C-style enum representing the values for the various colors supported
pub enum Color {
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
#[repr(transparent)] // ensures u8 data layout
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        // The color code is represented as a set of 8 bits which are the MSB for a 16 bit number
        // Background color is represented by 4 bits 12-14 (for the color code, 4-7)
        // Foreground color is represented by 4 bits 8-11 (for the color code, 0-3)
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)] // ensures struct field order (C behaviour)
struct ScreenChar {
    ascii_character : u8,
    color_code: ColorCode,
}

// VGA Grid
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)] // ensures same memory layout
struct Buffer{
    // represents the VGA Grid
    // Volatile avoid optimizing of character writing which can skip writes
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize, // current column
    color_code: ColorCode, // color code for the current buffer
    buffer: &'static mut Buffer, // valid for whole program run time due to static lifetime
}

impl Writer{
    pub fn write_byte(&mut self, byte: u8){
        match byte {
            b'\n' => self.new_line(), // if the char is newline, add a new line
            byte => { // any other character
                if self.column_position >= BUFFER_WIDTH {
                    // if the column is beyond the width of the buffer, move to the next line
                    self.new_line();
                }

                let row = BUFFER_HEIGHT -1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar{
                    ascii_character: byte,
                    color_code,
                });
                // avoids compiler optimization from messing this up
                self.column_position+=1; // move to the next column
            }
        }
    }

    fn new_line(&mut self){
        for row in 1..BUFFER_HEIGHT {
            for col in 1..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read(); // use read because of the Volatile buffer
                self.buffer.chars[row-1][col].write(character); // move the current line up by 1
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize){
        let blank = ScreenChar{
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH{
            self.buffer.chars[row][col].write(blank); // write 'space' into all the columns in a row to erase the row's contents
        }
    }

    pub fn write_string(&mut self,s: &str){
        // write a whole string
        for byte in s.bytes(){
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte), // check if the character is a valid printable character
                _ => self.write_byte(0xfe), // if not, print a 'box' character onto the screen
            }
        }
    }
}

// implementing the fmt::Write trait allows the use of the Rust write! formatting macro
impl fmt::Write for Writer{
    fn write_str(&mut self,s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

// use static to allow global use of WRITER.
// this is a problem because statics are initialized at compile time and the raw pointer can't be converted to a reference at this time.
// use a lazy static to workaround the issue - initialized at first access.
use lazy_static::lazy_static;
// the workaround now makes it hard to use this WRITER because the reference is immutable and using a mutable static is bad.
// the compiler won't be happy about this at all, but we need interior mutability.
// we can fix this by using locks.
// GOTCHA! We're building the kernel so there are currently no OS Supported Locking features available to us.
// The solution is to use spinlocks which don't need any OS support. 
use spin::Mutex;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer{
        column_position:0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe{&mut *(0xb8000 as *mut Buffer)},
    });
}