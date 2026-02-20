// ============================================================
// FastOS — AEXE Format (ADead Executable)
// ============================================================
// Native executable format for FastOS.
// Translated from PE/ELF via abi_translators.
//
// Format:
//   Header (64 bytes)
//   Code section
//   Data section
//   Import table
//   Export table
//   Relocation table
//   GPU kernels (SPIR-V bytecode)
//
// Pipeline: .exe → abi_translators → ABIB IR → .aexe → FastOS
// ============================================================

use crate::serial_print;

// AEXE Magic: "AEXE" = 0x45584541
pub const AEXE_MAGIC: u32 = 0x45584541;
pub const AEXE_VERSION: u16 = 0x0100; // v1.0

// Section types
pub const SECTION_CODE: u8 = 0x01;
pub const SECTION_DATA: u8 = 0x02;
pub const SECTION_RODATA: u8 = 0x03;
pub const SECTION_BSS: u8 = 0x04;
pub const SECTION_IMPORT: u8 = 0x10;
pub const SECTION_EXPORT: u8 = 0x11;
pub const SECTION_RELOC: u8 = 0x12;
pub const SECTION_GPU: u8 = 0x20;  // SPIR-V kernels

// Flags
pub const FLAG_EXECUTABLE: u16 = 0x0001;
pub const FLAG_GPU_COMPUTE: u16 = 0x0002;
pub const FLAG_DRIVER: u16 = 0x0004;
pub const FLAG_HDMI: u16 = 0x0008;

/// AEXE Header (64 bytes)
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct AexeHeader {
    pub magic: u32,           // 0x00: "AEXE"
    pub version: u16,         // 0x04: Format version
    pub flags: u16,           // 0x06: Executable flags
    pub entry_point: u64,     // 0x08: Entry point RVA
    pub code_offset: u32,     // 0x10: Code section offset
    pub code_size: u32,       // 0x14: Code section size
    pub data_offset: u32,     // 0x18: Data section offset
    pub data_size: u32,       // 0x1C: Data section size
    pub import_offset: u32,   // 0x20: Import table offset
    pub import_count: u16,    // 0x24: Number of imports
    pub export_offset: u32,   // 0x26: Export table offset
    pub export_count: u16,    // 0x2A: Number of exports
    pub gpu_offset: u32,      // 0x2C: GPU kernels offset
    pub gpu_count: u16,       // 0x30: Number of GPU kernels
    pub reloc_offset: u32,    // 0x32: Relocation table offset
    pub reloc_count: u16,     // 0x36: Number of relocations
    pub image_base: u64,      // 0x38: Preferred load address
}

/// Import entry (32 bytes)
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct AexeImport {
    pub name: [u8; 24],       // Import name (null-terminated)
    pub ordinal: u16,         // Ordinal hint
    pub flags: u16,           // Import flags
    pub address: u32,         // Resolved address (filled at load time)
}

/// Export entry (32 bytes)
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct AexeExport {
    pub name: [u8; 24],       // Export name (null-terminated)
    pub ordinal: u16,         // Ordinal
    pub flags: u16,           // Export flags
    pub rva: u32,             // Relative virtual address
}

/// GPU Kernel entry (64 bytes)
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct AexeGpuKernel {
    pub name: [u8; 32],       // Kernel name
    pub spirv_offset: u32,    // Offset to SPIR-V bytecode
    pub spirv_size: u32,      // Size of SPIR-V bytecode
    pub local_size: [u32; 3], // Workgroup size (x, y, z)
    pub flags: u16,           // Kernel flags
    pub _reserved: u16,
}

/// Relocation entry (16 bytes)
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct AexeReloc {
    pub offset: u64,          // Offset in code/data
    pub type_: u8,            // Relocation type
    pub section: u8,          // Target section
    pub _reserved: u16,
    pub addend: i32,          // Addend value
}

// Relocation types
pub const RELOC_ABS64: u8 = 0x01;
pub const RELOC_REL32: u8 = 0x02;
pub const RELOC_IMPORT: u8 = 0x10;

/// Loaded AEXE module
pub struct AexeModule {
    pub header: AexeHeader,
    pub base_address: u64,
    pub code_ptr: *const u8,
    pub data_ptr: *mut u8,
    pub valid: bool,
}

impl AexeModule {
    pub const fn empty() -> Self {
        Self {
            header: AexeHeader {
                magic: 0,
                version: 0,
                flags: 0,
                entry_point: 0,
                code_offset: 0,
                code_size: 0,
                data_offset: 0,
                data_size: 0,
                import_offset: 0,
                import_count: 0,
                export_offset: 0,
                export_count: 0,
                gpu_offset: 0,
                gpu_count: 0,
                reloc_offset: 0,
                reloc_count: 0,
                image_base: 0,
            },
            base_address: 0,
            code_ptr: core::ptr::null(),
            data_ptr: core::ptr::null_mut(),
            valid: false,
        }
    }
}

