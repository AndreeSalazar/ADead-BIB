// ============================================================
// Cache Deserializer — bytes → AST desde fastos.bib
// ============================================================

use super::{ADeadCache, CACHE_MAGIC, CACHE_VERSION, TypeTable, SymbolTable, ImplTable};

/// Error de deserializacion
#[derive(Debug)]
pub enum DeserializeError {
    InvalidMagic,
    VersionMismatch { expected: u32, got: u32 },
    TruncatedData,
    IoError(std::io::Error),
}

impl std::fmt::Display for DeserializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DeserializeError::InvalidMagic => write!(f, "Invalid cache magic bytes (not fastos.bib)"),
            DeserializeError::VersionMismatch { expected, got } =>
                write!(f, "Cache version mismatch: expected {}, got {}", expected, got),
            DeserializeError::TruncatedData => write!(f, "Cache file truncated"),
            DeserializeError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for DeserializeError {}

impl From<std::io::Error> for DeserializeError {
    fn from(e: std::io::Error) -> Self {
        DeserializeError::IoError(e)
    }
}

/// Deserializa bytes a ADeadCache
pub fn deserialize(bytes: &[u8]) -> Result<ADeadCache, DeserializeError> {
    if bytes.len() < 44 {
        return Err(DeserializeError::TruncatedData);
    }

    // Magic
    let mut magic = [0u8; 8];
    magic.copy_from_slice(&bytes[0..8]);
    if magic != CACHE_MAGIC {
        return Err(DeserializeError::InvalidMagic);
    }

    // Version
    let version = u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
    if version != CACHE_VERSION {
        return Err(DeserializeError::VersionMismatch {
            expected: CACHE_VERSION,
            got: version,
        });
    }

    // Timestamp
    let timestamp = u64::from_le_bytes([
        bytes[12], bytes[13], bytes[14], bytes[15],
        bytes[16], bytes[17], bytes[18], bytes[19],
    ]);

    // Hash
    let hash = u64::from_le_bytes([
        bytes[20], bytes[21], bytes[22], bytes[23],
        bytes[24], bytes[25], bytes[26], bytes[27],
    ]);

    // AST data length
    let ast_len = u32::from_le_bytes([bytes[28], bytes[29], bytes[30], bytes[31]]) as usize;

    let ast_data = if ast_len > 0 && bytes.len() >= 32 + ast_len {
        bytes[32..32 + ast_len].to_vec()
    } else {
        Vec::new()
    };

    Ok(ADeadCache {
        magic,
        version,
        timestamp,
        hash,
        ast_data,
        types: TypeTable::new(),
        symbols: SymbolTable::new(),
        ub_reports: Vec::new(),
        impls: ImplTable::new(),
    })
}

/// Lee y deserializa un archivo fastos.bib
pub fn read_from_file(path: &str) -> Result<ADeadCache, DeserializeError> {
    let bytes = std::fs::read(path)?;
    deserialize(&bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cache::serializer;

    #[test]
    fn test_roundtrip() {
        let cache = ADeadCache::new(0xCAFEBABE);
        let bytes = serializer::serialize(&cache);
        let result = deserialize(&bytes);
        assert!(result.is_ok());
        let loaded = result.unwrap();
        assert_eq!(loaded.hash, 0xCAFEBABE);
        assert!(loaded.is_valid());
    }

    #[test]
    fn test_invalid_magic() {
        let bytes = vec![0u8; 44];
        let result = deserialize(&bytes);
        assert!(matches!(result, Err(DeserializeError::InvalidMagic)));
    }
}
