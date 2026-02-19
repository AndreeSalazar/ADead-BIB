// ============================================================
// FastOS — Start Menu (Rust)
// ============================================================
// Windows-style Start menu popup from taskbar.
// Shows app list, user info, shutdown option.
// ============================================================

use crate::vga::{VgaWriter, Color};
use crate::keyboard::Keyboard;
use crate::window::WindowManager;

const MENU_ROW: usize = 11;
const MENU_COL: usize = 1;
const MENU_W: usize = 28;
const MENU_H: usize = 13;

struct MenuItem {
    label: &'static str,
    key: u8,
    app_idx: i8, // -1 = shutdown, -2 = about
}

const ITEMS: [MenuItem; 8] = [
    MenuItem { label: "1. Terminal",     key: b'1', app_idx: 0 },
    MenuItem { label: "2. File Manager", key: b'2', app_idx: 1 },
    MenuItem { label: "3. Text Editor",  key: b'3', app_idx: 2 },
    MenuItem { label: "4. Calculator",   key: b'4', app_idx: 3 },
    MenuItem { label: "5. System Info",  key: b'5', app_idx: 4 },
    MenuItem { label: "6. Settings",     key: b'6', app_idx: 5 },
    MenuItem { label: "A. About FastOS", key: b'a', app_idx: -2 },
    MenuItem { label: "Q. Shutdown",     key: b'q', app_idx: -1 },
];

pub fn show_start_menu(vga: &mut VgaWriter, kb: &mut Keyboard, wm: &mut WindowManager) {
    let mut selected: usize = 0;

    loop {
        draw_menu(vga, selected);

        let ch = kb.read_char();
        match ch {
            27 => return, // ESC — close menu
            b'w' => {
                if selected > 0 { selected -= 1; }
            }
            b's' => {
                if selected < ITEMS.len() - 1 { selected += 1; }
            }
            b'\n' => {
                let item = &ITEMS[selected];
                handle_selection(vga, kb, wm, item.app_idx);
                return;
            }
            _ => {
                // Check hotkeys
                for (i, item) in ITEMS.iter().enumerate() {
                    if ch == item.key || ch == item.key.to_ascii_uppercase() {
                        handle_selection(vga, kb, wm, item.app_idx);
                        return;
                    }
                    let _ = i;
                }
            }
        }
    }
}

fn draw_menu(vga: &mut VgaWriter, selected: usize) {
    // Menu background
    vga.fill_rect(MENU_ROW, MENU_COL, MENU_W, MENU_H, b' ', Color::White, Color::LightGrey);

    // Border
    vga.draw_hline(MENU_ROW, MENU_COL, MENU_W, 0xC4, Color::DarkGrey, Color::LightGrey);
    vga.draw_hline(MENU_ROW + MENU_H - 1, MENU_COL, MENU_W, 0xC4, Color::DarkGrey, Color::LightGrey);
    vga.draw_vline(MENU_ROW, MENU_COL, MENU_H, 0xB3, Color::DarkGrey, Color::LightGrey);
    vga.draw_vline(MENU_ROW, MENU_COL + MENU_W - 1, MENU_H, 0xB3, Color::DarkGrey, Color::LightGrey);

    // Corners
    vga.put_char_at(MENU_ROW, MENU_COL, 0xDA, Color::DarkGrey, Color::LightGrey);
    vga.put_char_at(MENU_ROW, MENU_COL + MENU_W - 1, 0xBF, Color::DarkGrey, Color::LightGrey);
    vga.put_char_at(MENU_ROW + MENU_H - 1, MENU_COL, 0xC0, Color::DarkGrey, Color::LightGrey);
    vga.put_char_at(MENU_ROW + MENU_H - 1, MENU_COL + MENU_W - 1, 0xD9, Color::DarkGrey, Color::LightGrey);

    // Title
    vga.write_str_at(MENU_ROW + 1, MENU_COL + 2, "FastOS Start", Color::White, Color::Cyan);
    vga.fill_rect(MENU_ROW + 1, MENU_COL + 1, MENU_W - 2, 1, b' ', Color::White, Color::Cyan);
    vga.write_str_at(MENU_ROW + 1, MENU_COL + 2, "FastOS Start", Color::White, Color::Cyan);

    // Separator
    vga.draw_hline(MENU_ROW + 2, MENU_COL + 1, MENU_W - 2, 0xC4, Color::DarkGrey, Color::LightGrey);

    // Menu items
    for (i, item) in ITEMS.iter().enumerate() {
        let row = MENU_ROW + 3 + i;
        let is_sel = i == selected;
        let fg = if is_sel { Color::White } else { Color::Black };
        let bg = if is_sel { Color::Cyan } else { Color::LightGrey };

        vga.fill_rect(row, MENU_COL + 1, MENU_W - 2, 1, b' ', fg, bg);
        vga.write_str_at(row, MENU_COL + 3, item.label, fg, bg);

        if is_sel {
            vga.put_char_at(row, MENU_COL + 1, 0x10, fg, bg); // arrow
        }
    }
}

