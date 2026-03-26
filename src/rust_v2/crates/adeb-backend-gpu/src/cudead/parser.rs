// ============================================================
// CUDead-BIB — CUDA Parser
// ============================================================
// Parsea código .cu con sintaxis CUDead-BIB
// Detecta __cudead_kernel__, __cudead_device__, etc.
// ============================================================

use super::primitives::{KernelDef, KernelParam, KernelType, ParamType};

/// Token types for CUDA parsing
#[derive(Debug, Clone, PartialEq)]
pub enum CuToken {
    // Keywords
    CudeadKernel,    // __cudead_kernel__
    CudeadDevice,    // __cudead_device__
    Global,          // __global__ (CUDA compat)
    Device,          // __device__ (CUDA compat)
    Shared,          // __shared__
    Constant,        // __constant__
    
    // Types
    Float,
    Double,
    Int,
    Long,
    Uint,
    Void,
    
    // Identifiers and literals
    Ident(String),
    IntLit(i64),
    FloatLit(f64),
    StringLit(String),
    
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Ampersand,
    Pipe,
    Caret,
    Tilde,
    Bang,
    Eq,
    EqEq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    AndAnd,
    OrOr,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    PlusPlus,
    MinusMinus,
    
    // Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Semicolon,
    Colon,
    Dot,
    Arrow,
    
    // Control flow
    If,
    Else,
    For,
    While,
    Do,
    Return,
    Break,
    Continue,
    
    // CUDA specific
    ThreadIdx,
    BlockIdx,
    BlockDim,
    GridDim,
    SyncThreads,
    
    // Special
    TripleLt,  // <<<
    TripleGt,  // >>>
    
    // End
    Eof,
}

/// Lexer for CUDA code
pub struct CuLexer {
    input: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
}

impl CuLexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn peek_n(&self, n: usize) -> Option<char> {
        self.input.get(self.pos + n).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.pos += 1;
        if c == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        Some(c)
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else if c == '/' && self.peek_n(1) == Some('/') {
                // Line comment
                while let Some(c) = self.peek() {
                    if c == '\n' {
                        break;
                    }
                    self.advance();
                }
            } else if c == '/' && self.peek_n(1) == Some('*') {
                // Block comment
                self.advance();
                self.advance();
                while let Some(c) = self.peek() {
                    if c == '*' && self.peek_n(1) == Some('/') {
                        self.advance();
                        self.advance();
                        break;
                    }
                    self.advance();
                }
            } else {
                break;
            }
        }
    }

    fn read_ident(&mut self) -> String {
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                s.push(c);
                self.advance();
            } else {
                break;
            }
        }
        s
    }

    fn read_number(&mut self) -> CuToken {
        let mut s = String::new();
        let mut is_float = false;

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                s.push(c);
                self.advance();
            } else if c == '.' && !is_float {
                is_float = true;
                s.push(c);
                self.advance();
            } else if c == 'f' || c == 'F' {
                self.advance();
                is_float = true;
                break;
            } else {
                break;
            }
        }

        if is_float {
            CuToken::FloatLit(s.parse().unwrap_or(0.0))
        } else {
            CuToken::IntLit(s.parse().unwrap_or(0))
        }
    }

    pub fn next_token(&mut self) -> CuToken {
        self.skip_whitespace();

        let c = match self.peek() {
            Some(c) => c,
            None => return CuToken::Eof,
        };

        // Identifiers and keywords
        if c.is_alphabetic() || c == '_' {
            let ident = self.read_ident();
            return match ident.as_str() {
                "__cudead_kernel__" => CuToken::CudeadKernel,
                "__cudead_device__" => CuToken::CudeadDevice,
                "__global__" => CuToken::Global,
                "__device__" => CuToken::Device,
                "__shared__" => CuToken::Shared,
                "__constant__" => CuToken::Constant,
                "float" => CuToken::Float,
                "double" => CuToken::Double,
                "int" => CuToken::Int,
                "long" => CuToken::Long,
                "unsigned" => CuToken::Uint,
                "void" => CuToken::Void,
                "if" => CuToken::If,
                "else" => CuToken::Else,
                "for" => CuToken::For,
                "while" => CuToken::While,
                "do" => CuToken::Do,
                "return" => CuToken::Return,
                "break" => CuToken::Break,
                "continue" => CuToken::Continue,
                "threadIdx" => CuToken::ThreadIdx,
                "blockIdx" => CuToken::BlockIdx,
                "blockDim" => CuToken::BlockDim,
                "gridDim" => CuToken::GridDim,
                "__syncthreads" => CuToken::SyncThreads,
                _ => CuToken::Ident(ident),
            };
        }

        // Numbers
        if c.is_ascii_digit() {
            return self.read_number();
        }

        // Operators and delimiters
        self.advance();
        match c {
            '+' => {
                if self.peek() == Some('+') {
                    self.advance();
                    CuToken::PlusPlus
                } else if self.peek() == Some('=') {
                    self.advance();
                    CuToken::PlusEq
                } else {
                    CuToken::Plus
                }
            }
            '-' => {
                if self.peek() == Some('-') {
                    self.advance();
                    CuToken::MinusMinus
                } else if self.peek() == Some('=') {
                    self.advance();
                    CuToken::MinusEq
                } else if self.peek() == Some('>') {
                    self.advance();
                    CuToken::Arrow
                } else {
                    CuToken::Minus
                }
            }
            '*' => {
                if self.peek() == Some('=') {
                    self.advance();
                    CuToken::StarEq
                } else {
                    CuToken::Star
                }
            }
            '/' => {
                if self.peek() == Some('=') {
                    self.advance();
                    CuToken::SlashEq
                } else {
                    CuToken::Slash
                }
            }
            '%' => CuToken::Percent,
            '&' => {
                if self.peek() == Some('&') {
                    self.advance();
                    CuToken::AndAnd
                } else {
                    CuToken::Ampersand
                }
            }
            '|' => {
                if self.peek() == Some('|') {
                    self.advance();
                    CuToken::OrOr
                } else {
                    CuToken::Pipe
                }
            }
            '^' => CuToken::Caret,
            '~' => CuToken::Tilde,
            '!' => {
                if self.peek() == Some('=') {
                    self.advance();
                    CuToken::Ne
                } else {
                    CuToken::Bang
                }
            }
            '=' => {
                if self.peek() == Some('=') {
                    self.advance();
                    CuToken::EqEq
                } else {
                    CuToken::Eq
                }
            }
            '<' => {
                if self.peek() == Some('<') && self.peek_n(1) == Some('<') {
                    self.advance();
                    self.advance();
                    CuToken::TripleLt
                } else if self.peek() == Some('=') {
                    self.advance();
                    CuToken::Le
                } else {
                    CuToken::Lt
                }
            }
            '>' => {
                if self.peek() == Some('>') && self.peek_n(1) == Some('>') {
                    self.advance();
                    self.advance();
                    CuToken::TripleGt
                } else if self.peek() == Some('=') {
                    self.advance();
                    CuToken::Ge
                } else {
                    CuToken::Gt
                }
            }
            '(' => CuToken::LParen,
            ')' => CuToken::RParen,
            '{' => CuToken::LBrace,
            '}' => CuToken::RBrace,
            '[' => CuToken::LBracket,
            ']' => CuToken::RBracket,
            ',' => CuToken::Comma,
            ';' => CuToken::Semicolon,
            ':' => CuToken::Colon,
            '.' => CuToken::Dot,
            _ => CuToken::Eof,
        }
    }

    pub fn tokenize(&mut self) -> Vec<CuToken> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            if tok == CuToken::Eof {
                tokens.push(tok);
                break;
            }
            tokens.push(tok);
        }
        tokens
    }
}

