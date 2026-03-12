/* FastOS v2.1 — PIT Timer Driver (Intel 8253/8254)
 * ADead-BIB Native OS
 *
 * PIT (Programmable Interval Timer):
 *   Channel 0: System timer (IRQ0 → vector 0x20 after PIC remap)
 *   Channel 1: DRAM refresh (legacy, unused)
 *   Channel 2: PC speaker
 *
 * Ports:
 *   0x40: Channel 0 data
 *   0x41: Channel 1 data
 *   0x42: Channel 2 data
 *   0x43: Mode/Command register
 *
 * Command byte (port 0x43):
 *   Bits 7-6: Channel (00=ch0)
 *   Bits 5-4: Access mode (11=lo/hi byte)
 *   Bits 3-1: Mode (011=square wave, mode 3)
 *   Bit 0:    BCD (0=binary)
 *   → 0x36 = channel 0, lo/hi, mode 3, binary
 *
 * Base frequency: 1,193,182 Hz
 * Divisor for ~100 Hz: 11932 (0x2E9C)
 * Divisor for ~1000 Hz: 1193 (0x04A9)
 *
 * Inline init for kernel_main():
 *   __outb(0x43, 0x36);           // Channel 0, lo/hi, mode 3
 *   __outb(0x40, 0x9C);           // Divisor low byte (11932)
 *   __outb(0x40, 0x2E);           // Divisor high byte
 *
 * Tick counter (global variable):
 *   int timer_ticks = 0;
 *   // In IRQ0 handler: timer_ticks = timer_ticks + 1;
 *   // Uptime seconds: timer_ticks / 100
 *
 * Without IDT/IRQ: use TSC (RDTSC) for timing
 *   __rdtsc() returns 64-bit timestamp counter
 *   Ryzen 5 5600X base: 3.7 GHz → 3,700,000,000 ticks/sec
 */
