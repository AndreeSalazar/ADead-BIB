// ============================================================
// FastOS — Calculator App (Rust)
// ============================================================
// Simple integer calculator inside a window.
// Supports +, -, *, / with two operands.
// ============================================================

use crate::vga::{VgaWriter, Color};
use crate::keyboard::Keyboard;
use crate::window::WindowManager;

pub fn run(vga: &mut VgaWriter, kb: &mut Keyboard, wm: &mut WindowManager) {
    wm.draw_top(vga);

    let win = wm.top().unwrap();
    let (cr, cc, cw, _ch) = win.content_area();

    draw_calc_ui(vga, cr, cc, cw);

    let mut display_val: i64 = 0;
    let mut stored_val: i64 = 0;
    let mut pending_op: u8 = 0; // '+', '-', '*', '/'
    let mut entering = true;
    let mut fresh = true;

    update_display(vga, cr + 2, cc + 2, cw - 4, display_val);

    loop {
        let ch = kb.read_char();
        match ch {
            27 => return, // ESC
            b'0'..=b'9' => {
                let digit = (ch - b'0') as i64;
                if fresh {
                    display_val = digit;
                    fresh = false;
                } else {
                    display_val = display_val.wrapping_mul(10).wrapping_add(digit);
                }
                entering = true;
                update_display(vga, cr + 2, cc + 2, cw - 4, display_val);
            }
            b'+' | b'-' | b'*' | b'/' => {
                if entering && pending_op != 0 {
                    display_val = calculate(stored_val, display_val, pending_op);
                    update_display(vga, cr + 2, cc + 2, cw - 4, display_val);
                }
                stored_val = display_val;
                pending_op = ch;
                fresh = true;
                entering = false;
                show_op(vga, cr + 3, cc + 2, ch);
            }
            b'=' | b'\n' => {
                if pending_op != 0 {
                    display_val = calculate(stored_val, display_val, pending_op);
                    update_display(vga, cr + 2, cc + 2, cw - 4, display_val);
                    pending_op = 0;
                    stored_val = 0;
                    fresh = true;
                    entering = false;
                    show_op(vga, cr + 3, cc + 2, b' ');
                }
            }
            b'c' | b'C' => {
                display_val = 0;
                stored_val = 0;
                pending_op = 0;
                entering = true;
                fresh = true;
                update_display(vga, cr + 2, cc + 2, cw - 4, display_val);
                show_op(vga, cr + 3, cc + 2, b' ');
            }
            8 => {
                // Backspace — remove last digit
                display_val /= 10;
                update_display(vga, cr + 2, cc + 2, cw - 4, display_val);
            }
            _ => {}
        }
    }
}

fn calculate(a: i64, b: i64, op: u8) -> i64 {
    match op {
        b'+' => a.wrapping_add(b),
        b'-' => a.wrapping_sub(b),
        b'*' => a.wrapping_mul(b),
        b'/' => {
            if b == 0 { 0 } else { a / b }
        }
        _ => b,
    }
}

fn draw_calc_ui(vga: &mut VgaWriter, cr: usize, cc: usize, cw: usize) {
    // Title
    vga.write_str_at(cr, cc + 2, "Calculator", Color::Black, Color::LightGrey);
    vga.fill_rect(cr + 1, cc + 1, cw - 2, 1, 0xC4, Color::DarkGrey, Color::LightGrey);

    // Display area (dark background)
    vga.fill_rect(cr + 2, cc + 1, cw - 2, 2, b' ', Color::LightGreen, Color::Black);

    // Separator
    vga.fill_rect(cr + 4, cc + 1, cw - 2, 1, 0xC4, Color::DarkGrey, Color::LightGrey);

    // Button labels
    let buttons = [
        "  7  8  9  /  ",
        "  4  5  6  *  ",
        "  1  2  3  -  ",
        "  0  C  =  +  ",
    ];
    for (i, row_text) in buttons.iter().enumerate() {
        let row = cr + 5 + i * 2;
        vga.write_str_at(row, cc + 5, row_text, Color::Black, Color::LightGrey);
    }

    // Help
    vga.write_str_at(cr + 11, cc + 2, "Keys: 0-9 + - * / = C", Color::DarkGrey, Color::LightGrey);
    vga.write_str_at(cr + 12, cc + 2, "ESC: Close", Color::DarkGrey, Color::LightGrey);
}

fn update_display(vga: &mut VgaWriter, row: usize, col: usize, width: usize, val: i64) {
    // Clear display
    for c in 0..width {
        vga.put_char_at(row, col + c, b' ', Color::LightGreen, Color::Black);
    }

    // Convert to string (right-aligned)
    let negative = val < 0;
    let mut abs_val = if negative { (0i64).wrapping_sub(val) as u64 } else { val as u64 };
    let mut buf = [0u8; 20];
    let mut i = 0;

    if abs_val == 0 {
        buf[0] = b'0';
        i = 1;
    } else {
        while abs_val > 0 {
            buf[i] = b'0' + (abs_val % 10) as u8;
            abs_val /= 10;
            i += 1;
        }
    }

    if negative {
        buf[i] = b'-';
        i += 1;
    }

    // Right-align
    let start = if width > i { col + width - i } else { col };
    let mut c = start;
    while i > 0 {
        i -= 1;
        vga.put_char_at(row, c, buf[i], Color::LightGreen, Color::Black);
        c += 1;
    }
}

fn show_op(vga: &mut VgaWriter, row: usize, col: usize, op: u8) {
    vga.put_char_at(row, col, b'[', Color::Yellow, Color::Black);
    vga.put_char_at(row, col + 1, op, Color::Yellow, Color::Black);
    vga.put_char_at(row, col + 2, b']', Color::Yellow, Color::Black);
}
