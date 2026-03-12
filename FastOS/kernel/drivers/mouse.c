/* FastOS v2.1 — PS/2 Mouse Driver (i8042 auxiliary)
 * ADead-BIB Native OS
 *
 * PS/2 mouse via i8042 controller auxiliary port:
 *   Port 0x60: Data register (shared with keyboard)
 *   Port 0x64: Status/Command register
 *     Status bit 0: Output buffer full (data ready)
 *     Status bit 5: Auxiliary data (1=mouse, 0=keyboard)
 *     Status bit 1: Input buffer full (don't write)
 *
 * Mouse initialization sequence:
 *   1. Enable auxiliary port:     cmd 0xA8 to port 0x64
 *   2. Read controller config:    cmd 0x20 to port 0x64, read from 0x60
 *   3. Set bit 1 (IRQ12 enable):  OR with 0x02
 *   4. Clear bit 5 (mouse clock): AND with 0xDF
 *   5. Write config back:         cmd 0x60 to port 0x64, write to 0x60
 *   6. Reset mouse defaults:      cmd 0xD4, then 0xF6 to 0x60
 *   7. Enable streaming:          cmd 0xD4, then 0xF4 to 0x60
 *
 * Mouse packet format (3 bytes):
 *   Byte 1 (status):
 *     Bit 0: Left button
 *     Bit 1: Right button
 *     Bit 2: Middle button
 *     Bit 3: Always 1 (sync bit)
 *     Bit 4: X sign bit
 *     Bit 5: Y sign bit
 *     Bit 6: X overflow
 *     Bit 7: Y overflow
 *   Byte 2: X movement (unsigned, sign in byte 1 bit 4)
 *   Byte 3: Y movement (unsigned, sign in byte 1 bit 5)
 *
 * Sign extension for signed movement:
 *   if (status & 0x10) dx = byte2 | 0xFFFFFF00;  // negative X
 *   if (status & 0x20) dy = byte3 | 0xFFFFFF00;  // negative Y
 *
 * VGA text mode cursor (80x25):
 *   Position: (mouse_x, mouse_y) clamped to [0,79] x [0,24]
 *   VGA offset: mouse_y * 160 + mouse_x * 2
 *   Cursor char: 0xDB (full block) with attribute 0x4F (white on red)
 *   Previous cell saved and restored on cursor move
 *
 * Inspired by:
 *   Linux: drivers/input/mouse/psmouse-base.c
 *   Windows: mouclass.sys + i8042prt.sys
 */
