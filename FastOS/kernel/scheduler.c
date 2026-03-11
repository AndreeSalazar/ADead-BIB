/*
 * FastOS v2.0 — Process Scheduler
 * Round-robin preemptivo
 *
 * Tipos canonicos de kernel.h (sin redefiniciones locales):
 *   process_t, cpu_context_t, proc_state_t, MAX_PROCESSES
 *
 * Integracion BG:
 *   bg_level4_heartbeat() en cada tick del scheduler.
 */

#include "../include/kernel.h"
#include "../include/types.h"
#include "../include/fastos.h"


/* ============================================================
 * Estado global del scheduler
 * ============================================================ */

static process_t  processes[MAX_PROCESSES];
static uint32_t   current_pid     = 0;
static uint32_t   next_pid        = 1;
static uint64_t   scheduler_ticks = 0;

#define TIME_SLICE_DEFAULT 10   /* Ticks del timer por time-slice */

/* Forward declaration — schedule() es llamada por process_exit/block
 * antes de su definicion en este archivo. */
static void schedule(void);

/* ============================================================
 * Acceso a procesos
 * ============================================================ */

process_t *get_current_process(void) {
    return &processes[current_pid];
}

process_t *get_process(uint32_t pid) {
    if (pid >= MAX_PROCESSES)            return NULL;
    if (processes[pid].state == PROC_UNUSED) return NULL;
    return &processes[pid];
}

static int find_free_slot(void) {
    for (int i = 1; i < MAX_PROCESSES; i++) {
        if (processes[i].state == PROC_UNUSED) return i;
    }
    return -1;
}

/* ============================================================
 * Gestion de Procesos
 * ============================================================ */

/* Crear proceso nuevo.
 * Firma alineada con kernel.h: process_create(name, entry, security_level) */
int process_create(const char *name, void (*entry)(void), uint8_t security_level) {
    int slot = find_free_slot();
    if (slot < 0) {
        kprintf("[SCHED] No free process slots\n");
        return -1;
    }

    process_t *proc = &processes[slot];

    proc->pid            = next_pid++;
    proc->ppid           = current_pid;
    proc->state          = PROC_READY;
    proc->priority       = 1;
    proc->security_level = security_level;

    /* Copiar nombre */
    int i;
    for (i = 0; i < 31 && name[i]; i++) proc->name[i] = name[i];
    proc->name[i] = '\0';

    /* Contexto inicial */
    proc->context.rip    = (uint64_t)entry;
    proc->context.rflags = 0x202;      /* IF habilitado */
    proc->context.cs     = 0x08;       /* Kernel code segment */
    proc->context.ss     = 0x10;       /* Kernel data segment */

    /* Stacks en areas fijas — kmalloc() los asignara cuando este listo */
    proc->kernel_stack   = 0x200000 + (uint64_t)slot * KERNEL_STACK_SIZE;
    proc->user_stack     = 0x400000 + (uint64_t)slot * USER_STACK_SIZE;
    proc->context.rsp    = proc->kernel_stack + KERNEL_STACK_SIZE - 8;
    proc->context.rbp    = proc->context.rsp;
    proc->page_table     = 0;          /* Hereda CR3 del kernel por ahora */

    proc->time_slice     = TIME_SLICE_DEFAULT;
    proc->total_time     = 0;

    kprintf("[SCHED] Created process %d: %s (sec=%d)\n",
            (int)proc->pid, proc->name, (int)proc->security_level);
    return (int)proc->pid;
}

/* Terminar proceso actual */
void process_exit(int exit_code) {
    process_t *proc = get_current_process();
    if (proc->pid == 0) {
        KERNEL_PANIC(2, "Cannot exit kernel idle process");
    }
    kprintf("[SCHED] Process %d exited (code=%d)\n",
            (int)proc->pid, exit_code);
    proc->state = PROC_ZOMBIE;
    schedule();
}

/* Bloquear proceso actual (espera evento) */
void process_block(void) {
    process_t *proc = get_current_process();
    proc->state = PROC_BLOCKED;
    schedule();
}

/* Desbloquear proceso por PID */
void process_unblock(uint32_t pid) {
    process_t *proc = get_process(pid);
    if (proc && proc->state == PROC_BLOCKED) proc->state = PROC_READY;
}

/* ============================================================
 * Nucleo del Scheduler — Round-Robin
 * ============================================================ */

static int find_next_process(void) {
    int start = (int)current_pid;
    int next  = start;

    do {
        next = (next + 1) % MAX_PROCESSES;
        if (processes[next].state == PROC_READY) return next;
    } while (next != start);

    return 0;   /* Fallback: idle (PID 0) */
}

