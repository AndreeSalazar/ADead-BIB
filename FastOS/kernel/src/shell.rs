// ============================================================
// FastOS — Shell (Rust)
// ============================================================
// Interactive command-line shell for FastOS.
// Personality: green terminal, "fastos>" prompt.
//
// Commands:
//   help     — Show available commands
//   info     — System information
//   clear    — Clear screen
//   ver      — FastOS version
//   reboot   — Reboot system
//   halt     — Shutdown
//   echo     — Echo text
//   about    — About FastOS
// ============================================================

use crate::vga::{VgaWriter, Color};
use crate::keyboard::Keyboard;

pub fn run_shell(vga: &mut VgaWriter) {
    let mut kb = Keyboard::new();

    // Shell banner
    vga.set_color(Color::LightGreen, Color::Black);
    vga.write_str("FastOS Shell v1.0\n");
    vga.set_color(Color::LightGrey, Color::Black);
    vga.write_str("Type 'help' for commands.\n\n");

    let mut cmd_buf = [0u8; 128];

    loop {
        // Prompt
        vga.set_color(Color::LightGreen, Color::Black);
        vga.write_str("fastos");
        vga.set_color(Color::White, Color::Black);
        vga.write_str("> ");
        vga.set_color(Color::LightGrey, Color::Black);

        // Read command
        let len = read_line_echo(vga, &mut kb, &mut cmd_buf);

        if len == 0 {
            continue;
        }

        // Parse and execute
        let cmd = &cmd_buf[..len];
        execute_command(vga, cmd);
    }
}

fn read_line_echo(vga: &mut VgaWriter, kb: &mut Keyboard, buf: &mut [u8]) -> usize {
    let mut pos = 0;
    loop {
        let ch = kb.read_char();
        match ch {
            b'\n' => {
                vga.write_char(b'\n');
                return pos;
            }
            8 => {
                // Backspace
                if pos > 0 {
                    pos -= 1;
                    vga.backspace();
                }
            }
            _ => {
                if pos < buf.len() {
                    buf[pos] = ch;
                    pos += 1;
                    vga.write_char(ch);
                }
            }
        }
    }
}

fn execute_command(vga: &mut VgaWriter, cmd: &[u8]) {
    if starts_with(cmd, b"help") {
        cmd_help(vga);
    } else if starts_with(cmd, b"info") {
        cmd_info(vga);
    } else if starts_with(cmd, b"clear") {
        vga.set_color(Color::LightGreen, Color::Black);
        vga.clear();
    } else if starts_with(cmd, b"ver") {
        cmd_version(vga);
    } else if starts_with(cmd, b"about") {
        cmd_about(vga);
    } else if starts_with(cmd, b"halt") {
        vga.set_color(Color::Yellow, Color::Black);
        vga.write_str("Halting system...\n");
        unsafe {
            core::arch::asm!("cli");
            core::arch::asm!("hlt");
        }
    } else if starts_with(cmd, b"reboot") {
        vga.set_color(Color::Yellow, Color::Black);
        vga.write_str("Rebooting...\n");
        // Triple fault to reboot (write to keyboard controller)
        crate::outb(0x64, 0xFE);
    } else if starts_with(cmd, b"echo ") {
        // Echo everything after "echo "
        vga.set_color(Color::LightGrey, Color::Black);
        for &b in &cmd[5..] {
            vga.write_char(b);
        }
        vga.write_char(b'\n');
    } else {
        vga.set_color(Color::LightRed, Color::Black);
        vga.write_str("Unknown command: ");
        vga.set_color(Color::White, Color::Black);
        for &b in cmd {
            vga.write_char(b);
        }
        vga.write_char(b'\n');
        vga.set_color(Color::LightGrey, Color::Black);
        vga.write_str("Type 'help' for available commands.\n");
    }
}

