/*
 * FastOS v2.0 — Interrupt Handler (IDT)
 * x86-64 interrupt descriptor table and handlers
 */

#include "../include/kernel.h"
#include "../include/types.h"

/* ============================================================
 * IDT Entry
 * ============================================================ */

typedef struct {
    uint16_t offset_low;
    uint16_t selector;
    uint8_t  ist;
    uint8_t  type_attr;
    uint16_t offset_mid;
    uint32_t offset_high;
    uint32_t reserved;
} __packed idt_entry_t;

/* Usamos idt_ptr_t de kernel.h — misma estructura, nombre canonico */
/* IDT with 256 entries */
static idt_entry_t idt[256];
static idt_ptr_t   idtr;

/* ============================================================
 * IDT Entry Types
 * ============================================================ */

#define IDT_INTERRUPT_GATE 0x8E  /* P=1, DPL=0, Type=1110 */
#define IDT_TRAP_GATE      0x8F  /* P=1, DPL=0, Type=1111 */
#define IDT_USER_INTERRUPT 0xEE  /* P=1, DPL=3, Type=1110 */

/* ============================================================
 * Exception Names
 * ============================================================ */

static const char *exception_names[] = {
    "Division Error",
    "Debug",
    "NMI",
    "Breakpoint",
    "Overflow",
    "Bound Range Exceeded",
    "Invalid Opcode",
    "Device Not Available",
    "Double Fault",
    "Coprocessor Segment Overrun",
    "Invalid TSS",
    "Segment Not Present",
    "Stack-Segment Fault",
    "General Protection Fault",
    "Page Fault",
    "Reserved",
    "x87 FPU Error",
    "Alignment Check",
    "Machine Check",
    "SIMD Exception",
    "Virtualization Exception",
    "Control Protection Exception",
};

/* ============================================================
 * Interrupt Frame (pushed by CPU)
 * ============================================================ */

typedef struct {
    uint64_t r15, r14, r13, r12, r11, r10, r9, r8;
    uint64_t rdi, rsi, rbp, rbx, rdx, rcx, rax;
    uint64_t int_no, error_code;
    uint64_t rip, cs, rflags, rsp, ss;
} __packed interrupt_frame_t;

/* ============================================================
 * Set IDT Entry
 * ============================================================ */

static void idt_set_entry(int num, uint64_t handler, uint16_t selector, uint8_t type) {
    idt[num].offset_low  = handler & 0xFFFF;
    idt[num].offset_mid  = (handler >> 16) & 0xFFFF;
    idt[num].offset_high = (handler >> 32) & 0xFFFFFFFF;
    idt[num].selector    = selector;
    idt[num].ist         = 0;
    idt[num].type_attr   = type;
    idt[num].reserved    = 0;
}

/* ============================================================
 * Exception Handlers
 * ============================================================ */

void exception_handler(interrupt_frame_t *frame) {
    uint64_t int_no = frame->int_no;
    
    if (int_no < 22) {
        kprintf("\n[EXCEPTION] %s (#%d)\n", exception_names[int_no], (int)int_no);
    } else {
        kprintf("\n[EXCEPTION] Unknown (#%d)\n", (int)int_no);
    }
    
    kprintf("  Error Code: 0x%016llX\n", frame->error_code);
    kprintf("  RIP: 0x%016llX  CS: 0x%04llX\n", frame->rip, frame->cs);
    kprintf("  RSP: 0x%016llX  SS: 0x%04llX\n", frame->rsp, frame->ss);
    kprintf("  RFLAGS: 0x%016llX\n", frame->rflags);
    kprintf("  RAX: 0x%016llX  RBX: 0x%016llX\n", frame->rax, frame->rbx);
    kprintf("  RCX: 0x%016llX  RDX: 0x%016llX\n", frame->rcx, frame->rdx);
    kprintf("  RSI: 0x%016llX  RDI: 0x%016llX\n", frame->rsi, frame->rdi);
    kprintf("  RBP: 0x%016llX\n", frame->rbp);
    
    /* Page fault specific info */
    if (int_no == 14) {
        uint64_t cr2 = read_cr2();
        kprintf("  CR2 (fault addr): 0x%016llX\n", cr2);
        kprintf("  Flags: %s %s %s\n",
                (frame->error_code & 1) ? "Present" : "Not-Present",
                (frame->error_code & 2) ? "Write" : "Read",
                (frame->error_code & 4) ? "User" : "Kernel");
    }
    
    KERNEL_PANIC(9, "Unhandled CPU exception");
}

/* ============================================================
 * IRQ Handlers (PIC)
 * ============================================================ */

#define PIC1_CMD  0x20
#define PIC1_DATA 0x21
#define PIC2_CMD  0xA0
#define PIC2_DATA 0xA1

#define PIC_EOI   0x20

/* IRQ base after remapping */
#define IRQ_BASE  32

/* External handler declarations */
extern void scheduler_tick(void);

void irq_handler(interrupt_frame_t *frame) {
    uint64_t irq = frame->int_no - IRQ_BASE;
    
    switch (irq) {
        case 0:  /* Timer (PIT) */
            scheduler_tick();
            break;
            
        case 1:  /* Keyboard */
            {
                uint8_t scancode = inb(0x60);
                kprintf("[KB] Scancode: 0x%02X\n", scancode);
            }
            break;
            
        case 12: /* Mouse */
            /* TODO: Handle mouse */
            break;
            
        default:
            kprintf("[IRQ] Unhandled IRQ %d\n", (int)irq);
            break;
    }
    
    /* Send EOI */
    if (irq >= 8) {
        outb(PIC2_CMD, PIC_EOI);
    }
    outb(PIC1_CMD, PIC_EOI);
}

