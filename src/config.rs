//! Constants used in rCore


//! Constants used in rCore
pub const USER_STACK_SIZE: usize = 4096 * 4;
pub const KERNEL_STACK_SIZE: usize = 4096 * 2;
pub const KERNEL_HEAP_SIZE: usize = 0x20_0000;

pub const PAGE_SIZE: usize = 0x1000;
pub const PAGE_SIZE_BITS: usize = 0xc;

/// 使用 xv6-riscv 的地址空间
pub const MAX_VA: usize = 1 << (9 + 9 + 9 + 12 - 1);
pub const TRAMPOLINE: usize = MAX_VA - PAGE_SIZE;
pub const TRAP_CONTEXT: usize = TRAMPOLINE - PAGE_SIZE;


pub use crate::boards::{CLOCK_FREQ, MMIO, MEMORY_END};
