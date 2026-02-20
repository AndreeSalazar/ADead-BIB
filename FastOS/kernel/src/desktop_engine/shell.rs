// ============================================================
// FastOS — Shell (Fase 5)
// ============================================================
// Desktop shell: taskbar, desktop icons, wallpaper, start menu.
// Renders directly to framebuffer using the compositor.
// ============================================================

use crate::drivers::framebuffer::{self, color};

/// Taskbar height in pixels
const TASKBAR_HEIGHT: u32 = 40;

/// Desktop icon grid
const ICON_SIZE: u32 = 48;
const ICON_SPACING: u32 = 80;
const ICON_START_X: u32 = 20;
const ICON_START_Y: u32 = 20;
const MAX_ICONS: usize = 12;

/// Desktop icon
#[derive(Clone, Copy)]
pub struct DesktopIcon {
    pub label: [u8; 16],
    pub label_len: usize,
    pub icon_char: u8,
    pub icon_color: u32,
    pub active: bool,
}

impl DesktopIcon {
    pub const fn empty() -> Self {
        DesktopIcon {
            label: [0u8; 16], label_len: 0,
            icon_char: 0, icon_color: 0,
            active: false,
        }
    }
}

/// Shell state
static mut SHELL: ShellState = ShellState {
    icons: [DesktopIcon::empty(); MAX_ICONS],
    icon_count: 0,
    start_menu_open: false,
    wallpaper_color: 0xFF1E1E2E,
    screen_w: 0,
    screen_h: 0,
};

struct ShellState {
    icons: [DesktopIcon; MAX_ICONS],
    icon_count: usize,
    start_menu_open: bool,
    wallpaper_color: u32,
    screen_w: u32,
    screen_h: u32,
}

/// Initialize the shell
pub fn init() {
    unsafe {
        SHELL.screen_w = framebuffer::width();
        SHELL.screen_h = framebuffer::height();
        SHELL.start_menu_open = false;
        SHELL.wallpaper_color = 0xFF1A1A2E; // Dark navy wallpaper

        // Add default desktop icons
        add_icon("Terminal", b'>', color::GREEN);
        add_icon("Files", b'#', color::YELLOW);
        add_icon("Settings", b'*', color::GREY);
        add_icon("Calculator", b'=', color::WHITE);
        add_icon("Editor", b'E', color::CYAN);
        add_icon("SysInfo", b'i', color::MAGENTA);
    }
}

/// Add a desktop icon
pub fn add_icon(label: &str, icon_char: u8, icon_color: u32) {
    unsafe {
        if SHELL.icon_count >= MAX_ICONS { return; }
        let mut icon = DesktopIcon::empty();
        let bytes = label.as_bytes();
        let len = if bytes.len() > 15 { 15 } else { bytes.len() };
        icon.label[..len].copy_from_slice(&bytes[..len]);
        icon.label_len = len;
        icon.icon_char = icon_char;
        icon.icon_color = icon_color;
        icon.active = true;
        SHELL.icons[SHELL.icon_count] = icon;
        SHELL.icon_count += 1;
    }
}

/// Draw the wallpaper — dragon-themed gradient
/// Fast: vertical gradient + grid accents
pub fn draw_wallpaper() {
    unsafe {
        let w = SHELL.screen_w;
        let h = SHELL.screen_h;
        if w == 0 || h == 0 { return; }

        // Vertical gradient: medium blue top → dark navy bottom
        for y in 0..h {
            let t = y * 255 / h;
            let r = (0x20 + t * 0x08 / 255).min(255);
            let g = (0x40 + t * 0x30 / 255).min(255);
            let b = (0x80 + t * 0x40 / 255).min(255);

            let base_color = 0xFF000000 | (r << 16) | (g << 8) | b;

            for x in 0..w {
                let mut color = base_color;

                // Grid lines
                if x % 64 == 0 || y % 64 == 0 {
                    let gr = (r + 15).min(255);
                    let gg = (g + 20).min(255);
                    let gb = (b + 15).min(255);
                    color = 0xFF000000 | (gr << 16) | (gg << 8) | gb;
                }

                // Diagonal cyan streaks
                let diag = (x + y * 2) % 120;
                if diag < 2 && y > h / 4 && y < h * 3 / 4 {
                    let gg = (g + 60).min(255);
                    let gb = (b + 30).min(255);
                    color = 0xFF000000 | (r << 16) | (gg << 8) | gb;
                }

                framebuffer::put_pixel(x, y, color);
            }
        }

        // "FastOS" branding top-left
        framebuffer::draw_string_transparent(16, 12, "FastOS", 0xFF00E5CC);
    }
}

