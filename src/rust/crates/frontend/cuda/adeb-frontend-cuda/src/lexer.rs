//! CUDA Lexer — stub

#[derive(Debug, Clone)]
pub enum CudaToken {
    Ident(String),
    Number(i64),
    Eof,
}

pub struct CudaLexer;

impl CudaLexer {
    pub fn new() -> Self { Self }
}

impl Default for CudaLexer {
    fn default() -> Self { Self::new() }
}
