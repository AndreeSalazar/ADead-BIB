// ============================================================================
// FastOS Timer Driver (PIT - Programmable Interval Timer)
// ============================================================================
// Timer para scheduling y delays
//
// Author: Eddi Andreé Salazar Matos 🇵🇪
// ============================================================================

#![allow(dead_code)]

use core::arch::asm;
use core::sync::atomic::{AtomicU64, Ordering};

/// Frecuencia base del PIT (1.193182 MHz)
const PIT_FREQUENCY: u32 = 1193182;

/// Frecuencia deseada (100 Hz = 10ms por tick)
const TARGET_FREQUENCY: u32 = 100;

/// Divisor para obtener la frecuencia deseada
const PIT_DIVISOR: u16 = (PIT_FREQUENCY / TARGET_FREQUENCY) as u16;

/// Puertos del PIT
const PIT_CHANNEL0: u16 = 0x40;
const PIT_COMMAND: u16 = 0x43;

/// Contador de ticks global
static TICKS: AtomicU64 = AtomicU64::new(0);

/// Tiempo de inicio del sistema (en ticks)
static UPTIME_SECONDS: AtomicU64 = AtomicU64::new(0);

/// Inicializar el timer PIT
pub fn init() {
    unsafe {
        // Comando: Canal 0, modo 3 (square wave), acceso lobyte/hibyte
        outb(PIT_COMMAND, 0x36);
        
        // Enviar divisor (low byte primero, luego high byte)
        outb(PIT_CHANNEL0, (PIT_DIVISOR & 0xFF) as u8);
        outb(PIT_CHANNEL0, ((PIT_DIVISOR >> 8) & 0xFF) as u8);
    }
}

/// Handler de interrupción del timer (llamado desde IRQ0)
pub fn tick() {
    let ticks = TICKS.fetch_add(1, Ordering::SeqCst) + 1;
    
    // Actualizar segundos cada 100 ticks (1 segundo)
    if ticks % TARGET_FREQUENCY as u64 == 0 {
        UPTIME_SECONDS.fetch_add(1, Ordering::SeqCst);
    }
}

/// Obtener ticks actuales
pub fn get_ticks() -> u64 {
    TICKS.load(Ordering::SeqCst)
}

/// Obtener tiempo de actividad en segundos
pub fn get_uptime() -> u64 {
    UPTIME_SECONDS.load(Ordering::SeqCst)
}

/// Obtener tiempo de actividad en milisegundos
pub fn get_uptime_ms() -> u64 {
    let ticks = TICKS.load(Ordering::SeqCst);
    ticks * 10 // Cada tick = 10ms
}

/// Esperar un número de ticks
pub fn wait_ticks(count: u64) {
    let start = get_ticks();
    while get_ticks() - start < count {
        unsafe { asm!("hlt"); }
    }
}

/// Esperar milisegundos
pub fn sleep_ms(ms: u64) {
    let ticks = ms / 10; // Cada tick = 10ms
    if ticks > 0 {
        wait_ticks(ticks);
    } else {
        // Para delays menores a 10ms, usar busy wait
        for _ in 0..(ms * 10000) {
            unsafe { asm!("nop"); }
        }
    }
}

/// Esperar segundos
pub fn sleep(seconds: u64) {
    wait_ticks(seconds * TARGET_FREQUENCY as u64);
}

/// Obtener hora actual (formato HH:MM:SS desde uptime)
pub fn get_time_string() -> [u8; 8] {
    let total_seconds = get_uptime();
    let hours = (total_seconds / 3600) % 24;
    let minutes = (total_seconds / 60) % 60;
    let seconds = total_seconds % 60;
    
    [
        b'0' + (hours / 10) as u8,
        b'0' + (hours % 10) as u8,
        b':',
        b'0' + (minutes / 10) as u8,
        b'0' + (minutes % 10) as u8,
        b':',
        b'0' + (seconds / 10) as u8,
        b'0' + (seconds % 10) as u8,
    ]
}

/// Leer RTC (Real Time Clock) para hora real
pub fn read_rtc_time() -> (u8, u8, u8) {
    unsafe {
        // Leer segundos
        outb(0x70, 0x00);
        let seconds = bcd_to_binary(inb(0x71));
        
        // Leer minutos
        outb(0x70, 0x02);
        let minutes = bcd_to_binary(inb(0x71));
        
        // Leer horas
        outb(0x70, 0x04);
        let hours = bcd_to_binary(inb(0x71));
        
        (hours, minutes, seconds)
    }
}

/// Convertir BCD a binario
fn bcd_to_binary(bcd: u8) -> u8 {
    ((bcd >> 4) * 10) + (bcd & 0x0F)
}

// Funciones de I/O
#[inline]
unsafe fn outb(port: u16, value: u8) {
    asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack));
}

#[inline]
unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    asm!("in al, dx", out("al") value, in("dx") port, options(nomem, nostack));
    value
}
