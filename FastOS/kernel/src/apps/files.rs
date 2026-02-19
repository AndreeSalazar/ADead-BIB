// ============================================================
// FastOS — File Manager App (Rust)
// ============================================================
// Simple file browser with virtual filesystem.
// Shows directories and files in a list view.
// ============================================================

use crate::vga::{VgaWriter, Color};
use crate::keyboard::Keyboard;
use crate::window::WindowManager;

struct FileEntry {
    name: &'static str,
    is_dir: bool,
    size: u32,
}

const ROOT_FILES: [FileEntry; 10] = [
    FileEntry { name: "/boot",     is_dir: true,  size: 0 },
    FileEntry { name: "/kernel",   is_dir: true,  size: 0 },
    FileEntry { name: "/system",   is_dir: true,  size: 0 },
    FileEntry { name: "/home",     is_dir: true,  size: 0 },
    FileEntry { name: "/tmp",      is_dir: true,  size: 0 },
    FileEntry { name: "/dev",      is_dir: true,  size: 0 },
    FileEntry { name: "config.fos",   is_dir: false, size: 256 },
    FileEntry { name: "kernel.bin",   is_dir: false, size: 32768 },
    FileEntry { name: "README.txt",   is_dir: false, size: 1024 },
    FileEntry { name: "fastos.log",   is_dir: false, size: 512 },
];

pub fn run(vga: &mut VgaWriter, kb: &mut Keyboard, wm: &mut WindowManager) {
    wm.draw_top(vga);

    let win = wm.top().unwrap();
    let (cr, cc, cw, _ch) = win.content_area();

    // Header
    vga.write_str_at(cr, cc + 1, "File Manager - /", Color::Black, Color::LightGrey);
    vga.fill_rect(cr + 1, cc + 1, cw - 2, 1, 0xC4, Color::DarkGrey, Color::LightGrey);

    // Column headers
    vga.write_str_at(cr + 2, cc + 2, "Name", Color::Black, Color::LightGrey);
    vga.write_str_at(cr + 2, cc + 30, "Type", Color::Black, Color::LightGrey);
    vga.write_str_at(cr + 2, cc + 40, "Size", Color::Black, Color::LightGrey);
    vga.fill_rect(cr + 3, cc + 1, cw - 2, 1, 0xC4, Color::DarkGrey, Color::LightGrey);

    let mut selected: usize = 0;

    loop {
        // Draw file list
        for (i, entry) in ROOT_FILES.iter().enumerate() {
            let row = cr + 4 + i;
            let is_sel = i == selected;
            let fg = if is_sel { Color::White } else { Color::Black };
            let bg = if is_sel { Color::Cyan } else { Color::LightGrey };

            // Clear row
            vga.fill_rect(row, cc + 1, cw - 2, 1, b' ', fg, bg);

            // Selection marker
            if is_sel {
                vga.put_char_at(row, cc + 1, 0x10, fg, bg);
            }

            // Icon
            let icon = if entry.is_dir { 0xFE } else { 0xF0 };
            let icon_color = if entry.is_dir { Color::Yellow } else { Color::Blue };
            let icon_fg = if is_sel { icon_color } else { icon_color };
            vga.put_char_at(row, cc + 3, icon, icon_fg, bg);

            // Name
            vga.write_str_at(row, cc + 5, entry.name, fg, bg);

            // Type
            let type_str = if entry.is_dir { "DIR" } else { "FILE" };
            vga.write_str_at(row, cc + 30, type_str, fg, bg);

            // Size
            if !entry.is_dir {
                write_size_at(vga, row, cc + 40, entry.size, fg, bg);
            } else {
                vga.write_str_at(row, cc + 40, "-", fg, bg);
            }
        }

        // Status bar
        let status_row = cr + 15;
        vga.fill_rect(status_row, cc + 1, cw - 2, 1, b' ', Color::DarkGrey, Color::LightGrey);
        vga.write_str_at(status_row, cc + 2, "W/S:Navigate  Enter:Open  ESC:Close", Color::DarkGrey, Color::LightGrey);

        let ch = kb.read_char();
        match ch {
            27 => return,
            b'w' => {
                if selected > 0 { selected -= 1; }
            }
            b's' => {
                if selected < ROOT_FILES.len() - 1 { selected += 1; }
            }
            b'\n' => {
                let entry = &ROOT_FILES[selected];
                if entry.is_dir {
                    show_dir_contents(vga, kb, entry.name, cr, cc, cw);
                } else {
                    show_file_info(vga, kb, entry, cr, cc, cw);
                }
                // Redraw window
                wm.draw_top(vga);
                vga.write_str_at(cr, cc + 1, "File Manager - /", Color::Black, Color::LightGrey);
                vga.fill_rect(cr + 1, cc + 1, cw - 2, 1, 0xC4, Color::DarkGrey, Color::LightGrey);
                vga.write_str_at(cr + 2, cc + 2, "Name", Color::Black, Color::LightGrey);
                vga.write_str_at(cr + 2, cc + 30, "Type", Color::Black, Color::LightGrey);
                vga.write_str_at(cr + 2, cc + 40, "Size", Color::Black, Color::LightGrey);
                vga.fill_rect(cr + 3, cc + 1, cw - 2, 1, 0xC4, Color::DarkGrey, Color::LightGrey);
            }
            _ => {}
        }
    }
}

