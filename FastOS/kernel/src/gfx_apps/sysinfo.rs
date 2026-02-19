// ============================================================
// FastOS — Graphical System Info (Fase 7)
// ============================================================
// Displays system information: CPU, memory, kernel version.
// ============================================================

use crate::drivers::framebuffer::{self, color};
use crate::desktop_engine::wm;

/// Open a sysinfo window
pub fn open() -> wm::WinId {
    let id = wm::create_window("System Info", 200, 90, 400, 300);
    draw_content(id);
    id
}

fn draw_content(win_id: wm::WinId) {
    if let Some((cx, cy, cw, ch)) = wm::get_content_rect(win_id) {
        framebuffer::fill_rect(cx, cy, cw, ch, color::SURFACE);

        // Header
        framebuffer::fill_rect(cx, cy, cw, 36, color::ACCENT);
        framebuffer::draw_string(cx + 12, cy + 10, "FastOS System Information", color::WHITE, color::ACCENT);

        let mut y = cy + 48;
        let lx = cx + 16;
        let vx = cx + 160;
        let row_h = 24u32;

        // OS info
        draw_row(lx, vx, y, cw, "OS:", "FastOS v1.0");
        y += row_h;
        draw_row(lx, vx, y, cw, "Format:", "FsOS (custom)");
        y += row_h;
        draw_row(lx, vx, y, cw, "Arch:", "x86_64 Long Mode");
        y += row_h;
        draw_row(lx, vx, y, cw, "Stack:", "ADead-BIB + Rust");
        y += row_h;

        // Separator
        framebuffer::draw_hline(cx + 8, y, cw - 16, color::BORDER);
        y += 8;

        // Memory info
        let free_kb = crate::kernel_core::memory::free_memory() / 1024;
        let total_kb = crate::kernel_core::memory::total_memory() / 1024;
        let used_kb = total_kb - free_kb;

        let mut buf = [0u8; 32];
        let n = fmt_kb(&mut buf, total_kb);
        draw_row_bytes(lx, vx, y, "Total RAM:", &buf[..n]);
        y += row_h;

        let n = fmt_kb(&mut buf, used_kb);
        draw_row_bytes(lx, vx, y, "Used:", &buf[..n]);
        y += row_h;

        let n = fmt_kb(&mut buf, free_kb);
        draw_row_bytes(lx, vx, y, "Free:", &buf[..n]);
        y += row_h;

        // Memory bar
        let bar_x = cx + 16;
        let bar_w = cw - 32;
        let bar_h = 16u32;
        framebuffer::fill_rounded_rect(bar_x, y, bar_w, bar_h, 4, color::BORDER);
        let used_w = if total_kb > 0 { (used_kb as u64 * bar_w as u64 / total_kb as u64) as u32 } else { 0 };
        if used_w > 0 {
            framebuffer::fill_rounded_rect(bar_x, y, used_w, bar_h, 4, color::ACCENT);
        }
        y += row_h + 4;

        // Separator
        framebuffer::draw_hline(cx + 8, y, cw - 16, color::BORDER);
        y += 8;

        // Heap info
        let heap_used = crate::kernel_core::memory::heap_used();
        let heap_free = crate::kernel_core::memory::heap_free();
        let n = fmt_kb(&mut buf, heap_used / 1024);
        draw_row_bytes(lx, vx, y, "Heap used:", &buf[..n]);
        y += row_h;
        let n = fmt_kb(&mut buf, heap_free / 1024);
        draw_row_bytes(lx, vx, y, "Heap free:", &buf[..n]);
    }
}

fn draw_row(lx: u32, vx: u32, y: u32, _cw: u32, label: &str, value: &str) {
    framebuffer::draw_string(lx, y, label, color::TEXT_SECONDARY, color::SURFACE);
    framebuffer::draw_string(vx, y, value, color::TEXT_PRIMARY, color::SURFACE);
}

fn draw_row_bytes(lx: u32, vx: u32, y: u32, label: &str, value: &[u8]) {
    framebuffer::draw_string(lx, y, label, color::TEXT_SECONDARY, color::SURFACE);
    let mut x = vx;
    for &b in value {
        framebuffer::draw_char(x, y, b, color::TEXT_PRIMARY, color::SURFACE);
        x += 8;
    }
}

fn fmt_kb(buf: &mut [u8], kb: usize) -> usize {
    let mut pos = fmt_usize(buf, kb);
    let suffix = b" KB";
    buf[pos..pos + suffix.len()].copy_from_slice(suffix);
    pos + suffix.len()
}

fn fmt_usize(buf: &mut [u8], val: usize) -> usize {
    if val == 0 { buf[0] = b'0'; return 1; }
    let mut tmp = [0u8; 20];
    let mut n = val;
    let mut i = 0;
    while n > 0 { tmp[i] = b'0' + (n % 10) as u8; n /= 10; i += 1; }
    for j in 0..i { buf[j] = tmp[i - 1 - j]; }
    i
}
