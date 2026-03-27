//! CUDA AST — stub

#[derive(Debug, Clone)]
pub enum CudaAstNode {
    Kernel { name: String, body: Vec<CudaAstNode> },
    Statement(String),
}
