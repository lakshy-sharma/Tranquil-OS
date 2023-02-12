//    Tranquil OS: The simple OS.
//    Copyright (C) 2023  lakshy Sharma

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

// Disabling the standard library and standard rust entrypoints.
// Welcome to the jungle...
#![no_std]
#![no_main]

// Since we are no longer using the standard library. 
// We need a custom panic handler.
// So when the time came this function volunteered to do the noble job of handling all code panics.
// Since this function never returns we just capture the inputs from the PanicInfo and mark it as a diverging function using !
use core::panic::PanicInfo;
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}

// Adding custom drivers.
mod vga_buffer;

// Disabling the name mangling done by rust compiler.
// This prevents mangling of the start point. 
// This is required sicne linker needs a static entrypoint.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("It is better to do one's own duty imperfectly, than to do someone else's duty even though perfectly. (Bhagvad Gita 18.47)");
    loop {}
}