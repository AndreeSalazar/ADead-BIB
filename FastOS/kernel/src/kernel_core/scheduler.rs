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
        // Mark old process as Ready (if it was Running)
        if let Some(proc) = process::get_process_mut(from) {
            if proc.state == ProcessState::Running {
                proc.state = ProcessState::Ready;
            }
            if proc.thread_count > 0 && proc.threads[0].state == ThreadState::Running {
                proc.threads[0].state = ThreadState::Ready;
            }
        }

        // Mark new process as Running
        if let Some(proc) = process::get_process_mut(to) {
            proc.state = ProcessState::Running;
            if proc.thread_count > 0 {
                proc.threads[0].state = ThreadState::Running;
            }
        }

        process::set_current(to);
        TOTAL_SWITCHES += 1;

        // NOTE: Actual register save/restore requires assembly.
        // The context switch stub will be implemented in ADead-BIB
        // or inline assembly when we have real user processes.
        // For now, this handles the state machine correctly.
    }
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
