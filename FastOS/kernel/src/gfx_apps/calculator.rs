// ============================================================
// FastOS — Graphical Calculator (Fase 7)
// ============================================================
// Simple 4-function calculator with button grid.
// ============================================================

use crate::drivers::framebuffer::{self, color};
use crate::desktop_engine::wm;

static mut CALC: CalcState = CalcState {
    display: [b' '; 16],
    display_len: 1,
    accumulator: 0,
    current: 0,
    op: 0,
    new_input: true,
};

struct CalcState {
    display: [u8; 16],
    display_len: usize,
    accumulator: i64,
    current: i64,
    op: u8,
    new_input: bool,
}

/// Open a calculator window
pub fn open() -> wm::WinId {
    let id = wm::create_window("Calculator", 300, 120, 240, 320);
    unsafe {
        CALC.display = [b' '; 16];
        CALC.display[0] = b'0';
        CALC.display_len = 1;
        CALC.accumulator = 0;
        CALC.current = 0;
        CALC.op = 0;
        CALC.new_input = true;
    }
    draw_content(id);
    id
}

/// Process a keypress
pub fn on_key(win_id: wm::WinId, key: u8) {
    unsafe {
        match key {
            b'0'..=b'9' => {
                if CALC.new_input {
                    CALC.display_len = 0;
                    CALC.current = 0;
                    CALC.new_input = false;
                }
                if CALC.display_len < 15 {
                    CALC.display[CALC.display_len] = key;
                    CALC.display_len += 1;
                    CALC.current = CALC.current * 10 + (key - b'0') as i64;
                }
            }
            b'+' | b'-' | b'*' | b'/' => {
                do_op();
                CALC.op = key;
                CALC.new_input = true;
            }
            b'=' | b'\n' => {
                do_op();
                CALC.op = 0;
                CALC.new_input = true;
            }
            b'c' | b'C' | 27 => {
                CALC.accumulator = 0;
                CALC.current = 0;
                CALC.op = 0;
                CALC.display[0] = b'0';
                CALC.display_len = 1;
                CALC.new_input = true;
            }
            8 => { // Backspace
                if CALC.display_len > 1 && !CALC.new_input {
                    CALC.display_len -= 1;
                    CALC.current /= 10;
                } else {
                    CALC.display[0] = b'0';
                    CALC.display_len = 1;
                    CALC.current = 0;
                    CALC.new_input = true;
                }
            }
            _ => {}
        }
        draw_content(win_id);
    }
}

unsafe fn do_op() {
    match CALC.op {
        b'+' => CALC.accumulator += CALC.current,
        b'-' => CALC.accumulator -= CALC.current,
        b'*' => CALC.accumulator *= CALC.current,
        b'/' => {
            if CALC.current != 0 {
                CALC.accumulator /= CALC.current;
            }
        }
        _ => { CALC.accumulator = CALC.current; }
    }
    CALC.current = CALC.accumulator;
    // Format result into display
    let val = CALC.accumulator;
    CALC.display_len = fmt_i64(&mut CALC.display, val);
}

fn fmt_i64(buf: &mut [u8; 16], val: i64) -> usize {
    if val == 0 { buf[0] = b'0'; return 1; }
    let mut tmp = [0u8; 16];
    let neg = val < 0;
    let mut n = if neg { -(val as i128) as u64 } else { val as u64 };
    let mut i = 0usize;
    while n > 0 && i < 15 { tmp[i] = b'0' + (n % 10) as u8; n /= 10; i += 1; }
    let mut pos = 0;
    if neg { buf[pos] = b'-'; pos += 1; }
    for j in 0..i { buf[pos] = tmp[i - 1 - j]; pos += 1; }
    pos
}

fn draw_content(win_id: wm::WinId) {
    if let Some((cx, cy, cw, ch)) = wm::get_content_rect(win_id) {
        framebuffer::fill_rect(cx, cy, cw, ch, 0xFF2D2D2D);

        // Display area
        framebuffer::fill_rounded_rect(cx + 8, cy + 8, cw - 16, 48, 6, 0xFF1A1A1A);
        unsafe {
            let text = &CALC.display[..CALC.display_len];
            let tx = cx + cw - 16 - (CALC.display_len as u32 * 8);
            for (i, &ch_byte) in text.iter().enumerate() {
                framebuffer::draw_char(tx + i as u32 * 8, cy + 24, ch_byte, color::WHITE, 0xFF1A1A1A);
            }
        }

        // Button grid (4x5)
        let buttons: [[&str; 4]; 5] = [
            ["C", "(", ")", "/"],
            ["7", "8", "9", "*"],
            ["4", "5", "6", "-"],
            ["1", "2", "3", "+"],
            ["0", ".", "<", "="],
        ];

        let btn_w = (cw - 48) / 4;
        let btn_h = 36u32;
        let grid_y = cy + 68;

        for (row, btns) in buttons.iter().enumerate() {
            for (col, label) in btns.iter().enumerate() {
                let bx = cx + 8 + col as u32 * (btn_w + 8);
                let by = grid_y + row as u32 * (btn_h + 6);

                let bg = match *label {
                    "=" => color::ACCENT,
                    "C" => color::CLOSE_RED,
                    "+" | "-" | "*" | "/" => 0xFF444444,
                    _ => 0xFF3D3D3D,
                };

                framebuffer::fill_rounded_rect(bx, by, btn_w, btn_h, 4, bg);
                let lx = bx + btn_w / 2 - 4;
                let ly = by + btn_h / 2 - 8;
                framebuffer::draw_string(lx, ly, label, color::WHITE, bg);
            }
        }
    }
}