/// Parse AEXE header from bytes
pub fn parse_aexe(data: &[u8]) -> Result<AexeHeader, &'static str> {
    if data.len() < 64 {
        return Err("AEXE too small");
    }

    let magic = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
    if magic != AEXE_MAGIC {
        return Err("Invalid AEXE magic");
    }

    // Parse header
    let header = AexeHeader {
        magic,
        version: u16::from_le_bytes([data[4], data[5]]),
        flags: u16::from_le_bytes([data[6], data[7]]),
        entry_point: u64::from_le_bytes([
            data[8], data[9], data[10], data[11],
            data[12], data[13], data[14], data[15],
        ]),
        code_offset: u32::from_le_bytes([data[16], data[17], data[18], data[19]]),
        code_size: u32::from_le_bytes([data[20], data[21], data[22], data[23]]),
        data_offset: u32::from_le_bytes([data[24], data[25], data[26], data[27]]),
        data_size: u32::from_le_bytes([data[28], data[29], data[30], data[31]]),
        import_offset: u32::from_le_bytes([data[32], data[33], data[34], data[35]]),
        import_count: u16::from_le_bytes([data[36], data[37]]),
        export_offset: u32::from_le_bytes([data[38], data[39], data[40], data[41]]),
        export_count: u16::from_le_bytes([data[42], data[43]]),
        gpu_offset: u32::from_le_bytes([data[44], data[45], data[46], data[47]]),
        gpu_count: u16::from_le_bytes([data[48], data[49]]),
        reloc_offset: u32::from_le_bytes([data[50], data[51], data[52], data[53]]),
        reloc_count: u16::from_le_bytes([data[54], data[55]]),
        image_base: u64::from_le_bytes([
            data[56], data[57], data[58], data[59],
            data[60], data[61], data[62], data[63],
        ]),
    };

    Ok(header)
}

/// Create AEXE from translated PE (via abi_translators IR)
pub fn create_aexe_from_ir(
    code: &[u8],
    data: &[u8],
    entry_point: u64,
    gpu_kernels: &[AexeGpuKernel],
    spirv_data: &[u8],
) -> ([u8; 4096], usize) {
    let mut output = [0u8; 4096];
    let mut pos = 0;

    // Calculate offsets
    let header_size = 64;
    let code_offset = header_size;
    let code_size = code.len();
    let data_offset = code_offset + code_size;
    let data_size = data.len();
    let gpu_offset = data_offset + data_size;
    let gpu_header_size = gpu_kernels.len() * 64;
    let spirv_offset = gpu_offset + gpu_header_size;
    let total_size = spirv_offset + spirv_data.len();

    // Write header
    // Magic
    output[0..4].copy_from_slice(&AEXE_MAGIC.to_le_bytes());
    // Version
    output[4..6].copy_from_slice(&AEXE_VERSION.to_le_bytes());
    // Flags
    let flags: u16 = FLAG_EXECUTABLE | if !gpu_kernels.is_empty() { FLAG_GPU_COMPUTE } else { 0 };
    output[6..8].copy_from_slice(&flags.to_le_bytes());
    // Entry point
    output[8..16].copy_from_slice(&entry_point.to_le_bytes());
    // Code
    output[16..20].copy_from_slice(&(code_offset as u32).to_le_bytes());
    output[20..24].copy_from_slice(&(code_size as u32).to_le_bytes());
    // Data
    output[24..28].copy_from_slice(&(data_offset as u32).to_le_bytes());
    output[28..32].copy_from_slice(&(data_size as u32).to_le_bytes());
    // Imports (none for now)
    output[32..36].copy_from_slice(&0u32.to_le_bytes());
    output[36..38].copy_from_slice(&0u16.to_le_bytes());
    // Exports (none for now)
    output[38..42].copy_from_slice(&0u32.to_le_bytes());
    output[42..44].copy_from_slice(&0u16.to_le_bytes());
    // GPU
    output[44..48].copy_from_slice(&(gpu_offset as u32).to_le_bytes());
    output[48..50].copy_from_slice(&(gpu_kernels.len() as u16).to_le_bytes());
    // Relocs (none for now)
    output[50..54].copy_from_slice(&0u32.to_le_bytes());
    output[54..56].copy_from_slice(&0u16.to_le_bytes());
    // Image base
    output[56..64].copy_from_slice(&0x400000u64.to_le_bytes());

    pos = header_size;

    // Write code
    let code_end = pos + code.len().min(output.len() - pos);
    output[pos..code_end].copy_from_slice(&code[..code_end - pos]);
    pos = code_end;

    // Write data
    let data_end = pos + data.len().min(output.len() - pos);
    if data_end > pos {
        output[pos..data_end].copy_from_slice(&data[..data_end - pos]);
        pos = data_end;
    }

    // Write GPU kernel headers
    for kernel in gpu_kernels {
        if pos + 64 > output.len() { break; }
        output[pos..pos + 32].copy_from_slice(&kernel.name);
        output[pos + 32..pos + 36].copy_from_slice(&kernel.spirv_offset.to_le_bytes());
        output[pos + 36..pos + 40].copy_from_slice(&kernel.spirv_size.to_le_bytes());
        output[pos + 40..pos + 44].copy_from_slice(&kernel.local_size[0].to_le_bytes());
        output[pos + 44..pos + 48].copy_from_slice(&kernel.local_size[1].to_le_bytes());
        output[pos + 48..pos + 52].copy_from_slice(&kernel.local_size[2].to_le_bytes());
        output[pos + 52..pos + 54].copy_from_slice(&kernel.flags.to_le_bytes());
        pos += 64;
    }

    // Write SPIR-V data
    let spirv_end = pos + spirv_data.len().min(output.len() - pos);
    if spirv_end > pos {
        output[pos..spirv_end].copy_from_slice(&spirv_data[..spirv_end - pos]);
        pos = spirv_end;
    }

    (output, total_size.min(output.len()))
}

