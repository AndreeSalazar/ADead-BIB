// ============================================================
// FastOS — Window Manager (Fase 5)
// ============================================================
// Graphical window manager: create/destroy windows, move/resize,
// focus management, window decorations (title bar, close button).
// ============================================================

use crate::drivers::framebuffer::{self, color};
use super::compositor;

/// Maximum windows
const MAX_WINDOWS: usize = 12;

/// Window decoration constants
const TITLEBAR_HEIGHT: u32 = 28;
const BORDER_WIDTH: u32 = 1;
const CLOSE_BTN_SIZE: u32 = 20;

/// Window ID type
pub type WinId = u32;

/// Window state
#[derive(Clone, Copy, PartialEq)]
pub enum WinState {
    Normal,
    Minimized,
    Maximized,
    Closed,
}

/// A graphical window
#[derive(Clone, Copy)]
pub struct GfxWindow {
    pub id: WinId,
    pub layer_id: u32,
    pub title: [u8; 64],
    pub title_len: usize,
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub state: WinState,
    pub focused: bool,
    pub content_color: u32,
}

impl GfxWindow {
    pub const fn empty() -> Self {
        GfxWindow {
            id: 0, layer_id: 0,
            title: [0u8; 64], title_len: 0,
            x: 0, y: 0, w: 0, h: 0,
            state: WinState::Closed,
            focused: false,
            content_color: color::SURFACE,
        }
    }

    /// Content area (inside decorations)
    pub fn content_rect(&self) -> (u32, u32, u32, u32) {
        (
            self.x + BORDER_WIDTH,
            self.y + TITLEBAR_HEIGHT,
            self.w - 2 * BORDER_WIDTH,
            self.h - TITLEBAR_HEIGHT - BORDER_WIDTH,
        )
    }

    /// Draw window decorations directly to framebuffer
    pub fn draw_decorations(&self) {
        if self.state == WinState::Closed || self.state == WinState::Minimized { return; }

        let title_color = if self.focused { color::ACCENT } else { 0xFF888888 };

        // Title bar background
        framebuffer::fill_rect(self.x, self.y, self.w, TITLEBAR_HEIGHT, title_color);

        // Title text
        let title_str = &self.title[..self.title_len];
        let mut tx = self.x + 10;
        let ty = self.y + 6;
        for &ch in title_str {
            if ch == 0 { break; }
            framebuffer::draw_char(tx, ty, ch, color::WHITE, title_color);
            tx += 8;
        }

        // Close button (red circle with X)
        let cx = self.x + self.w - CLOSE_BTN_SIZE - 6;
        let cy = self.y + 4;
        framebuffer::fill_rect(cx, cy, CLOSE_BTN_SIZE, CLOSE_BTN_SIZE, color::CLOSE_RED);
        framebuffer::draw_char(cx + 6, cy + 2, b'X', color::WHITE, color::CLOSE_RED);

        // Window border
        framebuffer::draw_rect(self.x, self.y, self.w, self.h, if self.focused { color::ACCENT } else { color::BORDER });

        // Content area background
        let (cx2, cy2, cw, ch) = self.content_rect();
        framebuffer::fill_rect(cx2, cy2, cw, ch, self.content_color);
    }

    /// Check if a point is in the close button
    pub fn hit_close(&self, mx: u32, my: u32) -> bool {
        let cx = self.x + self.w - CLOSE_BTN_SIZE - 6;
        let cy = self.y + 4;
        mx >= cx && mx < cx + CLOSE_BTN_SIZE && my >= cy && my < cy + CLOSE_BTN_SIZE
    }

    /// Check if a point is in the title bar (for dragging)
    pub fn hit_titlebar(&self, mx: u32, my: u32) -> bool {
        mx >= self.x && mx < self.x + self.w &&
        my >= self.y && my < self.y + TITLEBAR_HEIGHT
    }

    /// Check if a point is inside the window
    pub fn contains(&self, mx: u32, my: u32) -> bool {
        mx >= self.x && mx < self.x + self.w &&
        my >= self.y && my < self.y + self.h
    }
}

/// Window Manager state
pub struct WindowManager {
    windows: [GfxWindow; MAX_WINDOWS],
    count: usize,
    focused_id: WinId,
    next_id: WinId,
}

static mut WM: WindowManager = WindowManager {
    windows: [GfxWindow::empty(); MAX_WINDOWS],
    count: 0,
    focused_id: 0,
    next_id: 1,
};