/* ============================================================
 * Syscall Handler (INT 0x80 style)
 * ============================================================ */

extern int64_t syscall_dispatch(uint64_t num, uint64_t a1, uint64_t a2,
                                uint64_t a3, uint64_t a4, uint64_t a5, uint64_t a6);

void syscall_handler(interrupt_frame_t *frame) {
    int64_t result = syscall_dispatch(
        frame->rax,  /* syscall number */
        frame->rdi,  /* arg1 */
        frame->rsi,  /* arg2 */
        frame->rdx,  /* arg3 */
        frame->r10,  /* arg4 */
        frame->r8,   /* arg5 */
        frame->r9    /* arg6 */
    );
    
    frame->rax = result;
}

/* ============================================================
 * Common Interrupt Handler (called from assembly stubs)
 * ============================================================ */

void interrupt_handler(interrupt_frame_t *frame) {
    uint64_t int_no = frame->int_no;
    
    if (int_no < 32) {
        /* CPU Exception */
        exception_handler(frame);
    } else if (int_no < 48) {
        /* Hardware IRQ */
        irq_handler(frame);
    } else if (int_no == 0x80) {
        /* Syscall (Linux style) */
        syscall_handler(frame);
    } else {
        kprintf("[INT] Unknown interrupt %d\n", (int)int_no);
    }
}

/* ============================================================
 * PIC Initialization
 * ============================================================ */

static void pic_init(void) {
    /* ICW1: Initialize + ICW4 needed */
    outb(PIC1_CMD, 0x11);
    outb(PIC2_CMD, 0x11);
    io_wait();
    
    /* ICW2: Remap IRQs to 32-47 */
    outb(PIC1_DATA, IRQ_BASE);      /* IRQ 0-7  -> INT 32-39 */
    outb(PIC2_DATA, IRQ_BASE + 8);  /* IRQ 8-15 -> INT 40-47 */
    io_wait();
    
    /* ICW3: Cascade */
    outb(PIC1_DATA, 0x04);  /* IRQ2 has slave */
    outb(PIC2_DATA, 0x02);  /* Slave ID 2 */
    io_wait();
    
    /* ICW4: 8086 mode */
    outb(PIC1_DATA, 0x01);
    outb(PIC2_DATA, 0x01);
    io_wait();
    
    /* Mask all IRQs except timer (0) and keyboard (1) */
    outb(PIC1_DATA, 0xFC);  /* Enable IRQ 0, 1 */
    outb(PIC2_DATA, 0xFF);  /* Mask all slave IRQs */
}

/* ============================================================
 * PIT (Timer) Initialization
 * ============================================================ */

#define PIT_FREQ 1193182
#define TIMER_HZ 100  /* 100 Hz = 10ms per tick */

static void pit_init(void) {
    uint16_t divisor = PIT_FREQ / TIMER_HZ;
    
    outb(0x43, 0x36);  /* Channel 0, lobyte/hibyte, mode 3 */
    outb(0x40, divisor & 0xFF);
    outb(0x40, (divisor >> 8) & 0xFF);
}

/* ============================================================
 * IDT Initialization
 * ============================================================ */

/* Stub placeholder — los handlers reales estaran en asm/isr_stubs.asm
 * Por ahora apuntan a dummy para que el IDT quede cargado y valido. */
static void dummy_handler(void) {
    /* hlt y spin — si se dispara un IRQ sin handler real, no crashea */
    asm volatile("cli; hlt");
    while (1) { asm volatile("hlt"); }
}

void idt_init(void) {
    kprintf("[IDT] Initializing interrupt descriptor table...\n");
    
    /* Clear IDT */
    for (int i = 0; i < 256; i++) {
        idt_set_entry(i, (uint64_t)dummy_handler, 0x08, IDT_INTERRUPT_GATE);
    }
    
    /* Setup IDTR */
    idtr.limit = sizeof(idt) - 1;
    idtr.base = (uint64_t)&idt;
    
    /* Load IDT */
    lidt(&idtr);
    
    /* Initialize PIC */
    pic_init();
    kprintf("[IDT] PIC remapped to IRQ %d-%d\n", IRQ_BASE, IRQ_BASE + 15);
    
    /* Initialize PIT */
    pit_init();
    kprintf("[IDT] PIT configured at %d Hz\n", TIMER_HZ);
    
    kprintf("[IDT] IDT loaded at 0x%016llX\n", idtr.base);
}

/* Enable interrupts */
void interrupts_enable(void) {
    sti();
    kprintf("[IDT] Interrupts enabled\n");
}

/* Disable interrupts */
void interrupts_disable(void) {
    cli();
}

/* ============================================================
 * interrupts_init() — Llamada por kernel_main()
 * Wrapper que secuencia la inicializacion completa de interrupciones.
 * El step de ADead-BIB confirma que kernel_main() espera esta firma
 * exacta como forward declaration.
 * ============================================================ */
void interrupts_init(void) {
    idt_init();
    interrupts_enable();
    kprintf("[IDT] Interrupt subsystem online\n");
}
