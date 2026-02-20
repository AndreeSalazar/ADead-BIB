// ============================================================
// FastOS Kernel — Main Entry Point (Rust)
// ============================================================
// The Rust security layer for FastOS.
// ADead-BIB handles hardware. Rust handles logic. C glues them.
//
// Boot flow:
//   1. Stage2 (ADead-BIB) loads kernel, sets VBE mode, writes BootInfo
//   2. kernel_main() reads BootInfo at 0x9000
//   3. Initialize: GDT → IDT → Memory → Drivers → Desktop
//   4. If no graphical FB → fallback to VGA text mode
//
// Format: FsOS (not PE, not ELF — our own)
// ============================================================

#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

// ---- New architecture modules ----
pub mod arch;
pub mod boot;
pub mod kernel_core;
pub mod drivers;

// ---- Desktop Engine (Fase 5) ----
pub mod desktop_engine;

// ---- Graphical Apps (Fase 7) ----
pub mod gfx_apps;

// ---- Legacy / existing modules ----
mod vga;
mod keyboard;
mod panic;
mod shell;
mod installer;
mod login;
pub mod desktop;
pub mod window;
pub mod startmenu;
pub mod apps;

use vga::{VgaWriter, Color};

// ============================================================
// Hardware functions — inline assembly (bare-metal, no C runtime)
// ============================================================

// ============================================================
// Serial port output for debugging (COM1 = 0x3F8)
// ============================================================
fn serial_init() {
    outb(0x3F8 + 1, 0x00); // Disable interrupts
    outb(0x3F8 + 3, 0x80); // Enable DLAB
    outb(0x3F8 + 0, 0x03); // 38400 baud (low byte)
    outb(0x3F8 + 1, 0x00); // 38400 baud (high byte)
    outb(0x3F8 + 3, 0x03); // 8 bits, no parity, 1 stop
    outb(0x3F8 + 2, 0xC7); // Enable FIFO
    outb(0x3F8 + 4, 0x0B); // IRQs enabled, RTS/DSR set
}

fn serial_putc(c: u8) {
    // Avoid hard lock if UART never reports TX-ready in some QEMU/host setups.
    // Best effort: if timeout expires, drop the byte and continue boot.
    for _ in 0..1_000_000 {
        if inb(0x3F8 + 5) & 0x20 != 0 {
            outb(0x3F8, c);
            return;
        }
    }
}

pub fn serial_print(s: &str) {
    for b in s.bytes() {
        serial_putc(b);
    }
}

fn serial_print_hex(val: u64) {
    let hex = b"0123456789ABCDEF";
    for i in (0..16).rev() {
        let nibble = ((val >> (i * 4)) & 0xF) as usize;
        serial_putc(hex[nibble]);
    }
}

fn serial_print_dec(val: u32) {
    if val == 0 {
        serial_putc(b'0');
        return;
    }
    let mut buf = [0u8; 10];
    let mut i = 0;
    let mut v = val;
    while v > 0 {
        buf[i] = b'0' + (v % 10) as u8;
        v /= 10;
        i += 1;
    }
    while i > 0 {
        i -= 1;
        serial_putc(buf[i]);
    }
}

