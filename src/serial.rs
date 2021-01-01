use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;
use core::fmt;

// use lazy static to create a static port writer
// same as VGA, but now for a port instead
lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

//Macros for Serial printing

#[doc(hidden)]
pub fn _print(args: fmt::Arguments){
    use fmt::Write;
    SERIAL1.lock().write_fmt(args).expect("Printing to serial failed");
}

// Print to host through serial I/O
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

// Print to host through serial I/O with a newline
#[macro_export]
macro_rules! serial_println {
   () => ($crate::serial_print!("\n"));
   ($fmt:expr) => ($crate::serial_print!(concat!($fmt,"\n")));
   ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(concat!($fmt,"\n"), $($arg)*));
}