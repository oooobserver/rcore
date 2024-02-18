use riscv::register::sstatus::{self, Sstatus, SPP};


// Make this structure in memory layout like C
#[repr(C)]
pub struct TrapContext {
    /// general regs[0..31]
    pub x: [usize; 32],
    /// CSR sstatus      
    pub sstatus: Sstatus,
    /// CSR sepc
    pub sepc: usize,
}


impl TrapContext {
    // Set stack pointer 
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }
    // init app context
    pub fn app_init_context(entry: usize, sp: usize) -> Self {
        let mut sstatus = sstatus::read(); 
        // Initialize to user mode
        sstatus.set_spp(SPP::User); //previous privilege mode: user mode
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry, // Return address
        };
        // User stack pointer
        cx.set_sp(sp); 
        cx 
    }
}