// ============================================================
// Kernel Entry Point
// ============================================================
#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn kernel_main() -> ! {
    // Serial debug output first (visible via -serial stdio)
    serial_init();
    serial_print("[FastOS] kernel_main reached!\r\n");

    // IMMEDIATELY clear VGA screen to show we reached the kernel
    // This helps debug if we're stuck in stage2 or kernel
    {
        let vga_buffer = 0xB8000 as *mut u16;
        for i in 0..(80 * 25) {
            unsafe { *vga_buffer.add(i) = 0x0F00 | b' ' as u16; } // white on black
        }
        // Write "KERNEL OK" at top-left
        let msg = b"KERNEL OK - Initializing...";
        for (i, &c) in msg.iter().enumerate() {
            unsafe { *vga_buffer.add(i) = 0x0A00 | c as u16; } // green on black
        }
    }

        // Stage2 may leave IF=1. Until we install a valid IDT, any hardware IRQ
    // can cause #GP -> double fault -> triple fault reset. Keep interrupts
    // disabled during early boot and the current static graphical shell.
    cli();
    serial_print("[FastOS] Interrupts disabled (no IDT yet)\r\n");

    // ---- Phase 1: Read BootInfo FIRST (before touching anything else) ----
    serial_print("[FastOS] Reading BootInfo at 0x9000...\r\n");
    let boot_info = unsafe { boot::BootInfo::from_address(0x9000) };
    let mut has_framebuffer = false;

    if boot_info.is_valid() {
        serial_print("[FastOS] BootInfo VALID\r\n");
        serial_print("[FastOS] FB addr=0x");
        serial_print_hex(boot_info.framebuffer_addr);
        serial_print(" w=");
        serial_print_dec(boot_info.framebuffer_width);
        serial_print(" h=");
        serial_print_dec(boot_info.framebuffer_height);
        serial_print(" bpp=");
        serial_print_dec(boot_info.framebuffer_bpp);
        serial_print(" pitch=");
        serial_print_dec(boot_info.framebuffer_pitch);
        serial_print("\r\n");

        if boot_info.framebuffer_addr != 0
            && boot_info.framebuffer_addr != 0xB8000
            && boot_info.framebuffer_width >= 640
            && boot_info.framebuffer_bpp >= 24
        {
            has_framebuffer = true;
            serial_print("[FastOS] Graphical FB detected!\r\n");
        }
    } else {
        serial_print("[FastOS] BootInfo INVALID (magic mismatch)\r\n");
    }

    // ---- Phase 2: Initialize drivers (framebuffer must be first for graphical mode) ----
    if boot_info.is_valid() {
        serial_print("[FastOS] Drivers init...\r\n");
        drivers::init(boot_info);
        serial_print("[FastOS] Drivers OK\r\n");
    }

    // ---- Phase 3: Core subsystems ----
    // NOTE: Skip GDT/IDT reinit for now — stage2's GDT (CS=0x18, DS=0x20)
    // is functional. Desktop rendering doesn't need TSS or custom IDT.
    serial_print("[FastOS] Memory init...\r\n");
    kernel_core::memory::init();
    serial_print("[FastOS] Memory OK\r\n");

    if has_framebuffer {
        serial_print("[FastOS] Launching GRAPHICAL desktop\r\n");

        // ---- Launch graphical desktop engine ----
        run_graphical_desktop();
    } else {
        serial_print("[FastOS] Launching VGA TEXT desktop\r\n");

        // VGA text mode - simple direct write
        let vga_buffer = 0xB8000 as *mut u16;
        
        // Clear screen (white on black)
        for i in 0..(80 * 25) {
            unsafe { core::ptr::write_volatile(vga_buffer.add(i), 0x0F20); }
        }
        
        // Write banner
        let banner = b"============================================";
        let title  = b"       FastOS v1.0 - 64-bit Kernel";
        let line2  = b"============================================";
        let status = b"  [OK] All subsystems initialized.";
        let help   = b"  Press keys: W/A/S/D to navigate, Q to quit";
        
        // Row 0
        for (i, &c) in banner.iter().enumerate() {
            unsafe { core::ptr::write_volatile(vga_buffer.add(i), 0x0A00 | c as u16); }
        }
        // Row 1
        for (i, &c) in title.iter().enumerate() {
            unsafe { core::ptr::write_volatile(vga_buffer.add(80 + i), 0x0B00 | c as u16); }
        }
        // Row 2
        for (i, &c) in line2.iter().enumerate() {
            unsafe { core::ptr::write_volatile(vga_buffer.add(160 + i), 0x0A00 | c as u16); }
        }
        // Row 3
        for (i, &c) in status.iter().enumerate() {
            unsafe { core::ptr::write_volatile(vga_buffer.add(240 + i), 0x0F00 | c as u16); }
        }
        // Row 5
        for (i, &c) in help.iter().enumerate() {
            unsafe { core::ptr::write_volatile(vga_buffer.add(400 + i), 0x0E00 | c as u16); }
        }
        
        serial_print("[FastOS] VGA: Desktop displayed, entering input loop\r\n");
        
        // Simple keyboard polling loop
        let mut cursor_pos: usize = 80 * 7; // Row 7
        loop {
            // Check keyboard status
            let status: u8;
            unsafe { core::arch::asm!("in al, dx", in("dx") 0x64u16, out("al") status, options(nomem, nostack)); }
            
            if status & 0x01 != 0 {
                // Read scancode
                let scancode: u8;
                unsafe { core::arch::asm!("in al, dx", in("dx") 0x60u16, out("al") scancode, options(nomem, nostack)); }
                
                // Only process key press (not release)
                if scancode & 0x80 == 0 {
                    // Simple scancode to char
                    let ch = match scancode {
                        0x10 => b'q', 0x11 => b'w', 0x12 => b'e', 0x13 => b'r',
                        0x1E => b'a', 0x1F => b's', 0x20 => b'd', 0x21 => b'f',
                        0x39 => b' ',
                        _ => 0,
                    };
                    
                    if ch == b'q' {
                        // Quit - halt
                        serial_print("[FastOS] User pressed Q - halting\r\n");
                        loop {
                            unsafe { core::arch::asm!("hlt", options(nomem, nostack)); }
                        }
                    }
                    
                    if ch != 0 {
                        // Echo character to screen
                        unsafe { 
                            core::ptr::write_volatile(vga_buffer.add(cursor_pos), 0x0F00 | ch as u16);
                        }
                        cursor_pos += 1;
                        if cursor_pos >= 80 * 24 {
                            cursor_pos = 80 * 7;
                        }
                    }
                }
            }
            
            // Small delay
            for _ in 0..1000 {
                core::hint::spin_loop();
            }
        }
    }

    // Should never reach here
    loop { hlt(); }
}

