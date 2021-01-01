#![feature(core_intrinsics)]
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::intrinsics;
use core::panic::PanicInfo;

extern crate bootloader;

mod vga_buffer;
mod serial;

#[cfg(not(test))] //used when not a test
#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> !{
    println!("{}",_info);
    loop{}
}

#[cfg(test)] //used when in test mode
#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> !{
    serial_println!("FAILED");
    serial_println!("Error: {}",_info);
    exit_qemu(QemuExitCode::Failed);
    loop{}
}

#[no_mangle]
pub fn _start() -> ! {
    //use core::fmt::Write; // import the trait to use the macro
    // with spinlock mutex, the lock() method directly returns the reference instead of Result<T, E> since this is a spinlock and it blocks until the lock is available.
    //write!(vga_buffer::WRITER.lock(),"Hello, {}, {}", 42, 1.337).unwrap();
    
    println!("Hello world!");

    #[cfg(test)]
    test_main();

    loop{}
    
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}


pub fn exit_qemu(exit_code: QemuExitCode){
    // use Port mapped I/O to exit Qemu
    // 0xf4 is the port that is defined in the config
    use x86_64::instructions::port::Port;
    unsafe{
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[cfg(test)]
//only include in a test config
pub fn test_runner(tests: &[&dyn Fn()]){
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion(){
    serial_println!("trivial assertion... ");
    assert_eq!(1,0);
    serial_println!("ok");
}

