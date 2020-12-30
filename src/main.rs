#![feature(core_intrinsics)]
#![no_std]
#![no_main]

extern crate bootloader;

mod vga_buffer;

use core::intrinsics;
use core::panic::PanicInfo;

#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> !{
    intrinsics::abort()
}

#[no_mangle]
pub fn _start() -> ! {
    use core::fmt::Write; // import the trait to use the macro
    // with spinlock mutex, the lock() method directly returns the reference instead of Result<T, E> since this is a spinlock and it blocks until the lock is available.
    write!(vga_buffer::WRITER.lock(),"Hello, {}, {}", 42, 1.337).unwrap();
    loop{}
}
