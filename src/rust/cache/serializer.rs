// ============================================================
// Cache Serializer — AST → bytes para fastos.bib
// ============================================================

use super::{ADeadCache, CACHE_MAGIC, CACHE_VERSION};

/// Serializa un ADeadCache a bytes para escribir a fastos.bib
pub fn serialize(cache: &ADeadCache) -> Vec<u8> {
    let mut bytes = Vec::new();

    // Header: magic (8 bytes)
    bytes.extend_from_slice(&cache.magic);

    // Version (4 bytes, little-endian)
    bytes.extend_from_slice(&cache.version.to_le_bytes());

    // Timestamp (8 bytes)
    bytes.extend_from_slice(&cache.timestamp.to_le_bytes());

    // Hash (8 bytes)
    bytes.extend_from_slice(&cache.hash.to_le_bytes());

    // AST data length + data
    let ast_len = cache.ast_data.len() as u32;
    bytes.extend_from_slice(&ast_len.to_le_bytes());
    bytes.extend_from_slice(&cache.ast_data);

    // Types count
    let types_count = cache.types.entries.len() as u32;
    bytes.extend_from_slice(&types_count.to_le_bytes());

    // Symbols count
    let symbols_count = cache.symbols.entries.len() as u32;
    bytes.extend_from_slice(&symbols_count.to_le_bytes());

    // UB reports count
    let ub_count = cache.ub_reports.len() as u32;
    bytes.extend_from_slice(&ub_count.to_le_bytes());

    bytes
}

/// Escribe el cache a un archivo fastos.bib
pub fn write_to_file(cache: &ADeadCache, path: &str) -> std::io::Result<usize> {
    let bytes = serialize(cache);
    std::fs::write(path, &bytes)?;
    Ok(bytes.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_header() {
        let cache = ADeadCache::new(0xDEADBEEF);
        let bytes = serialize(&cache);
        // Minimum: 8 (magic) + 4 (version) + 8 (timestamp) + 8 (hash) + 4 (ast_len) + 4+4+4 (counts)
        assert!(bytes.len() >= 44);
        assert_eq!(&bytes[0..8], b"ADEAD.BI");
    }
}
