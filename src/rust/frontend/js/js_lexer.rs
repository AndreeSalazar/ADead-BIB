// ============================================================
// JsDead-BIB — JavaScript Lexer
// ============================================================
// Tokenizer for JavaScript source code
// Implícitamente estricto — semicolons obligatorios (no ASI)
// Sin Node.js. Sin V8. Sin runtime. JS → ASM directo.
// ============================================================

use super::js_ast::DeclKind;

/// JavaScript token
#[derive(Debug, Clone, PartialEq)]
pub enum JsToken {
    // Literales
    NumberInt(i64),
    NumberFloat(f64),
    StringLiteral(String),
    TemplateLiteral(String),
    BoolLiteral(bool),
    NullLiteral,
    UndefinedLiteral,

    // Keywords
    Var,
    Let,
    Const,
    Function,
    Class,
    Extends,
    If,
    Else,
    For,
    While,
    Do,
    Switch,
    Case,
    Default,
    Break,
    Continue,
    Return,
    Throw,
    Try,
    Catch,
    Finally,
    New,
    Delete,
    Typeof,
    Instanceof,
    In,
    Of,
    Void,
    This,
    Super,
    Static,
    Get,
    Set,
    Async,
    Await,
    Yield,
    Import,
    Export,
    From,

    // Operadores
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    StarStar,
    EqEq,
    EqEqEq,
    BangEq,
    BangEqEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    AmpAmp,
    PipePipe,
    Bang,
    QuestionQuestion,
    Amp,
    Pipe,
    Caret,
    Tilde,
    LtLt,
    GtGt,
    GtGtGt,
    Eq,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    PercentEq,
    PlusPlus,
    MinusMinus,
    Question,
    Colon,
    Arrow,
    DotDotDot,

    // Puntuación
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    LParen,
    RParen,
    Semicolon,
    Comma,
    Dot,
    OptionalChain,

    // Identificadores
    Identifier(String),

    // Especiales
    EOF,
    Newline,
}

/// Lexer state
pub struct JsLexer {
    input: Vec<char>,
    pos: usize,
    line: usize,
    lines: Vec<usize>, // token index → line number
}

impl JsLexer {
    pub fn new(source: &str) -> Self {
        Self {
            input: source.chars().collect(),
            pos: 0,
            line: 1,
            lines: Vec::new(),
        }
    }

    /// Tokenize the entire source, returning (tokens, line_numbers)
    pub fn tokenize(&mut self) -> (Vec<JsToken>, Vec<usize>) {
        let mut tokens = Vec::new();

        loop {
            self.skip_whitespace_and_comments();
            if self.pos >= self.input.len() {
                self.lines.push(self.line);
                tokens.push(JsToken::EOF);
                break;
            }

            let tok = self.next_token();
            self.lines.push(self.line);
            if tok == JsToken::Newline {
                continue; // skip newlines since we require semicolons
            }
            tokens.push(tok);
        }

        (tokens, self.lines.clone())
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn peek_at(&self, offset: usize) -> Option<char> {
        self.input.get(self.pos + offset).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.input.get(self.pos).copied();
        if ch == Some('\n') {
            self.line += 1;
        }
        self.pos += 1;
        ch
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            // Skip whitespace (but not newlines for ASI tracking)
            while self.pos < self.input.len() {
                let ch = self.input[self.pos];
                if ch == ' ' || ch == '\t' || ch == '\r' || ch == '\n' {
                    if ch == '\n' {
                        self.line += 1;
                    }
                    self.pos += 1;
                } else {
                    break;
                }
            }

            // Skip single-line comments: //
            if self.pos + 1 < self.input.len()
                && self.input[self.pos] == '/'
                && self.input[self.pos + 1] == '/'
            {
                self.pos += 2;
                while self.pos < self.input.len() && self.input[self.pos] != '\n' {
                    self.pos += 1;
                }
                continue;
            }

            // Skip multi-line comments: /* ... */
            if self.pos + 1 < self.input.len()
                && self.input[self.pos] == '/'
                && self.input[self.pos + 1] == '*'
            {
                self.pos += 2;
                while self.pos + 1 < self.input.len() {
                    if self.input[self.pos] == '\n' {
                        self.line += 1;
                    }
                    if self.input[self.pos] == '*' && self.input[self.pos + 1] == '/' {
                        self.pos += 2;
                        break;
                    }
                    self.pos += 1;
                }
                continue;
            }

            break;
        }
    }

