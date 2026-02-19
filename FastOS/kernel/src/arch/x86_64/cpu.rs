// ============================================================
// FastOS — x86_64 CPU Utilities
// ============================================================
// CPU control: interrupts, halt, control registers, MSRs.
// ============================================================

/// Disable interrupts (CLI)
#[inline]
pub fn cli() {
    unsafe { core::arch::asm!("cli", options(nomem, nostack)); }
}

/// Enable interrupts (STI)
#[inline]
pub fn sti() {
    unsafe { core::arch::asm!("sti", options(nomem, nostack)); }
}

/// Halt CPU until next interrupt (HLT)
#[inline]
pub fn hlt() {
    unsafe { core::arch::asm!("hlt", options(nomem, nostack)); }
}

/// Halt loop — disable interrupts and halt forever
#[inline]
pub fn halt_loop() -> ! {
    loop {
        cli();
        hlt();
    }
}

/// Read CR0 register
#[inline]
pub fn read_cr0() -> u64 {
    let val: u64;
    unsafe { core::arch::asm!("mov {}, cr0", out(reg) val, options(nomem, nostack)); }
    val
}

/// Write CR0 register
#[inline]
pub fn write_cr0(val: u64) {
    unsafe { core::arch::asm!("mov cr0, {}", in(reg) val, options(nomem, nostack)); }
}

/// Read CR2 register (page fault linear address)
#[inline]
pub fn read_cr2() -> u64 {
    let val: u64;
    unsafe { core::arch::asm!("mov {}, cr2", out(reg) val, options(nomem, nostack)); }
    val
}

/// Read CR3 register (page table base)
#[inline]
pub fn read_cr3() -> u64 {
    let val: u64;
    unsafe { core::arch::asm!("mov {}, cr3", out(reg) val, options(nomem, nostack)); }
    val
}

/// Write CR3 register (switch page tables, flushes TLB)
#[inline]
pub fn write_cr3(val: u64) {
    unsafe { core::arch::asm!("mov cr3, {}", in(reg) val, options(nomem, nostack)); }
}

/// Read CR4 register
#[inline]
pub fn read_cr4() -> u64 {
    let val: u64;
    unsafe { core::arch::asm!("mov {}, cr4", out(reg) val, options(nomem, nostack)); }
    val
}

/// Write CR4 register
#[inline]
pub fn write_cr4(val: u64) {
    unsafe { core::arch::asm!("mov cr4, {}", in(reg) val, options(nomem, nostack)); }
}

/// Read a Model Specific Register (MSR)
#[inline]
pub fn read_msr(msr: u32) -> u64 {
    let (low, high): (u32, u32);
    unsafe {
        core::arch::asm!(
            "rdmsr",
            in("ecx") msr,
            out("eax") low,
            out("edx") high,
            options(nomem, nostack)
        );
    }
    ((high as u64) << 32) | (low as u64)
}

/// Write a Model Specific Register (MSR)
#[inline]
pub fn write_msr(msr: u32, val: u64) {
    let low = val as u32;
    let high = (val >> 32) as u32;
    unsafe {
        core::arch::asm!(
            "wrmsr",
            in("ecx") msr,
            in("eax") low,
            in("edx") high,
            options(nomem, nostack)
        );
    }
}

/// Invalidate TLB entry for a given virtual address
#[inline]
pub fn invlpg(addr: u64) {
    unsafe {
        core::arch::asm!("invlpg [{}]", in(reg) addr, options(nostack));
    }
}

/// Load GDT from GDTR pointer
#[inline]
pub fn lgdt(gdtr: &GdtDescriptor) {
    unsafe {
        core::arch::asm!("lgdt [{}]", in(reg) gdtr as *const _, options(nostack));
    }
}

/// Load IDT from IDTR pointer
#[inline]
pub fn lidt(idtr: &IdtDescriptor) {
    unsafe {
        core::arch::asm!("lidt [{}]", in(reg) idtr as *const _, options(nostack));
    }
}

/// Load Task Register
#[inline]
pub fn ltr(selector: u16) {
    unsafe {
        core::arch::asm!("ltr {:x}", in(reg) selector, options(nostack));
    }
}

/// GDTR/IDTR descriptor (packed 10 bytes: 2-byte limit + 8-byte base)
#[repr(C, packed)]
pub struct GdtDescriptor {
    pub limit: u16,
    pub base: u64,
}

#[repr(C, packed)]
pub struct IdtDescriptor {
    pub limit: u16,
    pub base: u64,
}

/// CPUID result
pub struct CpuidResult {
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
}

/// Execute CPUID instruction
#[inline]
pub fn cpuid(leaf: u32) -> CpuidResult {
    let (eax, ebx, ecx, edx): (u32, u32, u32, u32);
    unsafe {
        core::arch::asm!(
            "push rbx",
            "cpuid",
            "mov esi, ebx",
            "pop rbx",
            inout("eax") leaf => eax,
            out("esi") ebx,
            out("ecx") ecx,
            out("edx") edx,
            options(nostack)
        );
    }
    CpuidResult { eax, ebx, ecx, edx }
}
