// ============================================================
// FastOS — Ring 0/3 Separation + SYSCALL Interface
// ============================================================
// Kernel mode (Ring 0) vs User mode (Ring 3) enforcement.
// SYSCALL/SYSRET via MSRs for fast user→kernel transitions.
// ============================================================

/// Privilege levels
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Ring {
    Kernel = 0,  // Ring 0 — full hardware access
    User = 3,    // Ring 3 — restricted, must use syscalls
}

/// Check if current code is running in kernel mode
pub fn is_kernel_mode() -> bool {
    let cs: u16;
    unsafe {
        core::arch::asm!("mov {:x}, cs", out(reg) cs, options(nomem, nostack));
    }
    (cs & 3) == 0
}

/// Syscall numbers
pub mod syscall {
    pub const SYS_EXIT: u64    = 0;
    pub const SYS_WRITE: u64   = 1;
    pub const SYS_READ: u64    = 2;
    pub const SYS_OPEN: u64    = 3;
    pub const SYS_CLOSE: u64   = 4;
    pub const SYS_FORK: u64    = 5;
    pub const SYS_EXEC: u64    = 6;
    pub const SYS_WAIT: u64    = 7;
    pub const SYS_MMAP: u64    = 8;
    pub const SYS_MUNMAP: u64  = 9;
    pub const SYS_GETPID: u64  = 10;
    pub const SYS_YIELD: u64   = 11;
    pub const SYS_SLEEP: u64   = 12;
}

// ============================================================
// MSR Constants for SYSCALL/SYSRET
// ============================================================

/// IA32_STAR — Segment selectors for SYSCALL/SYSRET
const MSR_STAR: u32 = 0xC000_0081;
/// IA32_LSTAR — RIP for SYSCALL entry (64-bit)
const MSR_LSTAR: u32 = 0xC000_0082;
/// IA32_CSTAR — RIP for SYSCALL entry (compat mode, unused)
const MSR_CSTAR: u32 = 0xC000_0083;
/// IA32_FMASK — RFLAGS mask on SYSCALL
const MSR_FMASK: u32 = 0xC000_0084;
/// IA32_EFER — Extended Feature Enable Register
const MSR_EFER: u32 = 0xC000_0080;

/// EFER bit: System Call Extensions
const EFER_SCE: u64 = 1 << 0;

// ============================================================
// SYSCALL Setup
// ============================================================

/// Initialize SYSCALL/SYSRET mechanism
/// Must be called after GDT is set up.
///
/// GDT layout expected:
///   0x08 = Kernel Code (CS for SYSCALL)
///   0x10 = Kernel Data (SS for SYSCALL)
///   0x1B = User Code (CS for SYSRET, RPL=3)
///   0x23 = User Data (SS for SYSRET, RPL=3)
pub fn init_syscall() {
    unsafe {
        // Enable SCE (System Call Extensions) in EFER
        let efer = rdmsr(MSR_EFER);
        wrmsr(MSR_EFER, efer | EFER_SCE);

        // STAR: bits 47:32 = kernel CS (0x08), bits 63:48 = user CS base (0x18)
        // On SYSCALL: CS = STAR[47:32], SS = STAR[47:32] + 8
        // On SYSRET:  CS = STAR[63:48] + 16 (64-bit), SS = STAR[63:48] + 8
        let star = (0x08u64 << 32) | (0x18u64 << 48);
        wrmsr(MSR_STAR, star);

        // LSTAR: entry point for SYSCALL
        wrmsr(MSR_LSTAR, syscall_entry as u64);

        // FMASK: clear IF (bit 9) on SYSCALL entry (disable interrupts)
        wrmsr(MSR_FMASK, 0x200);
    }
}

/// Read a Model Specific Register
unsafe fn rdmsr(msr: u32) -> u64 {
    let (low, high): (u32, u32);
    core::arch::asm!(
        "rdmsr",
        in("ecx") msr,
        out("eax") low,
        out("edx") high,
        options(nomem, nostack)
    );
    (high as u64) << 32 | low as u64
}

/// Write a Model Specific Register
unsafe fn wrmsr(msr: u32, value: u64) {
    let low = value as u32;
    let high = (value >> 32) as u32;
    core::arch::asm!(
        "wrmsr",
        in("ecx") msr,
        in("eax") low,
        in("edx") high,
        options(nomem, nostack)
    );
}

// ============================================================
// SYSCALL Entry Point
// ============================================================