/// Kernel AST node
#[derive(Debug, Clone)]
pub struct KernelAst {
    pub name: String,
    pub kernel_type: KernelType,
    pub params: Vec<KernelParam>,
    pub body: String,
    pub shared_memory: usize,
    pub line: usize,
}

/// Complete AST
#[derive(Debug, Clone)]
pub struct CudeadAst {
    pub kernels: Vec<KernelAst>,
    pub includes: Vec<String>,
    pub globals: Vec<String>,
}

impl CudeadAst {
    pub fn new() -> Self {
        Self {
            kernels: Vec::new(),
            includes: Vec::new(),
            globals: Vec::new(),
        }
    }
}

impl Default for CudeadAst {
    fn default() -> Self {
        Self::new()
    }
}

/// CUDA Parser
pub struct CudeadParser {
    tokens: Vec<CuToken>,
    pos: usize,
}

impl CudeadParser {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            pos: 0,
        }
    }

    fn peek(&self) -> &CuToken {
        self.tokens.get(self.pos).unwrap_or(&CuToken::Eof)
    }

    fn advance(&mut self) -> &CuToken {
        let tok = self.peek().clone();
        self.pos += 1;
        self.tokens.get(self.pos - 1).unwrap_or(&CuToken::Eof)
    }

    fn expect(&mut self, expected: CuToken) -> Result<(), super::CudeadError> {
        if self.peek() == &expected {
            self.advance();
            Ok(())
        } else {
            Err(super::CudeadError::ParseError(format!(
                "Expected {:?}, got {:?}",
                expected,
                self.peek()
            )))
        }
    }

    pub fn parse(&self, source: &str) -> Result<CudeadAst, super::CudeadError> {
        let mut lexer = CuLexer::new(source);
        let tokens = lexer.tokenize();

        let mut parser = CudeadParser {
            tokens,
            pos: 0,
        };

        parser.parse_program()
    }

    fn parse_program(&mut self) -> Result<CudeadAst, super::CudeadError> {
        let mut ast = CudeadAst::new();

        while self.peek() != &CuToken::Eof {
            match self.peek() {
                CuToken::CudeadKernel | CuToken::Global => {
                    let kernel = self.parse_kernel(KernelType::Kernel)?;
                    ast.kernels.push(kernel);
                }
                CuToken::CudeadDevice | CuToken::Device => {
                    let kernel = self.parse_kernel(KernelType::Device)?;
                    ast.kernels.push(kernel);
                }
                _ => {
                    // Skip unknown tokens
                    self.advance();
                }
            }
        }

        Ok(ast)
    }

    fn parse_kernel(&mut self, kernel_type: KernelType) -> Result<KernelAst, super::CudeadError> {
        // Skip kernel qualifier
        self.advance();

        // Parse return type (usually void)
        let _ret_type = self.parse_type()?;

        // Parse kernel name
        let name = match self.peek() {
            CuToken::Ident(s) => {
                let n = s.clone();
                self.advance();
                n
            }
            _ => {
                return Err(super::CudeadError::ParseError(
                    "Expected kernel name".to_string(),
                ))
            }
        };

        // Parse parameters
        self.expect(CuToken::LParen)?;
        let params = self.parse_params()?;
        self.expect(CuToken::RParen)?;

        // Parse body
        self.expect(CuToken::LBrace)?;
        let body = self.parse_body()?;

        Ok(KernelAst {
            name,
            kernel_type,
            params,
            body,
            shared_memory: 0,
            line: 0,
        })
    }

    fn parse_type(&mut self) -> Result<ParamType, super::CudeadError> {
        match self.peek() {
            CuToken::Float => {
                self.advance();
                Ok(ParamType::Float)
            }
            CuToken::Double => {
                self.advance();
                Ok(ParamType::Double)
            }
            CuToken::Int => {
                self.advance();
                Ok(ParamType::Int)
            }
            CuToken::Long => {
                self.advance();
                Ok(ParamType::Long)
            }
            CuToken::Uint => {
                self.advance();
                // Check for "unsigned int"
                if self.peek() == &CuToken::Int {
                    self.advance();
                }
                Ok(ParamType::Uint)
            }
            CuToken::Void => {
                self.advance();
                Ok(ParamType::Void)
            }
            _ => Ok(ParamType::Int), // Default
        }
    }

    fn parse_params(&mut self) -> Result<Vec<KernelParam>, super::CudeadError> {
        let mut params = Vec::new();

        while self.peek() != &CuToken::RParen {
            let param_type = self.parse_type()?;

            // Check for pointer
            let is_pointer = if self.peek() == &CuToken::Star {
                self.advance();
                true
            } else {
                false
            };

            // Parse name
            let name = match self.peek() {
                CuToken::Ident(s) => {
                    let n = s.clone();
                    self.advance();
                    n
                }
                _ => {
                    return Err(super::CudeadError::ParseError(
                        "Expected parameter name".to_string(),
                    ))
                }
            };

            params.push(KernelParam {
                name,
                param_type,
                is_pointer,
            });

            // Check for comma
            if self.peek() == &CuToken::Comma {
                self.advance();
            }
        }

        Ok(params)
    }

    fn parse_body(&mut self) -> Result<String, super::CudeadError> {
        let mut body = String::new();
        let mut brace_count = 1;

        while brace_count > 0 {
            match self.peek() {
                CuToken::LBrace => {
                    brace_count += 1;
                    body.push('{');
                    self.advance();
                }
                CuToken::RBrace => {
                    brace_count -= 1;
                    if brace_count > 0 {
                        body.push('}');
                    }
                    self.advance();
                }
                CuToken::Eof => break,
                _ => {
                    // Convert token back to string (simplified)
                    body.push_str(&format!("{:?} ", self.peek()));
                    self.advance();
                }
            }
        }

        Ok(body)
    }
}

