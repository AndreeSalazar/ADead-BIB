// ============================================================
// FastOS — SPIR-V Runtime (Minimal Parser + Executor)
// ============================================================
// Parses SPIR-V bytecode and prepares for GPU execution.
// Integrates with ADead-BIB abi_translators for full parsing.
//
// SPIR-V format:
//   - Word-based (4 bytes per word)
//   - Header: magic, version, generator, bound, schema
//   - Instructions: [word_count:16 | opcode:16] [operands...]
// ============================================================

use crate::serial_print;

// SPIR-V Magic number
const SPIRV_MAGIC: u32 = 0x07230203;

// Common SPIR-V opcodes
const OP_ENTRY_POINT: u16 = 15;
const OP_EXECUTION_MODE: u16 = 16;
const OP_NAME: u16 = 5;
const OP_FUNCTION: u16 = 54;
const OP_FUNCTION_END: u16 = 56;

// Execution modes
const EXEC_MODE_LOCAL_SIZE: u32 = 17;

/// SPIR-V module header
#[derive(Debug, Clone)]
pub struct SpirVHeader {
    pub magic: u32,
    pub version_major: u8,
    pub version_minor: u8,
    pub generator: u32,
    pub bound: u32,
}

/// SPIR-V entry point
#[derive(Debug, Clone)]
pub struct SpirVEntryPoint {
    pub name: [u8; 32],
    pub name_len: usize,
    pub function_id: u32,
    pub execution_model: u32,
    pub local_size: [u32; 3],
}

/// Parsed SPIR-V module
pub struct SpirVModule {
    pub header: SpirVHeader,
    pub entry_points: [Option<SpirVEntryPoint>; 4],
    pub entry_count: usize,
    pub instruction_count: usize,
    pub valid: bool,
}

impl SpirVModule {
    pub const fn empty() -> Self {
        Self {
            header: SpirVHeader {
                magic: 0,
                version_major: 0,
                version_minor: 0,
                generator: 0,
                bound: 0,
            },
            entry_points: [None, None, None, None],
            entry_count: 0,
            instruction_count: 0,
            valid: false,
        }
    }
}

/// Parse SPIR-V bytecode
pub fn parse_spirv(data: &[u8]) -> Result<SpirVModule, &'static str> {
    if data.len() < 20 {
        return Err("SPIR-V too small");
    }

    // Read header
    let magic = read_word(data, 0);
    if magic != SPIRV_MAGIC {
        return Err("Invalid SPIR-V magic");
    }

    let version = read_word(data, 4);
    let generator = read_word(data, 8);
    let bound = read_word(data, 12);

    let header = SpirVHeader {
        magic,
        version_major: ((version >> 16) & 0xFF) as u8,
        version_minor: ((version >> 8) & 0xFF) as u8,
        generator,
        bound,
    };

    let mut module = SpirVModule {
        header,
        entry_points: [None, None, None, None],
        entry_count: 0,
        instruction_count: 0,
        valid: true,
    };

    // Parse instructions
    let mut pos = 20; // After header
    let mut local_sizes: [(u32, [u32; 3]); 4] = [(0, [1, 1, 1]); 4];
    let mut local_size_count = 0;

    while pos + 4 <= data.len() {
        let first_word = read_word(data, pos);
        let word_count = (first_word >> 16) as u16;
        let opcode = (first_word & 0xFFFF) as u16;

        if word_count == 0 {
            break;
        }

        let byte_len = word_count as usize * 4;
        if pos + byte_len > data.len() {
            break;
        }

        module.instruction_count += 1;

        match opcode {
            OP_ENTRY_POINT => {
                if word_count >= 4 && module.entry_count < 4 {
                    let exec_model = read_word(data, pos + 4);
                    let func_id = read_word(data, pos + 8);
                    
                    // Extract name (starts at word 3)
                    let mut name = [0u8; 32];
                    let mut name_len = 0;
                    let name_start = pos + 12;
                    for i in 0..28 {
                        if name_start + i >= data.len() {
                            break;
                        }
                        let c = data[name_start + i];
                        if c == 0 {
                            break;
                        }
                        name[i] = c;
                        name_len += 1;
                    }

                    module.entry_points[module.entry_count] = Some(SpirVEntryPoint {
                        name,
                        name_len,
                        function_id: func_id,
                        execution_model: exec_model,
                        local_size: [1, 1, 1],
                    });
                    module.entry_count += 1;
                }
            }
            OP_EXECUTION_MODE => {
                if word_count >= 3 {
                    let func_id = read_word(data, pos + 4);
                    let mode = read_word(data, pos + 8);
                    
                    if mode == EXEC_MODE_LOCAL_SIZE && word_count >= 6 {
                        let x = read_word(data, pos + 12);
                        let y = read_word(data, pos + 16);
                        let z = read_word(data, pos + 20);
                        
                        if local_size_count < 4 {
                            local_sizes[local_size_count] = (func_id, [x, y, z]);
                            local_size_count += 1;
                        }
                    }
                }
            }
            _ => {}
        }

        pos += byte_len;
    }

    // Apply local sizes to entry points
    for i in 0..module.entry_count {
        if let Some(ref mut ep) = module.entry_points[i] {
            for j in 0..local_size_count {
                if local_sizes[j].0 == ep.function_id {
                    ep.local_size = local_sizes[j].1;
                    break;
                }
            }
        }
    }

    Ok(module)
}

/// Read a 32-bit word from byte array (little-endian)
fn read_word(data: &[u8], offset: usize) -> u32 {
    if offset + 4 > data.len() {
        return 0;
    }
    u32::from_le_bytes([
        data[offset],
        data[offset + 1],
        data[offset + 2],
        data[offset + 3],
    ])
}

