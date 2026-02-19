// ============================================================
// FastOS — Graphical Terminal (Fase 7)
// ============================================================
// Framebuffer-based terminal emulator with command execution.
// ============================================================

use crate::drivers::framebuffer::{self, color};
use crate::desktop_engine::wm;

/// Terminal state
const MAX_LINES: usize = 32;
const MAX_LINE_LEN: usize = 80;

struct TermState {
    lines: [[u8; MAX_LINE_LEN]; MAX_LINES],
    line_lens: [usize; MAX_LINES],
    line_count: usize,
    cursor_col: usize,
    cmd_buf: [u8; 64],
    cmd_len: usize,
}

static mut TERM: TermState = TermState {
    lines: [[0u8; MAX_LINE_LEN]; MAX_LINES],
    line_lens: [0; MAX_LINES],
    line_count: 0,
    cursor_col: 0,
    cmd_buf: [0u8; 64],
    cmd_len: 0,
};

/// Open a terminal window and return its window ID
pub fn open() -> wm::WinId {
    let id = wm::create_window("Terminal", 100, 60, 500, 360);
    unsafe {
        TERM.line_count = 0;
        TERM.cmd_len = 0;
        TERM.cursor_col = 0;
        push_line(b"FastOS Terminal v1.0");
        push_line(b"Type 'help' for commands.");
        push_line(b"");
    }
    draw_content(id);
    id
}

/// Process a keypress in the terminal
pub fn on_key(win_id: wm::WinId, key: u8) {
    unsafe {
        match key {
            b'\n' => {
                // Execute command
                let cmd = &TERM.cmd_buf[..TERM.cmd_len];
                let mut prompt = [0u8; 72];
                prompt[0] = b'>';
                prompt[1] = b' ';
                let len = if TERM.cmd_len > 70 { 70 } else { TERM.cmd_len };
                prompt[2..2+len].copy_from_slice(&cmd[..len]);
                push_line_slice(&prompt[..2+len]);
                exec_cmd(cmd);
                TERM.cmd_len = 0;
                TERM.cursor_col = 0;
                draw_content(win_id);
            }
            8 => { // Backspace
                if TERM.cmd_len > 0 {
                    TERM.cmd_len -= 1;
                    TERM.cursor_col = TERM.cmd_len;
                    draw_content(win_id);
                }
            }
            c if c >= 0x20 && c < 0x7F => {
                if TERM.cmd_len < 63 {
                    TERM.cmd_buf[TERM.cmd_len] = c;
                    TERM.cmd_len += 1;
                    TERM.cursor_col = TERM.cmd_len;
                    draw_content(win_id);
                }
            }
            _ => {}
        }
    }
}

fn exec_cmd(cmd: &[u8]) {
    if starts_with(cmd, b"help") {
        push_line(b"Commands: help ver mem clear");
    } else if starts_with(cmd, b"ver") {
        push_line(b"FastOS v1.0 (64-bit, FsOS format)");
    } else if starts_with(cmd, b"mem") {
        let free = crate::kernel_core::memory::free_memory();
        let total = crate::kernel_core::memory::total_memory();
        let mut buf = [0u8; 48];
        let n = fmt_mem(&mut buf, free, total);
        push_line_slice(&buf[..n]);
    } else if starts_with(cmd, b"clear") {
        unsafe { TERM.line_count = 0; }
    } else if cmd.len() > 0 {
        let mut buf = [0u8; 72];
        let prefix = b"Unknown: ";
        buf[..prefix.len()].copy_from_slice(prefix);
        let len = if cmd.len() > 60 { 60 } else { cmd.len() };
        buf[prefix.len()..prefix.len()+len].copy_from_slice(&cmd[..len]);
        push_line_slice(&buf[..prefix.len()+len]);
    }
}

fn push_line(s: &[u8]) {
    push_line_slice(s);
}

fn push_line_slice(s: &[u8]) {
    unsafe {
        if TERM.line_count >= MAX_LINES {
            // Scroll up
            for i in 0..(MAX_LINES - 1) {
                TERM.lines[i] = TERM.lines[i + 1];
                TERM.line_lens[i] = TERM.line_lens[i + 1];
            }
            TERM.line_count = MAX_LINES - 1;
        }
        let idx = TERM.line_count;
        let len = if s.len() > MAX_LINE_LEN { MAX_LINE_LEN } else { s.len() };
        TERM.lines[idx] = [0u8; MAX_LINE_LEN];
        TERM.lines[idx][..len].copy_from_slice(&s[..len]);
        TERM.line_lens[idx] = len;
        TERM.line_count += 1;
    }
}

fn draw_content(win_id: wm::WinId) {
    if let Some((cx, cy, cw, ch)) = wm::get_content_rect(win_id) {
        // Clear content
        framebuffer::fill_rect(cx, cy, cw, ch, 0xFF1E1E1E);

        unsafe {
            let max_rows = (ch / 16) as usize;
            let start = if TERM.line_count > max_rows { TERM.line_count - max_rows } else { 0 };

            for (i, line_idx) in (start..TERM.line_count).enumerate() {
                let y = cy + i as u32 * 16 + 2;
                let line = &TERM.lines[line_idx][..TERM.line_lens[line_idx]];
                let mut x = cx + 4;
                for &ch_byte in line {
                    framebuffer::draw_char(x, y, ch_byte, color::GREEN, 0xFF1E1E1E);
                    x += 8;
                }
            }

            // Draw current input line
            let input_y = cy + ch - 18;
            framebuffer::draw_string(cx + 4, input_y, "> ", color::CYAN, 0xFF1E1E1E);
            let mut x = cx + 20;
            for i in 0..TERM.cmd_len {
                framebuffer::draw_char(x, input_y, TERM.cmd_buf[i], color::WHITE, 0xFF1E1E1E);
                x += 8;
            }
            // Cursor blink
            framebuffer::fill_rect(x, input_y, 8, 16, color::WHITE);
        }
    }
}

fn fmt_mem(buf: &mut [u8], free: usize, total: usize) -> usize {
    let prefix = b"Memory: ";
    buf[..prefix.len()].copy_from_slice(prefix);
    let mut pos = prefix.len();
    pos += fmt_usize(&mut buf[pos..], free / 1024);
    buf[pos] = b'/';
    pos += 1;
    pos += fmt_usize(&mut buf[pos..], total / 1024);
    let suffix = b" KB";
    buf[pos..pos+suffix.len()].copy_from_slice(suffix);
    pos + suffix.len()
}

fn fmt_usize(buf: &mut [u8], val: usize) -> usize {
    if val == 0 { buf[0] = b'0'; return 1; }
    let mut tmp = [0u8; 20];
    let mut n = val;
    let mut i = 0;
    while n > 0 { tmp[i] = b'0' + (n % 10) as u8; n /= 10; i += 1; }
    for j in 0..i { buf[j] = tmp[i - 1 - j]; }
    i
}

fn starts_with(h: &[u8], n: &[u8]) -> bool {
    h.len() >= n.len() && &h[..n.len()] == n
}