impl Default for CudeadParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_keywords() {
        let mut lexer = CuLexer::new("__cudead_kernel__ void test()");
        assert_eq!(lexer.next_token(), CuToken::CudeadKernel);
        assert_eq!(lexer.next_token(), CuToken::Void);
    }

    #[test]
    fn test_lexer_operators() {
        let mut lexer = CuLexer::new("+ - * / <<<>>>");
        assert_eq!(lexer.next_token(), CuToken::Plus);
        assert_eq!(lexer.next_token(), CuToken::Minus);
        assert_eq!(lexer.next_token(), CuToken::Star);
        assert_eq!(lexer.next_token(), CuToken::Slash);
        assert_eq!(lexer.next_token(), CuToken::TripleLt);
        assert_eq!(lexer.next_token(), CuToken::TripleGt);
    }

    #[test]
    fn test_parse_kernel() {
        let source = r#"
            __cudead_kernel__ void vectorAdd(float *A, float *B, float *C, int n) {
                int i = blockIdx.x * blockDim.x + threadIdx.x;
                if (i < n) {
                    C[i] = A[i] + B[i];
                }
            }
        "#;

        let parser = CudeadParser::new();
        let ast = parser.parse(source).unwrap();
        assert_eq!(ast.kernels.len(), 1);
        assert_eq!(ast.kernels[0].name, "vectorAdd");
        assert_eq!(ast.kernels[0].params.len(), 4);
    }
}
