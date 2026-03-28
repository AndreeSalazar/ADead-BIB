//! CUDA Parser — stub

pub struct CudaParser;
pub struct CudaKernel {
    pub name: String,
    pub params: Vec<String>,
}

impl CudaParser {
    pub fn new() -> Self { Self }
}

impl Default for CudaParser {
    fn default() -> Self { Self::new() }
}
