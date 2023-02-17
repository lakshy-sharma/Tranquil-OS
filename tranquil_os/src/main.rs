//    Tranquil OS: The simple OS.
//    Copyright (C) 2023  Lakshy Sharma

//    This program is free software; you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation; either version 2 of the License, or
//    (at your option) any later version.

//    This program is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.

//    You should have received a copy of the GNU General Public License along
//    with this program; if not, write to the Free Software Foundation, Inc.,
//    51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

// Since we are no longer using the standard library. 
// We need a custom panic handler.
// So when the time came this function volunteered to do the noble job of handling all code panics.
// Since this function never returns we just capture the inputs from the PanicInfo and mark it as a diverging function using !

// Disabling the standard library and standard rust entrypoints.
// Welcome to the jungle...
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(tranquil_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
use core::panic::PanicInfo;

// Disabling the name mangling done by rust compiler.
// This prevents mangling of the start point. 
// This is required since the linker needs a static entrypoint.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("It is better to do one's own duty imperfectly, than to do someone else's duty even though perfectly. (Bhagvad Gita 18.47)");
    #[cfg(test)]
    test_main();
    loop {}
}
// Adding custom drivers.
mod vga_buffer;
mod serial;

// Normal panic handler.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}

// Handler for test mode.
// This is done to make use of serial ports when any test panics.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    tranquil_os::test_panic_handler(info)
}