    fn next_token(&mut self) -> JsToken {
        let ch = match self.peek() {
            Some(c) => c,
            None => return JsToken::EOF,
        };

        // Numbers
        if ch.is_ascii_digit() {
            return self.read_number();
        }

        // Strings
        if ch == '"' || ch == '\'' {
            return self.read_string(ch);
        }

        // Template literals
        if ch == '`' {
            return self.read_template_literal();
        }

        // Identifiers and keywords
        if ch.is_ascii_alphabetic() || ch == '_' || ch == '$' {
            return self.read_identifier();
        }

        // Operators and punctuation
        self.advance();
        match ch {
            '\n' => JsToken::Newline,

            '{' => JsToken::LBrace,
            '}' => JsToken::RBrace,
            '[' => JsToken::LBracket,
            ']' => JsToken::RBracket,
            '(' => JsToken::LParen,
            ')' => JsToken::RParen,
            ';' => JsToken::Semicolon,
            ',' => JsToken::Comma,
            '~' => JsToken::Tilde,

            '.' => {
                if self.peek() == Some('.') && self.peek_at(1) == Some('.') {
                    self.advance();
                    self.advance();
                    JsToken::DotDotDot
                } else {
                    JsToken::Dot
                }
            }

            ':' => JsToken::Colon,

            '?' => {
                if self.peek() == Some('?') {
                    self.advance();
                    JsToken::QuestionQuestion
                } else if self.peek() == Some('.') {
                    self.advance();
                    JsToken::OptionalChain
                } else {
                    JsToken::Question
                }
            }

            '+' => {
                if self.peek() == Some('+') {
                    self.advance();
                    JsToken::PlusPlus
                } else if self.peek() == Some('=') {
                    self.advance();
                    JsToken::PlusEq
                } else {
                    JsToken::Plus
                }
            }

            '-' => {
                if self.peek() == Some('-') {
                    self.advance();
                    JsToken::MinusMinus
                } else if self.peek() == Some('=') {
                    self.advance();
                    JsToken::MinusEq
                } else {
                    JsToken::Minus
                }
            }

            '*' => {
                if self.peek() == Some('*') {
                    self.advance();
                    JsToken::StarStar
                } else if self.peek() == Some('=') {
                    self.advance();
                    JsToken::StarEq
                } else {
                    JsToken::Star
                }
            }

            '/' => {
                if self.peek() == Some('=') {
                    self.advance();
                    JsToken::SlashEq
                } else {
                    JsToken::Slash
                }
            }

            '%' => {
                if self.peek() == Some('=') {
                    self.advance();
                    JsToken::PercentEq
                } else {
                    JsToken::Percent
                }
            }

            '=' => {
                if self.peek() == Some('=') {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        JsToken::EqEqEq
                    } else {
                        JsToken::EqEq
                    }
                } else if self.peek() == Some('>') {
                    self.advance();
                    JsToken::Arrow
                } else {
                    JsToken::Eq
                }
            }

            '!' => {
                if self.peek() == Some('=') {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        JsToken::BangEqEq
                    } else {
                        JsToken::BangEq
                    }
                } else {
                    JsToken::Bang
                }
            }

            '<' => {
                if self.peek() == Some('=') {
                    self.advance();
                    JsToken::LtEq
                } else if self.peek() == Some('<') {
                    self.advance();
                    JsToken::LtLt
                } else {
                    JsToken::Lt
                }
            }

            '>' => {
                if self.peek() == Some('=') {
                    self.advance();
                    JsToken::GtEq
                } else if self.peek() == Some('>') {
                    self.advance();
                    if self.peek() == Some('>') {
                        self.advance();
                        JsToken::GtGtGt
                    } else {
                        JsToken::GtGt
                    }
                } else {
                    JsToken::Gt
                }
            }

            '&' => {
                if self.peek() == Some('&') {
                    self.advance();
                    JsToken::AmpAmp
                } else {
                    JsToken::Amp
                }
            }

            '|' => {
                if self.peek() == Some('|') {
                    self.advance();
                    JsToken::PipePipe
                } else {
                    JsToken::Pipe
                }
            }

            '^' => JsToken::Caret,

