# FastOS — Drivers

> Todos los drivers escritos en Rust. Hardware access via ADead-BIB extern "C".

---

## Driver Priority

```text
Priority 1 (Critical — sin estos no hay GUI):
  ✦ Framebuffer driver (VBE/GOP)
  ✦ Timer driver (PIT)

Priority 2 (Essential — input):
  ✦ Keyboard driver (PS/2)
  ✦ Mouse driver (PS/2)

Priority 3 (Storage):
  ✦ Disk driver (ATA PIO → AHCI)

Priority 4 (Future):
  ○ USB stack
  ○ Network (NIC)
  ○ Audio (AC97/HDA)
  ○ GPU (Vulkan compute)
```

---

## Framebuffer Driver (`kernel/src/drivers/framebuffer.rs`)

**El driver más importante.** Sin framebuffer no hay GUI.

### BIOS VBE (Video BIOS Extensions)

```text
Setup (done in stage2, real mode):
  1. INT 10h, AX=4F01h — Get mode info
     CX = 0x118 (1024x768x32bpp)
     ES:DI = mode info buffer

  2. INT 10h, AX=4F02h — Set VBE mode
     BX = 0x4118 (mode + linear framebuffer bit)

  3. Save framebuffer address from mode info offset 0x28

Mode info passed to kernel via BootInfo struct at 0x9000.
```

### Framebuffer Structure

```rust
pub struct Framebuffer {
    buffer: &'static mut [u32],  // Mapped framebuffer memory
    width: u32,                   // 1024
    height: u32,                  // 768
    pitch: u32,                   // Bytes per scanline
    bpp: u32,                     // 32
}
```

### Primitives

```rust
impl Framebuffer {
    // Basic
    fn put_pixel(&mut self, x: u32, y: u32, color: u32);
    fn get_pixel(&self, x: u32, y: u32) -> u32;

    // Shapes
    fn fill_rect(&mut self, x: u32, y: u32, w: u32, h: u32, color: u32);
    fn draw_rect(&mut self, x: u32, y: u32, w: u32, h: u32, color: u32);
    fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: u32);
    fn draw_circle(&mut self, cx: i32, cy: i32, r: i32, color: u32);
    fn fill_circle(&mut self, cx: i32, cy: i32, r: i32, color: u32);

    // Text
    fn draw_char(&mut self, x: u32, y: u32, ch: u8, fg: u32, bg: u32);
    fn draw_string(&mut self, x: u32, y: u32, s: &str, fg: u32, bg: u32);

    // Bitmap
    fn draw_bitmap(&mut self, x: u32, y: u32, w: u32, h: u32, data: &[u32]);
    fn blit(&mut self, x: u32, y: u32, src: &[u32], sw: u32, sh: u32);

    // Alpha blending
    fn blend_pixel(&mut self, x: u32, y: u32, color: u32);
    fn blend_rect(&mut self, x: u32, y: u32, src: &[u32], sw: u32, sh: u32);

    // Screen
    fn clear(&mut self, color: u32);
    fn scroll_up(&mut self, lines: u32, bg: u32);
}
```

### Color Format

```text
Pixel: 0xAARRGGBB (32-bit)
  AA = Alpha (FF = opaque, 00 = transparent)
  RR = Red
  GG = Green
  BB = Blue

Common colors:
  BLACK       = 0xFF000000
  WHITE       = 0xFFFFFFFF
  RED         = 0xFFFF0000
  GREEN       = 0xFF00FF00
  BLUE        = 0xFF0000FF
  ACCENT      = 0xFF0078D4  (Windows Blue)
  BACKGROUND  = 0xFFF3F3F3  (Light grey)
  TASKBAR     = 0xE01C1C1C  (Dark, semi-transparent)
```

### Bitmap Font (8x16)

```text
256 characters, each 8 pixels wide × 16 pixels tall
1 bit per pixel → 16 bytes per character
Total: 256 × 16 = 4096 bytes

Storage: const FONT_8X16: [u8; 4096] = [...];

Rendering:
  for row in 0..16:
    byte = FONT_8X16[char_index * 16 + row]
    for col in 0..8:
      if byte & (0x80 >> col) != 0:
        put_pixel(x + col, y + row, fg_color)
      else:
        put_pixel(x + col, y + row, bg_color)
```

---

## Keyboard Driver (`kernel/src/drivers/keyboard.rs`)

### PS/2 Keyboard

