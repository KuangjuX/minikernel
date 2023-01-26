//! The main module and entrypoint
//!
//! Various facilities of the kernels are implemented as submodules. The most
//! important ones are:
//!
//! - [`trap`]: Handles all cases of switching from userspace to the kernel
//! - [`task`]: Task management
//! - [`syscall`]: System call handling and implementation
//!
//! The operating system also starts in this module. Kernel code starts
//! executing from `entry.asm`, after which [`rust_main()`] is called to
//! initialize various pieces of functionality. (See its source code for
//! details.)
//!
//! We then call [`task::run_first_task()`] and for the first time go to
//! userspace.

// #![deny(missing_docs)]
// #![deny(warnings)]
#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]


// use crate::config::{PAGE_SIZE, TRAP_CONTEXT};


extern crate alloc;

#[macro_use]
extern crate bitflags;



#[macro_use]
mod console;
mod config;
mod lang_items;
mod loader;
mod mm;
mod sbi;
mod sync;
pub mod syscall;
pub mod task;
mod timer;
pub mod trap;
mod boards;

core::arch::global_asm!(include_str!("asm/entry.asm"));
core::arch::global_asm!(include_str!("link_app.S"));

/// clear BSS segment
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
/// the rust entry-point of os
pub fn rust_main() -> ! {    
    clear_bss();
    println!("[kernel] Hello, world!");
    mm::init();
    println!("[kernel] paging enable......");
    mm::remap_test();
    trap::init();
    println!("[kernel] trap enable......");
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    task::run_first_task();
    // unsafe{ exception_test() };
    panic!("Unreachable in rust_main!");
}

/// 测试地址异常
pub unsafe fn exception_test() {
    core::ptr::read(0x1000 as *const u8);
}