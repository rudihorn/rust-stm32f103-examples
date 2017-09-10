#![no_std]
#![no_main]
#![feature(asm, lang_items, start, naked_functions, used)]

extern crate stm32;

mod std_repl;
mod interrupts;
mod mem;
mod print;
mod system;

use print::{print, println, print_int, enable_uart};
use system::{enable_led, init_clock};


pub static mut INT_DATA: u32 = 5; // static value
pub static mut INT_BSS: u32 = 0; // default value

pub fn main() {
    unsafe { stm32::mem::init_mem() };

    // setup system
    init_clock();
    enable_led();
    enable_uart();

    println("STM32 Memory Test App");
    println("  - by Rudi Horn");

    unsafe { 
        print("int_data: "); print_int(INT_DATA); println("");
        print("int_bss: "); print_int(INT_BSS); println("");

        INT_DATA = 9; INT_BSS = 99; 

        print("int_data: "); print_int(INT_DATA); println("");
        print("int_bss: "); print_int(INT_BSS); println("");
    }

    loop {
    } 
}