/// Run the graphical desktop engine (framebuffer mode)
fn run_graphical_desktop() -> ! {
    serial_print("[FastOS] GFX: shell init...\r\n");
    desktop_engine::shell::init();

    serial_print("[FastOS] GFX: wallpaper...\r\n");
    desktop_engine::shell::draw_wallpaper();

    serial_print("[FastOS] GFX: icons...\r\n");
    desktop_engine::shell::draw_icons();

    serial_print("[FastOS] GFX: taskbar...\r\n");
    desktop_engine::shell::draw_taskbar();

    serial_print("[FastOS] GFX: desktop rendered!\r\n");

    // Simple idle loop — no cursor/mouse/keyboard yet.
    // With IF=0 (interrupts disabled), HLT would freeze the VM forever.
    serial_print("[FastOS] GFX: entering idle loop (spin)\r\n");
    loop {
        core::hint::spin_loop();
    }
}

/// Redraw the full desktop (wallpaper + icons + taskbar + optional start menu + cursor)
fn redraw_desktop() {
    desktop_engine::shell::draw_wallpaper();
    desktop_engine::shell::draw_icons();
    desktop_engine::shell::draw_taskbar();
    if desktop_engine::shell::is_start_menu_open() {
        desktop_engine::shell::draw_start_menu();
    }
    desktop_engine::cursor::show();
}

// ============================================================
// Hardware functions — inline assembly (bare-metal)
// ============================================================
#[inline(always)]
pub fn hlt() {
    unsafe { core::arch::asm!("hlt", options(nomem, nostack)); }
}

#[inline(always)]
pub fn outb(port: u16, value: u8) {
    unsafe { core::arch::asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack)); }
}

#[inline(always)]
pub fn inb(port: u16) -> u8 {
    let value: u8;
    unsafe { core::arch::asm!("in al, dx", in("dx") port, out("al") value, options(nomem, nostack)); }
    value
}

#[inline(always)]
pub fn cli() {
    unsafe { core::arch::asm!("cli", options(nomem, nostack)); }
}

#[inline(always)]
pub fn sti() {
    unsafe { core::arch::asm!("sti", options(nomem, nostack)); }
}