/// Integer square root (no floating point in kernel)
fn isqrt(n: u32) -> u32 {
    if n == 0 { return 0; }
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x
}

/// Draw desktop icons
pub fn draw_icons() {
    unsafe {
        let mut ix = ICON_START_X;
        let mut iy = ICON_START_Y;

        for i in 0..SHELL.icon_count {
            let icon = &SHELL.icons[i];
            if !icon.active { continue; }

            // Icon background (rounded square)
            framebuffer::fill_rounded_rect(ix, iy, ICON_SIZE, ICON_SIZE, 8, 0x40FFFFFF);

            // Icon character (centered, large)
            let char_x = ix + ICON_SIZE / 2 - 4;
            let char_y = iy + ICON_SIZE / 2 - 8;
            framebuffer::draw_char(char_x, char_y, icon.icon_char, icon.icon_color, 0x40FFFFFF);

            // Label below icon
            let label = &icon.label[..icon.label_len];
            let label_x = ix + (ICON_SIZE / 2).saturating_sub((icon.label_len as u32 * 8) / 2);
            let label_y = iy + ICON_SIZE + 4;
            for (j, &ch) in label.iter().enumerate() {
                if ch == 0 { break; }
                framebuffer::draw_char(label_x + j as u32 * 8, label_y, ch, color::WHITE, 0x00000000);
            }

            // Next position
            iy += ICON_SPACING + ICON_SIZE;
            if iy + ICON_SIZE + 30 >= SHELL.screen_h - TASKBAR_HEIGHT {
                iy = ICON_START_Y;
                ix += ICON_SPACING + ICON_SIZE;
            }
        }
    }
}

/// Draw the taskbar at the bottom of the screen
pub fn draw_taskbar() {
    unsafe {
        let w = SHELL.screen_w;
        let h = SHELL.screen_h;
        let ty = h - TASKBAR_HEIGHT;

        // Taskbar background (semi-transparent dark)
        framebuffer::fill_rect(0, ty, w, TASKBAR_HEIGHT, 0xE01C1C1C);

        // Top border line
        framebuffer::draw_hline(0, ty, w, 0xFF333333);

        // Start button (centered, Windows 11 style)
        let start_x = w / 2 - 20;
        let start_y = ty + 8;
        framebuffer::fill_rounded_rect(start_x, start_y, 40, 24, 4, color::ACCENT);
        framebuffer::draw_string(start_x + 6, start_y + 4, "Start", color::WHITE, color::ACCENT);

        // Clock on the right
        let clock_x = w - 60;
        let clock_y = ty + 12;
        framebuffer::draw_string(clock_x, clock_y, "12:00", color::WHITE, 0xE01C1C1C);

        // System tray icons
        let tray_x = w - 120;
        framebuffer::draw_char(tray_x, clock_y, b'^', color::LIGHT_GREY, 0xE01C1C1C);
        framebuffer::draw_char(tray_x + 16, clock_y, b'W', color::LIGHT_GREY, 0xE01C1C1C);
        framebuffer::draw_char(tray_x + 32, clock_y, b'V', color::LIGHT_GREY, 0xE01C1C1C);
    }
}

