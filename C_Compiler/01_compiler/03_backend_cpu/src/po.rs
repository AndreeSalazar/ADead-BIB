// ============================================================
// .Po Output — FastOS format
// ============================================================

pub struct PoOutput;

impl PoOutput {
    pub fn new() -> Self {
        Self
    }

    pub fn generate(
        &self,
        code: &[u8],
        data: &[u8],
        output_path: &str,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        let header_size = 32u32;
        let code_offset = header_size;
        let code_size = code.len() as u32;
        let data_offset = code_offset + code_size;
        let data_size = data.len() as u32;

        let mut header = [0u8; 32];
        header[0..4].copy_from_slice(&0x506F4F53u32.to_le_bytes());
        header[4] = 0x80;
        header[5] = 64u8;
        header[6..8].copy_from_slice(&0u16.to_le_bytes());
        header[8..12].copy_from_slice(&code_offset.to_le_bytes());
        header[12..16].copy_from_slice(&code_size.to_le_bytes());
        header[16..20].copy_from_slice(&data_offset.to_le_bytes());
        header[20..24].copy_from_slice(&data_size.to_le_bytes());
        header[24..28].copy_from_slice(&0u32.to_le_bytes());
        header[28..32].copy_from_slice(&0u32.to_le_bytes());

        let mut bin = Vec::new();
        bin.extend_from_slice(&header);
        bin.extend_from_slice(code);
        bin.extend_from_slice(data);
        std::fs::write(output_path, &bin)?;
        Ok(bin.len())
    }
}

impl Default for PoOutput {
    fn default() -> Self {
        Self::new()
    }
}
