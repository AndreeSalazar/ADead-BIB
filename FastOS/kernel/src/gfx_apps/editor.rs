// ============================================================
// FastOS — Graphical Text Editor (Fase 7)
// ============================================================
// Simple text editor with line display and basic editing.
// ============================================================

use crate::drivers::framebuffer::{self, color};
use crate::desktop_engine::wm;

const MAX_EDITOR_LINES: usize = 64;
const MAX_EDITOR_LINE_LEN: usize = 80;

struct EditorState {
    lines: [[u8; MAX_EDITOR_LINE_LEN]; MAX_EDITOR_LINES],
    line_lens: [usize; MAX_EDITOR_LINES],
    line_count: usize,
    cursor_row: usize,
    cursor_col: usize,
    scroll_offset: usize,
    modified: bool,
}

static mut EDITOR: EditorState = EditorState {
    lines: [[0u8; MAX_EDITOR_LINE_LEN]; MAX_EDITOR_LINES],
    line_lens: [0; MAX_EDITOR_LINES],
    line_count: 1,
    cursor_row: 0,
    cursor_col: 0,
    scroll_offset: 0,
    modified: false,
};

/// Open an editor window
pub fn open() -> wm::WinId {
    let id = wm::create_window("Editor", 120, 70, 520, 380);
    unsafe {
        EDITOR.line_count = 1;
        EDITOR.line_lens[0] = 0;
        EDITOR.cursor_row = 0;
        EDITOR.cursor_col = 0;
        EDITOR.scroll_offset = 0;
        EDITOR.modified = false;
    }
    draw_content(id);
    id
}

/// Process a keypress
pub fn on_key(win_id: wm::WinId, key: u8) {
    unsafe {
        match key {
            b'\n' => {
                // Insert new line
                if EDITOR.line_count < MAX_EDITOR_LINES - 1 {
                    // Shift lines down
                    let mut i = EDITOR.line_count;
                    while i > EDITOR.cursor_row + 1 {
                        EDITOR.lines[i] = EDITOR.lines[i - 1];
                        EDITOR.line_lens[i] = EDITOR.line_lens[i - 1];
                        i -= 1;
                    }
                    // Split current line at cursor
                    let row = EDITOR.cursor_row;
                    let col = EDITOR.cursor_col;
                    let rest_len = EDITOR.line_lens[row] - col;
                    EDITOR.lines[row + 1] = [0u8; MAX_EDITOR_LINE_LEN];
                    EDITOR.lines[row + 1][..rest_len].copy_from_slice(&EDITOR.lines[row][col..col + rest_len]);
                    EDITOR.line_lens[row + 1] = rest_len;
                    EDITOR.line_lens[row] = col;
                    EDITOR.line_count += 1;
                    EDITOR.cursor_row += 1;
                    EDITOR.cursor_col = 0;
                    EDITOR.modified = true;
                }
            }
            8 => { // Backspace
                if EDITOR.cursor_col > 0 {
                    let row = EDITOR.cursor_row;
                    let col = EDITOR.cursor_col;
                    // Shift chars left
                    for i in (col - 1)..EDITOR.line_lens[row].saturating_sub(1) {
                        EDITOR.lines[row][i] = EDITOR.lines[row][i + 1];
                    }
                    EDITOR.line_lens[row] = EDITOR.line_lens[row].saturating_sub(1);
                    EDITOR.cursor_col -= 1;
                    EDITOR.modified = true;
                } else if EDITOR.cursor_row > 0 {
                    // Merge with previous line
                    let prev = EDITOR.cursor_row - 1;
                    let prev_len = EDITOR.line_lens[prev];
                    let cur = EDITOR.cursor_row;
                    let cur_len = EDITOR.line_lens[cur];
                    if prev_len + cur_len <= MAX_EDITOR_LINE_LEN {
                        EDITOR.lines[prev][prev_len..prev_len + cur_len]
                            .copy_from_slice(&EDITOR.lines[cur][..cur_len]);
                        EDITOR.line_lens[prev] = prev_len + cur_len;
                        // Shift lines up
                        for i in cur..(EDITOR.line_count - 1) {
                            EDITOR.lines[i] = EDITOR.lines[i + 1];
                            EDITOR.line_lens[i] = EDITOR.line_lens[i + 1];
                        }
                        EDITOR.line_count -= 1;
                        EDITOR.cursor_row -= 1;
                        EDITOR.cursor_col = prev_len;
                        EDITOR.modified = true;
                    }
                }
            }
            c if c >= 0x20 && c < 0x7F => {
                let row = EDITOR.cursor_row;
                let col = EDITOR.cursor_col;
                if EDITOR.line_lens[row] < MAX_EDITOR_LINE_LEN - 1 {
                    // Shift chars right
                    let mut i = EDITOR.line_lens[row];
                    while i > col {
                        EDITOR.lines[row][i] = EDITOR.lines[row][i - 1];
                        i -= 1;
                    }
                    EDITOR.lines[row][col] = c;
                    EDITOR.line_lens[row] += 1;
                    EDITOR.cursor_col += 1;
                    EDITOR.modified = true;
                }
            }
            _ => {}
        }
        draw_content(win_id);
    }
}

