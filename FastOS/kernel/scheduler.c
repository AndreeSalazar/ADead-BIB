/*
 * FastOS v2.0 — Process Scheduler
 * Round-robin preemptive scheduler
 */

#include "../include/kernel.h"
#include "../include/types.h"

/* ============================================================
 * Process States
 * ============================================================ */

typedef enum {
    PROC_UNUSED = 0,
    PROC_READY,
    PROC_RUNNING,
    PROC_BLOCKED,
    PROC_ZOMBIE,
} proc_state_t;

/* ============================================================
 * CPU Context (saved on context switch)
 * ============================================================ */

typedef struct {
    uint64_t rax, rbx, rcx, rdx;
    uint64_t rsi, rdi, rbp, rsp;
    uint64_t r8, r9, r10, r11;
    uint64_t r12, r13, r14, r15;
    uint64_t rip, rflags;
    uint64_t cs, ss;
    uint64_t cr3;  /* Page table */
} __packed cpu_context_t;

/* ============================================================
 * Process Control Block (PCB)
 * ============================================================ */

#define MAX_PROCESSES 64
#define KERNEL_STACK_SIZE 4096
#define USER_STACK_SIZE   (64 * 1024)

typedef struct {
    uint32_t pid;
    uint32_t ppid;
    proc_state_t state;
    uint8_t priority;
    uint8_t security_level;  /* BG security level */
    
    cpu_context_t context;
    
    uint64_t kernel_stack;
    uint64_t user_stack;
    uint64_t page_table;
    
    uint64_t time_slice;
    uint64_t total_time;
    
    char name[32];
} process_t;

/* ============================================================
 * Scheduler State
 * ============================================================ */

static process_t processes[MAX_PROCESSES];
static uint32_t current_pid = 0;
static uint32_t next_pid = 1;
static uint64_t scheduler_ticks = 0;

#define TIME_SLICE_DEFAULT 10  /* Timer ticks per slice */

/* ============================================================
 * Process Management
 * ============================================================ */

process_t* get_current_process(void) {
    return &processes[current_pid];
}

process_t* get_process(uint32_t pid) {
    if (pid >= MAX_PROCESSES) return NULL;
    if (processes[pid].state == PROC_UNUSED) return NULL;
    return &processes[pid];
}

static int find_free_slot(void) {
    for (int i = 1; i < MAX_PROCESSES; i++) {
        if (processes[i].state == PROC_UNUSED) {
            return i;
        }
    }
    return -1;
}

/* Create new process */
int process_create(const char *name, void (*entry)(void), uint8_t security_level) {
    int slot = find_free_slot();
    if (slot < 0) {
        kprintf("[SCHED] No free process slots\n");
        return -1;
    }
    
    process_t *proc = &processes[slot];
    
    proc->pid = next_pid++;
    proc->ppid = current_pid;
    proc->state = PROC_READY;
    proc->priority = 1;
    proc->security_level = security_level;
    
    /* Copy name */
    int i;
    for (i = 0; i < 31 && name[i]; i++) {
        proc->name[i] = name[i];
    }
    proc->name[i] = '\0';
    
    /* Setup context */
    proc->context.rip = (uint64_t)entry;
    proc->context.rflags = 0x202;  /* IF enabled */
    proc->context.cs = 0x08;       /* Kernel code segment */
    proc->context.ss = 0x10;       /* Kernel data segment */
    
    /* Allocate stacks (simplified - use real allocator) */
    proc->kernel_stack = 0x200000 + slot * KERNEL_STACK_SIZE;
    proc->user_stack = 0x400000 + slot * USER_STACK_SIZE;
    proc->context.rsp = proc->kernel_stack + KERNEL_STACK_SIZE - 8;
    proc->context.rbp = proc->context.rsp;
    
    proc->time_slice = TIME_SLICE_DEFAULT;
    proc->total_time = 0;
    
    kprintf("[SCHED] Created process %d: %s (security=%d)\n", 
            proc->pid, proc->name, proc->security_level);
    
    return proc->pid;
}

/* Terminate process */
void process_exit(int exit_code) {
    process_t *proc = get_current_process();
    if (proc->pid == 0) {
        kernel_panic("Cannot exit kernel process");
    }
    
    kprintf("[SCHED] Process %d exited with code %d\n", proc->pid, exit_code);
    proc->state = PROC_ZOMBIE;
    
    /* Yield to scheduler */
    schedule();
}

/* Block current process */
void process_block(void) {
    process_t *proc = get_current_process();
    proc->state = PROC_BLOCKED;
    schedule();
}

