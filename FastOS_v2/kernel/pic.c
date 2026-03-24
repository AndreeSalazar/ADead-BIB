/*
 * FastOS v2.0 — PIC 8259 Driver
 * Remaps master to IRQ 32-39, slave to IRQ 40-47
 *
 * ICW sequence:
 *   ICW1 (0x11): Init + ICW4 needed
 *   ICW2: Vector offset (0x20 master, 0x28 slave)
 *   ICW3: Wiring (master has slave on IRQ2=0x04, slave ID=0x02)
 *   ICW4 (0x01): 8086 mode
 */

#include "include/kernel.h"

#define PIC1_CMD   0x20
#define PIC1_DATA  0x21
#define PIC2_CMD   0xA0
#define PIC2_DATA  0xA1

#define PIC_EOI    0x20

void pic_init(void) {
    /* Save current masks */
    uint8_t mask1 = inb(PIC1_DATA);
    uint8_t mask2 = inb(PIC2_DATA);

    /* ICW1: initialize + ICW4 needed */
    outb(PIC1_CMD, 0x11); io_wait();
    outb(PIC2_CMD, 0x11); io_wait();

    /* ICW2: vector offsets */
    outb(PIC1_DATA, 0x20); io_wait();  /* Master: IRQ 0-7 → vectors 32-39 */
    outb(PIC2_DATA, 0x28); io_wait();  /* Slave:  IRQ 8-15 → vectors 40-47 */

    /* ICW3: master/slave wiring */
    outb(PIC1_DATA, 0x04); io_wait();  /* Master: slave on IRQ2 (bit 2) */
    outb(PIC2_DATA, 0x02); io_wait();  /* Slave: cascade identity 2 */

    /* ICW4: 8086 mode */
    outb(PIC1_DATA, 0x01); io_wait();
    outb(PIC2_DATA, 0x01); io_wait();

    /* Mask all IRQs initially — drivers unmask as needed */
    outb(PIC1_DATA, 0xFF);
    outb(PIC2_DATA, 0xFF);

    (void)mask1;
    (void)mask2;
}

/* Send End-Of-Interrupt to PIC */
void pic_send_eoi(uint8_t irq) {
    if (irq >= 8) {
        outb(PIC2_CMD, PIC_EOI);  /* Slave PIC needs EOI too */
    }
    outb(PIC1_CMD, PIC_EOI);
}

/* Mask (disable) a specific IRQ line */
void pic_set_mask(uint8_t irq) {
    uint16_t port;
    if (irq < 8) {
        port = PIC1_DATA;
    } else {
        port = PIC2_DATA;
        irq -= 8;
    }
    outb(port, inb(port) | (1 << irq));
}

/* Unmask (enable) a specific IRQ line */
void pic_clear_mask(uint8_t irq) {
    uint16_t port;
    if (irq < 8) {
        port = PIC1_DATA;
    } else {
        port = PIC2_DATA;
        irq -= 8;
        /* Also unmask cascade (IRQ2) on master */
        outb(PIC1_DATA, inb(PIC1_DATA) & ~(1 << 2));
    }
    outb(port, inb(port) & ~(1 << irq));
}
