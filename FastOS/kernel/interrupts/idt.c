/* FastOS v2.1 — IDT (Interrupt Descriptor Table)
 * ADead-BIB Native OS
 *
 * 256 entries, 16 bytes each = 4096 bytes total
 * IDT placed at physical 0x300000 (3MB mark)
 *
 * Entry format (64-bit long mode):
 *   [0:1]  offset_low   (16 bits)
 *   [2:3]  selector     (16 bits) = 0x08 (kernel code segment)
 *   [4]    ist          (3 bits, interrupt stack table index)
 *   [5]    type_attr    (8 bits) = 0x8E (present, DPL=0, interrupt gate)
 *   [6:7]  offset_mid   (16 bits)
 *   [8:11] offset_high  (32 bits)
 *   [12:15] reserved    (32 bits = 0)
 *
 * PIC remap: Master 0x20-0x27, Slave 0x28-0x2F
 *   ICW1: 0x11 (init + ICW4 needed)
 *   ICW2: 0x20 master, 0x28 slave (vector offsets)
 *   ICW3: 0x04 master (slave on IRQ2), 0x02 slave
 *   ICW4: 0x01 (8086 mode)
 *
 * Handlers:
 *   IRQ0  (0x20) = PIT timer
 *   IRQ1  (0x21) = Keyboard
 *   0x00-0x1F    = CPU exceptions (div0, page fault, etc.)
 *
 * Inline version for kernel_main():
 *   // PIC remap
 *   __outb(0x20, 0x11); __outb(0xA0, 0x11);  // ICW1
 *   __outb(0x21, 0x20); __outb(0xA1, 0x28);  // ICW2
 *   __outb(0x21, 0x04); __outb(0xA1, 0x02);  // ICW3
 *   __outb(0x21, 0x01); __outb(0xA1, 0x01);  // ICW4
 *   __outb(0x21, 0xFC); __outb(0xA1, 0xFF);  // Mask: only IRQ0+IRQ1
 *
 *   // IDT at 0x300000
 *   // For each vector 0-255:
 *   //   __store32(0x300000, vec*16+0, handler_low16 | (0x08 << 16));
 *   //   __store32(0x300000, vec*16+4, 0x00008E00 | handler_mid16);
 *   //   __store32(0x300000, vec*16+8, handler_high32);
 *   //   __store32(0x300000, vec*16+12, 0);
 *
 *   // LIDT
 *   // IDTR at 0x300FFC: limit=0x0FFF, base=0x300000
 *   __store16(0x300FFC, 0, 0x0FFF);
 *   __store32(0x300FFC, 2, 0x300000);
 *   // __lidt(0x300FFC);
 *   // __sti();
 */