/* Unblock process */
void process_unblock(uint32_t pid) {
    process_t *proc = get_process(pid);
    if (proc && proc->state == PROC_BLOCKED) {
        proc->state = PROC_READY;
    }
}

/* ============================================================
 * Scheduler
 * ============================================================ */

/* Find next runnable process (round-robin) */
static int find_next_process(void) {
    int start = current_pid;
    int next = start;
    
    do {
        next = (next + 1) % MAX_PROCESSES;
        if (processes[next].state == PROC_READY) {
            return next;
        }
    } while (next != start);
    
    /* No ready process, return idle (0) */
    return 0;
}

/* Context switch (simplified - real impl in assembly) */
static void context_switch(process_t *old, process_t *new) {
    if (old == new) return;
    
    /* Save old context would happen here via assembly */
    /* Load new context */
    
    /* Switch page tables if different */
    if (old->page_table != new->page_table && new->page_table != 0) {
        write_cr3(new->page_table);
    }
    
    /* In real implementation, this would be assembly:
     * - Save all registers to old->context
     * - Load all registers from new->context
     * - IRETQ to new process
     */
}

/* Main scheduler function */
void schedule(void) {
    process_t *old = get_current_process();
    
    /* Mark current as ready if it was running */
    if (old->state == PROC_RUNNING) {
        old->state = PROC_READY;
    }
    
    /* Find next process */
    int next = find_next_process();
    process_t *new = &processes[next];
    
    if (new->state != PROC_READY && next != 0) {
        /* Fallback to idle */
        next = 0;
        new = &processes[0];
    }
    
    new->state = PROC_RUNNING;
    new->time_slice = TIME_SLICE_DEFAULT;
    current_pid = next;
    
    context_switch(old, new);
}

/* Timer tick handler */
void scheduler_tick(void) {
    scheduler_ticks++;
    
    process_t *proc = get_current_process();
    proc->total_time++;
    
    if (proc->time_slice > 0) {
        proc->time_slice--;
    }
    
    /* Preempt if time slice expired */
    if (proc->time_slice == 0 && proc->pid != 0) {
        schedule();
    }
}

/* Yield current time slice */
void yield(void) {
    process_t *proc = get_current_process();
    proc->time_slice = 0;
    schedule();
}

/* ============================================================
 * Idle Process
 * ============================================================ */

static void idle_process(void) {
    while (1) {
        hlt();  /* Wait for interrupt */
    }
}

/* ============================================================
 * Initialization
 * ============================================================ */

void scheduler_init(void) {
    kprintf("[SCHED] Initializing scheduler...\n");
    
    /* Clear all process slots */
    for (int i = 0; i < MAX_PROCESSES; i++) {
        processes[i].state = PROC_UNUSED;
        processes[i].pid = 0;
    }
    
    /* Create idle process (PID 0) */
    processes[0].pid = 0;
    processes[0].ppid = 0;
    processes[0].state = PROC_RUNNING;
    processes[0].priority = 255;  /* Lowest priority */
    processes[0].security_level = 0;  /* Kernel level */
    processes[0].context.rip = (uint64_t)idle_process;
    
    const char *name = "idle";
    for (int i = 0; i < 5; i++) {
        processes[0].name[i] = name[i];
    }
    
    current_pid = 0;
    
    kprintf("[SCHED] Scheduler ready (max %d processes)\n", MAX_PROCESSES);
}

/* Debug: list all processes */
void scheduler_list(void) {
    kprintf("\n[SCHED] Process List:\n");
    kprintf("  PID  PPID  STATE     PRIO  SEC  NAME\n");
    kprintf("  ---  ----  --------  ----  ---  ----\n");
    
    for (int i = 0; i < MAX_PROCESSES; i++) {
        if (processes[i].state != PROC_UNUSED) {
            const char *state_str;
            switch (processes[i].state) {
                case PROC_READY:   state_str = "READY   "; break;
                case PROC_RUNNING: state_str = "RUNNING "; break;
                case PROC_BLOCKED: state_str = "BLOCKED "; break;
                case PROC_ZOMBIE:  state_str = "ZOMBIE  "; break;
                default:           state_str = "UNKNOWN "; break;
            }
            kprintf("  %3d  %4d  %s  %4d  %3d  %s\n",
                    processes[i].pid,
                    processes[i].ppid,
                    state_str,
                    processes[i].priority,
                    processes[i].security_level,
                    processes[i].name);
        }
    }
}
