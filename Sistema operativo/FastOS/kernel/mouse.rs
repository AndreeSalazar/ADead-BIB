// ============================================================================
// FastOS Mouse Driver (PS/2)
// ============================================================================

use core::arch::asm;

/// Estado del mouse
pub static mut MOUSE: MouseState = MouseState::new();

/// Estado del mouse
pub struct MouseState {
    pub x: i32,
    pub y: i32,
    pub buttons: u8,
    pub packet: [u8; 3],
    pub packet_index: usize,
    pub screen_w: i32,
    pub screen_h: i32,
}

impl MouseState {
    pub const fn new() -> Self {
        MouseState {
            x: 640,
            y: 360,
            buttons: 0,
            packet: [0; 3],
            packet_index: 0,
            screen_w: 1280,
            screen_h: 720,
        }
    }

    pub fn set_screen_size(&mut self, w: i32, h: i32) {
        self.screen_w = w;
        self.screen_h = h;
        self.x = w / 2;
        self.y = h / 2;
    }

    pub fn left_button(&self) -> bool {
        self.buttons & 0x01 != 0
    }

    pub fn right_button(&self) -> bool {
        self.buttons & 0x02 != 0
    }
}

/// Inicializar mouse PS/2
pub fn init(screen_w: i32, screen_h: i32) {
    unsafe {
        MOUSE.set_screen_size(screen_w, screen_h);
        
        // Esperar a que el controlador esté listo
        wait_write();
        outb(0x64, 0xA8);  // Habilitar puerto auxiliar (mouse)
        
        wait_write();
        outb(0x64, 0x20);  // Leer byte de comando
        wait_read();
        let status = inb(0x60);
        
        wait_write();
        outb(0x64, 0x60);  // Escribir byte de comando
        wait_write();
        outb(0x60, status | 0x02);  // Habilitar IRQ12
        
        // Enviar comando al mouse
        mouse_write(0xF6);  // Set defaults
        mouse_read();
        
        mouse_write(0xF4);  // Enable data reporting
        mouse_read();
    }
}

/// Procesar byte del mouse (llamado desde polling)
pub fn handle_byte(byte: u8) {
    unsafe {
        let mouse = &mut MOUSE;
        
        // Si es el primer byte, debe tener bit 3 activo (always 1)
        if mouse.packet_index == 0 {
            if byte & 0x08 == 0 {
                return; // No es un byte válido de inicio, ignorar
            }
        }
        
        mouse.packet[mouse.packet_index] = byte;
        mouse.packet_index += 1;
        
        if mouse.packet_index >= 3 {
            mouse.packet_index = 0;
            
            // Botones
            mouse.buttons = mouse.packet[0] & 0x07;
            
            // Movimiento X
            let mut dx = mouse.packet[1] as i32;
            if mouse.packet[0] & 0x10 != 0 {
                dx -= 256;
            }
            
            // Movimiento Y (invertido)
            let mut dy = mouse.packet[2] as i32;
            if mouse.packet[0] & 0x20 != 0 {
                dy -= 256;
            }
            
            // Actualizar posición
            mouse.x += dx;
            mouse.y -= dy;  // Y invertido
            
            // Limitar a pantalla
            if mouse.x < 0 { mouse.x = 0; }
            if mouse.y < 0 { mouse.y = 0; }
            if mouse.x >= mouse.screen_w { mouse.x = mouse.screen_w - 1; }
            if mouse.y >= mouse.screen_h { mouse.y = mouse.screen_h - 1; }
        }
    }
}

/// Obtener posición del mouse
pub fn get_position() -> (i32, i32) {
    unsafe { (MOUSE.x, MOUSE.y) }
}

/// Obtener estado de botones
pub fn get_buttons() -> u8 {
    unsafe { MOUSE.buttons }
}

// Funciones de bajo nivel

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

unsafe fn wait_write() {
    for _ in 0..100000 {
        if inb(0x64) & 0x02 == 0 {
            return;
        }
    }
}

unsafe fn wait_read() {
    for _ in 0..100000 {
        if inb(0x64) & 0x01 != 0 {
            return;
        }
    }
}

unsafe fn mouse_write(cmd: u8) {
    wait_write();
    outb(0x64, 0xD4);
    wait_write();
    outb(0x60, cmd);
}

unsafe fn mouse_read() -> u8 {
    wait_read();
    inb(0x60)
}
