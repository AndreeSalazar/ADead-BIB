// Lexer para ADead-BIB
// Tokeniza el código fuente

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Def,
    Print,
    Return,
    
    // Identifiers
    Identifier(String),
    
    // Literals
    Number(i64),
    String(String),
    
    // Operators
    Plus,      // +
    Minus,     // -
    Star,      // *
    Slash,     // /
    Equals,    // =
    
    // Punctuation
    LParen,    // (
    RParen,    // )
    Colon,     // :
    Newline,   // \n
    
    // EOF
    Eof,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let current = if chars.is_empty() { None } else { Some(chars[0]) };
        
        Self {
            input: chars,
            position: 0,
            current_char: current,
        }
    }
    
    fn advance(&mut self) {
        self.position += 1;
        if self.position >= self.input.len() {
            self.current_char = None;
        } else {
            self.current_char = Some(self.input[self.position]);
        }
    }
    
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch == ' ' || ch == '\t' || ch == '\r' {
                self.advance();
            } else {
                break;
            }
        }
    }
    
    fn skip_comment(&mut self) {
        if self.current_char == Some('#') {
            while let Some(ch) = self.current_char {
                if ch == '\n' {
                    break;
                }
                self.advance();
            }
        }
    }
    
    fn read_number(&mut self) -> i64 {
        let mut num_str = String::new();
        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() {
                num_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        num_str.parse().unwrap_or(0)
    }
    
    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();
        while let Some(ch) = self.current_char {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        ident
    }
    
    fn read_string(&mut self) -> String {
        self.advance(); // Skip opening "
        let mut s = String::new();
        while let Some(ch) = self.current_char {
            if ch == '"' {
                self.advance(); // Skip closing "
                break;
            }
            s.push(ch);
            self.advance();
        }
        s
    }
    
    pub fn next_token(&mut self) -> Token {
        // Skip whitespace and comments
        loop {
            self.skip_whitespace();
            if self.current_char == Some('#') {
                self.skip_comment();
            } else {
                break;
            }
        }
        
        match self.current_char {
            None => Token::Eof,
            
            Some('+') => {
                self.advance();
                Token::Plus
            }
            
            Some('-') => {
                self.advance();
                Token::Minus
            }
            
            Some('*') => {
                self.advance();
                Token::Star
            }
            
            Some('/') => {
                self.advance();
                Token::Slash
            }
            
            Some('=') => {
                self.advance();
                Token::Equals
            }
            
            Some('(') => {
                self.advance();
                Token::LParen
            }
            
            Some(')') => {
                self.advance();
                Token::RParen
            }
            
            Some(':') => {
                self.advance();
                Token::Colon
            }
            
            Some('\n') => {
                self.advance();
                Token::Newline
            }
            
            Some('"') => {
                Token::String(self.read_string())
            }
            
            Some(ch) if ch.is_ascii_digit() => {
                Token::Number(self.read_number())
            }
            
            Some(ch) if ch.is_ascii_alphabetic() || ch == '_' => {
                let ident = self.read_identifier();
                match ident.as_str() {
                    "def" => Token::Def,
                    "print" => Token::Print,
                    "return" => Token::Return,
                    _ => Token::Identifier(ident),
                }
            }
            
            Some(ch) => {
                eprintln!("Carácter inesperado: {}", ch);
                self.advance();
                self.next_token()
            }
        }
    }
    
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();
            let is_eof = matches!(token, Token::Eof);
            tokens.push(token);
            if is_eof {
                break;
            }
        }
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_tokens() {
        let mut lexer = Lexer::new("def main():");
        assert_eq!(lexer.next_token(), Token::Def);
        assert_eq!(lexer.next_token(), Token::Identifier("main".to_string()));
        assert_eq!(lexer.next_token(), Token::LParen);
        assert_eq!(lexer.next_token(), Token::RParen);
        assert_eq!(lexer.next_token(), Token::Colon);
    }
    
    #[test]
    fn test_string() {
        let mut lexer = Lexer::new(r#""Hello, World!""#);
        assert_eq!(lexer.next_token(), Token::String("Hello, World!".to_string()));
    }
}

