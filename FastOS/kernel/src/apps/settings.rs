// ============================================================
// FastOS — Settings App (Rust)
// ============================================================
// System settings panel inside a window.
// Allows changing theme colors, viewing system config.
// ============================================================

use crate::vga::{VgaWriter, Color};
use crate::keyboard::Keyboard;
use crate::window::WindowManager;

struct SettingItem {
    label: &'static str,
    value: &'static str,
}

const SETTINGS: [SettingItem; 8] = [
    SettingItem { label: "Username",       value: "admin" },
    SettingItem { label: "Hostname",       value: "fastos-pc" },
    SettingItem { label: "Theme",          value: "Classic Green" },
    SettingItem { label: "Resolution",     value: "80x25 Text Mode" },
    SettingItem { label: "Keyboard",       value: "US QWERTY (PS/2)" },
    SettingItem { label: "Language",       value: "English" },
    SettingItem { label: "Boot Mode",      value: "64-bit Long Mode" },
    SettingItem { label: "Auto Login",     value: "Disabled" },
];

pub fn run(vga: &mut VgaWriter, kb: &mut Keyboard, wm: &mut WindowManager) {
    wm.draw_top(vga);

    let win = wm.top().unwrap();
    let (cr, cc, cw, _ch) = win.content_area();

    // Title
    vga.write_str_at(cr, cc + 2, "Settings", Color::Black, Color::LightGrey);
    vga.fill_rect(cr + 1, cc + 1, cw - 2, 1, 0xC4, Color::DarkGrey, Color::LightGrey);

    let mut selected: usize = 0;

    loop {
        // Draw settings list
        for (i, item) in SETTINGS.iter().enumerate() {
            let row = cr + 2 + i * 2;
            let is_sel = i == selected;
            let fg = if is_sel { Color::White } else { Color::Black };
            let bg = if is_sel { Color::Cyan } else { Color::LightGrey };

            vga.fill_rect(row, cc + 1, cw - 2, 1, b' ', fg, bg);

            if is_sel {
                vga.put_char_at(row, cc + 2, 0x10, fg, bg);
            }

            vga.write_str_at(row, cc + 4, item.label, fg, bg);
            vga.write_str_at(row, cc + 22, ":", fg, bg);
            vga.write_str_at(row, cc + 24, item.value, fg, bg);
        }

        // Status
        let status_row = cr + 2 + SETTINGS.len() * 2;
        vga.fill_rect(status_row, cc + 1, cw - 2, 1, b' ', Color::DarkGrey, Color::LightGrey);
        vga.write_str_at(status_row, cc + 2, "W/S:Navigate  ESC:Close", Color::DarkGrey, Color::LightGrey);

        let ch = kb.read_char();
        match ch {
            27 => return,
            b'w' => {
                if selected > 0 { selected -= 1; }
            }
            b's' => {
                if selected < SETTINGS.len() - 1 { selected += 1; }
            }
            _ => {}
        }
    }
}
