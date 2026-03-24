/*
 * FastOS v2.0 — IDT (Interrupt Descriptor Table)
 * 256 entries, 16 bytes each, for 64-bit Long Mode
 *
 * Entry format:
 *   offset_low[16] | selector[16] | ist[3] | reserved[5] |
 *   type_attr[8]   | offset_mid[16] | offset_high[32] | reserved[32]
 *
 * type_attr = 0x8E: Present, DPL=0, 64-bit interrupt gate
 * type_attr = 0xEE: Present, DPL=3, 64-bit interrupt gate (for syscalls)
 */

#include "include/kernel.h"

#define IDT_ENTRIES 256

typedef struct {
    uint16_t offset_low;
    uint16_t selector;
    uint8_t  ist;
    uint8_t  type_attr;
    uint16_t offset_mid;
    uint32_t offset_high;
    uint32_t reserved;
} __packed idt_entry_t;

static idt_entry_t idt[IDT_ENTRIES] __aligned(16);

/* ISR stubs defined in isr.asm */
extern void isr0(void);   extern void isr1(void);
extern void isr2(void);   extern void isr3(void);
extern void isr4(void);   extern void isr5(void);
extern void isr6(void);   extern void isr7(void);
extern void isr8(void);   extern void isr9(void);
extern void isr10(void);  extern void isr11(void);
extern void isr12(void);  extern void isr13(void);
extern void isr14(void);  extern void isr15(void);
extern void isr16(void);  extern void isr17(void);
extern void isr18(void);  extern void isr19(void);
extern void isr20(void);  extern void isr21(void);
extern void isr22(void);  extern void isr23(void);
extern void isr24(void);  extern void isr25(void);
extern void isr26(void);  extern void isr27(void);
extern void isr28(void);  extern void isr29(void);
extern void isr30(void);  extern void isr31(void);

/* IRQ handlers (remapped to 32-47) */
extern void irq0(void);   extern void irq1(void);
extern void irq2(void);   extern void irq3(void);
extern void irq4(void);   extern void irq5(void);
extern void irq6(void);   extern void irq7(void);
extern void irq8(void);   extern void irq9(void);
extern void irq10(void);  extern void irq11(void);
extern void irq12(void);  extern void irq13(void);
extern void irq14(void);  extern void irq15(void);

static void idt_set_gate(int n, uint64_t handler, uint16_t selector,
                          uint8_t type_attr) {
    idt[n].offset_low  = handler & 0xFFFF;
    idt[n].selector    = selector;
    idt[n].ist         = 0;
    idt[n].type_attr   = type_attr;
    idt[n].offset_mid  = (handler >> 16) & 0xFFFF;
    idt[n].offset_high = (uint32_t)(handler >> 32);
    idt[n].reserved    = 0;
}

/* CPU exception names for panic messages */
static const char *exception_names[32] = {
    "Division by Zero",      "Debug",
    "NMI",                   "Breakpoint",
    "Overflow",              "Bound Range Exceeded",
    "Invalid Opcode",        "Device Not Available",
    "Double Fault",          "Coprocessor Segment Overrun",
    "Invalid TSS",           "Segment Not Present",
    "Stack-Segment Fault",   "General Protection Fault",
    "Page Fault",            "Reserved",
    "x87 FPU Error",         "Alignment Check",
    "Machine Check",         "SIMD FP Exception",
    "Virtualization Exc",    "Control Protection Exc",
    "Reserved", "Reserved", "Reserved", "Reserved",
    "Reserved", "Reserved",
    "Hypervisor Injection",  "VMM Communication Exc",
    "Security Exception",    "Reserved"
};

/* C-level interrupt handler — called from isr.asm common stub */
typedef struct {
    uint64_t r15, r14, r13, r12, r11, r10, r9, r8;
    uint64_t rbp, rdi, rsi, rdx, rcx, rbx, rax;
    uint64_t int_no, err_code;
    uint64_t rip, cs, rflags, rsp, ss;
} __packed isr_frame_t;

/* IRQ handler function pointers */
typedef void (*irq_handler_t)(isr_frame_t *frame);
static irq_handler_t irq_handlers[16] = {0};

void irq_register_handler(uint8_t irq, irq_handler_t handler) {
    if (irq < 16) irq_handlers[irq] = handler;
}

/* Called from isr.asm for all interrupts */
void isr_handler(isr_frame_t *frame) {
    uint64_t int_no = frame->int_no;

    if (int_no < 32) {
        /* CPU exception */
        kprintf("\n!!! CPU EXCEPTION: %s (#%d)\n", exception_names[int_no], (int)int_no);
        kprintf("    Error Code: 0x%016llx\n", frame->err_code);
        kprintf("    RIP: 0x%016llx  CS: 0x%04x\n", frame->rip, (int)frame->cs);
        kprintf("    RSP: 0x%016llx  SS: 0x%04x\n", frame->rsp, (int)frame->ss);
        kprintf("    RAX: 0x%016llx  RBX: 0x%016llx\n", frame->rax, frame->rbx);
        kprintf("    RCX: 0x%016llx  RDX: 0x%016llx\n", frame->rcx, frame->rdx);
        if (int_no == 14) {
            kprintf("    CR2 (fault addr): 0x%016llx\n", read_cr2());
        }
        kernel_panic("Unhandled CPU exception");
    } else if (int_no >= 32 && int_no < 48) {
        /* Hardware IRQ */
        uint8_t irq = (uint8_t)(int_no - 32);
        if (irq_handlers[irq]) {
            irq_handlers[irq](frame);
        }
        pic_send_eoi(irq);
    }
}

