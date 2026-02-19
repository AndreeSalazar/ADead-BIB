// ============================================================
// FastOS — Process Management
// ============================================================
// Process and thread structures for multitasking.
// Each process has its own address space, PID, and state.
// ============================================================

/// Maximum number of concurrent processes
pub const MAX_PROCESSES: usize = 64;

/// Maximum threads per process
pub const MAX_THREADS: usize = 4;

/// Process ID type
pub type Pid = u32;

/// Process states
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProcessState {
    Created,
    Ready,
    Running,
    Blocked,
    Sleeping,
    Zombie,
    Dead,
}

/// Thread states
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThreadState {
    Ready,
    Running,
    Blocked,
    Dead,
}

/// CPU register context (saved/restored on context switch)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CpuContext {
    // General purpose registers
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rsp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    // Instruction pointer and flags
    pub rip: u64,
    pub rflags: u64,
    // Segment registers
    pub cs: u64,
    pub ss: u64,
}

impl CpuContext {
    pub const fn empty() -> Self {
        CpuContext {
            rax: 0, rbx: 0, rcx: 0, rdx: 0,
            rsi: 0, rdi: 0, rbp: 0, rsp: 0,
            r8: 0, r9: 0, r10: 0, r11: 0,
            r12: 0, r13: 0, r14: 0, r15: 0,
            rip: 0, rflags: 0x202, // IF set
            cs: 0x08, ss: 0x10,    // Kernel code/data segments
        }
    }

    /// Create a context for a new kernel thread
    pub fn new_kernel(entry: u64, stack_top: u64) -> Self {
        let mut ctx = Self::empty();
        ctx.rip = entry;
        ctx.rsp = stack_top;
        ctx.rbp = stack_top;
        ctx.cs = 0x08;  // Kernel code segment
        ctx.ss = 0x10;  // Kernel data segment
        ctx
    }

    /// Create a context for a new user thread
    pub fn new_user(entry: u64, stack_top: u64) -> Self {
        let mut ctx = Self::empty();
        ctx.rip = entry;
        ctx.rsp = stack_top;
        ctx.rbp = stack_top;
        ctx.cs = 0x1B;  // User code segment (RPL=3)
        ctx.ss = 0x23;  // User data segment (RPL=3)
        ctx
    }
}

/// Thread structure
#[derive(Clone, Copy)]
pub struct Thread {
    pub id: u32,
    pub state: ThreadState,
    pub context: CpuContext,
    pub stack_base: u64,
    pub stack_size: usize,
    pub kernel_stack: u64,
}

impl Thread {
    pub const fn empty() -> Self {
        Thread {
            id: 0,
            state: ThreadState::Dead,
            context: CpuContext::empty(),
            stack_base: 0,
            stack_size: 0,
            kernel_stack: 0,
        }
    }
}

/// Process structure
pub struct Process {
    pub pid: Pid,
    pub name: [u8; 32],
    pub name_len: usize,
    pub state: ProcessState,
    pub parent_pid: Pid,
    pub priority: u8,
    pub threads: [Thread; MAX_THREADS],
    pub thread_count: usize,
    pub page_table: u64,       // CR3 value (PML4 physical address)
    pub cpu_time: u64,         // Total CPU ticks consumed
    pub start_time: u64,       // Tick when process was created
    pub exit_code: i32,
}

impl Process {
    pub const fn empty() -> Self {
        Process {
            pid: 0,
            name: [0; 32],
            name_len: 0,
            state: ProcessState::Dead,
            parent_pid: 0,
            priority: 5,
            threads: [Thread::empty(); MAX_THREADS],
            thread_count: 0,
            page_table: 0,
            cpu_time: 0,
            start_time: 0,
            exit_code: 0,
        }
    }

    /// Set process name
    pub fn set_name(&mut self, name: &str) {
        let bytes = name.as_bytes();
        let len = if bytes.len() > 32 { 32 } else { bytes.len() };
        self.name[..len].copy_from_slice(&bytes[..len]);
        self.name_len = len;
    }

    /// Check if process is alive
    pub fn is_alive(&self) -> bool {
        matches!(self.state, ProcessState::Created | ProcessState::Ready | ProcessState::Running | ProcessState::Blocked | ProcessState::Sleeping)
    }
}

// ============================================================
// Process Table (global)
// ============================================================