fn draw_content(win_id: wm::WinId) {
    if let Some((cx, cy, cw, ch)) = wm::get_content_rect(win_id) {
        framebuffer::fill_rect(cx, cy, cw, ch, color::SURFACE);

        // Toolbar
        framebuffer::fill_rect(cx, cy, cw, 24, color::BACKGROUND);
        framebuffer::draw_hline(cx, cy + 24, cw, color::BORDER);
        framebuffer::draw_string(cx + 8, cy + 4, "File  Edit  View", color::TEXT_SECONDARY, color::BACKGROUND);

        unsafe {
            let modified_str = if EDITOR.modified { " [modified]" } else { "" };
            framebuffer::draw_string(cx + cw - 100, cy + 4, modified_str, color::WARNING, color::BACKGROUND);
        }

        // Line number gutter
        let gutter_w = 32u32;
        let text_y = cy + 28;
        let line_h = 16u32;
        let visible_lines = ((ch - 28) / line_h) as usize;

        unsafe {
            // Auto-scroll to keep cursor visible
            if EDITOR.cursor_row >= EDITOR.scroll_offset + visible_lines {
                EDITOR.scroll_offset = EDITOR.cursor_row - visible_lines + 1;
            }
            if EDITOR.cursor_row < EDITOR.scroll_offset {
                EDITOR.scroll_offset = EDITOR.cursor_row;
            }

            for vi in 0..visible_lines {
                let line_idx = EDITOR.scroll_offset + vi;
                if line_idx >= EDITOR.line_count { break; }

                let ly = text_y + vi as u32 * line_h;

                // Line number
                framebuffer::fill_rect(cx, ly, gutter_w, line_h, 0xFFF0F0F0);
                let mut num_buf = [0u8; 4];
                let num_len = fmt_usize_small(&mut num_buf, line_idx + 1);
                let num_x = cx + gutter_w - (num_len as u32 * 8) - 4;
                for (j, &b) in num_buf[..num_len].iter().enumerate() {
                    framebuffer::draw_char(num_x + j as u32 * 8, ly, b, color::TEXT_SECONDARY, 0xFFF0F0F0);
                }

                // Line content
                let line = &EDITOR.lines[line_idx][..EDITOR.line_lens[line_idx]];
                let mut tx = cx + gutter_w + 4;
                for &ch_byte in line {
                    if tx + 8 > cx + cw { break; }
                    framebuffer::draw_char(tx, ly, ch_byte, color::TEXT_PRIMARY, color::SURFACE);
                    tx += 8;
                }

                // Cursor
                if line_idx == EDITOR.cursor_row {
                    let cursor_x = cx + gutter_w + 4 + EDITOR.cursor_col as u32 * 8;
                    framebuffer::fill_rect(cursor_x, ly, 2, line_h, color::ACCENT);
                }
            }
        }

        // Status bar
        let sy = cy + ch - 18;
        framebuffer::fill_rect(cx, sy, cw, 18, color::ACCENT);
        unsafe {
            let mut buf = [0u8; 24];
            let prefix = b"Ln ";
            buf[..3].copy_from_slice(prefix);
            let mut pos = 3;
            pos += fmt_usize_small(&mut buf[pos..], EDITOR.cursor_row + 1);
            buf[pos] = b',';
            pos += 1;
            let col_prefix = b" Col ";
            buf[pos..pos+5].copy_from_slice(col_prefix);
            pos += 5;
            pos += fmt_usize_small(&mut buf[pos..], EDITOR.cursor_col + 1);
            for (i, &b) in buf[..pos].iter().enumerate() {
                framebuffer::draw_char(cx + 8 + i as u32 * 8, sy + 1, b, color::WHITE, color::ACCENT);
            }
        }
    }
}

fn fmt_usize_small(buf: &mut [u8], val: usize) -> usize {
    if val == 0 { buf[0] = b'0'; return 1; }
    let mut tmp = [0u8; 8];
    let mut n = val;
    let mut i = 0;
    while n > 0 && i < 8 { tmp[i] = b'0' + (n % 10) as u8; n /= 10; i += 1; }
    for j in 0..i { buf[j] = tmp[i - 1 - j]; }
    i
}
