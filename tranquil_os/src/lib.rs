#![no_std]

#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
pub mod serial;
pub mod vga_buffer;
pub mod interrupts;
pub mod gdt;

// Central init controller for the kernel.
pub fn init() {
    interrupts::init_idt()
}

// Entrypoint handler for our test function.
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("It is better to do one's own duty imperfectly, than to do someone else's duty even though perfectly. (Bhagvad Gita 18.47)");

    // Start the init routines.
    init();
    test_main();
    loop {}
}

// Panic handler for our test cases.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

// The main test runner.
// This function captures al the test cases and test them.
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success)
}


// Basic test cases.
// Sanity testing.
#[test_case]
fn sanity_check() {
    assert_eq!(1, 1);
}

// Test single line output.
#[test_case]
fn test_println_single() {
    println!("test_println_simple output");
}

// Test multiline outputs.
#[test_case]
fn test_println_bulk() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}

// Qemu Serial Communications for Tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    // Write the exit code into the qemu serial port and ask it to close with a code.
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

// Implementations and Traits for testing.
pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T where  T: Fn() {
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}
