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

#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> !{
    println!("{}",_info);
    loop{}
    //intrinsics::abort()
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

#[cfg(test)]
//only include in a test config
pub fn test_runner(tests: &[&dyn Fn()]){
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion(){
    print!("trivial assertion... ");
    assert_eq!(1,1);
    println!("ok");
}