// ============================================================
// FastOS — Scheduler
// ============================================================
// Round-robin preemptive scheduler.
// Timer IRQ triggers context switch every N ticks.
// ============================================================

use crate::kernel_core::process::{self, Pid, ProcessState, ThreadState, MAX_PROCESSES};

/// Scheduler time slice in ticks (10ms at 1000Hz)
const TIME_SLICE: u64 = 10;

/// Scheduler state
static mut SCHEDULER_ENABLED: bool = false;
static mut TICKS_REMAINING: u64 = TIME_SLICE;
static mut TOTAL_SWITCHES: u64 = 0;

/// Initialize the scheduler
pub fn init() {
    process::init();
    unsafe {
        TICKS_REMAINING = TIME_SLICE;
        SCHEDULER_ENABLED = false;
        TOTAL_SWITCHES = 0;
    }
}

/// Enable the scheduler (call after all init is done)
pub fn enable() {
    unsafe { SCHEDULER_ENABLED = true; }
}

/// Disable the scheduler
pub fn disable() {
    unsafe { SCHEDULER_ENABLED = false; }
}

/// Check if scheduler is enabled
pub fn is_enabled() -> bool {
    unsafe { SCHEDULER_ENABLED }
}

/// Called from timer IRQ — decrement time slice and switch if needed
pub fn timer_tick() {
    unsafe {
        if !SCHEDULER_ENABLED { return; }

        if TICKS_REMAINING > 0 {
            TICKS_REMAINING -= 1;
            return;
        }

        // Time slice expired — schedule next process
        TICKS_REMAINING = TIME_SLICE;
        schedule();
    }
}

/// Select next process to run (round-robin)
pub fn schedule() {
    unsafe {
        let current = process::current_pid();

        // Find next ready process after current
        let mut next_pid: Option<Pid> = None;
        let mut checked = 0u32;
        let mut candidate = current + 1;

        while checked < MAX_PROCESSES as u32 {
            if candidate >= MAX_PROCESSES as u32 {
                candidate = 0;
            }

            if let Some(proc) = process::get_process(candidate) {
                if proc.state == ProcessState::Ready {
                    next_pid = Some(candidate);
                    break;
                }
            }

            candidate += 1;
            checked += 1;
        }

        // If no other process is ready, keep running current
        let next = match next_pid {
            Some(pid) => pid,
            None => return,
        };

        if next == current { return; }

        // Perform context switch
        context_switch(current, next);
    }
}

/// Perform a context switch from `from` to `to`
fn context_switch(from: Pid, to: Pid) {
    unsafe {
        // Get pointers to both contexts
        let old_ctx: *mut process::CpuContext;
        let new_ctx: *const process::CpuContext;

        // Mark old process as Ready (if it was Running)
        if let Some(proc) = process::get_process_mut(from) {
            if proc.state == ProcessState::Running {
                proc.state = ProcessState::Ready;
            }
            if proc.thread_count > 0 && proc.threads[0].state == ThreadState::Running {
                proc.threads[0].state = ThreadState::Ready;
            }
            old_ctx = &mut proc.threads[0].context as *mut process::CpuContext;
        } else {
            return;
        }

        // Mark new process as Running
        if let Some(proc) = process::get_process_mut(to) {
            proc.state = ProcessState::Running;
            if proc.thread_count > 0 {
                proc.threads[0].state = ThreadState::Running;
            }
            new_ctx = &proc.threads[0].context as *const process::CpuContext;
        } else {
            return;
        }

        process::set_current(to);
        TOTAL_SWITCHES += 1;

        // Actual register save/restore via inline assembly
        // CpuContext layout (offsets in bytes):
        //   0x00: rax, 0x08: rbx, 0x10: rcx, 0x18: rdx
        //   0x20: rsi, 0x28: rdi, 0x30: rbp, 0x38: rsp
        //   0x40: r8,  0x48: r9,  0x50: r10, 0x58: r11
        //   0x60: r12, 0x68: r13, 0x70: r14, 0x78: r15
        //   0x80: rip, 0x88: rflags
        switch_context(old_ctx, new_ctx);
    }
}

/// Low-level context switch: save callee-saved regs to old, restore from new
#[inline(never)]
unsafe fn switch_context(old: *mut process::CpuContext, new: *const process::CpuContext) {
    core::arch::asm!(
        // Save callee-saved registers to old context
        "mov [rdi + 0x08], rbx",
        "mov [rdi + 0x30], rbp",
        "mov [rdi + 0x38], rsp",
        "mov [rdi + 0x60], r12",
        "mov [rdi + 0x68], r13",
        "mov [rdi + 0x70], r14",
        "mov [rdi + 0x78], r15",
        // Save return address as RIP
        "lea rax, [rip + 2f]",
        "mov [rdi + 0x80], rax",
        // Save flags
        "pushfq",
        "pop rax",
        "mov [rdi + 0x88], rax",

        // Restore callee-saved registers from new context
        "mov rbx, [rsi + 0x08]",
        "mov rbp, [rsi + 0x30]",
        "mov rsp, [rsi + 0x38]",
        "mov r12, [rsi + 0x60]",
        "mov r13, [rsi + 0x68]",
        "mov r14, [rsi + 0x70]",
        "mov r15, [rsi + 0x78]",
        // Restore flags
        "mov rax, [rsi + 0x88]",
        "push rax",
        "popfq",
        // Jump to saved RIP
        "mov rax, [rsi + 0x80]",
        "jmp rax",

        // Return label — this is where we resume after being switched back
        "2:",
        in("rdi") old,
        in("rsi") new,
        // Clobber everything that might be modified
        out("rax") _,
        out("rcx") _,
        out("rdx") _,
        out("r8") _,
        out("r9") _,
        out("r10") _,
        out("r11") _,
    );
}

/// Yield current process voluntarily
pub fn yield_current() {
    unsafe {
        if !SCHEDULER_ENABLED { return; }
        TICKS_REMAINING = 0;
        schedule();
    }
}

/// Block current process (e.g., waiting for I/O)
pub fn block_current() {
    if let Some(proc) = process::get_process_mut(process::current_pid()) {
        proc.state = ProcessState::Blocked;
    }
    schedule();
}

/// Unblock a process by PID
pub fn unblock(pid: Pid) {
    if let Some(proc) = process::get_process_mut(pid) {
        if proc.state == ProcessState::Blocked {
            proc.state = ProcessState::Ready;
        }
    }
}

/// Sleep current process for N ticks
pub fn sleep_ticks(ticks: u64) {
    // Simple implementation: busy-wait with yields
    let target = crate::kernel_core::interrupts::get_ticks() + ticks;
    while crate::kernel_core::interrupts::get_ticks() < target {
        yield_current();
    }
}

/// Get total context switches
pub fn total_switches() -> u64 {
    unsafe { TOTAL_SWITCHES }
}

/// Get number of ready processes
pub fn ready_count() -> usize {
    unsafe {
        let mut count = 0;
        for pid in 0..MAX_PROCESSES as u32 {
            if let Some(proc) = process::get_process(pid) {
                if proc.state == ProcessState::Ready || proc.state == ProcessState::Running {
                    count += 1;
                }
            }
        }
        count
    }
}