fn cmd_help(vga: &mut VgaWriter) {
    vga.set_color(Color::White, Color::Black);
    vga.write_str("FastOS Commands:\n");
    vga.set_color(Color::LightGreen, Color::Black);
    vga.write_str("  help    ");
    vga.set_color(Color::LightGrey, Color::Black);
    vga.write_str("- Show this help\n");
    vga.set_color(Color::LightGreen, Color::Black);
    vga.write_str("  info    ");
    vga.set_color(Color::LightGrey, Color::Black);
    vga.write_str("- System information\n");
    vga.set_color(Color::LightGreen, Color::Black);
    vga.write_str("  ver     ");
    vga.set_color(Color::LightGrey, Color::Black);
    vga.write_str("- FastOS version\n");
    vga.set_color(Color::LightGreen, Color::Black);
    vga.write_str("  about   ");
    vga.set_color(Color::LightGrey, Color::Black);
    vga.write_str("- About FastOS\n");
    vga.set_color(Color::LightGreen, Color::Black);
    vga.write_str("  clear   ");
    vga.set_color(Color::LightGrey, Color::Black);
    vga.write_str("- Clear screen\n");
    vga.set_color(Color::LightGreen, Color::Black);
    vga.write_str("  echo    ");
    vga.set_color(Color::LightGrey, Color::Black);
    vga.write_str("- Echo text\n");
    vga.set_color(Color::LightGreen, Color::Black);
    vga.write_str("  halt    ");
    vga.set_color(Color::LightGrey, Color::Black);
    vga.write_str("- Shutdown system\n");
    vga.set_color(Color::LightGreen, Color::Black);
    vga.write_str("  reboot  ");
    vga.set_color(Color::LightGrey, Color::Black);
    vga.write_str("- Reboot system\n");
}

fn cmd_info(vga: &mut VgaWriter) {
    vga.set_color(Color::White, Color::Black);
    vga.write_str("System Information:\n");
    vga.set_color(Color::LightCyan, Color::Black);
    vga.write_str("  OS:       FastOS v1.0\n");
    vga.write_str("  Format:   FsOS (magic: 0x46734F53)\n");
    vga.write_str("  CPU Mode: 64-bit Long Mode\n");
    vga.write_str("  Kernel:   0x100000 (1MB)\n");
    vga.write_str("  VGA:      80x25 text mode\n");
    vga.write_str("  Stack:    ADead-BIB + Rust + C\n");
    vga.write_str("  No ASM. No PE. No ELF.\n");
}

fn cmd_version(vga: &mut VgaWriter) {
    vga.set_color(Color::LightGreen, Color::Black);
    vga.write_str("FastOS v1.0.0 (FsOS format)\n");
    vga.set_color(Color::LightGrey, Color::Black);
    vga.write_str("Built with ADead-BIB compiler\n");
}

fn cmd_about(vga: &mut VgaWriter) {
    vga.set_color(Color::White, Color::Black);
    vga.write_str("About FastOS:\n");
    vga.set_color(Color::LightGrey, Color::Black);
    vga.write_str("  FastOS is a custom operating system that uses\n");
    vga.write_str("  its own binary format (FsOS) instead of PE or ELF.\n\n");
    vga.set_color(Color::LightGreen, Color::Black);
    vga.write_str("  ADead-BIB ");
    vga.set_color(Color::LightGrey, Color::Black);
    vga.write_str("handles all hardware interaction.\n");
    vga.set_color(Color::Yellow, Color::Black);
    vga.write_str("  Rust      ");
    vga.set_color(Color::LightGrey, Color::Black);
    vga.write_str("provides memory safety and kernel logic.\n");
    vga.set_color(Color::LightBlue, Color::Black);
    vga.write_str("  C         ");
    vga.set_color(Color::LightGrey, Color::Black);
    vga.write_str("ensures ABI compatibility.\n\n");
    vga.write_str("  Author: Eddi Andree Salazar Matos\n");
}

/// Compare byte slices (prefix match)
fn starts_with(haystack: &[u8], needle: &[u8]) -> bool {
    if haystack.len() < needle.len() {
        return false;
    }
    &haystack[..needle.len()] == needle
}