/// Initialize the window manager
pub fn init() {
    unsafe {
        WM.count = 0;
        WM.focused_id = 0;
        WM.next_id = 1;
    }
}

/// Create a new window, returns its ID
pub fn create_window(title: &str, x: u32, y: u32, w: u32, h: u32) -> WinId {
    unsafe {
        if WM.count >= MAX_WINDOWS { return 0; }

        let id = WM.next_id;
        WM.next_id += 1;

        let layer_id = compositor::create_layer(x, y, w, h, WM.count as i32 + 10, color::SURFACE);

        let mut win = GfxWindow::empty();
        win.id = id;
        win.layer_id = layer_id;
        win.x = x;
        win.y = y;
        win.w = w;
        win.h = h;
        win.state = WinState::Normal;
        win.focused = true;
        win.content_color = color::SURFACE;

        // Copy title
        let bytes = title.as_bytes();
        let len = if bytes.len() > 63 { 63 } else { bytes.len() };
        win.title[..len].copy_from_slice(&bytes[..len]);
        win.title_len = len;

        // Unfocus previous
        for i in 0..WM.count {
            WM.windows[i].focused = false;
        }

        WM.windows[WM.count] = win;
        WM.count += 1;
        WM.focused_id = id;

        id
    }
}

/// Close a window by ID
pub fn close_window(id: WinId) {
    unsafe {
        for i in 0..WM.count {
            if WM.windows[i].id == id {
                compositor::destroy_layer(WM.windows[i].layer_id);
                WM.windows[i].state = WinState::Closed;

                // Remove from array
                for j in i..(WM.count - 1) {
                    WM.windows[j] = WM.windows[j + 1];
                }
                WM.windows[WM.count - 1] = GfxWindow::empty();
                WM.count -= 1;

                // Focus top window
                if WM.count > 0 {
                    WM.windows[WM.count - 1].focused = true;
                    WM.focused_id = WM.windows[WM.count - 1].id;
                } else {
                    WM.focused_id = 0;
                }
                return;
            }
        }
    }
}

/// Move a window
pub fn move_window(id: WinId, new_x: u32, new_y: u32) {
    unsafe {
        for i in 0..WM.count {
            if WM.windows[i].id == id {
                WM.windows[i].x = new_x;
                WM.windows[i].y = new_y;
                compositor::move_layer(WM.windows[i].layer_id, new_x, new_y);
                return;
            }
        }
    }
}

/// Resize a window
pub fn resize_window(id: WinId, new_w: u32, new_h: u32) {
    unsafe {
        for i in 0..WM.count {
            if WM.windows[i].id == id {
                WM.windows[i].w = new_w;
                WM.windows[i].h = new_h;
                compositor::resize_layer(WM.windows[i].layer_id, new_w, new_h);
                return;
            }
        }
    }
}

/// Focus a window (bring to front)
pub fn focus_window(id: WinId) {
    unsafe {
        for i in 0..WM.count {
            WM.windows[i].focused = WM.windows[i].id == id;
            if WM.windows[i].id == id {
                compositor::bring_to_front(WM.windows[i].layer_id);
            }
        }
        WM.focused_id = id;
    }
}

/// Handle a mouse click at (mx, my) — returns true if handled
pub fn handle_click(mx: u32, my: u32) -> bool {
    unsafe {
        // Check windows from top to bottom (reverse order)
        let mut i = WM.count;
        while i > 0 {
            i -= 1;
            let win = &WM.windows[i];
            if win.state != WinState::Normal { continue; }

            if win.contains(mx, my) {
                let wid = win.id;

                if win.hit_close(mx, my) {
                    close_window(wid);
                    return true;
                }

                if win.focused == false {
                    focus_window(wid);
                }

                return true;
            }
        }
        false
    }
}

/// Draw all window decorations
pub fn draw_all() {
    unsafe {
        for i in 0..WM.count {
            if WM.windows[i].state == WinState::Normal {
                WM.windows[i].draw_decorations();
            }
        }
    }
}

/// Get the focused window ID
pub fn focused_id() -> WinId {
    unsafe { WM.focused_id }
}

/// Get window count
pub fn window_count() -> usize {
    unsafe { WM.count }
}

/// Get a window's content rect by ID
pub fn get_content_rect(id: WinId) -> Option<(u32, u32, u32, u32)> {
    unsafe {
        for i in 0..WM.count {
            if WM.windows[i].id == id {
                return Some(WM.windows[i].content_rect());
            }
        }
        None
    }
}
