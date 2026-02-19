// ============================================================
// FastOS — System Info App (Rust)
// ============================================================
// Displays system information inside a window.
// Shows OS, CPU, memory, format, and stack details.
// ============================================================

use crate::vga::{VgaWriter, Color};
use crate::keyboard::Keyboard;
use crate::window::WindowManager;

pub fn run(vga: &mut VgaWriter, kb: &mut Keyboard, wm: &mut WindowManager) {
    wm.draw_top(vga);

    let win = wm.top().unwrap();
    let (cr, cc, cw, _ch) = win.content_area();

    // Title
    vga.write_str_at(cr, cc + 2, "System Information", Color::Black, Color::LightGrey);
    vga.fill_rect(cr + 1, cc + 1, cw - 2, 1, 0xC4, Color::DarkGrey, Color::LightGrey);

    // System info rows
    let mut row = cr + 2;

    draw_info_row(vga, row, cc + 2, "OS Name:", "FastOS v1.0.0", Color::Blue);
    row += 1;
    draw_info_row(vga, row, cc + 2, "Format:", "FsOS (magic: 0x46734F53)", Color::Blue);
    row += 1;
    draw_info_row(vga, row, cc + 2, "Binary:", "Not PE. Not ELF. FsOS.", Color::Blue);
    row += 2;

    draw_info_row(vga, row, cc + 2, "CPU Mode:", "64-bit Long Mode (x86_64)", Color::Cyan);
    row += 1;
    draw_info_row(vga, row, cc + 2, "Scaling:", "16-bit -> 32-bit -> 64-bit", Color::Cyan);
    row += 1;
    draw_info_row(vga, row, cc + 2, "Kernel:", "0x100000 (1 MB)", Color::Cyan);
    row += 1;
    draw_info_row(vga, row, cc + 2, "Stack:", "0x90000", Color::Cyan);
    row += 2;

    draw_info_row(vga, row, cc + 2, "Display:", "VGA Text 80x25 (16 colors)", Color::Green);
    row += 1;
    draw_info_row(vga, row, cc + 2, "Input:", "PS/2 Keyboard (Scancode Set 1)", Color::Green);
    row += 2;

    // Stack info
    vga.write_str_at(row, cc + 2, "Technology Stack:", Color::Black, Color::LightGrey);
    row += 1;
    vga.write_str_at(row, cc + 4, "ADead-BIB", Color::Green, Color::LightGrey);
    vga.write_str_at(row, cc + 16, "- Base / Hardware layer", Color::DarkGrey, Color::LightGrey);
    row += 1;
    vga.write_str_at(row, cc + 4, "Rust", Color::Brown, Color::LightGrey);
    vga.write_str_at(row, cc + 16, "- Security / Kernel logic", Color::DarkGrey, Color::LightGrey);
    row += 1;
    vga.write_str_at(row, cc + 4, "C", Color::Blue, Color::LightGrey);
    vga.write_str_at(row, cc + 16, "- Compatibility / ABI contract", Color::DarkGrey, Color::LightGrey);

    // Wait for ESC
    loop {
        let ch = kb.read_char();
        if ch == 27 { return; }
    }
}

fn draw_info_row(vga: &mut VgaWriter, row: usize, col: usize, label: &str, value: &str, value_color: Color) {
    vga.write_str_at(row, col, label, Color::Black, Color::LightGrey);
    let val_col = col + label.len() + 1;
    vga.write_str_at(row, val_col, value, value_color, Color::LightGrey);
}
