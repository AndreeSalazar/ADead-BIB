// ============================================================================
// FastOS - Disk Driver
// ============================================================================
// Driver de disco (ATA/AHCI básico)
// Lectura/escritura de sectores
//
// Author: Eddi Andreé Salazar Matos 🇵🇪
// ============================================================================

#![allow(dead_code)]

/// Puertos ATA primario
const ATA_PRIMARY_DATA: u16 = 0x1F0;
const ATA_PRIMARY_ERROR: u16 = 0x1F1;
const ATA_PRIMARY_SECTOR_COUNT: u16 = 0x1F2;
const ATA_PRIMARY_LBA_LOW: u16 = 0x1F3;
const ATA_PRIMARY_LBA_MID: u16 = 0x1F4;
const ATA_PRIMARY_LBA_HIGH: u16 = 0x1F5;
const ATA_PRIMARY_DRIVE: u16 = 0x1F6;
const ATA_PRIMARY_STATUS: u16 = 0x1F7;
const ATA_PRIMARY_COMMAND: u16 = 0x1F7;

/// Comandos ATA
const ATA_CMD_READ_SECTORS: u8 = 0x20;
const ATA_CMD_WRITE_SECTORS: u8 = 0x30;
const ATA_CMD_IDENTIFY: u8 = 0xEC;

/// Status bits
const ATA_STATUS_BSY: u8 = 0x80;
const ATA_STATUS_DRDY: u8 = 0x40;
const ATA_STATUS_DRQ: u8 = 0x08;
const ATA_STATUS_ERR: u8 = 0x01;

/// Driver de disco ATA
pub struct AtaDisk {
    base_port: u16,
    drive: u8, // 0 = master, 1 = slave
}

impl AtaDisk {
    /// Crear nuevo driver para disco primario master
    pub const fn primary_master() -> Self {
        AtaDisk {
            base_port: ATA_PRIMARY_DATA,
            drive: 0,
        }
    }

    /// Crear nuevo driver para disco primario slave
    pub const fn primary_slave() -> Self {
        AtaDisk {
            base_port: ATA_PRIMARY_DATA,
            drive: 1,
        }
    }

    /// Esperar a que el disco esté listo
    fn wait_ready(&self) {
        unsafe {
            loop {
                let status = inb(self.base_port + 7);
                if status & ATA_STATUS_BSY == 0 {
                    break;
                }
            }
        }
    }

    /// Esperar a que haya datos disponibles
    fn wait_drq(&self) -> bool {
        unsafe {
            for _ in 0..100000 {
                let status = inb(self.base_port + 7);
                if status & ATA_STATUS_ERR != 0 {
                    return false;
                }
                if status & ATA_STATUS_DRQ != 0 {
                    return true;
                }
            }
            false
        }
    }

    /// Leer sectores
    pub fn read_sectors(&self, lba: u32, count: u8, buffer: &mut [u8]) -> Result<(), &'static str> {
        if buffer.len() < (count as usize) * 512 {
            return Err("Buffer too small");
        }

        self.wait_ready();

        unsafe {
            // Seleccionar drive y LBA high bits
            outb(self.base_port + 6, 0xE0 | (self.drive << 4) | ((lba >> 24) as u8 & 0x0F));
            
            // Sector count
            outb(self.base_port + 2, count);
            
            // LBA
            outb(self.base_port + 3, lba as u8);
            outb(self.base_port + 4, (lba >> 8) as u8);
            outb(self.base_port + 5, (lba >> 16) as u8);
            
            // Comando de lectura
            outb(self.base_port + 7, ATA_CMD_READ_SECTORS);
        }

        // Leer datos
        for sector in 0..count as usize {
            if !self.wait_drq() {
                return Err("Disk read error");
            }

            let offset = sector * 512;
            for i in (0..512).step_by(2) {
                let word = unsafe { inw(self.base_port) };
                buffer[offset + i] = word as u8;
                buffer[offset + i + 1] = (word >> 8) as u8;
            }
        }

        Ok(())
    }

    /// Escribir sectores
    pub fn write_sectors(&self, lba: u32, count: u8, buffer: &[u8]) -> Result<(), &'static str> {
        if buffer.len() < (count as usize) * 512 {
            return Err("Buffer too small");
        }

        self.wait_ready();

        unsafe {
            // Seleccionar drive y LBA high bits
            outb(self.base_port + 6, 0xE0 | (self.drive << 4) | ((lba >> 24) as u8 & 0x0F));
            
            // Sector count
            outb(self.base_port + 2, count);
            
            // LBA
            outb(self.base_port + 3, lba as u8);
            outb(self.base_port + 4, (lba >> 8) as u8);
            outb(self.base_port + 5, (lba >> 16) as u8);
            
            // Comando de escritura
            outb(self.base_port + 7, ATA_CMD_WRITE_SECTORS);
        }

        // Escribir datos
        for sector in 0..count as usize {
            if !self.wait_drq() {
                return Err("Disk write error");
            }

            let offset = sector * 512;
            for i in (0..512).step_by(2) {
                let word = (buffer[offset + i] as u16) | ((buffer[offset + i + 1] as u16) << 8);
                unsafe { outw(self.base_port, word) };
            }
        }

        // Flush
        self.wait_ready();

        Ok(())
    }

    /// Identificar disco
    pub fn identify(&self) -> Option<DiskInfo> {
        self.wait_ready();

        unsafe {
            outb(self.base_port + 6, 0xA0 | (self.drive << 4));
            outb(self.base_port + 2, 0);
            outb(self.base_port + 3, 0);
            outb(self.base_port + 4, 0);
            outb(self.base_port + 5, 0);
            outb(self.base_port + 7, ATA_CMD_IDENTIFY);
        }

        if !self.wait_drq() {
            return None;
        }

        let mut buffer = [0u16; 256];
        for i in 0..256 {
            buffer[i] = unsafe { inw(self.base_port) };
        }

        Some(DiskInfo {
            sectors: ((buffer[61] as u32) << 16) | (buffer[60] as u32),
            model: extract_string(&buffer[27..47]),
            serial: extract_string(&buffer[10..20]),
        })
    }
}

/// Información del disco
pub struct DiskInfo {
    pub sectors: u32,
    pub model: [u8; 40],
    pub serial: [u8; 20],
}

impl DiskInfo {
    /// Tamaño en MB
    pub fn size_mb(&self) -> u32 {
        self.sectors / 2048
    }
}

/// Extraer string de buffer ATA
fn extract_string<const N: usize>(buffer: &[u16]) -> [u8; N] {
    let mut result = [0u8; N];
    for (i, &word) in buffer.iter().enumerate() {
        if i * 2 < N {
            result[i * 2] = (word >> 8) as u8;
        }
        if i * 2 + 1 < N {
            result[i * 2 + 1] = word as u8;
        }
    }
    result
}

// I/O port functions
#[inline]
unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    core::arch::asm!("in al, dx", out("al") value, in("dx") port, options(nomem, nostack, preserves_flags));
    value
}

#[inline]
unsafe fn inw(port: u16) -> u16 {
    let value: u16;
    core::arch::asm!("in ax, dx", out("ax") value, in("dx") port, options(nomem, nostack, preserves_flags));
    value
}

#[inline]
unsafe fn outb(port: u16, value: u8) {
    core::arch::asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack, preserves_flags));
}

#[inline]
unsafe fn outw(port: u16, value: u16) {
    core::arch::asm!("out dx, ax", in("dx") port, in("ax") value, options(nomem, nostack, preserves_flags));
}
