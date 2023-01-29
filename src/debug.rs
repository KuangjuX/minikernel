use crate::mm::{ PageTable, VirtPageNum };
use crate::config::TRAP_CONTEXT;
use crate::trap::TrapContext;


impl PageTable {
    pub fn print_trap_context(&self) {
        let trap_ctx_ppn = self.translate(VirtPageNum::from(TRAP_CONTEXT >> 12)).unwrap().ppn().0;
        println!("[kernel] trap ctx ppn: {:#x}", trap_ctx_ppn);
        unsafe{
            let trap_ctx = &*((trap_ctx_ppn << 12) as *const TrapContext);
            for i in 0..trap_ctx.x.len() {
                println!("[kernel] x{} -> {:#x}", i, trap_ctx.x[i]);
            }
            println!("[kernel] sepc -> {:#x}", trap_ctx.sepc);
            println!("[kernel] sstatus -> {:#x}", trap_ctx.sstatus.bits());
        }
    }
}