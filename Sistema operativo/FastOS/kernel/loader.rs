// ============================================================================
// FastOS ADead-BIB Binary Loader
// ============================================================================
// Carga y ejecuta binarios .adB compilados con ADead-BIB
// Soporta código CPU (x86-64) y GPU (preparado para SPIR-V)
// ============================================================================

use crate::adead_bib::{ADeadBinary, ADEAD_MAGIC, ADEAD_FLAG_CPU, ADEAD_FLAG_GPU};
use crate::syscall::SyscallTable;

/// Resultado de carga de binario
#[derive(Debug)]
pub enum LoadError {
    InvalidMagic,
    InvalidSize,
    UnsupportedVersion,
    UnsupportedFlags,
    ExecutionFailed,
}

/// Programa ADead-BIB cargado en memoria
pub struct LoadedProgram {
    pub header: ADeadBinary,
    pub code_ptr: *const u8,
    pub data_ptr: *const u8,
}

/// Cargador de binarios ADead-BIB
pub struct ADeadLoader {
    syscall_table: *const SyscallTable,
}

impl ADeadLoader {
    /// Crear nuevo cargador
    pub fn new(syscall_table: *const SyscallTable) -> Self {
        ADeadLoader { syscall_table }
    }

    /// Verificar si los datos son un binario ADead-BIB válido
    pub fn is_valid(data: &[u8]) -> bool {
        if data.len() < 32 {
            return false;
        }
        let magic = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        magic == ADEAD_MAGIC
    }

    /// Parsear header del binario
    pub fn parse_header(data: &[u8]) -> Result<ADeadBinary, LoadError> {
        if data.len() < 32 {
            return Err(LoadError::InvalidSize);
        }

        let magic = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        if magic != ADEAD_MAGIC {
            return Err(LoadError::InvalidMagic);
        }

        let version = u16::from_le_bytes([data[4], data[5]]);
        let flags = u16::from_le_bytes([data[6], data[7]]);
        let entry_point = u64::from_le_bytes([
            data[8], data[9], data[10], data[11],
            data[12], data[13], data[14], data[15],
        ]);
        let code_size = u64::from_le_bytes([
            data[16], data[17], data[18], data[19],
            data[20], data[21], data[22], data[23],
        ]);
        let data_size = u64::from_le_bytes([
            data[24], data[25], data[26], data[27],
            data[28], data[29], data[30], data[31],
        ]);

        Ok(ADeadBinary {
            magic,
            version,
            flags,
            entry_point,
            code_size,
            data_size,
        })
    }

    /// Cargar binario en memoria
    pub fn load(&self, data: &[u8]) -> Result<LoadedProgram, LoadError> {
        let header = Self::parse_header(data)?;

        // Verificar versión soportada
        if header.version > 1 {
            return Err(LoadError::UnsupportedVersion);
        }

        // Calcular punteros
        let header_size = 32usize;
        let code_ptr = unsafe { data.as_ptr().add(header_size) };
        let data_ptr = unsafe { code_ptr.add(header.code_size as usize) };

        Ok(LoadedProgram {
            header,
            code_ptr,
            data_ptr,
        })
    }

    /// Ejecutar programa cargado (solo CPU por ahora)
    pub unsafe fn execute(&self, program: &LoadedProgram) -> Result<i32, LoadError> {
        // Solo soportamos código CPU por ahora
        if program.header.flags & ADEAD_FLAG_CPU == 0 {
            return Err(LoadError::UnsupportedFlags);
        }

        // El entry point es relativo al inicio del código
        let entry = program.code_ptr.add(program.header.entry_point as usize);

        // Llamar al código como función que recibe puntero a syscall table
        // Signature: fn(syscall_table: *const SyscallTable) -> i32
        let func: extern "C" fn(*const SyscallTable) -> i32 = 
            core::mem::transmute(entry);

        let result = func(self.syscall_table);
        Ok(result)
    }
}

/// Crear un binario ADead-BIB de ejemplo (para testing)
pub fn create_test_binary() -> [u8; 64] {
    let mut binary = [0u8; 64];
    
    // Header
    binary[0..4].copy_from_slice(&ADEAD_MAGIC.to_le_bytes());  // Magic
    binary[4..6].copy_from_slice(&1u16.to_le_bytes());          // Version
    binary[6..8].copy_from_slice(&ADEAD_FLAG_CPU.to_le_bytes()); // Flags (CPU)
    binary[8..16].copy_from_slice(&0u64.to_le_bytes());         // Entry point
    binary[16..24].copy_from_slice(&16u64.to_le_bytes());       // Code size
    binary[24..32].copy_from_slice(&0u64.to_le_bytes());        // Data size
    
    // Código x86-64 simple: mov eax, 42; ret
    binary[32] = 0xB8;  // mov eax, imm32
    binary[33] = 42;    // valor 42
    binary[34] = 0x00;
    binary[35] = 0x00;
    binary[36] = 0x00;
    binary[37] = 0xC3;  // ret
    
    binary
}
