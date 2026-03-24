/*
 * FastOS v2.0 — PIT Timer Driver (Intel 8253/8254)
 * Channel 0, IRQ0 (vector 32 after PIC remap)
 * Base frequency: 1,193,182 Hz
 * Used for preemptive scheduler tick
 */

#include "include/kernel.h"

#define PIT_CH0_DATA  0x40
#define PIT_CMD       0x43
#define PIT_BASE_FREQ 1193182

static volatile uint64_t timer_ticks = 0;
static uint32_t timer_freq = 100;

/* Forward declaration for registration */
typedef struct {
    uint64_t r15, r14, r13, r12, r11, r10, r9, r8;
    uint64_t rbp, rdi, rsi, rdx, rcx, rbx, rax;
    uint64_t int_no, err_code;
    uint64_t rip, cs, rflags, rsp, ss;
} __packed isr_frame_t;

typedef void (*irq_handler_t)(isr_frame_t *frame);
extern void irq_register_handler(uint8_t irq, irq_handler_t handler);

/* IRQ0 handler — called every tick */
static void timer_irq_handler(isr_frame_t *frame) {
    (void)frame;
    timer_ticks++;
    scheduler_tick();
}

void timer_init(uint32_t freq_hz) {
    timer_freq = freq_hz;
    uint32_t divisor = PIT_BASE_FREQ / freq_hz;

    /* Channel 0, lo/hi byte, mode 3 (square wave), binary */
    outb(PIT_CMD, 0x36);
    outb(PIT_CH0_DATA, (uint8_t)(divisor & 0xFF));
    outb(PIT_CH0_DATA, (uint8_t)((divisor >> 8) & 0xFF));

    /* Register handler and unmask IRQ0 */
    irq_register_handler(0, timer_irq_handler);
    pic_clear_mask(0);
}

uint64_t timer_get_ticks(void) {
    return timer_ticks;
}

uint64_t timer_get_seconds(void) {
    return timer_ticks / timer_freq;
}