/* Context switch — esqueleto; la version real estara en asm/switch.asm */
static void context_switch(process_t *old, process_t *new_proc) {
    if (old == new_proc) return;

    /* Cambiar page table si el proceso tiene la suya */
    if (old->page_table != new_proc->page_table && new_proc->page_table != 0) {
        write_cr3(new_proc->page_table);
    }
    /*
     * Produccion (asm/switch.asm):
     *   mov [old->context.rsp], rsp
     *   ... (guardar todos los registros)
     *   mov rsp, [new_proc->context.rsp]
     *   ... (restaurar todos los registros)
     *   iretq
     */
}

static void schedule(void) {
    process_t *old = get_current_process();

    if (old->state == PROC_RUNNING) old->state = PROC_READY;

    int next        = find_next_process();
    process_t *new_proc = &processes[next];

    if (new_proc->state != PROC_READY && next != 0) {
        next     = 0;
        new_proc = &processes[0];
    }

    new_proc->state      = PROC_RUNNING;
    new_proc->time_slice = TIME_SLICE_DEFAULT;
    current_pid          = (uint32_t)next;

    context_switch(old, new_proc);
}

/* IRQ 0 - Timer tick */
void scheduler_tick(void) {
    scheduler_ticks++;

    process_t *proc = get_current_process();
    proc->total_time++;
    if (proc->time_slice > 0) proc->time_slice--;

    /* BG Nivel 4 — Dead Man's Switch: se debe llamar en cada tick */
    bg_level4_heartbeat();

    /* Preempcion deshabilitada: context_switch() es esqueleto (sin ASM real).
     * Llamar a schedule() desde IRQ corrompe el estado del CPU.
     * Se habilitara cuando asm/switch.asm este listo.
     *
     * if (proc->time_slice == 0 && proc->pid != 0) schedule();
     */
}

/* Yield voluntario */
void yield(void) {
    process_t *proc = get_current_process();
    proc->time_slice = 0;
    schedule();
}

/* ============================================================
 * Idle Process (PID 0)
 * ============================================================ */

static void idle_process(void) {
    while (1) { hlt(); }
}

/* ============================================================
 * Inicializacion
 * ============================================================ */

void scheduler_init(void) {
    kprintf("[SCHED] Initializing scheduler...\n");

    /* Limpiar tabla de procesos */
    for (int i = 0; i < MAX_PROCESSES; i++) {
        processes[i].state = PROC_UNUSED;
        processes[i].pid   = 0;
    }

    /* PID 0 = idle process (siempre existe) */
    processes[0].pid            = 0;
    processes[0].ppid           = 0;
    processes[0].state          = PROC_RUNNING;
    processes[0].priority       = 255;    /* Menor prioridad */
    processes[0].security_level = 0;      /* Nivel kernel */
    processes[0].context.rip    = (uint64_t)idle_process;
    processes[0].time_slice     = TIME_SLICE_DEFAULT;

    /* Nombre del idle */
    const char *idle_name = "idle";
    for (int i = 0; i < 5; i++) processes[0].name[i] = idle_name[i];

    current_pid = 0;

    /* Primer heartbeat BG */
    bg_level4_heartbeat();

    kprintf("[SCHED] Scheduler ready. Max=%d processes.\n", MAX_PROCESSES);
}

/* Wrappers — kernel.h declares process_current() and process_yield() */
process_t *process_current(void) {
    return get_current_process();
}

void process_yield(void) {
    yield();
}

/* Debug: listar todos los procesos */
void scheduler_list(void) {
    kprintf("\n[SCHED] Process List:\n");
    kprintf("  PID  PPID  STATE     PRIO  SEC  NAME\n");
    kprintf("  ---  ----  --------  ----  ---  ----\n");

    for (int i = 0; i < MAX_PROCESSES; i++) {
        if (processes[i].state == PROC_UNUSED) continue;

        const char *st;
        switch (processes[i].state) {
            case PROC_READY:   st = "READY   "; break;
            case PROC_RUNNING: st = "RUNNING "; break;
            case PROC_BLOCKED: st = "BLOCKED "; break;
            case PROC_ZOMBIE:  st = "ZOMBIE  "; break;
            default:           st = "UNKNOWN "; break;
        }
        kprintf("  %3d  %4d  %s  %4d  %3d  %s\n",
                (int)processes[i].pid,  (int)processes[i].ppid,
                st,
                (int)processes[i].priority, (int)processes[i].security_level,
                processes[i].name);
    }
}
