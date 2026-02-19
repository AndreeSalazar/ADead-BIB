// ============================================================
// FastOS — Text Editor App (Rust)
// ============================================================
// Simple text editor inside a window.
// Supports typing, backspace, newlines, and basic navigation.
// ============================================================

use crate::vga::{VgaWriter, Color};
use crate::keyboard::Keyboard;
use crate::window::WindowManager;

const MAX_LINES: usize = 16;
const MAX_LINE_LEN: usize = 64;

pub fn run(vga: &mut VgaWriter, kb: &mut Keyboard, wm: &mut WindowManager) {
    wm.draw_top(vga);

    let win = wm.top().unwrap();
    let (cr, cc, cw, _ch) = win.content_area();

    // Editor header
    vga.write_str_at(cr, cc + 1, "Text Editor - untitled.txt", Color::Black, Color::LightGrey);
    vga.fill_rect(cr + 1, cc + 1, cw - 2, 1, 0xC4, Color::DarkGrey, Color::LightGrey);

    // Text buffer
    let mut lines = [[b' '; MAX_LINE_LEN]; MAX_LINES];
    let mut line_lens = [0usize; MAX_LINES];
    let mut cur_line: usize = 0;
    let mut cur_col: usize = 0;

    // Draw initial cursor
    draw_editor_content(vga, &lines, &line_lens, cr + 2, cc + 1, cw - 2, cur_line, cur_col);

    // Status bar
    let status_row = cr + MAX_LINES + 3;
    draw_status(vga, status_row, cc, cw, cur_line, cur_col);

    loop {
        let ch = kb.read_char();
        match ch {
            27 => return, // ESC
            b'\n' => {
                if cur_line < MAX_LINES - 1 {
                    cur_line += 1;
                    cur_col = 0;
                }
            }
            8 => {
                // Backspace
                if cur_col > 0 {
                    cur_col -= 1;
                    lines[cur_line][cur_col] = b' ';
                    if line_lens[cur_line] > 0 {
                        line_lens[cur_line] -= 1;
                    }
                } else if cur_line > 0 {
                    cur_line -= 1;
                    cur_col = line_lens[cur_line];
                }
            }
            b'\t' => {
                // Tab = 4 spaces
                let spaces = 4 - (cur_col % 4);
                for _ in 0..spaces {
                    if cur_col < MAX_LINE_LEN {
                        lines[cur_line][cur_col] = b' ';
                        cur_col += 1;
                        if cur_col > line_lens[cur_line] {
                            line_lens[cur_line] = cur_col;
                        }
                    }
                }
            }
            _ => {
                if cur_col < MAX_LINE_LEN {
                    lines[cur_line][cur_col] = ch;
                    cur_col += 1;
                    if cur_col > line_lens[cur_line] {
                        line_lens[cur_line] = cur_col;
                    }
                }
            }
        }

        draw_editor_content(vga, &lines, &line_lens, cr + 2, cc + 1, cw - 2, cur_line, cur_col);
        draw_status(vga, status_row, cc, cw, cur_line, cur_col);
    }
}

fn draw_editor_content(
    vga: &mut VgaWriter,
    lines: &[[u8; MAX_LINE_LEN]; MAX_LINES],
    line_lens: &[usize; MAX_LINES],
    start_row: usize,
    start_col: usize,
    width: usize,
    cur_line: usize,
    cur_col: usize,
) {
    for (i, line) in lines.iter().enumerate() {
        let row = start_row + i;
        let is_cur = i == cur_line;

        // Line number
        let ln = i + 1;
        let d1 = if ln >= 10 { b'0' + (ln / 10) as u8 } else { b' ' };
        let d2 = b'0' + (ln % 10) as u8;
        vga.put_char_at(row, start_col, d1, Color::DarkGrey, Color::LightGrey);
        vga.put_char_at(row, start_col + 1, d2, Color::DarkGrey, Color::LightGrey);
        vga.put_char_at(row, start_col + 2, 0xB3, Color::DarkGrey, Color::LightGrey);

        // Content
        let content_col = start_col + 3;
        let max_chars = if width > 5 { width - 5 } else { 1 };
        for j in 0..max_chars {
            let ch = if j < line_lens[i] { line[j] } else { b' ' };
            let is_cursor = is_cur && j == cur_col;
            let fg = if is_cursor { Color::Black } else { Color::Black };
            let bg = if is_cursor { Color::LightGreen } else { Color::White };
            vga.put_char_at(row, content_col + j, ch, fg, bg);
        }
    }
}

fn draw_status(vga: &mut VgaWriter, row: usize, cc: usize, cw: usize, line: usize, col: usize) {
    vga.fill_rect(row, cc + 1, cw - 2, 1, b' ', Color::White, Color::Cyan);
    vga.write_str_at(row, cc + 2, "Ln:", Color::White, Color::Cyan);
    write_num_at(vga, row, cc + 5, line + 1, Color::White, Color::Cyan);
    vga.write_str_at(row, cc + 8, "Col:", Color::White, Color::Cyan);
    write_num_at(vga, row, cc + 12, col + 1, Color::White, Color::Cyan);
    vga.write_str_at(row, cc + 20, "ESC:Close", Color::Yellow, Color::Cyan);
}

fn write_num_at(vga: &mut VgaWriter, row: usize, col: usize, val: usize, fg: Color, bg: Color) {
    let mut buf = [0u8; 5];
    let mut v = val;
    let mut i = 0;
    if v == 0 {
        vga.put_char_at(row, col, b'0', fg, bg);
        return;
    }
    while v > 0 {
        buf[i] = b'0' + (v % 10) as u8;
        v /= 10;
        i += 1;
    }
    let mut c = col;
    while i > 0 {
        i -= 1;
        vga.put_char_at(row, c, buf[i], fg, bg);
        c += 1;
    }
}