fn show_dir_contents(vga: &mut VgaWriter, kb: &mut Keyboard, name: &str, cr: usize, cc: usize, cw: usize) {
    // Show a simple "inside directory" view
    vga.fill_rect(cr + 4, cc + 1, cw - 2, 12, b' ', Color::Black, Color::LightGrey);
    vga.write_str_at(cr + 4, cc + 2, "Directory: ", Color::Black, Color::LightGrey);
    vga.write_str_at(cr + 4, cc + 13, name, Color::Blue, Color::LightGrey);
    vga.write_str_at(cr + 6, cc + 4, "(empty - virtual filesystem)", Color::DarkGrey, Color::LightGrey);
    vga.write_str_at(cr + 8, cc + 4, "Press ESC to go back", Color::DarkGrey, Color::LightGrey);

    loop {
        let ch = kb.read_char();
        if ch == 27 { return; }
    }
}

fn show_file_info(vga: &mut VgaWriter, kb: &mut Keyboard, entry: &FileEntry, cr: usize, cc: usize, cw: usize) {
    vga.fill_rect(cr + 4, cc + 1, cw - 2, 12, b' ', Color::Black, Color::LightGrey);
    vga.write_str_at(cr + 4, cc + 2, "File: ", Color::Black, Color::LightGrey);
    vga.write_str_at(cr + 4, cc + 8, entry.name, Color::Blue, Color::LightGrey);
    vga.write_str_at(cr + 5, cc + 2, "Size: ", Color::Black, Color::LightGrey);
    write_size_at(vga, cr + 5, cc + 8, entry.size, Color::Black, Color::LightGrey);
    vga.write_str_at(cr + 5, cc + 16, "bytes", Color::DarkGrey, Color::LightGrey);
    vga.write_str_at(cr + 6, cc + 2, "Format: FsOS", Color::DarkGrey, Color::LightGrey);
    vga.write_str_at(cr + 8, cc + 4, "Press ESC to go back", Color::DarkGrey, Color::LightGrey);

    loop {
        let ch = kb.read_char();
        if ch == 27 { return; }
    }
}

fn write_size_at(vga: &mut VgaWriter, row: usize, col: usize, size: u32, fg: Color, bg: Color) {
    if size == 0 {
        vga.put_char_at(row, col, b'0', fg, bg);
        return;
    }
    let mut buf = [0u8; 10];
    let mut val = size;
    let mut i = 0;
    while val > 0 {
        buf[i] = b'0' + (val % 10) as u8;
        val /= 10;
        i += 1;
    }
    let mut c = col;
    while i > 0 {
        i -= 1;
        vga.put_char_at(row, c, buf[i], fg, bg);
        c += 1;
    }
}
