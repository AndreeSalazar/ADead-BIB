// ============================================================
// FastOS — Desktop Manager (Rust)
// ============================================================
// Windows-style TUI desktop in VGA 80x25 text mode.
//
// Layout (80 cols x 25 rows):
//   Row 0:        Title bar (cyan)
//   Rows 1-23:    Desktop area (blue) with icons
//   Row 24:       Taskbar (dark grey) with Start, clock, tray
//
// Desktop icons (navigated with arrow keys + Enter):
//   [Terminal]  [Files]  [Editor]  [Calc]  [Info]  [Settings]
//
// Keyboard:
//   Arrow keys  — move selection
//   Enter       — open app
//   Escape      — close window / back to desktop
//   Tab         — cycle focus
//   F1          — open Start menu
// ============================================================

use crate::vga::{VgaWriter, Color};
use crate::keyboard::Keyboard;
use crate::window::{Window, WindowManager};
use crate::apps;
use crate::startmenu;

const DESKTOP_BG: Color = Color::Blue;
const TASKBAR_BG: Color = Color::DarkGrey;
const TASKBAR_FG: Color = Color::White;
const ICON_FG: Color = Color::White;
const ICON_BG: Color = Color::Blue;
const ICON_SEL_FG: Color = Color::Black;
const ICON_SEL_BG: Color = Color::LightCyan;

// Desktop icon definition
struct DesktopIcon {
    label: &'static str,
    symbol: u8,
    row: usize,
    col: usize,
}

const ICONS: [DesktopIcon; 6] = [
    DesktopIcon { label: "Terminal", symbol: b'>', row: 3, col: 4 },
    DesktopIcon { label: "Files",    symbol: 0xB1, row: 3, col: 18 },
    DesktopIcon { label: "Editor",   symbol: 0xF0, row: 3, col: 32 },
    DesktopIcon { label: "Calc",     symbol: b'#', row: 3, col: 46 },
    DesktopIcon { label: "SysInfo",  symbol: b'i', row: 3, col: 60 },
    DesktopIcon { label: "Settings", symbol: 0xFE, row: 8, col: 4 },
];

pub fn run_desktop(vga: &mut VgaWriter) {
    let mut kb = Keyboard::new();
    let mut selected: usize = 0;
    let mut wm = WindowManager::new();

    // Initial draw
    draw_desktop(vga, selected);
    draw_taskbar(vga);

    loop {
        // Non-blocking keyboard check
        let ch = match kb.try_read_char() {
            Some(c) => c,
            None => {
                // No key pressed, just spin and continue
                core::hint::spin_loop();
                continue;
            }
        };

        // Redraw after input
        draw_desktop(vga, selected);
        draw_taskbar(vga);
        match ch {
            // Arrow keys come as escape sequences in scancode
            // We handle raw scancodes in keyboard.rs, but for simplicity
            // we use letter keys for navigation too
            b'\t' | b'd' => {
                // Next icon
                selected = (selected + 1) % ICONS.len();
            }
            b'a' => {
                // Previous icon
                if selected == 0 {
                    selected = ICONS.len() - 1;
                } else {
                    selected -= 1;
                }
            }
            b'w' => {
                // Up row (jump back by ~row width)
                if selected >= 5 {
                    selected -= 5;
                }
            }
            b's' => {
                // Down row
                if selected + 5 < ICONS.len() {
                    selected += 5;
                } else if selected < 5 {
                    selected = ICONS.len() - 1;
                }
            }
            b'\n' => {
                // Open selected app
                open_app(vga, &mut kb, &mut wm, selected);
            }
            b'1' => open_app(vga, &mut kb, &mut wm, 0), // Quick Terminal
            b'2' => open_app(vga, &mut kb, &mut wm, 1), // Quick Files
            b'3' => open_app(vga, &mut kb, &mut wm, 2), // Quick Editor
            b'4' => open_app(vga, &mut kb, &mut wm, 3), // Quick Calc
            b'5' => open_app(vga, &mut kb, &mut wm, 4), // Quick SysInfo
            b'6' => open_app(vga, &mut kb, &mut wm, 5), // Quick Settings
            b'q' => {
                // Shutdown confirmation
                if confirm_shutdown(vga, &mut kb) {
                    shutdown_screen(vga);
                    return;
                }
            }
            b'`' | b'm' => {
                // Start menu
                startmenu::show_start_menu(vga, &mut kb, &mut wm);
            }
            _ => {}
        }
    }
}

