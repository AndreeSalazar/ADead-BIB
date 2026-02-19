// ============================================================
// FastOS — PIT Timer Driver
// ============================================================
// Programmable Interval Timer (8253/8254).
// Channel 0 connected to IRQ 0 for system tick.
// ============================================================

use crate::arch::x86_64::port;

/// PIT I/O ports
const PIT_CHANNEL0: u16 = 0x40;
const PIT_COMMAND: u16  = 0x43;

/// PIT base frequency: 1,193,182 Hz
const PIT_BASE_FREQ: u32 = 1_193_182;

/// System tick counter
static mut TICKS: u64 = 0;
static mut FREQUENCY: u32 = 0;

/// Initialize PIT at the given frequency (Hz)
pub fn init(freq: u32) {
    let divisor = PIT_BASE_FREQ / freq;
    let divisor = if divisor > 65535 { 65535 } else if divisor < 1 { 1 } else { divisor };

    unsafe { FREQUENCY = freq; }

    // Command: channel 0, lobyte/hibyte, rate generator (mode 2)
    port::outb(PIT_COMMAND, 0x36);

    // Send divisor (low byte first, then high byte)
    port::outb(PIT_CHANNEL0, (divisor & 0xFF) as u8);
    port::outb(PIT_CHANNEL0, ((divisor >> 8) & 0xFF) as u8);
}

/// Called from timer IRQ handler — increment tick counter
pub fn tick() {
    unsafe { TICKS += 1; }
}

/// Get current tick count
pub fn get_ticks() -> u64 {
    unsafe { TICKS }
}

/// Get configured frequency
pub fn get_frequency() -> u32 {
    unsafe { FREQUENCY }
}

/// Get uptime in milliseconds
pub fn uptime_ms() -> u64 {
    let freq = get_frequency() as u64;
    if freq == 0 { return 0; }
    (get_ticks() * 1000) / freq
}

/// Get uptime in seconds
pub fn uptime_secs() -> u64 {
    let freq = get_frequency() as u64;
    if freq == 0 { return 0; }
    get_ticks() / freq
}

/// Busy-wait for approximately `ms` milliseconds
pub fn sleep_ms(ms: u64) {
    let target = get_ticks() + (ms * get_frequency() as u64) / 1000;
    while get_ticks() < target {
        crate::arch::x86_64::cpu::hlt();
    }
}

/// Busy-wait for approximately `us` microseconds (rough)
pub fn sleep_us(us: u64) {
    // At 1000 Hz, minimum resolution is 1ms
    // For sub-ms delays, use I/O port delay loop
    let loops = us / 10;
    for _ in 0..loops {
        port::io_wait();
    }
}
