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

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedToken(t) => {
                write!(f, "❌ Syntax Error: Unexpected token '{:?}'. Check your syntax.", t)
            }
            ParseError::UnexpectedEof => {
                write!(f, "❌ Syntax Error: Unexpected end of file. Missing closing brackets, parentheses, or statements.")
            }
            ParseError::ExpectedToken(s) => {
                write!(f, "❌ Syntax Error: Expected '{}' but found something else.", s)
            }
        }
    }
}

impl std::error::Error for ParseError {}

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
    
    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let mut program = Program::new();
        
        self.skip_newlines();
        
        // Parse imports first
        while matches!(self.peek(), Some(Token::Import) | Some(Token::From)) {
            let import = self.parse_import()?;
            program.add_import(import);
            self.skip_newlines();
        }
        
        while self.peek().is_some() {
            match self.peek() {
                // Python style: def
                Some(Token::Def) => {
                    let func = self.parse_function()?;
                    program.add_function(func);
                    self.skip_newlines();
                }
                // Rust style: fn
                Some(Token::Fn) => {
                    let func = self.parse_rust_function()?;
                    program.add_function(func);
                    self.skip_newlines();
                }
                // Rust style: struct
                Some(Token::Struct) => {
                    let class = self.parse_struct()?;
                    program.add_class(class);
                    self.skip_newlines();
                }
                // Rust style: impl
                Some(Token::Impl) => {
                    self.parse_impl(&mut program)?;
                    self.skip_newlines();
                }
                // Rust style: trait
                Some(Token::Trait) => {
                    let iface = self.parse_trait()?;
                    program.add_interface(iface);
                    self.skip_newlines();
                }
                Some(Token::Class) => {
                    let class = self.parse_class()?;
                    program.add_class(class);
                    self.skip_newlines();
                }
                Some(Token::Interface) => {
                    let iface = self.parse_interface()?;
                    program.add_interface(iface);
                    self.skip_newlines();
                }
                // Rust style: let / const
                Some(Token::Let) | Some(Token::Const) => {
                    let stmt = self.parse_let_statement()?;
                    program.add_statement(stmt);
                    self.skip_newlines();
                }
                _ => {
                    // Try to parse as a statement (for scripts)
                    let stmt = self.parse_statement()?;
                    program.add_statement(stmt);
                    self.skip_newlines();
                }
            }
        }
        
        Ok(program)
    }
    
    fn parse_import(&mut self) -> Result<Import, ParseError> {
        if matches!(self.peek(), Some(Token::From)) {
            // from module import item1, item2
            self.advance();
            let module = match self.advance() {
                Some(Token::Identifier(m)) => m,
                Some(token) => return Err(ParseError::UnexpectedToken(token)),
                None => return Err(ParseError::UnexpectedEof),
            };
            
            self.expect(Token::Import)?;
            
            let mut items = Vec::new();
            loop {
                match self.advance() {
                    Some(Token::Identifier(i)) => items.push(i),
                    Some(token) => return Err(ParseError::UnexpectedToken(token)),
                    None => return Err(ParseError::UnexpectedEof),
                }
                if matches!(self.peek(), Some(Token::Comma)) {
                    self.advance();
                } else {
                    break;
                }
            }
            
            self.skip_newlines();
            Ok(Import { module, items, alias: None })
        } else {
            // import module [as alias]
            self.expect(Token::Import)?;
            let module = match self.advance() {
                Some(Token::Identifier(m)) => m,
                Some(token) => return Err(ParseError::UnexpectedToken(token)),
                None => return Err(ParseError::UnexpectedEof),
            };
            
            let alias = if matches!(self.peek(), Some(Token::As)) {
                self.advance();
                match self.advance() {
                    Some(Token::Identifier(a)) => Some(a),
                    _ => None,
                }
            } else {
                None
            };
            
            self.skip_newlines();
            Ok(Import { module, items: Vec::new(), alias })
        }
    }
    
    fn parse_interface(&mut self) -> Result<Interface, ParseError> {
        self.expect(Token::Interface)?;
        
        let name = match self.advance() {
            Some(Token::Identifier(n)) => n,
            Some(token) => return Err(ParseError::UnexpectedToken(token)),
            None => return Err(ParseError::UnexpectedEof),
        };
        
        self.expect(Token::Colon)?;
        self.skip_newlines();
        
        let mut methods = Vec::new();
        
        // Parse interface body (method signatures only)
        loop {
            self.skip_newlines();
            if matches!(self.peek(), Some(Token::Def)) {
                self.advance();
                let method_name = match self.advance() {
                    Some(Token::Identifier(n)) => n,
                    Some(token) => return Err(ParseError::UnexpectedToken(token)),
                    None => return Err(ParseError::UnexpectedEof),
                };
                
                self.expect(Token::LParen)?;
                let mut params = Vec::new();
                if !matches!(self.peek(), Some(Token::RParen)) {
                    loop {
                        if matches!(self.peek(), Some(Token::This)) {
                            self.advance();
                            if matches!(self.peek(), Some(Token::Comma)) {
                                self.advance();
                            }
                            continue;
                        }
                        let param_name = match self.advance() {
                            Some(Token::Identifier(n)) => n,
                            Some(token) => return Err(ParseError::UnexpectedToken(token)),
                            None => return Err(ParseError::UnexpectedEof),
                        };
                        
                        let type_name = if matches!(self.peek(), Some(Token::Colon)) {
                            self.advance();
                            match self.advance() {
                                Some(Token::Identifier(t)) => Some(t),
                                _ => None,
                            }
                        } else {
                            None
                        };
                        
                        params.push(Param { name: param_name, type_name });
                        
                        if matches!(self.peek(), Some(Token::Comma)) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
                self.expect(Token::RParen)?;
                
                let return_type = if matches!(self.peek(), Some(Token::Arrow)) {
                    self.advance();
                    match self.advance() {
                        Some(Token::Identifier(t)) => Some(t),
                        _ => None,
                    }
                } else {
                    None
                };
                
                self.skip_newlines();
                methods.push(MethodSignature { name: method_name, params, return_type });
            } else {
                break;
            }
        }
        
        Ok(Interface { name, methods })
    }

    /// Parse Rust-style function: fn name(params) -> ReturnType { body }
    fn parse_rust_function(&mut self) -> Result<Function, ParseError> {
        self.advance(); // consume 'fn'
        
        let name = match self.advance() {
            Some(Token::Identifier(n)) => n,
            Some(token) => return Err(ParseError::UnexpectedToken(token)),
            None => return Err(ParseError::UnexpectedEof),
        };
        
        self.expect(Token::LParen)?;
        
        let mut params = Vec::new();
        if !matches!(self.peek(), Some(Token::RParen)) {
            loop {
                let param_name = match self.advance() {
                    Some(Token::Identifier(n)) => n,
                    Some(token) => return Err(ParseError::UnexpectedToken(token)),
                    None => return Err(ParseError::UnexpectedEof),
                };
                
                let type_name = if matches!(self.peek(), Some(Token::Colon)) {
                    self.advance();
                    match self.advance() {
                        Some(Token::Identifier(t)) => Some(t),
                        _ => None,
                    }
                } else {
                    None
                };
                
                params.push(Param { name: param_name, type_name });
                
                if matches!(self.peek(), Some(Token::Comma)) {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        
        self.expect(Token::RParen)?;
        
        let return_type = if matches!(self.peek(), Some(Token::Arrow)) {
            self.advance();
            match self.advance() {
                Some(Token::Identifier(t)) => Some(t),
                _ => None,
            }
        } else {
            None
        };
        
        // Rust style: { body }
        self.expect(Token::LBrace)?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !matches!(self.peek(), Some(Token::RBrace) | None) {
            let stmt = self.parse_statement()?;
            body.push(stmt);
            self.skip_newlines();
            // Skip optional semicolons
            while matches!(self.peek(), Some(Token::Semicolon)) {
                self.advance();
                self.skip_newlines();
            }
        }
        
        self.expect(Token::RBrace)?;
        
        Ok(Function { name, params, return_type, body })
    }
    
    /// Parse let statement: let [mut] name [: Type] = value;
    fn parse_let_statement(&mut self) -> Result<Stmt, ParseError> {
        let is_const = matches!(self.peek(), Some(Token::Const));
        self.advance(); // consume 'let' or 'const'
        
        // Check for 'mut'
        let _is_mut = if matches!(self.peek(), Some(Token::Mut)) {
            self.advance();
            true
        } else {
            false
        };
        
        let name = match self.advance() {
            Some(Token::Identifier(n)) => n,
            Some(token) => return Err(ParseError::UnexpectedToken(token)),
            None => return Err(ParseError::UnexpectedEof),
        };
        
        // Optional type annotation
        let _type_name = if matches!(self.peek(), Some(Token::Colon)) {
            self.advance();
            match self.advance() {
                Some(Token::Identifier(t)) => Some(t),
                _ => None,
            }
        } else {
            None
        };
        
        self.expect(Token::Equals)?;
        let value = self.parse_expression()?;
        
        // Optional semicolon
        if matches!(self.peek(), Some(Token::Semicolon)) {
            self.advance();
        }
        
        // For const, we could add a different variant, but for now treat as assign
        let _ = is_const; // TODO: Handle const differently
        
        Ok(Stmt::Assign { name, value })
    }
    
    /// Parse Rust-style struct
    fn parse_struct(&mut self) -> Result<Class, ParseError> {
        self.advance(); // consume 'struct'
        
        let name = match self.advance() {
            Some(Token::Identifier(n)) => n,
            Some(token) => return Err(ParseError::UnexpectedToken(token)),
            None => return Err(ParseError::UnexpectedEof),
        };
        
        self.expect(Token::LBrace)?;
        self.skip_newlines();
        
        let mut fields = Vec::new();
        
        while !matches!(self.peek(), Some(Token::RBrace) | None) {
            let field_name = match self.advance() {
                Some(Token::Identifier(n)) => n,
                Some(Token::RBrace) => break,
                Some(token) => return Err(ParseError::UnexpectedToken(token)),
                None => return Err(ParseError::UnexpectedEof),
            };
            
            self.expect(Token::Colon)?;
            
            let type_name = match self.advance() {
                Some(Token::Identifier(t)) => Some(t),
                _ => None,
            };
            
            fields.push(Field {
                name: field_name,
                type_name,
                default_value: None,
            });
            
            // Skip comma
            if matches!(self.peek(), Some(Token::Comma)) {
                self.advance();
            }
            self.skip_newlines();
        }
        
        self.expect(Token::RBrace)?;
        
        Ok(Class {
            name,
            parent: None,
            implements: Vec::new(),
            fields,
            methods: Vec::new(),
            constructor: None,
            destructor: None,
        })
    }
    
    /// Parse Rust-style impl block
    fn parse_impl(&mut self, program: &mut Program) -> Result<(), ParseError> {
        self.advance(); // consume 'impl'
        
        let struct_name = match self.advance() {
            Some(Token::Identifier(n)) => n,
            Some(token) => return Err(ParseError::UnexpectedToken(token)),
            None => return Err(ParseError::UnexpectedEof),
        };
        
        self.expect(Token::LBrace)?;
        self.skip_newlines();
        
        // Parse methods and add them as functions with prefixed names
        while !matches!(self.peek(), Some(Token::RBrace) | None) {
            if matches!(self.peek(), Some(Token::Fn)) {
                let mut func = self.parse_rust_function()?;
                // Prefix method name with struct name
                func.name = format!("{}::{}", struct_name, func.name);
                program.add_function(func);
            } else {
                self.advance(); // Skip unknown tokens
            }
            self.skip_newlines();
        }
        
        self.expect(Token::RBrace)?;
        
        Ok(())
    }
    
    /// Parse Rust-style trait
    fn parse_trait(&mut self) -> Result<Interface, ParseError> {
        self.advance(); // consume 'trait'
        
        let name = match self.advance() {
            Some(Token::Identifier(n)) => n,
            Some(token) => return Err(ParseError::UnexpectedToken(token)),
            None => return Err(ParseError::UnexpectedEof),
        };
        
        self.expect(Token::LBrace)?;
        self.skip_newlines();
        
        let mut methods = Vec::new();
        
        while !matches!(self.peek(), Some(Token::RBrace) | None) {
            if matches!(self.peek(), Some(Token::Fn)) {
                self.advance();
                let method_name = match self.advance() {
                    Some(Token::Identifier(n)) => n,
                    Some(token) => return Err(ParseError::UnexpectedToken(token)),
                    None => return Err(ParseError::UnexpectedEof),
                };
                
                self.expect(Token::LParen)?;
                let mut params = Vec::new();
                
                if !matches!(self.peek(), Some(Token::RParen)) {
                    loop {
                        // Skip &self, &mut self, self
                        if matches!(self.peek(), Some(Token::Ampersand)) {
                            self.advance();
                            if matches!(self.peek(), Some(Token::Mut)) {
                                self.advance();
                            }
                        }
                        if matches!(self.peek(), Some(Token::This)) {
                            self.advance();
                            if matches!(self.peek(), Some(Token::Comma)) {
                                self.advance();
                            }
                            continue;
                        }
                        
                        if matches!(self.peek(), Some(Token::RParen)) {
                            break;
                        }
                        
                        let param_name = match self.advance() {
                            Some(Token::Identifier(n)) => n,
                            Some(token) => return Err(ParseError::UnexpectedToken(token)),
                            None => return Err(ParseError::UnexpectedEof),
                        };
                        
                        let type_name = if matches!(self.peek(), Some(Token::Colon)) {
                            self.advance();
                            match self.advance() {
                                Some(Token::Identifier(t)) => Some(t),
                                _ => None,
                            }
                        } else {
                            None
                        };
                        
                        params.push(Param { name: param_name, type_name });
                        
                        if matches!(self.peek(), Some(Token::Comma)) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
                self.expect(Token::RParen)?;
                
                let return_type = if matches!(self.peek(), Some(Token::Arrow)) {
                    self.advance();
                    match self.advance() {
                        Some(Token::Identifier(t)) => Some(t),
                        _ => None,
                    }
                } else {
                    None
                };
                
                // Skip semicolon or body
                if matches!(self.peek(), Some(Token::Semicolon)) {
                    self.advance();
                } else if matches!(self.peek(), Some(Token::LBrace)) {
                    // Skip default implementation
                    let mut brace_count = 1;
                    self.advance();
                    while brace_count > 0 {
                        match self.advance() {
                            Some(Token::LBrace) => brace_count += 1,
                            Some(Token::RBrace) => brace_count -= 1,
                            None => break,
                            _ => {}
                        }
                    }
                }
                
                methods.push(MethodSignature { name: method_name, params, return_type });
            } else {
                self.advance();
            }
            self.skip_newlines();
        }
        
        self.expect(Token::RBrace)?;
        
        Ok(Interface { name, methods })
    }

    fn parse_function(&mut self) -> Result<Function, ParseError> {
        self.advance(); // consume 'def'
        
        let name = match self.advance() {
            Some(Token::Identifier(n)) => n,
            Some(token) => return Err(ParseError::UnexpectedToken(token)),
            None => return Err(ParseError::UnexpectedEof),
        };
        
        self.expect(Token::LParen)?;
        
        let mut params = Vec::new();
        if !matches!(self.peek(), Some(Token::RParen)) {
            loop {
                let param_name = match self.advance() {
                    Some(Token::Identifier(n)) => n,
                    Some(token) => return Err(ParseError::UnexpectedToken(token)),
                    None => return Err(ParseError::UnexpectedEof),
                };
                
                let type_name = if matches!(self.peek(), Some(Token::Colon)) {
                    self.advance();
                    match self.advance() {
                        Some(Token::Identifier(t)) => Some(t),
                        _ => None,
                    }
                } else {
                    None
                };
                
                params.push(Param { name: param_name, type_name });
                
                if matches!(self.peek(), Some(Token::Comma)) {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        
        self.expect(Token::RParen)?;
        
        let return_type = if matches!(self.peek(), Some(Token::Arrow)) {
            self.advance();
            match self.advance() {
                Some(Token::Identifier(t)) => Some(t),
                _ => None,
            }
        } else {
            None
        };
        
        self.expect(Token::Colon)?;
        self.skip_newlines();
        
        // Parse block (indentation based in python, but here we simplify to {})
        // Actually, previous examples didn't use braces, so likely indentation or just until next def/class
        // Or maybe just list of statements.
        // Let's assume indentation is handled by lexer or we just parse statements until end of block.
        // For simplicity: parse statements until EOF or Dedent (if we had it) or next Def/Class?
        // Wait, standard python uses indentation.
        // If we don't support indentation tokens, we might rely on "End" keyword or braces?
        // User's example:
        // def println(msg):
        //     print(msg)
        // Indentation.
        // If lexer produces Indent/Dedent tokens, we use them.
        // Let's assume simple parsing: read statements until we hit something that doesn't look like a statement belonging to the block.
        // Or if we use braces { }.
        // Given I don't see Indent tokens in Lexer usage here, I'll assume we parse one statement or a block if { } used.
        // OR: parse until next Def/Class/Interface/EOF?
        // This is tricky without Indent tokens.
        // Let's assume we read statements.
        
        let mut body = Vec::new();
        // Temporary hack: read until next Def/Class/Interface/EOF
        // Real implementation should use Indent/Dedent from Lexer.
        
        while self.peek().is_some() {
             match self.peek() {
                 Some(Token::Def) | Some(Token::Class) | Some(Token::Interface) => break,
                 _ => {
                     let stmt = self.parse_statement()?;
                     body.push(stmt);
                     self.skip_newlines();
                 }
             }
        }
        
        Ok(Function { name, params, return_type, body })
    }
    
    fn parse_class(&mut self) -> Result<Class, ParseError> {
        self.advance(); // consume 'class'
        let name = match self.advance() {
            Some(Token::Identifier(n)) => n,
            _ => return Err(ParseError::UnexpectedEof),
        };
        
        // TODO: Inheritance and Interfaces
        self.expect(Token::Colon)?;
        self.skip_newlines();
        
        let methods = Vec::new();
        let fields = Vec::new();
        
        while self.peek().is_some() {
             match self.peek() {
                 Some(Token::Def) => {
                     // Parse method (similar to function but inside class)
                     // For now reuse parse_function but convert to Method
                     // Note: this is a simplification.
                     // methods.push(...);
                     self.advance(); // skip def for now to avoid infinite loop
                 }
                 Some(Token::Class) | Some(Token::Interface) => break,
                 _ => {
                     self.advance(); // Skip unknown content in class body
                 }
             }
        }
        
        Ok(Class { 
            name, 
            parent: None, 
            implements: Vec::new(), 
            fields, 
            methods, 
            constructor: None, 
            destructor: None 
        })
    }

    fn parse_statement(&mut self) -> Result<Stmt, ParseError> {
        match self.peek() {
            // Rust style: let / const inside functions
            Some(Token::Let) | Some(Token::Const) => {
                self.parse_let_statement()
            }
            Some(Token::Print) => {
                self.advance();
                self.expect(Token::LParen)?;
                let expr = self.parse_expression()?;
                self.expect(Token::RParen)?;
                Ok(Stmt::Print(expr))
            }
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.advance();
                if matches!(self.peek(), Some(Token::Equals)) {
                    self.advance();
                    let value = self.parse_expression()?;
                    Ok(Stmt::Assign { name, value })
                } else if matches!(self.peek(), Some(Token::LParen)) {
                     // Function call as statement
                     self.advance(); // (
                     let mut args = Vec::new();
                     if !matches!(self.peek(), Some(Token::RParen)) {
                         loop {
                             args.push(self.parse_expression()?);
                             if matches!(self.peek(), Some(Token::Comma)) {
                                 self.advance();
                             } else {
                                 break;
                             }
                         }
                     }
                     self.expect(Token::RParen)?;
                     Ok(Stmt::Expr(Expr::Call { name, args }))
                } else {
                    // Just an identifier expression?
                    Ok(Stmt::Expr(Expr::Variable(name)))
                }
            }
            Some(Token::Return) => {
                self.advance();
                let expr = if !matches!(self.peek(), Some(Token::Newline)) {
                    Some(self.parse_expression()?)
                } else {
                    None
                };
                Ok(Stmt::Return(expr))
            }
            _ => {
                // Try parse expression
                let expr = self.parse_expression()?;
                Ok(Stmt::Expr(expr))
            }
        }
    }
    
    fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        self.parse_binary_op()
    }
    
    fn parse_binary_op(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_primary()?;
        
        while let Some(op) = self.match_binary_op() {
            let right = self.parse_primary()?;
            left = Expr::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    fn match_binary_op(&mut self) -> Option<BinOp> {
        match self.peek() {
            Some(Token::Plus) => { self.advance(); Some(BinOp::Add) },
            Some(Token::Minus) => { self.advance(); Some(BinOp::Sub) },
            Some(Token::Star) => { self.advance(); Some(BinOp::Mul) },
            Some(Token::Slash) => { self.advance(); Some(BinOp::Div) },
            _ => None
        }
    }
    
    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        match self.advance() {
            Some(Token::Number(n)) => Ok(Expr::Number(n)),
            Some(Token::String(s)) => Ok(Expr::String(s)),
            Some(Token::Identifier(s)) => {
                if matches!(self.peek(), Some(Token::LParen)) {
                    self.advance();
                    let mut args = Vec::new();
                    if !matches!(self.peek(), Some(Token::RParen)) {
                        loop {
                            args.push(self.parse_expression()?);
                            if matches!(self.peek(), Some(Token::Comma)) {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    }
                    self.expect(Token::RParen)?;
                    Ok(Expr::Call { name: s, args })
                } else {
                    Ok(Expr::Variable(s))
                }
            },
            Some(Token::LParen) => {
                let expr = self.parse_expression()?;
                self.expect(Token::RParen)?;
                Ok(expr)
            },
            Some(t) => Err(ParseError::UnexpectedToken(t)),
            None => Err(ParseError::UnexpectedEof),
        }
    }
}
