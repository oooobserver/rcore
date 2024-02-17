#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod lang_items;
mod sbi;
mod console;

use core::arch::global_asm;
// Allocate boot stack, call entry point
global_asm!(include_str!("entry.asm"));
// Link program
global_asm!(include_str!("link_app.S"));


// The OS entry point
#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    info!("hello shenye!");
    loop {}
}


// Clear the bss region
fn clear_bss(){
    extern "C" {
        fn sbss();
        fn ebss();
    }

    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    }); 
}

