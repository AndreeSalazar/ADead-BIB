// ============================================================
// Po Output — FastOS Native Format
// ============================================================
// Formato nativo de FastOS con header minimo de 24 bytes.
// Sin CRT innecesario. Sin overhead. Solo lo que usas.
// ============================================================

/// Magic bytes para .Po: "FASTOS"
pub const PO_MAGIC: [u8; 6] = *b"FASTOS";
pub const PO_VERSION: u16 = 1;

/// Header del formato .Po (24 bytes exactos)
#[derive(Debug, Clone)]
pub struct PoHeader {
    pub magic: [u8; 6],   // "FASTOS" (6 bytes)
    pub version: u16,     // version (2 bytes)
    pub code_offset: u32, // offset al code section (4 bytes)
    pub code_size: u32,   // tamano del code (4 bytes)
    pub data_offset: u32, // offset al data section (4 bytes)
    pub data_size: u32,   // tamano del data (4 bytes)
}

pub struct PoOutput;

impl PoOutput {
    pub fn new() -> Self {
        Self
    }

    /// Genera un binario .Po nativo
    pub fn generate(
        &self,
        code: &[u8],
        data: &[u8],
        output_path: &str,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        let header_size = 24u32;
        let code_offset = header_size;
        let code_size = code.len() as u32;
        let data_offset = code_offset + code_size;
        let data_size = data.len() as u32;

        let mut binary = Vec::new();

        // Header (24 bytes)
        binary.extend_from_slice(&PO_MAGIC); // 6 bytes
        binary.extend_from_slice(&PO_VERSION.to_le_bytes()); // 2 bytes
        binary.extend_from_slice(&code_offset.to_le_bytes()); // 4 bytes
        binary.extend_from_slice(&code_size.to_le_bytes()); // 4 bytes
        binary.extend_from_slice(&data_offset.to_le_bytes()); // 4 bytes
        binary.extend_from_slice(&data_size.to_le_bytes()); // 4 bytes

        // Code section
        binary.extend_from_slice(code);

        // Data section
        binary.extend_from_slice(data);

        let total = binary.len();
        std::fs::write(output_path, &binary)?;
        Ok(total)
    }
}

impl Default for PoOutput {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_po_header_size() {
        // 6 + 2 + 4 + 4 + 4 + 4 = 24 bytes
        assert_eq!(
            std::mem::size_of::<[u8; 6]>()
                + std::mem::size_of::<u16>()
                + 4 * std::mem::size_of::<u32>(),
            24
        );
    }
}
