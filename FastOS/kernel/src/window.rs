// ============================================================
// FastOS — Window Manager (Rust)
// ============================================================
// TUI window system for VGA 80x25 text mode.
// Each window has: title bar, border, content area, close [X].
// Windows are drawn with box-drawing characters.
// ============================================================

use crate::vga::{VgaWriter, Color};

const MAX_WINDOWS: usize = 8;

pub struct Window {
    pub title: &'static str,
    pub row: usize,
    pub col: usize,
    pub width: usize,
    pub height: usize,
    pub active: bool,
}

impl Window {
    pub fn new(title: &'static str, row: usize, col: usize, width: usize, height: usize) -> Self {
        Self {
            title,
            row,
            col,
            width,
            height,
            active: true,
        }
    }

    /// Draw the window frame with title bar and close button
    pub fn draw(&self, vga: &mut VgaWriter) {
        let r = self.row;
        let c = self.col;
        let w = self.width;
        let h = self.height;

        let title_bg = if self.active { Color::Cyan } else { Color::DarkGrey };
        let border_fg = if self.active { Color::LightCyan } else { Color::LightGrey };
        let border_bg = Color::LightGrey;

        // Title bar (row r)
        vga.fill_rect(r, c, w, 1, b' ', Color::White, title_bg);
        // Title text
        vga.write_str_at(r, c + 2, self.title, Color::White, title_bg);
        // Close button [X]
        vga.write_str_at(r, c + w - 4, "[X]", Color::Red, title_bg);

        // Content area background
        vga.fill_rect(r + 1, c, w, h - 2, b' ', Color::Black, Color::LightGrey);

        // Left and right borders
        for row_i in (r + 1)..(r + h - 1) {
            vga.put_char_at(row_i, c, 0xB3, border_fg, border_bg);
            vga.put_char_at(row_i, c + w - 1, 0xB3, border_fg, border_bg);
        }

        // Bottom border
        vga.put_char_at(r + h - 1, c, 0xC0, border_fg, border_bg);
        for j in 1..(w - 1) {
            vga.put_char_at(r + h - 1, c + j, 0xC4, border_fg, border_bg);
        }
        vga.put_char_at(r + h - 1, c + w - 1, 0xD9, border_fg, border_bg);

        // Status bar hint
        vga.write_str_at(r + h - 1, c + 2, " ESC:Close ", Color::DarkGrey, border_bg);
    }

    /// Get the content area bounds (row, col, width, height)
    pub fn content_area(&self) -> (usize, usize, usize, usize) {
        (self.row + 1, self.col + 1, self.width - 2, self.height - 2)
    }
}

pub struct WindowManager {
    stack: [Option<Window>; MAX_WINDOWS],
    count: usize,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            stack: [
                None, None, None, None,
                None, None, None, None,
            ],
            count: 0,
        }
    }

    pub fn push(&mut self, win: Window) {
        if self.count < MAX_WINDOWS {
            self.stack[self.count] = Some(win);
            self.count += 1;
        }
    }

    pub fn pop(&mut self) -> Option<Window> {
        if self.count > 0 {
            self.count -= 1;
            self.stack[self.count].take()
        } else {
            None
        }
    }

    pub fn top(&self) -> Option<&Window> {
        if self.count > 0 {
            self.stack[self.count - 1].as_ref()
        } else {
            None
        }
    }

    pub fn draw_top(&self, vga: &mut VgaWriter) {
        if let Some(win) = self.top() {
            win.draw(vga);
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }
}
