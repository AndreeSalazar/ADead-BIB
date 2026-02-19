// ============================================================
// FastOS — Terminal App (Rust)
// ============================================================
// Full terminal emulator inside a window.
// Reuses shell commands but runs inside the window manager.
// ============================================================

use crate::vga::{VgaWriter, Color};
use crate::keyboard::Keyboard;
use crate::window::WindowManager;

pub fn run(vga: &mut VgaWriter, kb: &mut Keyboard, wm: &mut WindowManager) {
    wm.draw_top(vga);

    let win = wm.top().unwrap();
    let (cr, cc, cw, ch) = win.content_area();

    // Terminal header
    vga.write_str_at(cr, cc + 1, "FastOS Terminal v1.0", Color::Green, Color::LightGrey);
    vga.write_str_at(cr + 1, cc + 1, "Type commands. ESC to close.", Color::DarkGrey, Color::LightGrey);

    let mut line_row = cr + 3;
    let mut cmd_buf = [0u8; 64];

    loop {
        if line_row >= cr + ch - 2 {
            // Scroll content area up
            scroll_content(vga, cr + 3, cc + 1, cw - 2, ch - 4);
            line_row = cr + ch - 3;
        }

        // Prompt
        vga.write_str_at(line_row, cc + 1, "fastos>", Color::Green, Color::LightGrey);
        let mut pos = 0;
        let input_col = cc + 9;

        loop {
            let c = kb.read_char();
            match c {
                27 => return, // ESC
                b'\n' => break,
                8 => {
                    if pos > 0 {
                        pos -= 1;
                        vga.put_char_at(line_row, input_col + pos, b' ', Color::Black, Color::LightGrey);
                    }
                }
                _ => {
                    if pos < 54 && pos < cmd_buf.len() {
                        cmd_buf[pos] = c;
                        vga.put_char_at(line_row, input_col + pos, c, Color::Black, Color::LightGrey);
                        pos += 1;
                    }
                }
            }
        }

        line_row += 1;

        if pos == 0 {
            continue;
        }

        // Execute command
        let cmd = &cmd_buf[..pos];
        line_row = exec_terminal_cmd(vga, cmd, line_row, cr, cc, cw, ch);
    }
}

fn exec_terminal_cmd(vga: &mut VgaWriter, cmd: &[u8], mut row: usize, cr: usize, cc: usize, cw: usize, ch: usize) -> usize {
    if row >= cr + ch - 2 {
        scroll_content(vga, cr + 3, cc + 1, cw - 2, ch - 4);
        row = cr + ch - 3;
    }

    if starts_with(cmd, b"help") {
        let lines = [
            "Commands: help ver info echo clear exit",
        ];
        for line in &lines {
            vga.write_str_at(row, cc + 2, line, Color::DarkGrey, Color::LightGrey);
            row += 1;
        }
    } else if starts_with(cmd, b"ver") {
        vga.write_str_at(row, cc + 2, "FastOS v1.0.0 (FsOS)", Color::Green, Color::LightGrey);
        row += 1;
    } else if starts_with(cmd, b"info") {
        let lines = [
            "OS: FastOS v1.0  Format: FsOS",
            "CPU: 64-bit Long Mode",
            "Stack: ADead-BIB + Rust + C",
        ];
        for line in &lines {
            if row < cr + ch - 2 {
                vga.write_str_at(row, cc + 2, line, Color::Blue, Color::LightGrey);
                row += 1;
            }
        }
    } else if starts_with(cmd, b"echo ") {
        let msg_start = 5;
        let mut col = cc + 2;
        for &b in &cmd[msg_start..] {
            if col < cc + cw - 2 {
                vga.put_char_at(row, col, b, Color::Black, Color::LightGrey);
                col += 1;
            }
        }
        row += 1;
    } else if starts_with(cmd, b"clear") {
        // Clear content area
        for r in (cr + 3)..(cr + ch - 1) {
            for c in (cc + 1)..(cc + cw - 1) {
                vga.put_char_at(r, c, b' ', Color::Black, Color::LightGrey);
            }
        }
        return cr + 3;
    } else if starts_with(cmd, b"exit") {
        return 0; // Signal to exit (handled by caller checking ESC)
    } else {
        vga.write_str_at(row, cc + 2, "Unknown: ", Color::Red, Color::LightGrey);
        let mut col = cc + 11;
        for &b in cmd {
            if col < cc + cw - 2 {
                vga.put_char_at(row, col, b, Color::Red, Color::LightGrey);
                col += 1;
            }
        }
        row += 1;
    }

    row
}

fn scroll_content(vga: &mut VgaWriter, start_row: usize, start_col: usize, w: usize, h: usize) {
    // Move lines up within content area
    for r in 0..(h - 1) {
        for c in 0..w {
            let src_offset = ((start_row + r + 1) * 80 + start_col + c) * 2;
            let dst_offset = ((start_row + r) * 80 + start_col + c) * 2;
            unsafe {
                let s = (0xB8000 + src_offset) as *const u8;
                let d = (0xB8000 + dst_offset) as *mut u8;
                *d = *s;
                *d.add(1) = *s.add(1);
            }
        }
    }
    // Clear last line
    for c in 0..w {
        vga.put_char_at(start_row + h - 1, start_col + c, b' ', Color::Black, Color::LightGrey);
    }
}

fn starts_with(haystack: &[u8], needle: &[u8]) -> bool {
    if haystack.len() < needle.len() { return false; }
    &haystack[..needle.len()] == needle
}