void idt_init(void) {
    memset(idt, 0, sizeof(idt));

    /* CPU exceptions (vectors 0-31) — interrupt gates, DPL=0 */
    idt_set_gate(0,  (uint64_t)isr0,  0x08, 0x8E);
    idt_set_gate(1,  (uint64_t)isr1,  0x08, 0x8E);
    idt_set_gate(2,  (uint64_t)isr2,  0x08, 0x8E);
    idt_set_gate(3,  (uint64_t)isr3,  0x08, 0x8E);
    idt_set_gate(4,  (uint64_t)isr4,  0x08, 0x8E);
    idt_set_gate(5,  (uint64_t)isr5,  0x08, 0x8E);
    idt_set_gate(6,  (uint64_t)isr6,  0x08, 0x8E);
    idt_set_gate(7,  (uint64_t)isr7,  0x08, 0x8E);
    idt_set_gate(8,  (uint64_t)isr8,  0x08, 0x8E);
    idt_set_gate(9,  (uint64_t)isr9,  0x08, 0x8E);
    idt_set_gate(10, (uint64_t)isr10, 0x08, 0x8E);
    idt_set_gate(11, (uint64_t)isr11, 0x08, 0x8E);
    idt_set_gate(12, (uint64_t)isr12, 0x08, 0x8E);
    idt_set_gate(13, (uint64_t)isr13, 0x08, 0x8E);
    idt_set_gate(14, (uint64_t)isr14, 0x08, 0x8E);
    idt_set_gate(15, (uint64_t)isr15, 0x08, 0x8E);
    idt_set_gate(16, (uint64_t)isr16, 0x08, 0x8E);
    idt_set_gate(17, (uint64_t)isr17, 0x08, 0x8E);
    idt_set_gate(18, (uint64_t)isr18, 0x08, 0x8E);
    idt_set_gate(19, (uint64_t)isr19, 0x08, 0x8E);
    idt_set_gate(20, (uint64_t)isr20, 0x08, 0x8E);
    idt_set_gate(21, (uint64_t)isr21, 0x08, 0x8E);
    idt_set_gate(22, (uint64_t)isr22, 0x08, 0x8E);
    idt_set_gate(23, (uint64_t)isr23, 0x08, 0x8E);
    idt_set_gate(24, (uint64_t)isr24, 0x08, 0x8E);
    idt_set_gate(25, (uint64_t)isr25, 0x08, 0x8E);
    idt_set_gate(26, (uint64_t)isr26, 0x08, 0x8E);
    idt_set_gate(27, (uint64_t)isr27, 0x08, 0x8E);
    idt_set_gate(28, (uint64_t)isr28, 0x08, 0x8E);
    idt_set_gate(29, (uint64_t)isr29, 0x08, 0x8E);
    idt_set_gate(30, (uint64_t)isr30, 0x08, 0x8E);
    idt_set_gate(31, (uint64_t)isr31, 0x08, 0x8E);

    /* Hardware IRQs (vectors 32-47) — remapped by PIC to 0x20-0x2F */
    idt_set_gate(32, (uint64_t)irq0,  0x08, 0x8E);
    idt_set_gate(33, (uint64_t)irq1,  0x08, 0x8E);
    idt_set_gate(34, (uint64_t)irq2,  0x08, 0x8E);
    idt_set_gate(35, (uint64_t)irq3,  0x08, 0x8E);
    idt_set_gate(36, (uint64_t)irq4,  0x08, 0x8E);
    idt_set_gate(37, (uint64_t)irq5,  0x08, 0x8E);
    idt_set_gate(38, (uint64_t)irq6,  0x08, 0x8E);
    idt_set_gate(39, (uint64_t)irq7,  0x08, 0x8E);
    idt_set_gate(40, (uint64_t)irq8,  0x08, 0x8E);
    idt_set_gate(41, (uint64_t)irq9,  0x08, 0x8E);
    idt_set_gate(42, (uint64_t)irq10, 0x08, 0x8E);
    idt_set_gate(43, (uint64_t)irq11, 0x08, 0x8E);
    idt_set_gate(44, (uint64_t)irq12, 0x08, 0x8E);
    idt_set_gate(45, (uint64_t)irq13, 0x08, 0x8E);
    idt_set_gate(46, (uint64_t)irq14, 0x08, 0x8E);
    idt_set_gate(47, (uint64_t)irq15, 0x08, 0x8E);

    /* Load IDT register */
    desc_ptr_t idtr;
    idtr.limit = sizeof(idt) - 1;
    idtr.base  = (uint64_t)idt;
    lidt(&idtr);
}