static mut PROCESS_TABLE: [Process; MAX_PROCESSES] = {
    const EMPTY: Process = Process::empty();
    [EMPTY; MAX_PROCESSES]
};

static mut NEXT_PID: Pid = 1;
static mut CURRENT_PID: Pid = 0;

/// Initialize process management
pub fn init() {
    // Create PID 0: the kernel idle process
    unsafe {
        PROCESS_TABLE[0].pid = 0;
        PROCESS_TABLE[0].set_name("kernel_idle");
        PROCESS_TABLE[0].state = ProcessState::Running;
        PROCESS_TABLE[0].priority = 0;
        PROCESS_TABLE[0].thread_count = 1;
        PROCESS_TABLE[0].threads[0].id = 0;
        PROCESS_TABLE[0].threads[0].state = ThreadState::Running;
        CURRENT_PID = 0;
    }
}

/// Create a new kernel process
pub fn create_kernel_process(name: &str, entry: u64, stack_top: u64) -> Option<Pid> {
    unsafe {
        // Find free slot
        for i in 1..MAX_PROCESSES {
            if PROCESS_TABLE[i].state == ProcessState::Dead {
                let pid = NEXT_PID;
                NEXT_PID += 1;

                PROCESS_TABLE[i].pid = pid;
                PROCESS_TABLE[i].set_name(name);
                PROCESS_TABLE[i].state = ProcessState::Ready;
                PROCESS_TABLE[i].parent_pid = CURRENT_PID;
                PROCESS_TABLE[i].priority = 5;
                PROCESS_TABLE[i].thread_count = 1;
                PROCESS_TABLE[i].cpu_time = 0;
                PROCESS_TABLE[i].start_time = crate::kernel_core::interrupts::get_ticks();
                PROCESS_TABLE[i].exit_code = 0;

                // Set up main thread
                PROCESS_TABLE[i].threads[0].id = 0;
                PROCESS_TABLE[i].threads[0].state = ThreadState::Ready;
                PROCESS_TABLE[i].threads[0].context = CpuContext::new_kernel(entry, stack_top);
                PROCESS_TABLE[i].threads[0].stack_base = stack_top - 4096;
                PROCESS_TABLE[i].threads[0].stack_size = 4096;

                return Some(pid);
            }
        }
        None
    }
}

/// Kill a process by PID
pub fn kill(pid: Pid) -> bool {
    unsafe {
        for i in 0..MAX_PROCESSES {
            if PROCESS_TABLE[i].pid == pid && PROCESS_TABLE[i].is_alive() {
                PROCESS_TABLE[i].state = ProcessState::Zombie;
                PROCESS_TABLE[i].exit_code = -1;
                // Mark all threads as dead
                for t in 0..PROCESS_TABLE[i].thread_count {
                    PROCESS_TABLE[i].threads[t].state = ThreadState::Dead;
                }
                return true;
            }
        }
        false
    }
}

/// Get current running PID
pub fn current_pid() -> Pid {
    unsafe { CURRENT_PID }
}

/// Set current running PID
pub fn set_current(pid: Pid) {
    unsafe { CURRENT_PID = pid; }
}

/// Get process by PID
pub fn get_process(pid: Pid) -> Option<&'static Process> {
    unsafe {
        for i in 0..MAX_PROCESSES {
            if PROCESS_TABLE[i].pid == pid && PROCESS_TABLE[i].is_alive() {
                return Some(&PROCESS_TABLE[i]);
            }
        }
        None
    }
}

/// Get mutable process by PID
pub fn get_process_mut(pid: Pid) -> Option<&'static mut Process> {
    unsafe {
        for i in 0..MAX_PROCESSES {
            if PROCESS_TABLE[i].pid == pid && PROCESS_TABLE[i].is_alive() {
                return Some(&mut PROCESS_TABLE[i]);
            }
        }
        None
    }
}

/// Count active processes
pub fn active_count() -> usize {
    unsafe {
        PROCESS_TABLE.iter().filter(|p| p.is_alive()).count()
    }
}

/// Reap zombie processes (free their slots)
pub fn reap_zombies() {
    unsafe {
        for i in 0..MAX_PROCESSES {
            if PROCESS_TABLE[i].state == ProcessState::Zombie {
                PROCESS_TABLE[i].state = ProcessState::Dead;
            }
        }
    }
}
