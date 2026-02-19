// ============================================================
// FastOS — ATA PIO Disk Driver
// ============================================================
// ATA PIO mode for reading/writing disk sectors.
// Primary bus: I/O ports 0x1F0-0x1F7, control 0x3F6
// Secondary bus: I/O ports 0x170-0x177, control 0x376
// ============================================================

use crate::arch::x86_64::port;

// Primary ATA bus ports
const ATA_PRIMARY_DATA: u16       = 0x1F0;
const ATA_PRIMARY_ERROR: u16      = 0x1F1;
const ATA_PRIMARY_SECTOR_COUNT: u16 = 0x1F2;
const ATA_PRIMARY_LBA_LO: u16    = 0x1F3;
const ATA_PRIMARY_LBA_MID: u16   = 0x1F4;
const ATA_PRIMARY_LBA_HI: u16    = 0x1F5;
const ATA_PRIMARY_DRIVE: u16     = 0x1F6;
const ATA_PRIMARY_STATUS: u16    = 0x1F7;
const ATA_PRIMARY_COMMAND: u16   = 0x1F7;
const ATA_PRIMARY_CONTROL: u16   = 0x3F6;

// ATA commands
const ATA_CMD_READ_SECTORS: u8   = 0x20;
const ATA_CMD_WRITE_SECTORS: u8  = 0x30;
const ATA_CMD_IDENTIFY: u8      = 0xEC;
const ATA_CMD_FLUSH_CACHE: u8   = 0xE7;

// ATA status bits
const ATA_STATUS_BSY: u8  = 0x80;  // Busy
const ATA_STATUS_DRDY: u8 = 0x40;  // Drive ready
const ATA_STATUS_DRQ: u8  = 0x08;  // Data request
const ATA_STATUS_ERR: u8  = 0x01;  // Error

/// Sector size in bytes
pub const SECTOR_SIZE: usize = 512;

/// Disk identification info
pub struct DiskInfo {
    pub present: bool,
    pub model: [u8; 40],
    pub sectors: u64,
    pub size_mb: u64,
}

static mut DISK_INFO: DiskInfo = DiskInfo {
    present: false,
    model: [0; 40],
    sectors: 0,
    size_mb: 0,
};

/// Initialize disk driver — detect primary master drive
pub fn init() {
    // Select master drive
    port::outb(ATA_PRIMARY_DRIVE, 0xA0);
    port::outb(ATA_PRIMARY_SECTOR_COUNT, 0);
    port::outb(ATA_PRIMARY_LBA_LO, 0);
    port::outb(ATA_PRIMARY_LBA_MID, 0);
    port::outb(ATA_PRIMARY_LBA_HI, 0);

    // Send IDENTIFY command
    port::outb(ATA_PRIMARY_COMMAND, ATA_CMD_IDENTIFY);

    // Check if drive exists
    let status = port::inb(ATA_PRIMARY_STATUS);
    if status == 0 {
        // No drive
        return;
    }

    // Wait for BSY to clear
    if !wait_not_busy() { return; }

    // Check for non-ATA drive (LBA mid/hi should be 0)
    if port::inb(ATA_PRIMARY_LBA_MID) != 0 || port::inb(ATA_PRIMARY_LBA_HI) != 0 {
        return; // Not ATA
    }

    // Wait for DRQ or ERR
    if !wait_drq() { return; }

    // Read 256 words of identification data
    let mut identify_data = [0u16; 256];
    for i in 0..256 {
        identify_data[i] = port::inw(ATA_PRIMARY_DATA);
    }

    unsafe {
        DISK_INFO.present = true;

        // Extract model string (words 27-46, byte-swapped)
        for i in 0..20 {
            let word = identify_data[27 + i];
            DISK_INFO.model[i * 2] = (word >> 8) as u8;
            DISK_INFO.model[i * 2 + 1] = (word & 0xFF) as u8;
        }

        // Extract total sectors (LBA48: words 100-103, or LBA28: words 60-61)
        let lba48 = (identify_data[103] as u64) << 48
                   | (identify_data[102] as u64) << 32
                   | (identify_data[101] as u64) << 16
                   | (identify_data[100] as u64);

        if lba48 > 0 {
            DISK_INFO.sectors = lba48;
        } else {
            DISK_INFO.sectors = (identify_data[61] as u64) << 16
                              | (identify_data[60] as u64);
        }

        DISK_INFO.size_mb = DISK_INFO.sectors * SECTOR_SIZE as u64 / (1024 * 1024);
    }
}