/// Draw the start menu (if open)
pub fn draw_start_menu() {
    unsafe {
        if !SHELL.start_menu_open { return; }

        let w = SHELL.screen_w;
        let h = SHELL.screen_h;
        let menu_w = 300u32;
        let menu_h = 400u32;
        let mx = (w - menu_w) / 2;
        let my = h - TASKBAR_HEIGHT - menu_h - 8;

        // Menu background
        framebuffer::fill_rounded_rect(mx, my, menu_w, menu_h, 12, 0xF02D2D2D);

        // Menu border
        framebuffer::draw_rect(mx, my, menu_w, menu_h, 0xFF444444);

        // "Pinned" section header
        framebuffer::draw_string(mx + 16, my + 16, "Pinned", color::WHITE, 0xF02D2D2D);

        // App grid (2 columns)
        let apps = [
            ("Terminal", b'>'), ("Files", b'#'),
            ("Settings", b'*'), ("Calculator", b'='),
            ("Editor", b'E'), ("SysInfo", b'i'),
        ];

        let grid_x = mx + 16;
        let grid_y = my + 48;
        let cell_w = 130u32;
        let cell_h = 50u32;

        for (idx, (name, icon)) in apps.iter().enumerate() {
            let col_idx = (idx % 2) as u32;
            let row_idx = (idx / 2) as u32;
            let ax = grid_x + col_idx * cell_w;
            let ay = grid_y + row_idx * cell_h;

            // App tile
            framebuffer::fill_rounded_rect(ax, ay, cell_w - 8, cell_h - 8, 6, 0xFF3D3D3D);
            framebuffer::draw_char(ax + 8, ay + 10, *icon, color::ACCENT, 0xFF3D3D3D);
            framebuffer::draw_string(ax + 24, ay + 14, name, color::WHITE, 0xFF3D3D3D);
        }

        // "All apps" button
        let btn_y = my + menu_h - 48;
        framebuffer::draw_string(mx + menu_w / 2 - 32, btn_y, "All apps >", color::ACCENT, 0xF02D2D2D);

        // User section at bottom
        framebuffer::draw_hline(mx + 8, my + menu_h - 56, menu_w - 16, 0xFF444444);
        framebuffer::draw_string(mx + 16, my + menu_h - 32, "Admin", color::LIGHT_GREY, 0xF02D2D2D);

        // Power button
        let pw_x = mx + menu_w - 40;
        let pw_y = my + menu_h - 36;
        framebuffer::fill_rounded_rect(pw_x, pw_y, 24, 24, 4, 0xFF555555);
        framebuffer::draw_char(pw_x + 8, pw_y + 4, b'O', color::RED, 0xFF555555);
    }
}

/// Toggle start menu
pub fn toggle_start_menu() {
    unsafe {
        SHELL.start_menu_open = !SHELL.start_menu_open;
    }
}

/// Check if start menu is open
pub fn is_start_menu_open() -> bool {
    unsafe { SHELL.start_menu_open }
}

/// Draw the entire desktop (wallpaper + icons + taskbar + start menu)
pub fn draw_desktop() {
    draw_wallpaper();
    draw_icons();
    draw_taskbar();
    draw_start_menu();
}

/// Check if a click is on the taskbar start button
pub fn hit_start_button(mx: u32, my: u32) -> bool {
    unsafe {
        let w = SHELL.screen_w;
        let h = SHELL.screen_h;
        let ty = h - TASKBAR_HEIGHT;
        let start_x = w / 2 - 20;
        let start_y = ty + 8;
        mx >= start_x && mx < start_x + 40 && my >= start_y && my < start_y + 24
    }
}

/// Check if a click is on a desktop icon, returns icon index or None
pub fn hit_icon(mx: u32, my: u32) -> Option<usize> {
    unsafe {
        let mut ix = ICON_START_X;
        let mut iy = ICON_START_Y;

        for i in 0..SHELL.icon_count {
            if !SHELL.icons[i].active { continue; }

            if mx >= ix && mx < ix + ICON_SIZE && my >= iy && my < iy + ICON_SIZE {
                return Some(i);
            }

            iy += ICON_SPACING + ICON_SIZE;
            if iy + ICON_SIZE + 30 >= SHELL.screen_h - TASKBAR_HEIGHT {
                iy = ICON_START_Y;
                ix += ICON_SPACING + ICON_SIZE;
            }
        }
        None
    }
}

/// Get taskbar height
pub fn taskbar_height() -> u32 {
    TASKBAR_HEIGHT
}
