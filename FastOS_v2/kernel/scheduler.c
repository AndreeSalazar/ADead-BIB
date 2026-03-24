/*
 * FastOS v2.0 — Preemptive Round-Robin Scheduler
 * Manages processes with context switching
 * Timer IRQ0 triggers scheduler_tick() for preemption
 *
 * PCB includes full register state for context switch.
 * Each process gets a 16KB kernel stack.
 */

#include "include/kernel.h"

#define MAX_PROCESSES    64
#define KERNEL_STACK_SZ  (16 * 1024)
#define TIME_SLICE       10   /* Timer ticks per time slice */

typedef enum {
    PROC_FREE = 0,
    PROC_READY,
    PROC_RUNNING,
    PROC_BLOCKED,
    PROC_ZOMBIE
} proc_state_t;

typedef struct {
    uint64_t rsp;   /* Saved stack pointer — only need this for context switch */
} cpu_ctx_t;

typedef struct {
    uint32_t     pid;
    proc_state_t state;
    char         name[32];
    cpu_ctx_t    context;
    uint64_t     stack_base;    /* Bottom of allocated stack */
    uint64_t     ticks_left;
    uint64_t     total_ticks;
} process_t;

static process_t proc_table[MAX_PROCESSES];
static int current_pid = -1;
static int next_pid = 1;
static int scheduler_enabled = 0;

/* Idle process: runs when nothing else is ready */
static void idle_proc(void) {
    for (;;) hlt();
}

void scheduler_init(void) {
    memset(proc_table, 0, sizeof(proc_table));

    /* PID 0: kernel main (current context, no separate stack) */
    proc_table[0].pid = 0;
    proc_table[0].state = PROC_RUNNING;
    strcpy(proc_table[0].name, "kernel");
    proc_table[0].ticks_left = TIME_SLICE;
    current_pid = 0;
    next_pid = 1;

    scheduler_enabled = 1;
}

/* Create a new process with the given entry point */
int process_create(const char *name, void (*entry)(void)) {
    int slot = -1;
    for (int i = 0; i < MAX_PROCESSES; i++) {
        if (proc_table[i].state == PROC_FREE) {
            slot = i;
            break;
        }
    }
    if (slot < 0) return -1;

    /* Allocate kernel stack */
    uint8_t *stack = (uint8_t *)kmalloc(KERNEL_STACK_SZ);
    if (!stack) return -1;

    process_t *p = &proc_table[slot];
    p->pid = next_pid++;
    p->state = PROC_READY;
    strncpy(p->name, name, 31);
    p->name[31] = '\0';
    p->stack_base = (uint64_t)stack;
    p->ticks_left = TIME_SLICE;
    p->total_ticks = 0;

    /* Set up initial stack frame so context switch "returns" to entry point.
     * We push a fake frame: RFLAGS(IF=1), RBP=0, then 14 callee-saved + return addr.
     * The switch function pops: r15,r14,r13,r12,rbx,rbp, then ret → entry */
    uint64_t *sp = (uint64_t *)(stack + KERNEL_STACK_SZ);

    /* push a fake return address (process_exit) for when entry() returns */
    *(--sp) = (uint64_t)process_exit;

    /* push the entry point as the "return address" for the switch ret */
    *(--sp) = (uint64_t)entry;

    /* push fake callee-saved registers: rbp, rbx, r12, r13, r14, r15 */
    *(--sp) = 0;  /* rbp */
    *(--sp) = 0;  /* rbx */
    *(--sp) = 0;  /* r12 */
    *(--sp) = 0;  /* r13 */
    *(--sp) = 0;  /* r14 */
    *(--sp) = 0;  /* r15 */

    p->context.rsp = (uint64_t)sp;
    return p->pid;
}

/* Context switch: save current RSP, load next RSP (defined in syscall.asm) */
extern void switch_context(uint64_t *old_rsp, uint64_t new_rsp);

static void schedule(void) {
    if (!scheduler_enabled) return;

    int old_pid = current_pid;
    int next = -1;

    /* Round-robin: find next READY process */
    for (int i = 1; i <= MAX_PROCESSES; i++) {
        int idx = (current_pid + i) % MAX_PROCESSES;
        if (proc_table[idx].state == PROC_READY) {
            next = idx;
            break;
        }
    }

    if (next < 0) return;  /* Only current process, no switch needed */
    if (next == current_pid) return;

    /* Switch */
    if (proc_table[old_pid].state == PROC_RUNNING) {
        proc_table[old_pid].state = PROC_READY;
    }
    proc_table[next].state = PROC_RUNNING;
    proc_table[next].ticks_left = TIME_SLICE;
    current_pid = next;

    switch_context(&proc_table[old_pid].context.rsp,
                   proc_table[next].context.rsp);
}

/* Called by timer IRQ0 every tick */
void scheduler_tick(void) {
    if (!scheduler_enabled) return;
    if (current_pid < 0) return;

    proc_table[current_pid].total_ticks++;

    if (proc_table[current_pid].ticks_left > 0) {
        proc_table[current_pid].ticks_left--;
    }

    if (proc_table[current_pid].ticks_left == 0) {
        schedule();
    }
}

void process_exit(void) {
    if (current_pid > 0) {
        proc_table[current_pid].state = PROC_ZOMBIE;
        if (proc_table[current_pid].stack_base) {
            kfree((void *)proc_table[current_pid].stack_base);
        }
    }
    schedule();
    for (;;) hlt();  /* Should never reach here */
}

void process_yield(void) {
    proc_table[current_pid].ticks_left = 0;
    schedule();
}

/* List all processes (for shell "proc" command) */
void scheduler_list(void) {
    kprintf("PID  STATE    TICKS    NAME\n");
    kprintf("---  ------   ------   ----\n");
    for (int i = 0; i < MAX_PROCESSES; i++) {
        if (proc_table[i].state == PROC_FREE) continue;
        const char *st;
        switch (proc_table[i].state) {
            case PROC_READY:   st = "READY "; break;
            case PROC_RUNNING: st = "RUN   "; break;
            case PROC_BLOCKED: st = "BLOCK "; break;
            case PROC_ZOMBIE:  st = "ZOMBIE"; break;
            default:           st = "???   "; break;
        }
        kprintf("%3d  %s   %6llu   %s%s\n",
                (int)proc_table[i].pid, st,
                proc_table[i].total_ticks,
                proc_table[i].name,
                (i == current_pid) ? " *" : "");
    }
}
