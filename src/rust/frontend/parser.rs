// Parser para ADead-BIB
// Convierte tokens en AST

use super::lexer::{Lexer, Token};
use super::ast::*;
use std::iter::Peekable;
use std::vec::IntoIter;

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Token),
    UnexpectedEof,
    ExpectedToken(&'static str),
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
        }
    }
    
    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }
    
    fn advance(&mut self) -> Option<Token> {
        self.tokens.next()
    }
    
    fn expect(&mut self, expected: Token) -> Result<(), ParseError> {
        match self.advance() {
            Some(token) if token == expected => Ok(()),
            Some(token) => Err(ParseError::UnexpectedToken(token)),
            None => Err(ParseError::UnexpectedEof),
        }
    }
    
    fn skip_newlines(&mut self) {
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
    }
    
    pub fn parse_program(source: &str) -> Result<Program, ParseError> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        
        // Filtrar solo EOF, mantener newlines para parsing
        let tokens: Vec<Token> = tokens
            .into_iter()
            .filter(|t| !matches!(t, Token::Eof))
            .collect();
        
        let mut parser = Parser::new(tokens);
        parser.parse()
    }
    
    fn parse(&mut self) -> Result<Program, ParseError> {
        let mut program = Program::new();
        
        self.skip_newlines();
        
        while self.peek().is_some() {
            if let Some(Token::Def) = self.peek() {
                let func = self.parse_function()?;
                program.add_function(func);
                self.skip_newlines();
            } else {
                break;
            }
        }
        
        Ok(program)
    }
    
    fn parse_function(&mut self) -> Result<Function, ParseError> {
        // def
        self.expect(Token::Def)?;
        
        // identifier
        let name = match self.advance() {
            Some(Token::Identifier(name)) => name,
            Some(token) => return Err(ParseError::UnexpectedToken(token)),
            None => return Err(ParseError::UnexpectedEof),
        };
        
        // ()
        self.expect(Token::LParen)?;
        self.expect(Token::RParen)?;
        
        // :
        self.expect(Token::Colon)?;
        
        self.skip_newlines();
        
        // body (statements)
        let mut body = Vec::new();
        while let Some(stmt) = self.parse_statement()? {
            body.push(stmt);
            self.skip_newlines();
        }
        
        Ok(Function { name, body })
    }
    
    fn parse_statement(&mut self) -> Result<Option<Stmt>, ParseError> {
        // Skip leading newlines
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        match self.peek() {
            Some(Token::Print) => {
                self.advance(); // print
                self.expect(Token::LParen)?;
                
                let expr = self.parse_expression()?;
                self.expect(Token::RParen)?;
                
                // Optional newline after statement
                if matches!(self.peek(), Some(Token::Newline)) {
                    self.advance();
                }
                
                Ok(Some(Stmt::Print(expr)))
            }
            
            Some(Token::Identifier(name)) => {
                // Check if next token is = for assignment
                let name = name.clone();
                self.advance(); // consume identifier
                
                if matches!(self.peek(), Some(Token::Equals)) {
                    // Assignment: identifier = ...
                    self.advance(); // consume =
                    let value = self.parse_expression()?;
                    
                    // Optional newline
                    if matches!(self.peek(), Some(Token::Newline)) {
                        self.advance();
                    }
                    
                    Ok(Some(Stmt::Assign { name, value }))
                } else {
                    // Not an assignment, this is an expression statement
                    // For MVP, we don't handle this yet
                    Ok(None)
                }
            }
            
            Some(Token::Newline) => {
                self.advance();
                Ok(None)
            }
            
            _ => Ok(None),
        }
    }
    
    fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        match self.peek() {
            Some(Token::Number(n)) => {
                let n = *n;
                self.advance();
                Ok(Expr::Number(n))
            }
            
            Some(Token::String(s)) => {
                let s = s.clone();
                self.advance();
                Ok(Expr::String(s))
            }
            
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.advance();
                
                // Check if it's a function call
                if matches!(self.peek(), Some(Token::LParen)) {
                    self.advance(); // (
                    
                    let mut args = Vec::new();
                    if !matches!(self.peek(), Some(Token::RParen)) {
                        args.push(self.parse_expression()?);
                        // TODO: Parse multiple args
                    }
                    
                    self.expect(Token::RParen)?;
                    Ok(Expr::Call { name, args })
                } else {
                    Ok(Expr::Variable(name))
                }
            }
            
            Some(token) => Err(ParseError::UnexpectedToken(token.clone())),
            None => Err(ParseError::UnexpectedEof),
        }
    }
}

// Re-export para uso externo
pub fn parse(source: &str) -> Result<Program, Box<dyn std::error::Error>> {
    Parser::parse_program(source)
        .map_err(|e| format!("Parse error: {:?}", e).into())
}

