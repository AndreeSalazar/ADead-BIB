// ============================================================
// Flat Binary Generator — Raw binary output (boot sectors)
// ============================================================

pub struct FlatBinaryGenerator {
    org: u64,
    fixed_size: Option<usize>,
}

impl FlatBinaryGenerator {
    pub fn new(org: u64) -> Self {
        Self {
            org,
            fixed_size: None,
        }
    }

    pub fn set_fixed_size(&mut self, size: usize) {
        self.fixed_size = Some(size);
    }

    pub fn generate(&self, code: &[u8], data: &[u8]) -> Vec<u8> {
        let _ = self.org;
        let mut out = Vec::with_capacity(code.len() + data.len());
        out.extend_from_slice(code);
        out.extend_from_slice(data);
        if let Some(sz) = self.fixed_size {
            if out.len() < sz {
                out.resize(sz, 0);
            } else if out.len() > sz {
                out.truncate(sz);
            }
        }
        out
    }
}