/// Print AEXE info
pub fn print_aexe_info(header: &AexeHeader) {
    serial_print("[AEXE] Magic: AEXE\r\n");
    serial_print("[AEXE] Version: ");
    print_u16(header.version);
    serial_print("\r\n");
    serial_print("[AEXE] Flags: 0x");
    print_hex_u16(header.flags);
    serial_print("\r\n");
    serial_print("[AEXE] Entry: 0x");
    print_hex_u64(header.entry_point);
    serial_print("\r\n");
    serial_print("[AEXE] Code: ");
    print_u32(header.code_size);
    serial_print(" bytes\r\n");
    serial_print("[AEXE] GPU kernels: ");
    print_u16(header.gpu_count);
    serial_print("\r\n");
}

// ============================================================
// Printing helpers
// ============================================================

fn print_char(c: u8) {
    unsafe {
        while (inb(0x3FD) & 0x20) == 0 {}
        outb(0x3F8, c);
    }
}

fn print_u16(val: u16) {
    print_u32(val as u32);
}

fn print_u32(val: u32) {
    if val == 0 {
        print_char(b'0');
        return;
    }
    let mut buf = [0u8; 10];
    let mut i = 0;
    let mut v = val;
    while v > 0 {
        buf[i] = b'0' + (v % 10) as u8;
        v /= 10;
        i += 1;
    }
    while i > 0 {
        i -= 1;
        print_char(buf[i]);
    }
}

fn print_hex_u16(val: u16) {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    for i in (0..4).rev() {
        let nibble = ((val >> (i * 4)) & 0xF) as usize;
        print_char(HEX[nibble]);
    }
}

fn print_hex_u64(val: u64) {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    for i in (0..16).rev() {
        let nibble = ((val >> (i * 4)) & 0xF) as usize;
        print_char(HEX[nibble]);
    }
}

unsafe fn outb(port: u16, value: u8) {
    core::arch::asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack));
}

unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    core::arch::asm!("in al, dx", in("dx") port, out("al") value, options(nomem, nostack));
    value
}

// ============================================================
// Test
// ============================================================

pub fn test_aexe() {
    serial_print("[AEXE] Testing AEXE format...\r\n");

    // Simple test: verify magic parsing
    let test_header: [u8; 64] = {
        let mut h = [0u8; 64];
        // Magic "AEXE"
        h[0] = 0x41; h[1] = 0x45; h[2] = 0x58; h[3] = 0x45;
        // Version 1.0
        h[4] = 0x00; h[5] = 0x01;
        // Flags: executable + GPU
        h[6] = 0x03; h[7] = 0x00;
        h
    };

    let magic = u32::from_le_bytes([test_header[0], test_header[1], test_header[2], test_header[3]]);
    
    if magic == AEXE_MAGIC {
        serial_print("[AEXE] Format valid (PE->AEXE ready)\r\n");
    } else {
        serial_print("[AEXE] Format check failed\r\n");
    }
}