            _ => {
                // Unknown character, skip
                JsToken::Identifier(format!("__unknown_{}", ch as u32))
            }
        }
    }

    fn read_number(&mut self) -> JsToken {
        let mut num_str = String::new();
        let mut is_float = false;

        // Check for 0x, 0b, 0o prefixes
        if self.peek() == Some('0') {
            let next = self.peek_at(1);
            if next == Some('x') || next == Some('X') {
                num_str.push(self.advance().unwrap());
                num_str.push(self.advance().unwrap());
                while let Some(c) = self.peek() {
                    if c.is_ascii_hexdigit() || c == '_' {
                        if c != '_' {
                            num_str.push(c);
                        }
                        self.advance();
                    } else {
                        break;
                    }
                }
                return JsToken::NumberInt(
                    i64::from_str_radix(&num_str[2..], 16).unwrap_or(0),
                );
            }
            if next == Some('b') || next == Some('B') {
                num_str.push(self.advance().unwrap());
                num_str.push(self.advance().unwrap());
                while let Some(c) = self.peek() {
                    if c == '0' || c == '1' || c == '_' {
                        if c != '_' {
                            num_str.push(c);
                        }
                        self.advance();
                    } else {
                        break;
                    }
                }
                return JsToken::NumberInt(
                    i64::from_str_radix(&num_str[2..], 2).unwrap_or(0),
                );
            }
            if next == Some('o') || next == Some('O') {
                num_str.push(self.advance().unwrap());
                num_str.push(self.advance().unwrap());
                while let Some(c) = self.peek() {
                    if ('0'..='7').contains(&c) || c == '_' {
                        if c != '_' {
                            num_str.push(c);
                        }
                        self.advance();
                    } else {
                        break;
                    }
                }
                return JsToken::NumberInt(
                    i64::from_str_radix(&num_str[2..], 8).unwrap_or(0),
                );
            }
        }

        // Decimal number
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() || c == '_' {
                if c != '_' {
                    num_str.push(c);
                }
                self.advance();
            } else {
                break;
            }
        }

        // Decimal point
        if self.peek() == Some('.') && self.peek_at(1).map_or(false, |c| c.is_ascii_digit()) {
            is_float = true;
            num_str.push('.');
            self.advance();
            while let Some(c) = self.peek() {
                if c.is_ascii_digit() || c == '_' {
                    if c != '_' {
                        num_str.push(c);
                    }
                    self.advance();
                } else {
                    break;
                }
            }
        }

        // Exponent
        if self.peek() == Some('e') || self.peek() == Some('E') {
            is_float = true;
            num_str.push('e');
            self.advance();
            if self.peek() == Some('+') || self.peek() == Some('-') {
                num_str.push(self.advance().unwrap());
            }
            while let Some(c) = self.peek() {
                if c.is_ascii_digit() {
                    num_str.push(c);
                    self.advance();
                } else {
                    break;
                }
            }
        }

        if is_float {
            JsToken::NumberFloat(num_str.parse::<f64>().unwrap_or(0.0))
        } else {
            JsToken::NumberInt(num_str.parse::<i64>().unwrap_or(0))
        }
    }

    fn read_string(&mut self, quote: char) -> JsToken {
        self.advance(); // skip opening quote
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if c == quote {
                self.advance();
                break;
            }
            if c == '\\' {
                self.advance();
                match self.peek() {
                    Some('n') => {
                        s.push('\n');
                        self.advance();
                    }
                    Some('t') => {
                        s.push('\t');
                        self.advance();
                    }
                    Some('r') => {
                        s.push('\r');
                        self.advance();
                    }
                    Some('\\') => {
                        s.push('\\');
                        self.advance();
                    }
                    Some('\'') => {
                        s.push('\'');
                        self.advance();
                    }
                    Some('"') => {
                        s.push('"');
                        self.advance();
                    }
                    Some('0') => {
                        s.push('\0');
                        self.advance();
                    }
                    Some(c) => {
                        s.push(c);
                        self.advance();
                    }
                    None => break,
                }
            } else {
                s.push(c);
                self.advance();
            }
        }
        JsToken::StringLiteral(s)
    }

    fn read_template_literal(&mut self) -> JsToken {
        self.advance(); // skip `
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if c == '`' {
                self.advance();
                break;
            }
            if c == '$' && self.peek_at(1) == Some('{') {
                // Template interpolation — for MVP, just include as literal text
                s.push('$');
                s.push('{');
                self.advance();
                self.advance();
                let mut depth = 1;
                while let Some(ic) = self.peek() {
                    if ic == '{' {
                        depth += 1;
                    }
                    if ic == '}' {
                        depth -= 1;
                        if depth == 0 {
                            self.advance();
                            s.push('}');
                            break;
                        }
                    }
                    s.push(ic);
                    self.advance();
                }
            } else if c == '\\' {
                self.advance();
                match self.peek() {
                    Some('n') => {
                        s.push('\n');
                        self.advance();
                    }
                    Some('t') => {
                        s.push('\t');
                        self.advance();
                    }
                    Some(c) => {
                        s.push(c);
                        self.advance();
                    }
                    None => break,
                }
            } else {
                s.push(c);
                self.advance();
            }
        }
        JsToken::TemplateLiteral(s)
    }

    fn read_identifier(&mut self) -> JsToken {
        let mut ident = String::new();
        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || c == '_' || c == '$' {
                ident.push(c);
                self.advance();
            } else {
                break;
            }
        }

        // Keywords
        match ident.as_str() {
            "var" => JsToken::Var,
            "let" => JsToken::Let,
            "const" => JsToken::Const,
            "function" => JsToken::Function,
            "class" => JsToken::Class,
            "extends" => JsToken::Extends,
            "if" => JsToken::If,
            "else" => JsToken::Else,
            "for" => JsToken::For,
            "while" => JsToken::While,
            "do" => JsToken::Do,
            "switch" => JsToken::Switch,
            "case" => JsToken::Case,
            "default" => JsToken::Default,
            "break" => JsToken::Break,
            "continue" => JsToken::Continue,
            "return" => JsToken::Return,
            "throw" => JsToken::Throw,
            "try" => JsToken::Try,
            "catch" => JsToken::Catch,
            "finally" => JsToken::Finally,
            "new" => JsToken::New,
            "delete" => JsToken::Delete,
            "typeof" => JsToken::Typeof,
            "instanceof" => JsToken::Instanceof,
            "in" => JsToken::In,
            "of" => JsToken::Of,
            "void" => JsToken::Void,
            "this" => JsToken::This,
            "super" => JsToken::Super,
            "static" => JsToken::Static,
            "get" => JsToken::Get,
            "set" => JsToken::Set,
            "async" => JsToken::Async,
            "await" => JsToken::Await,
            "yield" => JsToken::Yield,
            "import" => JsToken::Import,
            "export" => JsToken::Export,
            "from" => JsToken::From,
            "true" => JsToken::BoolLiteral(true),
            "false" => JsToken::BoolLiteral(false),
            "null" => JsToken::NullLiteral,
            "undefined" => JsToken::UndefinedLiteral,
            _ => JsToken::Identifier(ident),
        }
    }
}