/// Read sectors from disk using LBA28
/// - lba: starting sector (LBA address)
/// - count: number of sectors to read (1-256, 0 means 256)
/// - buf: output buffer (must be at least count * 512 bytes)
/// Returns true on success
pub fn read_sectors(lba: u32, count: u8, buf: &mut [u8]) -> bool {
    let sectors = if count == 0 { 256usize } else { count as usize };
    if buf.len() < sectors * SECTOR_SIZE { return false; }

    // Select drive + LBA bits 24-27
    port::outb(ATA_PRIMARY_DRIVE, 0xE0 | ((lba >> 24) & 0x0F) as u8);

    // Set sector count
    port::outb(ATA_PRIMARY_SECTOR_COUNT, count);

    // Set LBA address
    port::outb(ATA_PRIMARY_LBA_LO, (lba & 0xFF) as u8);
    port::outb(ATA_PRIMARY_LBA_MID, ((lba >> 8) & 0xFF) as u8);
    port::outb(ATA_PRIMARY_LBA_HI, ((lba >> 16) & 0xFF) as u8);

    // Send READ SECTORS command
    port::outb(ATA_PRIMARY_COMMAND, ATA_CMD_READ_SECTORS);

    for sector in 0..sectors {
        // Wait for data ready
        if !wait_drq() { return false; }

        // Read 256 words (512 bytes)
        let offset = sector * SECTOR_SIZE;
        for i in 0..256 {
            let word = port::inw(ATA_PRIMARY_DATA);
            buf[offset + i * 2] = (word & 0xFF) as u8;
            buf[offset + i * 2 + 1] = (word >> 8) as u8;
        }
    }

    true
}

/// Write sectors to disk using LBA28
/// - lba: starting sector (LBA address)
/// - count: number of sectors to write (1-256, 0 means 256)
/// - buf: input buffer (must be at least count * 512 bytes)
/// Returns true on success
pub fn write_sectors(lba: u32, count: u8, buf: &[u8]) -> bool {
    let sectors = if count == 0 { 256usize } else { count as usize };
    if buf.len() < sectors * SECTOR_SIZE { return false; }

    // Select drive + LBA bits 24-27
    port::outb(ATA_PRIMARY_DRIVE, 0xE0 | ((lba >> 24) & 0x0F) as u8);

    // Set sector count
    port::outb(ATA_PRIMARY_SECTOR_COUNT, count);

    // Set LBA address
    port::outb(ATA_PRIMARY_LBA_LO, (lba & 0xFF) as u8);
    port::outb(ATA_PRIMARY_LBA_MID, ((lba >> 8) & 0xFF) as u8);
    port::outb(ATA_PRIMARY_LBA_HI, ((lba >> 16) & 0xFF) as u8);

    // Send WRITE SECTORS command
    port::outb(ATA_PRIMARY_COMMAND, ATA_CMD_WRITE_SECTORS);

    for sector in 0..sectors {
        // Wait for data ready
        if !wait_drq() { return false; }

        // Write 256 words (512 bytes)
        let offset = sector * SECTOR_SIZE;
        for i in 0..256 {
            let word = (buf[offset + i * 2 + 1] as u16) << 8
                     | (buf[offset + i * 2] as u16);
            port::outw(ATA_PRIMARY_DATA, word);
        }
    }

    // Flush cache
    port::outb(ATA_PRIMARY_COMMAND, ATA_CMD_FLUSH_CACHE);
    wait_not_busy();

    true
}

/// Get disk info
pub fn info() -> &'static DiskInfo {
    unsafe { &DISK_INFO }
}

/// Check if disk is present
pub fn is_present() -> bool {
    unsafe { DISK_INFO.present }
}

/// Get disk size in MB
pub fn size_mb() -> u64 {
    unsafe { DISK_INFO.size_mb }
}

/// Get total sectors
pub fn total_sectors() -> u64 {
    unsafe { DISK_INFO.sectors }
}

// ============================================================
// Internal helpers
// ============================================================

/// Wait for BSY flag to clear (timeout ~1 second)
fn wait_not_busy() -> bool {
    for _ in 0..100_000 {
        let status = port::inb(ATA_PRIMARY_STATUS);
        if status & ATA_STATUS_BSY == 0 {
            return true;
        }
        port::io_wait();
    }
    false
}

/// Wait for DRQ flag to set (data ready) or ERR
fn wait_drq() -> bool {
    for _ in 0..100_000 {
        let status = port::inb(ATA_PRIMARY_STATUS);
        if status & ATA_STATUS_ERR != 0 { return false; }
        if status & ATA_STATUS_DRQ != 0 { return true; }
        port::io_wait();
    }
    false
}

/// Software reset the ATA bus
pub fn reset() {
    port::outb(ATA_PRIMARY_CONTROL, 0x04); // Set SRST
    port::io_wait();
    port::io_wait();
    port::io_wait();
    port::io_wait();
    port::outb(ATA_PRIMARY_CONTROL, 0x00); // Clear SRST
    wait_not_busy();
}