fn handle_selection(vga: &mut VgaWriter, kb: &mut Keyboard, wm: &mut WindowManager, app_idx: i8) {
    match app_idx {
        -1 => {
            // Shutdown
            shutdown_from_menu(vga);
        }
        -2 => {
            // About
            show_about(vga, kb, wm);
        }
        idx if idx >= 0 && idx <= 5 => {
            crate::desktop::open_app(vga, kb, wm, idx as usize);
        }
        _ => {}
    }
}

fn shutdown_from_menu(vga: &mut VgaWriter) {
    vga.clear_with(Color::White, Color::Black);
    vga.write_str_at(10, 28, "Shutting down...", Color::White, Color::Black);
    vga.write_str_at(12, 25, "FastOS v1.0 - Goodbye!", Color::LightGreen, Color::Black);

    for _ in 0..10_000_000u64 {
        unsafe { core::hint::spin_loop(); }
    }

    crate::outb(0x604, 0x00);
    crate::outb(0x604, 0x20);

    loop { crate::hlt(); }
}

fn show_about(vga: &mut VgaWriter, kb: &mut Keyboard, _wm: &mut WindowManager) {
    use crate::window::Window;

    let win = Window::new("About FastOS", 4, 15, 50, 16);
    win.draw(vga);

    let (cr, cc, _cw, _ch) = win.content_area();

    vga.write_str_at(cr + 1, cc + 2, "FastOS v1.0.0", Color::Black, Color::LightGrey);
    vga.write_str_at(cr + 2, cc + 2, "Format: FsOS (0x46734F53)", Color::DarkGrey, Color::LightGrey);
    vga.write_str_at(cr + 4, cc + 2, "Built with:", Color::Black, Color::LightGrey);
    vga.write_str_at(cr + 5, cc + 4, "ADead-BIB  - Base / Hardware", Color::Green, Color::LightGrey);
    vga.write_str_at(cr + 6, cc + 4, "Rust       - Security / Kernel", Color::Brown, Color::LightGrey);
    vga.write_str_at(cr + 7, cc + 4, "C          - Compatibility / ABI", Color::Blue, Color::LightGrey);
    vga.write_str_at(cr + 9, cc + 2, "No ASM. No PE. No ELF.", Color::DarkGrey, Color::LightGrey);
    vga.write_str_at(cr + 10, cc + 2, "64-bit Long Mode", Color::DarkGrey, Color::LightGrey);
    vga.write_str_at(cr + 12, cc + 2, "Author: Eddi Andree Salazar Matos", Color::Black, Color::LightGrey);

    // Wait for ESC
    loop {
        let ch = kb.read_char();
        if ch == 27 || ch == b'\n' || ch == b'q' {
            return;
        }
    }
}
