/* FastOS v2.1 — PS/2 Keyboard Driver (i8042)
 * ADead-BIB Native OS
 *
 * i8042 controller (Intel 8042 compatible):
 *   Port 0x60: Data register (read scancode, write command data)
 *   Port 0x64: Status register (read), Command register (write)
 *
 * Status register bits:
 *   Bit 0: Output buffer full (data ready to read from 0x60)
 *   Bit 1: Input buffer full (don't write to 0x60/0x64)
 *   Bit 2: System flag (POST passed)
 *   Bit 5: Auxiliary output buffer full (mouse data)
 *
 * Scan code set 1 (default, what BIOS leaves):
 *   Make codes (key down): 0x01-0x58
 *   Break codes (key up): make code | 0x80 (bit 7 set)
 *
 * Integration: kernel.c polls inline with __inb(0x64) and __inb(0x60)
 * No IRQ needed — polling works perfectly at kernel level.
 * When IDT is set up, IRQ1 (vector 0x21) can be used instead.
 *
 * Scancode → ASCII table (set 1, US QWERTY):
 *   0x02-0x0B: 1234567890
 *   0x0C: -    0x0D: =    0x0E: Backspace
 *   0x0F: Tab  0x10-0x19: qwertyuiop
 *   0x1A: [    0x1B: ]    0x1C: Enter
 *   0x1D: LCtrl
 *   0x1E-0x26: asdfghjkl
 *   0x27: ;    0x28: '    0x29: `
 *   0x2A: LShift
 *   0x2B: \    0x2C-0x32: zxcvbnm
 *   0x33: ,    0x34: .    0x35: /
 *   0x36: RShift  0x38: LAlt  0x39: Space
 *   0x3A: CapsLock
 *   0x3B-0x44: F1-F10  0x57: F11  0x58: F12
 *
 * Shift state handling (inline):
 *   if (key == 0x2A || key == 0x36) shift = 1;
 *   if (key == 0xAA || key == 0xB6) shift = 0;
 *   if (shift) use uppercase table
 *
 * Extended keys (0xE0 prefix):
 *   0xE0 0x48: Up    0xE0 0x50: Down
 *   0xE0 0x4B: Left  0xE0 0x4D: Right
 *   0xE0 0x47: Home  0xE0 0x4F: End
 *   0xE0 0x49: PgUp  0xE0 0x51: PgDn
 *   0xE0 0x52: Ins   0xE0 0x53: Del
 */