/// SYSCALL entry — called when user code executes SYSCALL instruction
/// Convention: RAX = syscall number, RDI/RSI/RDX/R10/R8/R9 = args
/// RCX = return RIP (saved by CPU), R11 = return RFLAGS (saved by CPU)
#[naked]
unsafe extern "C" fn syscall_entry() {
    core::arch::asm!(
        // Save user stack pointer, switch to kernel stack
        // For now, just call the dispatcher directly (kernel-only processes)
        "push rcx",         // Save return RIP
        "push r11",         // Save return RFLAGS
        "push rbp",
        "push rbx",
        "push r12",
        "push r13",
        "push r14",
        "push r15",
        // Call Rust dispatcher: rax=syscall#, rdi/rsi/rdx/r10=args
        "mov rcx, r10",     // Linux convention: arg4 in R10, but C ABI wants RCX
        "call {dispatcher}",
        // Restore callee-saved registers
        "pop r15",
        "pop r14",
        "pop r13",
        "pop r12",
        "pop rbp",
        "pop rbx",
        "pop r11",          // Restore RFLAGS
        "pop rcx",          // Restore return RIP
        "sysretq",
        dispatcher = sym syscall_dispatch,
        options(noreturn)
    );
}

// ============================================================
// SYSCALL Dispatcher
// ============================================================

/// Syscall result type
pub type SyscallResult = i64;

/// Dispatch a syscall by number
/// RAX = syscall number, RDI = arg1, RSI = arg2, RDX = arg3, RCX = arg4
#[no_mangle]
extern "C" fn syscall_dispatch(
    arg1: u64,  // RDI
    arg2: u64,  // RSI
    arg3: u64,  // RDX
    arg4: u64,  // RCX (was R10)
) -> SyscallResult {
    // RAX was used for syscall number — we need to read it
    let syscall_num: u64;
    unsafe {
        core::arch::asm!("", out("rax") syscall_num, options(nomem, nostack));
    }

    match syscall_num {
        syscall::SYS_EXIT => sys_exit(arg1 as i32),
        syscall::SYS_WRITE => sys_write(arg1 as u32, arg2 as *const u8, arg3 as usize),
        syscall::SYS_READ => sys_read(arg1 as u32, arg2 as *mut u8, arg3 as usize),
        syscall::SYS_OPEN => 0,   // TODO: implement with VFS
        syscall::SYS_CLOSE => 0,  // TODO: implement with VFS
        syscall::SYS_FORK => -1,  // TODO: implement with process manager
        syscall::SYS_EXEC => -1,  // TODO: implement with process manager
        syscall::SYS_WAIT => -1,  // TODO: implement with process manager
        syscall::SYS_MMAP => -1,  // TODO: implement with memory manager
        syscall::SYS_MUNMAP => -1, // TODO: implement with memory manager
        syscall::SYS_GETPID => sys_getpid(),
        syscall::SYS_YIELD => sys_yield(),
        syscall::SYS_SLEEP => sys_sleep(arg1),
        _ => -1, // Unknown syscall
    }
}

// ============================================================
// Syscall Implementations
// ============================================================

/// SYS_EXIT: terminate current process
fn sys_exit(_code: i32) -> SyscallResult {
    // For now, just halt (single-process kernel)
    loop {
        unsafe { core::arch::asm!("hlt", options(nomem, nostack)); }
    }
}

/// SYS_WRITE: write to a file descriptor (fd=1 → VGA console)
fn sys_write(fd: u32, buf: *const u8, len: usize) -> SyscallResult {
    if fd == 1 || fd == 2 {
        // stdout/stderr → VGA text output
        // Safety: user must provide valid pointer (no MMU check yet)
        for i in 0..len {
            let ch = unsafe { *buf.add(i) };
            // Direct VGA write would go here
            let _ = ch;
        }
        return len as SyscallResult;
    }
    -1 // Invalid fd
}

/// SYS_READ: read from a file descriptor (fd=0 → keyboard)
fn sys_read(fd: u32, _buf: *mut u8, _len: usize) -> SyscallResult {
    if fd == 0 {
        // stdin → keyboard (would block until input)
        return 0;
    }
    -1 // Invalid fd
}

/// SYS_GETPID: return current process ID
fn sys_getpid() -> SyscallResult {
    0 // Kernel process for now
}

/// SYS_YIELD: voluntarily yield CPU
fn sys_yield() -> SyscallResult {
    // Would call scheduler::yield_current() when linked
    0
}

/// SYS_SLEEP: sleep for N milliseconds
fn sys_sleep(_ms: u64) -> SyscallResult {
    // Would call scheduler::sleep_ticks() when linked
    0
}
