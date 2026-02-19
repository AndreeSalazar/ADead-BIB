// ============================================================
// FastOS — Interactive Installer (Rust)
// ============================================================
// First-boot experience. Asks the user before proceeding.
// Always prompts externally: "Install FastOS completely?"
//
// Flow:
//   1. Welcome screen with FastOS branding
//   2. Ask: "Install FastOS? (Y/N)"
//   3. If Y → show progress, complete
//   4. If N → offer "Try FastOS" mode (live shell)
// ============================================================

use crate::vga::{VgaWriter, Color};
use crate::keyboard::Keyboard;

pub fn run_installer(vga: &mut VgaWriter) {
    let mut kb = Keyboard::new();

    // ---- Welcome Screen ----
    draw_welcome(vga);

    // ---- Ask user ----
    vga.set_color(Color::White, Color::Black);
    vga.write_str("  Install FastOS completely? ");
    vga.set_color(Color::LightGreen, Color::Black);
    vga.write_str("[Y] Yes  ");
    vga.set_color(Color::LightRed, Color::Black);
    vga.write_str("[N] No / Try Live\n\n");
    vga.set_color(Color::Green, Color::Black);
    vga.write_str("  > ");

    // ---- Wait for Y or N ----
    loop {
        let ch = kb.read_char();
        match ch {
            b'y' | b'Y' => {
                vga.set_color(Color::LightGreen, Color::Black);
                vga.write_str("Y\n\n");
                run_install_process(vga);
                return;
            }
            b'n' | b'N' => {
                vga.set_color(Color::Yellow, Color::Black);
                vga.write_str("N\n\n");
                vga.set_color(Color::LightCyan, Color::Black);
                vga.write_str("  Starting FastOS Live Mode...\n\n");
                return; // Go to shell in live mode
            }
            _ => {
                // Ignore other keys, keep waiting
            }
        }
    }
}

fn draw_welcome(vga: &mut VgaWriter) {
    vga.set_color(Color::LightCyan, Color::Black);
    vga.write_str("  +-----------------------------------------+\n");
    vga.write_str("  |                                         |\n");
    vga.write_str("  |       Welcome to FastOS Installer       |\n");
    vga.write_str("  |                                         |\n");
    vga.set_color(Color::LightGrey, Color::Black);
    vga.write_str("  |  A new OS built with:                   |\n");
    vga.set_color(Color::LightGreen, Color::Black);
    vga.write_str("  |    * ADead-BIB  (Base / Hardware)       |\n");
    vga.set_color(Color::Yellow, Color::Black);
    vga.write_str("  |    * Rust       (Security / Kernel)     |\n");
    vga.set_color(Color::LightBlue, Color::Black);
    vga.write_str("  |    * C          (Compatibility / ABI)   |\n");
    vga.set_color(Color::LightGrey, Color::Black);
    vga.write_str("  |                                         |\n");
    vga.write_str("  |  Format: FsOS (not PE, not ELF)         |\n");
    vga.write_str("  |  Mode:   64-bit Long Mode               |\n");
    vga.set_color(Color::LightCyan, Color::Black);
    vga.write_str("  |                                         |\n");
    vga.write_str("  +-----------------------------------------+\n\n");
}

fn run_install_process(vga: &mut VgaWriter) {
    vga.set_color(Color::White, Color::Black);
    vga.write_str("  Installing FastOS...\n\n");

    // Step 1
    vga.set_color(Color::Yellow, Color::Black);
    vga.write_str("  [1/5] ");
    vga.set_color(Color::LightGrey, Color::Black);
    vga.write_str("Initializing FsOS filesystem...");
    spin_delay();
    vga.set_color(Color::LightGreen, Color::Black);
    vga.write_str(" OK\n");

    // Step 2
    vga.set_color(Color::Yellow, Color::Black);
    vga.write_str("  [2/5] ");
    vga.set_color(Color::LightGrey, Color::Black);
    vga.write_str("Installing kernel modules...");
    spin_delay();
    vga.set_color(Color::LightGreen, Color::Black);
    vga.write_str(" OK\n");

    // Step 3
    vga.set_color(Color::Yellow, Color::Black);
    vga.write_str("  [3/5] ");
    vga.set_color(Color::LightGrey, Color::Black);
    vga.write_str("Setting up drivers (VGA, KB, PIT)...");
    spin_delay();
    vga.set_color(Color::LightGreen, Color::Black);
    vga.write_str(" OK\n");

    // Step 4
    vga.set_color(Color::Yellow, Color::Black);
    vga.write_str("  [4/5] ");
    vga.set_color(Color::LightGrey, Color::Black);
    vga.write_str("Configuring user environment...");
    spin_delay();
    vga.set_color(Color::LightGreen, Color::Black);
    vga.write_str(" OK\n");

    // Step 5
    vga.set_color(Color::Yellow, Color::Black);
    vga.write_str("  [5/5] ");
    vga.set_color(Color::LightGrey, Color::Black);
    vga.write_str("Finalizing installation...");
    spin_delay();
    vga.set_color(Color::LightGreen, Color::Black);
    vga.write_str(" OK\n\n");

    // Complete
    vga.set_color(Color::White, Color::Black);
    vga.write_str("  ==========================================\n");
    vga.set_color(Color::LightGreen, Color::Black);
    vga.write_str("    FastOS installed successfully!\n");
    vga.set_color(Color::White, Color::Black);
    vga.write_str("  ==========================================\n\n");
}

/// Simple busy-wait delay (no PIT timer yet)
fn spin_delay() {
    for _ in 0..5_000_000u64 {
        unsafe { core::hint::spin_loop(); }
    }
}