fn draw_desktop(vga: &mut VgaWriter, selected: usize) {
    // Desktop background (rows 1-23)
    vga.fill_rect(1, 0, 80, 23, b' ', Color::White, DESKTOP_BG);

    // Title bar (row 0)
    vga.fill_rect(0, 0, 80, 1, b' ', Color::White, Color::Cyan);
    vga.write_str_at(0, 2, "FastOS Desktop", Color::White, Color::Cyan);
    vga.write_str_at(0, 55, "ADead-BIB+Rust+C", Color::Yellow, Color::Cyan);
    vga.write_str_at(0, 76, "v1.0", Color::White, Color::Cyan);

    // Draw desktop icons
    for (i, icon) in ICONS.iter().enumerate() {
        let is_selected = i == selected;
        let fg = if is_selected { ICON_SEL_FG } else { ICON_FG };
        let bg = if is_selected { ICON_SEL_BG } else { ICON_BG };

        // Icon box (3 rows tall, label width + 4)
        let w = icon.label.len() + 4;
        let r = icon.row;
        let c = icon.col;

        // Top border
        vga.put_char_at(r, c, 0xDA, fg, bg);
        for j in 1..(w - 1) {
            vga.put_char_at(r, c + j, 0xC4, fg, bg);
        }
        vga.put_char_at(r, c + w - 1, 0xBF, fg, bg);

        // Middle: symbol
        vga.put_char_at(r + 1, c, 0xB3, fg, bg);
        vga.put_char_at(r + 1, c + 1, b' ', fg, bg);
        vga.put_char_at(r + 1, c + 2, icon.symbol, fg, bg);
        vga.put_char_at(r + 1, c + 3, b' ', fg, bg);
        // Label
        let mut lc = c + 4;
        for byte in icon.label.bytes() {
            if lc < c + w - 1 {
                vga.put_char_at(r + 1, lc, byte, fg, bg);
                lc += 1;
            }
        }
        while lc < c + w - 1 {
            vga.put_char_at(r + 1, lc, b' ', fg, bg);
            lc += 1;
        }
        vga.put_char_at(r + 1, c + w - 1, 0xB3, fg, bg);

        // Bottom border
        vga.put_char_at(r + 2, c, 0xC0, fg, bg);
        for j in 1..(w - 1) {
            vga.put_char_at(r + 2, c + j, 0xC4, fg, bg);
        }
        vga.put_char_at(r + 2, c + w - 1, 0xD9, fg, bg);
    }

    // Navigation hints
    vga.write_str_at(22, 2, "WASD:Move  Enter:Open  1-6:Quick  M:Menu  Q:Shutdown", Color::LightCyan, DESKTOP_BG);
}

fn draw_taskbar(vga: &mut VgaWriter) {
    // Taskbar at row 24
    vga.fill_rect(24, 0, 80, 1, b' ', TASKBAR_FG, TASKBAR_BG);

    // Start button
    vga.write_str_at(24, 1, "[M]Start", Color::White, Color::Green);

    // Quick launch
    vga.write_str_at(24, 11, "|", Color::LightGrey, TASKBAR_BG);
    vga.write_str_at(24, 13, "Terminal", Color::LightGrey, TASKBAR_BG);
    vga.write_str_at(24, 22, "|", Color::LightGrey, TASKBAR_BG);

    // System tray (right side)
    vga.write_str_at(24, 65, "|", Color::LightGrey, TASKBAR_BG);
    vga.write_str_at(24, 67, "FsOS 64bit", Color::LightCyan, TASKBAR_BG);
}

pub fn open_app(vga: &mut VgaWriter, kb: &mut Keyboard, wm: &mut WindowManager, app_idx: usize) {
    match app_idx {
        0 => {
            let win = Window::new("Terminal", 2, 5, 70, 20);
            wm.push(win);
            apps::terminal::run(vga, kb, wm);
            wm.pop();
        }
        1 => {
            let win = Window::new("File Manager", 2, 5, 70, 20);
            wm.push(win);
            apps::files::run(vga, kb, wm);
            wm.pop();
        }
        2 => {
            let win = Window::new("Text Editor", 2, 5, 70, 20);
            wm.push(win);
            apps::editor::run(vga, kb, wm);
            wm.pop();
        }
        3 => {
            let win = Window::new("Calculator", 5, 20, 40, 15);
            wm.push(win);
            apps::calc::run(vga, kb, wm);
            wm.pop();
        }
        4 => {
            let win = Window::new("System Info", 3, 10, 60, 18);
            wm.push(win);
            apps::sysinfo::run(vga, kb, wm);
            wm.pop();
        }
        5 => {
            let win = Window::new("Settings", 3, 10, 60, 18);
            wm.push(win);
            apps::settings::run(vga, kb, wm);
            wm.pop();
        }
        _ => {}
    }
}

fn confirm_shutdown(vga: &mut VgaWriter, kb: &mut Keyboard) -> bool {
    // Draw confirmation dialog
    let r = 9;
    let c = 22;
    let w = 36;
    let h = 7;

    vga.fill_rect(r, c, w, h, b' ', Color::White, Color::Red);

    // Border
    vga.draw_hline(r, c, w, 0xCD, Color::Yellow, Color::Red);
    vga.draw_hline(r + h - 1, c, w, 0xCD, Color::Yellow, Color::Red);
    vga.draw_vline(r, c, h, 0xBA, Color::Yellow, Color::Red);
    vga.draw_vline(r, c + w - 1, h, 0xBA, Color::Yellow, Color::Red);
    vga.put_char_at(r, c, 0xC9, Color::Yellow, Color::Red);
    vga.put_char_at(r, c + w - 1, 0xBB, Color::Yellow, Color::Red);
    vga.put_char_at(r + h - 1, c, 0xC8, Color::Yellow, Color::Red);
    vga.put_char_at(r + h - 1, c + w - 1, 0xBC, Color::Yellow, Color::Red);

    vga.write_str_at(r + 2, c + 5, "Shutdown FastOS?", Color::White, Color::Red);
    vga.write_str_at(r + 4, c + 4, "[Y] Shutdown  [N] Cancel", Color::Yellow, Color::Red);

    loop {
        let ch = kb.read_char();
        match ch {
            b'y' | b'Y' => return true,
            b'n' | b'N' | 27 => return false,
            _ => {}
        }
    }
}

fn shutdown_screen(vga: &mut VgaWriter) {
    vga.clear_with(Color::White, Color::Black);
    vga.write_str_at(10, 28, "Shutting down...", Color::White, Color::Black);
    vga.write_str_at(12, 25, "FastOS v1.0 - Goodbye!", Color::LightGreen, Color::Black);
    vga.write_str_at(14, 22, "Powered by ADead-BIB + Rust + C", Color::DarkGrey, Color::Black);

    for _ in 0..10_000_000u64 {
        unsafe { core::hint::spin_loop(); }
    }

    // ACPI shutdown (QEMU)
    crate::outb(0x604, 0x00);
    crate::outb(0x604, 0x20);

    // Fallback: halt
    loop {
        crate::hlt();
    }
}
