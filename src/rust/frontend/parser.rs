// Parser para ADead-BIB
// Convierte tokens en AST
// Lenguaje de uso general - Binario + HEX

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
            match self.peek() {
                Some(Token::Def) => {
                    let func = self.parse_function()?;
                    program.add_function(func);
                    self.skip_newlines();
                }
                Some(Token::Class) => {
                    let class = self.parse_class()?;
                    program.add_class(class);
                    self.skip_newlines();
                }
                _ => break,
            }
        }
        
        Ok(program)
    }
    
    fn parse_class(&mut self) -> Result<Class, ParseError> {
        self.expect(Token::Class)?;
        
        let name = match self.advance() {
            Some(Token::Identifier(n)) => n,
            Some(token) => return Err(ParseError::UnexpectedToken(token)),
            None => return Err(ParseError::UnexpectedEof),
        };
        
        // Herencia opcional: class Foo extends Bar:
        let parent = if matches!(self.peek(), Some(Token::Extends)) {
            self.advance();
            match self.advance() {
                Some(Token::Identifier(p)) => Some(p),
                Some(token) => return Err(ParseError::UnexpectedToken(token)),
                None => return Err(ParseError::UnexpectedEof),
            }
        } else {
            None
        };
        
        self.expect(Token::Colon)?;
        self.skip_newlines();
        
        let mut fields = Vec::new();
        let mut methods = Vec::new();
        
        // Parse class body
        loop {
            self.skip_newlines();
            match self.peek() {
                Some(Token::Def) | Some(Token::Virtual) | Some(Token::Override) | Some(Token::Static) => {
                    let method = self.parse_method()?;
                    methods.push(method);
                }
                Some(Token::Identifier(_)) => {
                    let field = self.parse_field()?;
                    fields.push(field);
                }
                _ => break,
            }
        }
        
        Ok(Class { name, parent, fields, methods })
    }
    
    fn parse_field(&mut self) -> Result<Field, ParseError> {
        let name = match self.advance() {
            Some(Token::Identifier(n)) => n,
            Some(token) => return Err(ParseError::UnexpectedToken(token)),
            None => return Err(ParseError::UnexpectedEof),
        };
        
        let default_value = if matches!(self.peek(), Some(Token::Equals)) {
            self.advance();
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        self.skip_newlines();
        Ok(Field { name, type_name: None, default_value })
    }
    
    fn parse_method(&mut self) -> Result<Method, ParseError> {
        let is_virtual = if matches!(self.peek(), Some(Token::Virtual)) {
            self.advance();
            true
        } else {
            false
        };
        
        let is_override = if matches!(self.peek(), Some(Token::Override)) {
            self.advance();
            true
        } else {
            false
        };
        
        let is_static = if matches!(self.peek(), Some(Token::Static)) {
            self.advance();
            true
        } else {
            false
        };
        
        self.expect(Token::Def)?;
        
        let name = match self.advance() {
            Some(Token::Identifier(n)) => n,
            Some(token) => return Err(ParseError::UnexpectedToken(token)),
            None => return Err(ParseError::UnexpectedEof),
        };
        
        self.expect(Token::LParen)?;
        
        let mut params = Vec::new();
        if !matches!(self.peek(), Some(Token::RParen)) {
            loop {
                // Skip 'self' parameter
                if matches!(self.peek(), Some(Token::This)) {
                    self.advance();
                    if matches!(self.peek(), Some(Token::Comma)) {
                        self.advance();
                    }
                    continue;
                }
                
                let param_name = match self.advance() {
                    Some(Token::Identifier(n)) => n,
                    Some(Token::RParen) => break,
                    Some(token) => return Err(ParseError::UnexpectedToken(token)),
                    None => return Err(ParseError::UnexpectedEof),
                };
                params.push(Param { name: param_name, type_name: None });
                
                if matches!(self.peek(), Some(Token::Comma)) {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        
        self.expect(Token::RParen)?;
        self.expect(Token::Colon)?;
        self.skip_newlines();
        
        let body = self.parse_block()?;
        
        Ok(Method { name, params, return_type: None, body, is_virtual, is_override, is_static })
    }
    
    fn parse_function(&mut self) -> Result<Function, ParseError> {
        self.expect(Token::Def)?;
        
        let name = match self.advance() {
            Some(Token::Identifier(name)) => name,
            Some(token) => return Err(ParseError::UnexpectedToken(token)),
            None => return Err(ParseError::UnexpectedEof),
        };
        
        self.expect(Token::LParen)?;
        
        // Parse parameters
        let mut params = Vec::new();
        if !matches!(self.peek(), Some(Token::RParen)) {
            loop {
                let param_name = match self.advance() {
                    Some(Token::Identifier(n)) => n,
                    Some(token) => return Err(ParseError::UnexpectedToken(token)),
                    None => return Err(ParseError::UnexpectedEof),
                };
                params.push(Param { name: param_name, type_name: None });
                
                if matches!(self.peek(), Some(Token::Comma)) {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        
        self.expect(Token::RParen)?;
        self.expect(Token::Colon)?;
        self.skip_newlines();
        
        let body = self.parse_block()?;
        
        Ok(Function { name, params, return_type: None, body })
    }
    
    fn parse_block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut stmts = Vec::new();
        loop {
            self.skip_newlines();
            match self.peek() {
                Some(Token::Def) | None => break,
                Some(Token::Else) | Some(Token::Elif) => break,
                _ => {}
            }
            if let Some(stmt) = self.parse_statement()? {
                stmts.push(stmt);
            } else {
                break;
            }
        }
        Ok(stmts)
    }
    
    fn parse_statement(&mut self) -> Result<Option<Stmt>, ParseError> {
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        match self.peek() {
            Some(Token::Print) => {
                self.advance();
                self.expect(Token::LParen)?;
                let expr = self.parse_expression()?;
                self.expect(Token::RParen)?;
                self.skip_newlines();
                
                // Detectar si es string o número
                match &expr {
                    Expr::String(_) => Ok(Some(Stmt::Print(expr))),
                    _ => Ok(Some(Stmt::PrintNum(expr))),
                }
            }
            
            Some(Token::If) => {
                self.advance();
                let condition = self.parse_expression()?;
                self.expect(Token::Colon)?;
                self.skip_newlines();
                
                let then_body = self.parse_if_body()?;
                
                let else_body = if matches!(self.peek(), Some(Token::Else)) {
                    self.advance();
                    self.expect(Token::Colon)?;
                    self.skip_newlines();
                    Some(self.parse_if_body()?)
                } else if matches!(self.peek(), Some(Token::Elif)) {
                    // elif se convierte en else { if ... }
                    let elif_stmt = self.parse_statement()?;
                    elif_stmt.map(|s| vec![s])
                } else {
                    None
                };
                
                Ok(Some(Stmt::If { condition, then_body, else_body }))
            }
            
            Some(Token::While) => {
                self.advance();
                let condition = self.parse_expression()?;
                self.expect(Token::Colon)?;
                self.skip_newlines();
                let body = self.parse_if_body()?;
                Ok(Some(Stmt::While { condition, body }))
            }
            
            Some(Token::For) => {
                self.advance();
                let var = match self.advance() {
                    Some(Token::Identifier(n)) => n,
                    Some(token) => return Err(ParseError::UnexpectedToken(token)),
                    None => return Err(ParseError::UnexpectedEof),
                };
                self.expect(Token::In)?;
                self.expect(Token::Range)?;
                self.expect(Token::LParen)?;
                let start = self.parse_expression()?;
                self.expect(Token::Comma)?;
                let end = self.parse_expression()?;
                self.expect(Token::RParen)?;
                self.expect(Token::Colon)?;
                self.skip_newlines();
                let body = self.parse_if_body()?;
                Ok(Some(Stmt::For { var, start, end, body }))
            }
            
            Some(Token::Return) => {
                self.advance();
                if matches!(self.peek(), Some(Token::Newline)) || self.peek().is_none() {
                    self.skip_newlines();
                    Ok(Some(Stmt::Return(None)))
                } else {
                    let expr = self.parse_expression()?;
                    self.skip_newlines();
                    Ok(Some(Stmt::Return(Some(expr))))
                }
            }
            
            Some(Token::Break) => {
                self.advance();
                self.skip_newlines();
                Ok(Some(Stmt::Break))
            }
            
            Some(Token::Continue) => {
                self.advance();
                self.skip_newlines();
                Ok(Some(Stmt::Continue))
            }
            
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.advance();
                
                if matches!(self.peek(), Some(Token::Equals)) {
                    self.advance();
                    let value = self.parse_expression()?;
                    self.skip_newlines();
                    Ok(Some(Stmt::Assign { name, value }))
                } else if matches!(self.peek(), Some(Token::LParen)) {
                    // Function call as statement
                    self.advance();
                    let mut args = Vec::new();
                    if !matches!(self.peek(), Some(Token::RParen)) {
                        args.push(self.parse_expression()?);
                        while matches!(self.peek(), Some(Token::Comma)) {
                            self.advance();
                            args.push(self.parse_expression()?);
                        }
                    }
                    self.expect(Token::RParen)?;
                    self.skip_newlines();
                    Ok(Some(Stmt::Expr(Expr::Call { name, args })))
                } else {
                    Ok(Some(Stmt::Expr(Expr::Variable(name))))
                }
            }
            
            Some(Token::Newline) => {
                self.advance();
                Ok(None)
            }
            
            _ => Ok(None),
        }
    }
    
    fn parse_if_body(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut stmts = Vec::new();
        loop {
            self.skip_newlines();
            match self.peek() {
                Some(Token::Def) | Some(Token::Else) | Some(Token::Elif) | None => break,
                _ => {}
            }
            
            // Check for dedent (simple heuristic: next statement at same level)
            if let Some(stmt) = self.parse_statement()? {
                stmts.push(stmt);
            } else {
                break;
            }
            
            // Simple block detection: stop after first statement for now
            // TODO: proper indentation tracking
            break;
        }
        Ok(stmts)
    }
    
    fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        self.parse_or()
    }
    
    fn parse_or(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_and()?;
        while matches!(self.peek(), Some(Token::Or)) {
            self.advance();
            let right = self.parse_and()?;
            left = Expr::BinaryOp {
                op: BinOp::Or,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }
    
    fn parse_and(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_comparison()?;
        while matches!(self.peek(), Some(Token::And)) {
            self.advance();
            let right = self.parse_comparison()?;
            left = Expr::BinaryOp {
                op: BinOp::And,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }
    
    fn parse_comparison(&mut self) -> Result<Expr, ParseError> {
        let left = self.parse_additive()?;
        
        let op = match self.peek() {
            Some(Token::EqEq) => Some(CmpOp::Eq),
            Some(Token::NotEq) => Some(CmpOp::Ne),
            Some(Token::Less) => Some(CmpOp::Lt),
            Some(Token::LessEq) => Some(CmpOp::Le),
            Some(Token::Greater) => Some(CmpOp::Gt),
            Some(Token::GreaterEq) => Some(CmpOp::Ge),
            _ => None,
        };
        
        if let Some(op) = op {
            self.advance();
            let right = self.parse_additive()?;
            Ok(Expr::Comparison {
                op,
                left: Box::new(left),
                right: Box::new(right),
            })
        } else {
            Ok(left)
        }
    }
    
    fn parse_additive(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_multiplicative()?;
        
        loop {
            let op = match self.peek() {
                Some(Token::Plus) => BinOp::Add,
                Some(Token::Minus) => BinOp::Sub,
                _ => break,
            };
            self.advance();
            let right = self.parse_multiplicative()?;
            left = Expr::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }
    
    fn parse_multiplicative(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_unary()?;
        
        loop {
            let op = match self.peek() {
                Some(Token::Star) => BinOp::Mul,
                Some(Token::Slash) => BinOp::Div,
                Some(Token::Percent) => BinOp::Mod,
                _ => break,
            };
            self.advance();
            let right = self.parse_unary()?;
            left = Expr::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }
    
    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        match self.peek() {
            Some(Token::Minus) => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expr::UnaryOp {
                    op: UnaryOp::Neg,
                    expr: Box::new(expr),
                })
            }
            Some(Token::Not) => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expr::UnaryOp {
                    op: UnaryOp::Not,
                    expr: Box::new(expr),
                })
            }
            _ => self.parse_postfix(),
        }
    }
    
    fn parse_postfix(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_primary()?;
        
        // Handle method calls, field access, and indexing
        loop {
            if matches!(self.peek(), Some(Token::Dot)) {
                self.advance();
                let member = match self.advance() {
                    Some(Token::Identifier(n)) => n,
                    Some(token) => return Err(ParseError::UnexpectedToken(token)),
                    None => return Err(ParseError::UnexpectedEof),
                };
                
                if matches!(self.peek(), Some(Token::LParen)) {
                    // Method call
                    self.advance();
                    let mut args = Vec::new();
                    if !matches!(self.peek(), Some(Token::RParen)) {
                        args.push(self.parse_expression()?);
                        while matches!(self.peek(), Some(Token::Comma)) {
                            self.advance();
                            args.push(self.parse_expression()?);
                        }
                    }
                    self.expect(Token::RParen)?;
                    expr = Expr::MethodCall {
                        object: Box::new(expr),
                        method: member,
                        args,
                    };
                } else {
                    // Field access
                    expr = Expr::FieldAccess {
                        object: Box::new(expr),
                        field: member,
                    };
                }
            } else if matches!(self.peek(), Some(Token::LBracket)) {
                // Array indexing: arr[0]
                self.advance();
                let index = self.parse_expression()?;
                self.expect(Token::RBracket)?;
                expr = Expr::Index {
                    object: Box::new(expr),
                    index: Box::new(index),
                };
            } else {
                break;
            }
        }
        
        Ok(expr)
    }
    
    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
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
            
            Some(Token::True) => {
                self.advance();
                Ok(Expr::Bool(true))
            }
            
            Some(Token::False) => {
                self.advance();
                Ok(Expr::Bool(false))
            }
            
            Some(Token::This) => {
                self.advance();
                Ok(Expr::This)
            }
            
            Some(Token::Super) => {
                self.advance();
                Ok(Expr::Super)
            }
            
            Some(Token::New) => {
                self.advance();
                let class_name = match self.advance() {
                    Some(Token::Identifier(n)) => n,
                    Some(token) => return Err(ParseError::UnexpectedToken(token)),
                    None => return Err(ParseError::UnexpectedEof),
                };
                self.expect(Token::LParen)?;
                let mut args = Vec::new();
                if !matches!(self.peek(), Some(Token::RParen)) {
                    args.push(self.parse_expression()?);
                    while matches!(self.peek(), Some(Token::Comma)) {
                        self.advance();
                        args.push(self.parse_expression()?);
                    }
                }
                self.expect(Token::RParen)?;
                Ok(Expr::New { class_name, args })
            }
            
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.advance();
                
                if matches!(self.peek(), Some(Token::LParen)) {
                    self.advance();
                    let mut args = Vec::new();
                    if !matches!(self.peek(), Some(Token::RParen)) {
                        args.push(self.parse_expression()?);
                        while matches!(self.peek(), Some(Token::Comma)) {
                            self.advance();
                            args.push(self.parse_expression()?);
                        }
                    }
                    self.expect(Token::RParen)?;
                    Ok(Expr::Call { name, args })
                } else {
                    Ok(Expr::Variable(name))
                }
            }
            
            Some(Token::LParen) => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(Token::RParen)?;
                Ok(expr)
            }
            
            Some(Token::LBracket) => {
                // Array literal: [1, 2, 3]
                self.advance();
                let mut elements = Vec::new();
                if !matches!(self.peek(), Some(Token::RBracket)) {
                    elements.push(self.parse_expression()?);
                    while matches!(self.peek(), Some(Token::Comma)) {
                        self.advance();
                        elements.push(self.parse_expression()?);
                    }
                }
                self.expect(Token::RBracket)?;
                Ok(Expr::Array(elements))
            }
            
            Some(Token::Null) => {
                self.advance();
                Ok(Expr::Null)
            }
            
            Some(token) => Err(ParseError::UnexpectedToken(token.clone())),
            None => Err(ParseError::UnexpectedEof),
        }
    }
}

pub fn parse(source: &str) -> Result<Program, Box<dyn std::error::Error>> {
    Parser::parse_program(source)
        .map_err(|e| format!("Parse error: {:?}", e).into())
}