```text
I/O Ports:
  0x60 — Data port (read scancode)
  0x64 — Status/command port

IRQ: 1 (INT 33 after PIC remap)

Scancode Set 1 (default BIOS):
  Make code:  key pressed
  Break code: key released (make | 0x80)
```

### Key Mapping

```text
Scancode → Keycode → Character

Modifiers tracked:
  Shift (left/right)
  Ctrl (left/right)
  Alt (left/right)
  Caps Lock
  Num Lock

Special keys:
  F1-F12, arrows, Home, End, PgUp, PgDn
  Insert, Delete, Print Screen, Pause
```

### Interface

```rust
pub struct Keyboard {
    shift: bool,
    ctrl: bool,
    alt: bool,
    caps_lock: bool,
}

impl Keyboard {
    fn handle_irq(&mut self);
    fn read_scancode(&self) -> u8;
    fn scancode_to_key(&self, scancode: u8) -> Option<KeyEvent>;
    fn is_key_pressed(&self, key: KeyCode) -> bool;
}

pub struct KeyEvent {
    keycode: KeyCode,
    character: Option<char>,
    pressed: bool,
    shift: bool,
    ctrl: bool,
    alt: bool,
}
```

---

## Mouse Driver (`kernel/src/drivers/mouse.rs`)

### PS/2 Mouse

```text
I/O Ports:
  0x60 — Data port
  0x64 — Command port

IRQ: 12 (INT 44 after PIC remap)

Initialization:
  1. Enable auxiliary device (command 0xA8 to port 0x64)
  2. Enable IRQ12 (command 0x20 to port 0x64, set bit 1)
  3. Set defaults (command 0xF6 to mouse via 0xD4)
  4. Enable data reporting (command 0xF4 to mouse via 0xD4)

Packet format (3 bytes):
  Byte 0: [Y overflow][X overflow][Y sign][X sign][1][Middle][Right][Left]
  Byte 1: X movement (signed)
  Byte 2: Y movement (signed)
```

### Interface

```rust
pub struct Mouse {
    x: i32,
    y: i32,
    buttons: MouseButtons,
    packet_index: u8,
    packet: [u8; 3],
}

pub struct MouseButtons {
    left: bool,
    right: bool,
    middle: bool,
}

impl Mouse {
    fn handle_irq(&mut self);
    fn get_position(&self) -> (i32, i32);
    fn get_buttons(&self) -> MouseButtons;
}
```

---

## Timer Driver (`kernel/src/drivers/timer.rs`)

### PIT (Programmable Interval Timer)

```text
I/O Ports:
  0x40 — Channel 0 data
  0x43 — Command register

IRQ: 0 (INT 32 after PIC remap)

Setup for ~1000 Hz:
  Base frequency: 1,193,182 Hz
  Divisor: 1193 (1193182 / 1000 ≈ 1193)
  Command: 0x36 (channel 0, lobyte/hibyte, rate generator)
  Write 0xA9 to port 0x40 (low byte of 1193)
  Write 0x04 to port 0x40 (high byte of 1193)
```

### Interface

```rust
pub struct Timer {
    ticks: u64,
    frequency: u32,  // Hz
}

impl Timer {
    fn init(frequency: u32);
    fn handle_irq(&mut self);
    fn get_ticks(&self) -> u64;
    fn get_uptime_ms(&self) -> u64;
    fn sleep_ms(&self, ms: u64);
}
```

---

## Disk Driver (`kernel/src/drivers/disk.rs`)

### ATA PIO (Initial)

```text
I/O Ports (Primary):
  0x1F0 — Data
  0x1F1 — Error/Features
  0x1F2 — Sector count
  0x1F3 — LBA low
  0x1F4 — LBA mid
  0x1F5 — LBA high
  0x1F6 — Drive/Head
  0x1F7 — Status/Command

Commands:
  0x20 — Read sectors (PIO)
  0x30 — Write sectors (PIO)
  0xEC — Identify drive
```

### Interface

```rust
pub struct AtaDisk {
    base_port: u16,
    is_slave: bool,
}

impl AtaDisk {
    fn identify(&self) -> Option<DriveInfo>;
    fn read_sectors(&self, lba: u64, count: u16, buf: &mut [u8]) -> Result<(), DiskError>;
    fn write_sectors(&self, lba: u64, count: u16, buf: &[u8]) -> Result<(), DiskError>;
}
```

---

## Driver Registration

```text
All drivers register with the kernel at boot:

kernel_main():
  framebuffer::init(boot_info);
  keyboard::init();
  mouse::init();
  timer::init(1000);  // 1000 Hz
  disk::init();
```