impl DeclKind {
    pub fn from_token(tok: &JsToken) -> Option<Self> {
        match tok {
            JsToken::Var => Some(DeclKind::Var),
            JsToken::Let => Some(DeclKind::Let),
            JsToken::Const => Some(DeclKind::Const),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let mut lexer = JsLexer::new("let x = 5;");
        let (tokens, _) = lexer.tokenize();
        assert_eq!(tokens[0], JsToken::Let);
        assert_eq!(tokens[1], JsToken::Identifier("x".into()));
        assert_eq!(tokens[2], JsToken::Eq);
        assert_eq!(tokens[3], JsToken::NumberInt(5));
        assert_eq!(tokens[4], JsToken::Semicolon);
    }

    #[test]
    fn test_string_literal() {
        let mut lexer = JsLexer::new(r#""hello world""#);
        let (tokens, _) = lexer.tokenize();
        assert_eq!(tokens[0], JsToken::StringLiteral("hello world".into()));
    }

    #[test]
    fn test_operators() {
        let mut lexer = JsLexer::new("=== !== => ?? ?. ...");
        let (tokens, _) = lexer.tokenize();
        assert_eq!(tokens[0], JsToken::EqEqEq);
        assert_eq!(tokens[1], JsToken::BangEqEq);
        assert_eq!(tokens[2], JsToken::Arrow);
        assert_eq!(tokens[3], JsToken::QuestionQuestion);
        assert_eq!(tokens[4], JsToken::OptionalChain);
        assert_eq!(tokens[5], JsToken::DotDotDot);
    }

    #[test]
    fn test_float_number() {
        let mut lexer = JsLexer::new("3.14");
        let (tokens, _) = lexer.tokenize();
        assert_eq!(tokens[0], JsToken::NumberFloat(3.14));
    }

    #[test]
    fn test_hex_binary() {
        let mut lexer = JsLexer::new("0xFF 0b1010");
        let (tokens, _) = lexer.tokenize();
        assert_eq!(tokens[0], JsToken::NumberInt(255));
        assert_eq!(tokens[1], JsToken::NumberInt(10));
    }

    #[test]
    fn test_keywords() {
        let mut lexer = JsLexer::new("function class if else for while return");
        let (tokens, _) = lexer.tokenize();
        assert_eq!(tokens[0], JsToken::Function);
        assert_eq!(tokens[1], JsToken::Class);
        assert_eq!(tokens[2], JsToken::If);
        assert_eq!(tokens[3], JsToken::Else);
        assert_eq!(tokens[4], JsToken::For);
        assert_eq!(tokens[5], JsToken::While);
        assert_eq!(tokens[6], JsToken::Return);
    }

    #[test]
    fn test_comments() {
        let mut lexer = JsLexer::new("let x = 5; // comment\nlet y = 10;");
        let (tokens, _) = lexer.tokenize();
        assert_eq!(tokens[0], JsToken::Let);
        assert_eq!(tokens[1], JsToken::Identifier("x".into()));
        // After comment, should get let y = 10
        assert!(tokens.iter().any(|t| *t == JsToken::Identifier("y".into())));
    }
}
