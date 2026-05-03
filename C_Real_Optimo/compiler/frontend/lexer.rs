//! Lexer C99 - Generado automáticamente
#![allow(dead_code)]

use super::token::Token;

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    line: usize,
    col: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, pos: 0, line: 1, col: 1 }
    }
    
    fn peek(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }
    
    fn peek_n(&self, n: usize) -> Option<char> {
        self.input[self.pos..].chars().nth(n)
    }
    
    fn advance(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.pos += c.len_utf8();
        if c == '\n' { self.line += 1; self.col = 1; } else { self.col += 1; }
        Some(c)
    }
    
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() && c != '\n' { self.advance(); } else { break; }
        }
    }
    
    fn skip_line_comment(&mut self) {
        while let Some(c) = self.peek() {
            if c == '\n' { break; }
            self.advance();
        }
    }
    
    fn skip_block_comment(&mut self) {
        self.advance(); self.advance(); // /*
        while let Some(c) = self.peek() {
            if c == '*' && self.peek_n(1) == Some('/') {
                self.advance(); self.advance();
                return;
            }
            self.advance();
        }
    }
    
    fn read_ident(&mut self) -> String {
        let start = self.pos;
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' { self.advance(); } else { break; }
        }
        self.input[start..self.pos].to_string()
    }
    
    fn read_number(&mut self) -> Token {
        let start = self.pos;
        let mut is_float = false;
        
        // Hex
        if self.peek() == Some('0') && matches!(self.peek_n(1), Some('x') | Some('X')) {
            self.advance(); self.advance();
            while let Some(c) = self.peek() {
                if c.is_ascii_hexdigit() { self.advance(); } else { break; }
            }
            let s = &self.input[start..self.pos];
            return Token::IntLit(i64::from_str_radix(&s[2..], 16).unwrap_or(0));
        }
        
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() { self.advance(); }
            else if c == '.' && !is_float { is_float = true; self.advance(); }
            else if (c == 'e' || c == 'E') && !is_float {
                is_float = true; self.advance();
                if matches!(self.peek(), Some('+') | Some('-')) { self.advance(); }
            }
            else { break; }
        }
        
        // Skip suffixes
        while matches!(self.peek(), Some('u') | Some('U') | Some('l') | Some('L') | Some('f') | Some('F')) {
            self.advance();
        }
        
        let s = &self.input[start..self.pos];
        if is_float {
            Token::FloatLit(s.trim_end_matches(|c: char| !c.is_ascii_digit() && c != '.').parse().unwrap_or(0.0))
        } else {
            Token::IntLit(s.trim_end_matches(|c: char| !c.is_ascii_digit()).parse().unwrap_or(0))
        }
    }
    
    fn read_string(&mut self) -> Token {
        self.advance(); // "
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if c == '"' { self.advance(); break; }
            if c == '\\' {
                self.advance();
                if let Some(esc) = self.peek() {
                    self.advance();
                    s.push(match esc {
                        'n' => '\n', 'r' => '\r', 't' => '\t', '\\' => '\\', '"' => '"', '0' => '\0',
                        _ => esc,
                    });
                }
            } else {
                s.push(c);
                self.advance();
            }
        }
        Token::StringLit(s)
    }
    
    fn read_char(&mut self) -> Token {
        self.advance(); // '
        let c = if self.peek() == Some('\\') {
            self.advance();
            match self.peek() {
                Some('n') => { self.advance(); '\n' }
                Some('r') => { self.advance(); '\r' }
                Some('t') => { self.advance(); '\t' }
                Some('0') => { self.advance(); '\0' }
                Some(c) => { self.advance(); c }
                None => '\0',
            }
        } else {
            let c = self.peek().unwrap_or('\0');
            self.advance();
            c
        };
        if self.peek() == Some('\'') { self.advance(); }
        Token::CharLit(c)
    }
    
    fn match_keyword(s: &str) -> Option<Token> {
        match s {
            "auto" => Some(Token::KwAuto),
            "break" => Some(Token::KwBreak),
            "case" => Some(Token::KwCase),
            "char" => Some(Token::KwChar),
            "const" => Some(Token::KwConst),
            "continue" => Some(Token::KwContinue),
            "default" => Some(Token::KwDefault),
            "do" => Some(Token::KwDo),
            "double" => Some(Token::KwDouble),
            "else" => Some(Token::KwElse),
            "enum" => Some(Token::KwEnum),
            "extern" => Some(Token::KwExtern),
            "float" => Some(Token::KwFloat),
            "for" => Some(Token::KwFor),
            "goto" => Some(Token::KwGoto),
            "if" => Some(Token::KwIf),
            "inline" => Some(Token::KwInline),
            "int" => Some(Token::KwInt),
            "long" => Some(Token::KwLong),
            "register" => Some(Token::KwRegister),
            "restrict" => Some(Token::KwRestrict),
            "return" => Some(Token::KwReturn),
            "short" => Some(Token::KwShort),
            "signed" => Some(Token::KwSigned),
            "sizeof" => Some(Token::KwSizeof),
            "static" => Some(Token::KwStatic),
            "struct" => Some(Token::KwStruct),
            "switch" => Some(Token::KwSwitch),
            "typedef" => Some(Token::KwTypedef),
            "union" => Some(Token::KwUnion),
            "unsigned" => Some(Token::KwUnsigned),
            "void" => Some(Token::KwVoid),
            "volatile" => Some(Token::KwVolatile),
            "while" => Some(Token::KwWhile),
            "_Bool" => Some(Token::KwBool),
            "_Complex" => Some(Token::KwComplex),
            "_Imaginary" => Some(Token::KwImaginary),
            _ => None,
        }
    }
    
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        
        let c = match self.peek() {
            Some(c) => c,
            None => return Token::Eof,
        };
        
        // Comments
        if c == '/' {
            if self.peek_n(1) == Some('/') { self.skip_line_comment(); return Token::Comment; }
            if self.peek_n(1) == Some('*') { self.skip_block_comment(); return Token::Comment; }
        }
        
        // Newline
        if c == '\n' { self.advance(); return Token::Newline; }
        
        // Identifiers & keywords
        if c.is_alphabetic() || c == '_' {
            let ident = self.read_ident();
            return Self::match_keyword(&ident).unwrap_or(Token::Ident(ident));
        }
        
        // Numbers
        if c.is_ascii_digit() { return self.read_number(); }
        
        // Strings
        if c == '"' { return self.read_string(); }
        
        // Chars
        if c == '\'' { return self.read_char(); }
        
        // Operators (longest match first)
        if self.input[self.pos..].starts_with("<<=") { self.pos += 3; return Token::LtLtEq; }
        if self.input[self.pos..].starts_with(">>=") { self.pos += 3; return Token::GtGtEq; }
        if self.input[self.pos..].starts_with("...") { self.pos += 3; return Token::Ellipsis; }
        if self.input[self.pos..].starts_with("++") { self.pos += 2; return Token::PlusPlus; }
        if self.input[self.pos..].starts_with("--") { self.pos += 2; return Token::MinusMinus; }
        if self.input[self.pos..].starts_with("->") { self.pos += 2; return Token::Arrow; }
        if self.input[self.pos..].starts_with("<<") { self.pos += 2; return Token::LtLt; }
        if self.input[self.pos..].starts_with(">>") { self.pos += 2; return Token::GtGt; }
        if self.input[self.pos..].starts_with("<=") { self.pos += 2; return Token::LtEq; }
        if self.input[self.pos..].starts_with(">=") { self.pos += 2; return Token::GtEq; }
        if self.input[self.pos..].starts_with("==") { self.pos += 2; return Token::EqEq; }
        if self.input[self.pos..].starts_with("!=") { self.pos += 2; return Token::BangEq; }
        if self.input[self.pos..].starts_with("&&") { self.pos += 2; return Token::AmpAmp; }
        if self.input[self.pos..].starts_with("||") { self.pos += 2; return Token::PipePipe; }
        if self.input[self.pos..].starts_with("+=") { self.pos += 2; return Token::PlusEq; }
        if self.input[self.pos..].starts_with("-=") { self.pos += 2; return Token::MinusEq; }
        if self.input[self.pos..].starts_with("*=") { self.pos += 2; return Token::StarEq; }
        if self.input[self.pos..].starts_with("/=") { self.pos += 2; return Token::SlashEq; }
        if self.input[self.pos..].starts_with("%=") { self.pos += 2; return Token::PercentEq; }
        if self.input[self.pos..].starts_with("&=") { self.pos += 2; return Token::AmpEq; }
        if self.input[self.pos..].starts_with("|=") { self.pos += 2; return Token::PipeEq; }
        if self.input[self.pos..].starts_with("^=") { self.pos += 2; return Token::CaretEq; }
        if c == '+' { self.advance(); return Token::Plus; }
        if c == '-' { self.advance(); return Token::Minus; }
        if c == '*' { self.advance(); return Token::Star; }
        if c == '/' { self.advance(); return Token::Slash; }
        if c == '%' { self.advance(); return Token::Percent; }
        if c == '&' { self.advance(); return Token::Ampersand; }
        if c == '|' { self.advance(); return Token::Pipe; }
        if c == '^' { self.advance(); return Token::Caret; }
        if c == '~' { self.advance(); return Token::Tilde; }
        if c == '!' { self.advance(); return Token::Bang; }
        if c == '=' { self.advance(); return Token::Eq; }
        if c == '<' { self.advance(); return Token::Lt; }
        if c == '>' { self.advance(); return Token::Gt; }
        if c == '?' { self.advance(); return Token::Question; }
        if c == ':' { self.advance(); return Token::Colon; }
        if c == '.' { self.advance(); return Token::Dot; }
        if c == ',' { self.advance(); return Token::Comma; }
        if c == ';' { self.advance(); return Token::Semicolon; }
        if c == '(' { self.advance(); return Token::LParen; }
        if c == ')' { self.advance(); return Token::RParen; }
        if c == '[' { self.advance(); return Token::LBracket; }
        if c == ']' { self.advance(); return Token::RBracket; }
        if c == '{' { self.advance(); return Token::LBrace; }
        if c == '}' { self.advance(); return Token::RBrace; }

        // Hash for preprocessor
        if c == '#' {
            self.advance();
            if self.peek() == Some('#') { self.advance(); return Token::HashHash; }
            return Token::Hash;
        }
        
        // Unknown - skip
        self.advance();
        self.next_token()
    }
    
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            if tok == Token::Eof { tokens.push(tok); break; }
            if !matches!(tok, Token::Comment | Token::Whitespace) {
                tokens.push(tok);
            }
        }
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_keywords() {
        let mut lex = Lexer::new("int main void return");
        assert!(matches!(lex.next_token(), Token::KwInt));
        assert!(matches!(lex.next_token(), Token::Ident(_)));
        assert!(matches!(lex.next_token(), Token::KwVoid));
        assert!(matches!(lex.next_token(), Token::KwReturn));
    }
    
    #[test]
    fn test_numbers() {
        let mut lex = Lexer::new("42 3.14 0xFF");
        assert!(matches!(lex.next_token(), Token::IntLit(42)));
        assert!(matches!(lex.next_token(), Token::FloatLit(_)));
        assert!(matches!(lex.next_token(), Token::IntLit(255)));
    }
    
    #[test]
    fn test_strings() {
        let mut lex = Lexer::new(r#""hello""#);
        assert!(matches!(lex.next_token(), Token::StringLit(_)));
    }
}