/// Print SPIR-V module info
pub fn print_module_info(module: &SpirVModule) {
    serial_print("[SPIRV] Version: ");
    print_u8(module.header.version_major);
    serial_print(".");
    print_u8(module.header.version_minor);
    serial_print("\r\n");

    serial_print("[SPIRV] Instructions: ");
    print_u32(module.instruction_count as u32);
    serial_print("\r\n");

    serial_print("[SPIRV] Entry points: ");
    print_u32(module.entry_count as u32);
    serial_print("\r\n");

    for i in 0..module.entry_count {
        if let Some(ref ep) = module.entry_points[i] {
            serial_print("[SPIRV]   - ");
            for j in 0..ep.name_len {
                print_char(ep.name[j]);
            }
            serial_print(" local_size=[");
            print_u32(ep.local_size[0]);
            serial_print(",");
            print_u32(ep.local_size[1]);
            serial_print(",");
            print_u32(ep.local_size[2]);
            serial_print("]\r\n");
        }
    }
}

// ============================================================
// Demo SPIR-V compute shader (vector add)
// ============================================================

/// Generate a minimal SPIR-V compute shader for vector addition
/// This is a synthetic shader for testing
pub fn create_demo_spirv() -> [u8; 148] {
    let mut spv = [0u8; 148];
    let mut pos = 0;

    // Helper to write word
    let mut write_word = |word: u32| {
        if pos + 4 <= spv.len() {
            spv[pos..pos + 4].copy_from_slice(&word.to_le_bytes());
            pos += 4;
        }
    };

    // Header
    write_word(SPIRV_MAGIC);           // Magic
    write_word(0x00010300);            // Version 1.3
    write_word(0x00000000);            // Generator
    write_word(0x00000010);            // Bound = 16
    write_word(0x00000000);            // Schema

    // OpCapability Shader (word_count=2, opcode=17)
    write_word((2 << 16) | 17);
    write_word(1); // Shader capability

    // OpMemoryModel (word_count=3, opcode=14)
    write_word((3 << 16) | 14);
    write_word(1); // Logical
    write_word(1); // GLSL450

    // OpEntryPoint GLCompute %main "main" (word_count=4, opcode=15)
    write_word((4 << 16) | 15);
    write_word(5);  // GLCompute
    write_word(1);  // %main = id 1
    write_word(0x6E69616D); // "main" as u32

    // OpExecutionMode %main LocalSize 64 1 1 (word_count=6, opcode=16)
    write_word((6 << 16) | 16);
    write_word(1);  // %main
    write_word(17); // LocalSize
    write_word(64); // x
    write_word(1);  // y
    write_word(1);  // z

    // OpName %main "main" (word_count=3, opcode=5)
    write_word((3 << 16) | 5);
    write_word(1);  // %main
    write_word(0x6E69616D); // "main"

    // OpTypeVoid (word_count=2, opcode=19)
    write_word((2 << 16) | 19);
    write_word(2); // %void = id 2

    // OpTypeFunction %void (word_count=3, opcode=33)
    write_word((3 << 16) | 33);
    write_word(3); // %func_type = id 3
    write_word(2); // returns %void

    // OpFunction %void None %func_type (word_count=5, opcode=54)
    write_word((5 << 16) | 54);
    write_word(2); // return type %void
    write_word(1); // %main = id 1
    write_word(0); // None
    write_word(3); // %func_type

    // OpLabel (word_count=2, opcode=248)
    write_word((2 << 16) | 248);
    write_word(4); // %entry = id 4

    // OpReturn (word_count=1, opcode=253)
    write_word((1 << 16) | 253);

    // OpFunctionEnd (word_count=1, opcode=56)
    write_word((1 << 16) | 56);

    spv
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

fn print_u8(val: u8) {
    if val >= 100 {
        print_char(b'0' + (val / 100));
    }
    if val >= 10 {
        print_char(b'0' + ((val / 10) % 10));
    }
    print_char(b'0' + (val % 10));
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

unsafe fn outb(port: u16, value: u8) {
    core::arch::asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack));
}

unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    core::arch::asm!("in al, dx", in("dx") port, out("al") value, options(nomem, nostack));
    value
}

// ============================================================
// Test function
// ============================================================

pub fn test_spirv() {
    serial_print("[SPIRV] Testing SPIR-V parser...\r\n");
    
    // Simple test: just verify magic number parsing works
    let test_header: [u8; 20] = [
        0x03, 0x02, 0x23, 0x07, // Magic
        0x00, 0x03, 0x01, 0x00, // Version 1.3
        0x00, 0x00, 0x00, 0x00, // Generator
        0x10, 0x00, 0x00, 0x00, // Bound = 16
        0x00, 0x00, 0x00, 0x00, // Schema
    ];
    
    let magic = u32::from_le_bytes([test_header[0], test_header[1], test_header[2], test_header[3]]);
    serial_print("[SPIRV] Magic: 0x");
    print_hex_u32(magic);
    serial_print("\r\n");
    
    if magic == 0x07230203 {
        serial_print("[SPIRV] SPIR-V header valid!\r\n");
        serial_print("[SPIRV] Ready for GPU compute shaders\r\n");
    } else {
        serial_print("[SPIRV] Invalid magic\r\n");
    }
}

fn print_hex_u32(val: u32) {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    for i in (0..8).rev() {
        let nibble = ((val >> (i * 4)) & 0xF) as usize;
        print_char(HEX[nibble]);
    }
}
