#![no_std]
#![no_main]
use core::panic;
use core::arch::asm;

#[macro_use]
mod sbi;
mod console;


core::arch::global_asm!(include_str!("asm/entry.S"));

#[panic_handler]
fn panic(_info: &panic::PanicInfo<'_>) -> ! {
    loop{}
}

#[no_mangle]
fn abort() -> ! {
    panic!("abort");
}
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
            .fill(0);
    }
}


#[no_mangle]
pub extern "C" fn kmain() {
    clear_bss();
    println!("Hello World");
}