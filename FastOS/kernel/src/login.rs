// ============================================================
// FastOS — Login Screen (Rust)
// ============================================================
// Windows-style login screen before desktop.
// Shows FastOS logo, username/password fields.
// Default user: "admin" / password: "fastos"
// ============================================================

use crate::vga::{VgaWriter, Color};
use crate::keyboard::Keyboard;

const DEFAULT_USER: &[u8] = b"admin";
const DEFAULT_PASS: &[u8] = b"fastos";

pub fn run_login(vga: &mut VgaWriter) -> bool {
    let mut kb = Keyboard::new();
    let mut attempts = 0u8;

    loop {
        draw_login_screen(vga);

        // Username
        vga.write_str_at(12, 30, "User: ", Color::White, Color::Blue);
        let mut user_buf = [0u8; 32];
        let user_len = read_field(vga, &mut kb, 12, 36, 20, &mut user_buf, false);

        // Password
        vga.write_str_at(14, 26, "Password: ", Color::White, Color::Blue);
        let mut pass_buf = [0u8; 32];
        let pass_len = read_field(vga, &mut kb, 14, 36, 20, &mut pass_buf, true);

        // Validate
        if &user_buf[..user_len] == DEFAULT_USER && &pass_buf[..pass_len] == DEFAULT_PASS {
            // Success animation
            vga.write_str_at(17, 27, "  Welcome to FastOS!  ", Color::White, Color::Green);
            spin_delay();
            return true;
        }

        attempts += 1;
        vga.write_str_at(17, 25, "  Invalid credentials!  ", Color::White, Color::Red);
        if attempts >= 3 {
            vga.write_str_at(18, 22, "  Too many attempts. Rebooting...  ", Color::Yellow, Color::Red);
            spin_delay();
            spin_delay();
            return false;
        }
        vga.write_str_at(18, 28, "  Try again...  ", Color::Yellow, Color::Blue);
        spin_delay();
    }
}

fn draw_login_screen(vga: &mut VgaWriter) {
    // Blue background (like Windows login)
    vga.clear_with(Color::White, Color::Blue);

    // Top bar
    vga.fill_rect(0, 0, 80, 1, b' ', Color::White, Color::Cyan);
    vga.write_str_at(0, 2, "FastOS v1.0", Color::White, Color::Cyan);
    vga.write_str_at(0, 60, "ADead-BIB + Rust + C", Color::Yellow, Color::Cyan);

    // Login box (centered)
    let box_row = 6;
    let box_col = 20;
    let box_w = 40;
    let box_h = 14;

    // Box background
    vga.fill_rect(box_row, box_col, box_w, box_h, b' ', Color::White, Color::Blue);

    // Box border (single line)
    vga.draw_hline(box_row, box_col, box_w, 0xCD, Color::LightCyan, Color::Blue);
    vga.draw_hline(box_row + box_h - 1, box_col, box_w, 0xCD, Color::LightCyan, Color::Blue);
    vga.draw_vline(box_row, box_col, box_h, 0xBA, Color::LightCyan, Color::Blue);
    vga.draw_vline(box_row, box_col + box_w - 1, box_h, 0xBA, Color::LightCyan, Color::Blue);

    // Corners
    vga.put_char_at(box_row, box_col, 0xC9, Color::LightCyan, Color::Blue);
    vga.put_char_at(box_row, box_col + box_w - 1, 0xBB, Color::LightCyan, Color::Blue);
    vga.put_char_at(box_row + box_h - 1, box_col, 0xC8, Color::LightCyan, Color::Blue);
    vga.put_char_at(box_row + box_h - 1, box_col + box_w - 1, 0xBC, Color::LightCyan, Color::Blue);

    // Title
    vga.write_str_at(8, 30, "FastOS Login", Color::White, Color::Blue);

    // Separator
    vga.draw_hline(9, box_col + 1, box_w - 2, 0xC4, Color::LightCyan, Color::Blue);

    // Labels will be drawn by caller

    // Bottom hint
    vga.write_str_at(24, 2, " Default: admin / fastos ", Color::DarkGrey, Color::Blue);
    vga.write_str_at(24, 55, " Press Enter to confirm ", Color::DarkGrey, Color::Blue);
}

fn read_field(vga: &mut VgaWriter, kb: &mut Keyboard, row: usize, col: usize, max_len: usize, buf: &mut [u8], masked: bool) -> usize {
    // Draw input field background
    for i in 0..max_len {
        vga.put_char_at(row, col + i, b'_', Color::LightGrey, Color::Black);
    }

    let mut pos = 0;
    loop {
        let ch = kb.read_char();
        match ch {
            b'\n' => return pos,
            8 => {
                // Backspace
                if pos > 0 {
                    pos -= 1;
                    vga.put_char_at(row, col + pos, b'_', Color::LightGrey, Color::Black);
                }
            }
            _ => {
                if pos < max_len && pos < buf.len() {
                    buf[pos] = ch;
                    let display = if masked { b'*' } else { ch };
                    vga.put_char_at(row, col + pos, display, Color::White, Color::Black);
                    pos += 1;
                }
            }
        }
    }
}

fn spin_delay() {
    for _ in 0..5_000_000u64 {
        unsafe { core::hint::spin_loop(); }
    }
}
