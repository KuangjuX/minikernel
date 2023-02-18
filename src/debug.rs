use crate::mm::{ PageTable, VirtPageNum, PageTableEntry };
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

    pub fn print_page_table(&self) {
        let root_pte_array = self.root_ppn().get_pte_array();
        println!("[kernel] print page table: ");
        print_page_table(root_pte_array, 3);
    }
}

pub fn print_page_table(pte_array: &[PageTableEntry], level: u8) {
    if level == 0 { return; }
    for i in 0..512 {
        let pte = pte_array[i];
        if pte.is_valid() {
            for _ in 0..(3 - level) {
                print!("  ");
            }
            println!("{}: {:#x} {:?}", i, pte.ppn().0, pte.flags());
        }
        if pte.is_valid() {
            assert!(level != 0);
            let pte_array = pte.ppn().get_pte_array();
            print_page_table(pte_array, level - 1);
        }
    }
}