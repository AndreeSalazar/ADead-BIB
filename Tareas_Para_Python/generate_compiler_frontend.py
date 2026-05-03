#!/usr/bin/env python3
"""
Genera el frontend del compilador C99 en Rust
- Lexer con todos los tokens C99
- Parser con AST completo
- Preprocessor básico
"""

from pathlib import Path

COMPILER_PATH = Path(r"C:\Users\andre\OneDrive\Documentos\ADead-BIB\C_Real_Optimo\compiler")

# ============== TOKENS C99 ==============
C99_KEYWORDS = [
    "auto", "break", "case", "char", "const", "continue", "default", "do",
    "double", "else", "enum", "extern", "float", "for", "goto", "if",
    "inline", "int", "long", "register", "restrict", "return", "short",
    "signed", "sizeof", "static", "struct", "switch", "typedef", "union",
    "unsigned", "void", "volatile", "while", "_Bool", "_Complex", "_Imaginary",
]

C99_OPERATORS = [
    ("Plus", "+"), ("Minus", "-"), ("Star", "*"), ("Slash", "/"), ("Percent", "%"),
    ("Ampersand", "&"), ("Pipe", "|"), ("Caret", "^"), ("Tilde", "~"), ("Bang", "!"),
    ("Eq", "="), ("Lt", "<"), ("Gt", ">"), ("Question", "?"), ("Colon", ":"),
    ("Dot", "."), ("Comma", ","), ("Semicolon", ";"),
    ("LParen", "("), ("RParen", ")"), ("LBracket", "["), ("RBracket", "]"),
    ("LBrace", "{"), ("RBrace", "}"),
    # Compound
    ("PlusPlus", "++"), ("MinusMinus", "--"), ("Arrow", "->"),
    ("LtLt", "<<"), ("GtGt", ">>"),
    ("LtEq", "<="), ("GtEq", ">="), ("EqEq", "=="), ("BangEq", "!="),
    ("AmpAmp", "&&"), ("PipePipe", "||"),
    ("PlusEq", "+="), ("MinusEq", "-="), ("StarEq", "*="), ("SlashEq", "/="),
    ("PercentEq", "%="), ("AmpEq", "&="), ("PipeEq", "|="), ("CaretEq", "^="),
    ("LtLtEq", "<<="), ("GtGtEq", ">>="), ("Ellipsis", "..."),
]

def generate_token_rs():
    """Genera token.rs con el enum Token"""
    code = '''//! Tokens C99 - Generado automáticamente
#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
'''
    for kw in C99_KEYWORDS:
        name = f"Kw{kw.replace('_', '').title()}"
        code += f"    {name},\n"
    
    code += "\n    // Operators & Punctuation\n"
    for name, _ in C99_OPERATORS:
        code += f"    {name},\n"
    
    code += '''
    // Literals
    IntLit(i64),
    FloatLit(f64),
    CharLit(char),
    StringLit(String),
    
    // Identifier
    Ident(String),
    
    // Special
    Eof,
    Newline,
    Whitespace,
    Comment,
    
    // Preprocessor
    Hash,
    HashHash,
    Include,
    Define,
    Ifdef,
    Ifndef,
    Endif,
    Undef,
    Pragma,
    Error,
    Line,
}

impl Token {
    pub fn is_keyword(&self) -> bool {
        matches!(self, '''
    
    kw_matches = " | ".join(f"Token::Kw{kw.replace('_', '').title()}" for kw in C99_KEYWORDS)
    code += kw_matches + ")\n    }\n"
    
    code += '''
    pub fn is_literal(&self) -> bool {
        matches!(self, Token::IntLit(_) | Token::FloatLit(_) | Token::CharLit(_) | Token::StringLit(_))
    }
    
    pub fn is_operator(&self) -> bool {
        !self.is_keyword() && !self.is_literal() && !matches!(self, Token::Ident(_) | Token::Eof)
    }
}
'''
    return code

def generate_lexer_rs():
    """Genera lexer.rs"""
    code = '''//! Lexer C99 - Generado automáticamente
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
        if c == '\\n' { self.line += 1; self.col = 1; } else { self.col += 1; }
        Some(c)
    }
    
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() && c != '\\n' { self.advance(); } else { break; }
        }
    }
    
    fn skip_line_comment(&mut self) {
        while let Some(c) = self.peek() {
            if c == '\\n' { break; }
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
            if c == '\\\\' {
                self.advance();
                if let Some(esc) = self.peek() {
                    self.advance();
                    s.push(match esc {
                        'n' => '\\n', 'r' => '\\r', 't' => '\\t', '\\\\' => '\\\\', '"' => '"', '0' => '\\0',
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
        let c = if self.peek() == Some('\\\\') {
            self.advance();
            match self.peek() {
                Some('n') => { self.advance(); '\\n' }
                Some('r') => { self.advance(); '\\r' }
                Some('t') => { self.advance(); '\\t' }
                Some('0') => { self.advance(); '\\0' }
                Some(c) => { self.advance(); c }
                None => '\\0',
            }
        } else {
            let c = self.peek().unwrap_or('\\0');
            self.advance();
            c
        };
        if self.peek() == Some('\\'') { self.advance(); }
        Token::CharLit(c)
    }
    
    fn match_keyword(s: &str) -> Option<Token> {
        match s {
'''
    for kw in C99_KEYWORDS:
        name = f"Kw{kw.replace('_', '').title()}"
        code += f'            "{kw}" => Some(Token::{name}),\n'
    
    code += '''            _ => None,
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
        if c == '\\n' { self.advance(); return Token::Newline; }
        
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
        if c == '\\'' { return self.read_char(); }
        
        // Operators (longest match first)
'''
    
    # Sort operators by length (longest first)
    sorted_ops = sorted(C99_OPERATORS, key=lambda x: -len(x[1]))
    for name, op in sorted_ops:
        escaped = op.replace("\\", "\\\\").replace('"', '\\"')
        if len(op) == 3:
            code += f'        if self.input[self.pos..].starts_with("{escaped}") {{ self.pos += 3; return Token::{name}; }}\n'
        elif len(op) == 2:
            code += f'        if self.input[self.pos..].starts_with("{escaped}") {{ self.pos += 2; return Token::{name}; }}\n'
    
    for name, op in sorted_ops:
        if len(op) == 1:
            escaped = op.replace("\\", "\\\\").replace("'", "\\'")
            code += f"        if c == '{escaped}' {{ self.advance(); return Token::{name}; }}\n"
    
    code += '''
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
'''
    return code

def generate_ast_rs():
    """Genera ast.rs con nodos del AST"""
    return '''//! AST C99 - Generado automáticamente
#![allow(dead_code)]

use std::rc::Rc;

pub type NodeId = usize;

#[derive(Debug, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
}

// ============== TYPES ==============
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Void,
    Bool,
    Char,
    Short,
    Int,
    Long,
    LongLong,
    Float,
    Double,
    Signed(Box<Type>),
    Unsigned(Box<Type>),
    Pointer(Box<Type>),
    Array(Box<Type>, Option<usize>),
    Function(Box<Type>, Vec<Type>),
    Struct(String),
    Union(String),
    Enum(String),
    Typedef(String),
}

impl Type {
    pub fn size(&self) -> usize {
        match self {
            Type::Void => 0,
            Type::Bool | Type::Char => 1,
            Type::Short => 2,
            Type::Int | Type::Float => 4,
            Type::Long | Type::LongLong | Type::Double | Type::Pointer(_) => 8,
            Type::Signed(t) | Type::Unsigned(t) => t.size(),
            Type::Array(t, Some(n)) => t.size() * n,
            _ => 8,
        }
    }
}

// ============== EXPRESSIONS ==============
#[derive(Debug, Clone)]
pub enum Expr {
    // Literals
    IntLit(i64),
    FloatLit(f64),
    CharLit(char),
    StringLit(String),
    
    // Identifiers
    Ident(String),
    
    // Operators
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    Ternary(Box<Expr>, Box<Expr>, Box<Expr>),
    
    // Access
    Index(Box<Expr>, Box<Expr>),
    Member(Box<Expr>, String),
    Arrow(Box<Expr>, String),
    
    // Calls
    Call(Box<Expr>, Vec<Expr>),
    
    // Casts
    Cast(Type, Box<Expr>),
    
    // Sizeof
    SizeofExpr(Box<Expr>),
    SizeofType(Type),
    
    // Compound
    Comma(Vec<Expr>),
    Assign(Box<Expr>, Box<Expr>),
    CompoundAssign(BinOp, Box<Expr>, Box<Expr>),
    
    // Address
    AddrOf(Box<Expr>),
    Deref(Box<Expr>),
    
    // Increment
    PreInc(Box<Expr>),
    PreDec(Box<Expr>),
    PostInc(Box<Expr>),
    PostDec(Box<Expr>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOp {
    Add, Sub, Mul, Div, Mod,
    BitAnd, BitOr, BitXor, Shl, Shr,
    Eq, Ne, Lt, Gt, Le, Ge,
    LogAnd, LogOr,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Neg, Not, BitNot,
}

// ============== STATEMENTS ==============
#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Return(Option<Expr>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    While(Expr, Box<Stmt>),
    DoWhile(Box<Stmt>, Expr),
    For(Option<Box<Stmt>>, Option<Expr>, Option<Expr>, Box<Stmt>),
    Switch(Expr, Vec<SwitchCase>),
    Break,
    Continue,
    Goto(String),
    Label(String, Box<Stmt>),
    Block(Vec<Stmt>),
    Decl(Decl),
    Empty,
}

#[derive(Debug, Clone)]
pub struct SwitchCase {
    pub value: Option<Expr>, // None = default
    pub stmts: Vec<Stmt>,
}

// ============== DECLARATIONS ==============
#[derive(Debug, Clone)]
pub struct Decl {
    pub ty: Type,
    pub name: String,
    pub init: Option<Expr>,
    pub is_static: bool,
    pub is_extern: bool,
    pub is_const: bool,
}

#[derive(Debug, Clone)]
pub struct FuncDecl {
    pub ret_type: Type,
    pub name: String,
    pub params: Vec<Param>,
    pub body: Option<Vec<Stmt>>,
    pub is_inline: bool,
    pub is_static: bool,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub ty: Type,
    pub name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct StructDecl {
    pub name: Option<String>,
    pub fields: Vec<Decl>,
    pub is_union: bool,
}

#[derive(Debug, Clone)]
pub struct EnumDecl {
    pub name: Option<String>,
    pub variants: Vec<(String, Option<i64>)>,
}

#[derive(Debug, Clone)]
pub struct TypedefDecl {
    pub ty: Type,
    pub name: String,
}

// ============== TRANSLATION UNIT ==============
#[derive(Debug, Clone)]
pub enum TopLevel {
    Func(FuncDecl),
    Var(Decl),
    Struct(StructDecl),
    Enum(EnumDecl),
    Typedef(TypedefDecl),
}

#[derive(Debug, Default)]
pub struct TranslationUnit {
    pub items: Vec<TopLevel>,
}

impl TranslationUnit {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
}
'''

def generate_parser_rs():
    """Genera parser.rs"""
    return '''//! Parser C99 - Generado automáticamente
#![allow(dead_code)]
#![allow(unused_variables)]

use super::token::Token;
use super::ast::*;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }
    
    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }
    
    fn peek_n(&self, n: usize) -> &Token {
        self.tokens.get(self.pos + n).unwrap_or(&Token::Eof)
    }
    
    fn advance(&mut self) -> Token {
        let tok = self.peek().clone();
        if !matches!(tok, Token::Eof) { self.pos += 1; }
        tok
    }
    
    fn expect(&mut self, expected: Token) -> Result<Token, String> {
        let tok = self.advance();
        if std::mem::discriminant(&tok) == std::mem::discriminant(&expected) {
            Ok(tok)
        } else {
            Err(format!("Expected {:?}, got {:?}", expected, tok))
        }
    }
    
    fn skip_newlines(&mut self) {
        while matches!(self.peek(), Token::Newline) { self.advance(); }
    }
    
    // ============== PARSING ==============
    
    pub fn parse(&mut self) -> Result<TranslationUnit, String> {
        let mut unit = TranslationUnit::new();
        
        while !matches!(self.peek(), Token::Eof) {
            self.skip_newlines();
            if matches!(self.peek(), Token::Eof) { break; }
            
            match self.parse_top_level() {
                Ok(item) => unit.items.push(item),
                Err(e) => {
                    // Skip to next semicolon or brace
                    while !matches!(self.peek(), Token::Semicolon | Token::RBrace | Token::Eof) {
                        self.advance();
                    }
                    if matches!(self.peek(), Token::Semicolon) { self.advance(); }
                }
            }
        }
        
        Ok(unit)
    }
    
    fn parse_top_level(&mut self) -> Result<TopLevel, String> {
        // Check for typedef
        if matches!(self.peek(), Token::KwTypedef) {
            return self.parse_typedef().map(TopLevel::Typedef);
        }
        
        // Check for struct/union
        if matches!(self.peek(), Token::KwStruct | Token::KwUnion) {
            return self.parse_struct().map(TopLevel::Struct);
        }
        
        // Check for enum
        if matches!(self.peek(), Token::KwEnum) {
            return self.parse_enum().map(TopLevel::Enum);
        }
        
        // Otherwise, declaration or function
        let ty = self.parse_type()?;
        let name = self.parse_ident()?;
        
        if matches!(self.peek(), Token::LParen) {
            // Function
            self.parse_function(ty, name).map(TopLevel::Func)
        } else {
            // Variable
            self.parse_var_decl(ty, name).map(TopLevel::Var)
        }
    }
    
    fn parse_type(&mut self) -> Result<Type, String> {
        let mut ty = match self.peek() {
            Token::KwVoid => { self.advance(); Type::Void }
            Token::KwChar => { self.advance(); Type::Char }
            Token::KwShort => { self.advance(); Type::Short }
            Token::KwInt => { self.advance(); Type::Int }
            Token::KwLong => { 
                self.advance();
                if matches!(self.peek(), Token::KwLong) {
                    self.advance();
                    Type::LongLong
                } else {
                    Type::Long
                }
            }
            Token::KwFloat => { self.advance(); Type::Float }
            Token::KwDouble => { self.advance(); Type::Double }
            Token::KwSigned => {
                self.advance();
                Type::Signed(Box::new(self.parse_type()?))
            }
            Token::KwUnsigned => {
                self.advance();
                if matches!(self.peek(), Token::KwChar | Token::KwShort | Token::KwInt | Token::KwLong) {
                    Type::Unsigned(Box::new(self.parse_type()?))
                } else {
                    Type::Unsigned(Box::new(Type::Int))
                }
            }
            Token::KwStruct => {
                self.advance();
                let name = self.parse_ident()?;
                Type::Struct(name)
            }
            Token::KwUnion => {
                self.advance();
                let name = self.parse_ident()?;
                Type::Union(name)
            }
            Token::KwEnum => {
                self.advance();
                let name = self.parse_ident()?;
                Type::Enum(name)
            }
            Token::Ident(name) => {
                let n = name.clone();
                self.advance();
                Type::Typedef(n)
            }
            _ => return Err(format!("Expected type, got {:?}", self.peek())),
        };
        
        // Pointers
        while matches!(self.peek(), Token::Star) {
            self.advance();
            ty = Type::Pointer(Box::new(ty));
        }
        
        Ok(ty)
    }
    
    fn parse_ident(&mut self) -> Result<String, String> {
        match self.advance() {
            Token::Ident(s) => Ok(s),
            tok => Err(format!("Expected identifier, got {:?}", tok)),
        }
    }
    
    fn parse_function(&mut self, ret_type: Type, name: String) -> Result<FuncDecl, String> {
        self.expect(Token::LParen)?;
        let params = self.parse_params()?;
        self.expect(Token::RParen)?;
        
        let body = if matches!(self.peek(), Token::LBrace) {
            Some(self.parse_block()?)
        } else {
            self.expect(Token::Semicolon)?;
            None
        };
        
        Ok(FuncDecl {
            ret_type,
            name,
            params,
            body,
            is_inline: false,
            is_static: false,
        })
    }
    
    fn parse_params(&mut self) -> Result<Vec<Param>, String> {
        let mut params = Vec::new();
        
        if matches!(self.peek(), Token::RParen) { return Ok(params); }
        if matches!(self.peek(), Token::KwVoid) && matches!(self.peek_n(1), Token::RParen) {
            self.advance();
            return Ok(params);
        }
        
        loop {
            let ty = self.parse_type()?;
            let name = if matches!(self.peek(), Token::Ident(_)) {
                Some(self.parse_ident()?)
            } else {
                None
            };
            params.push(Param { ty, name });
            
            if matches!(self.peek(), Token::Comma) {
                self.advance();
            } else {
                break;
            }
        }
        
        Ok(params)
    }
    
    fn parse_var_decl(&mut self, ty: Type, name: String) -> Result<Decl, String> {
        let init = if matches!(self.peek(), Token::Eq) {
            self.advance();
            Some(self.parse_expr()?)
        } else {
            None
        };
        self.expect(Token::Semicolon)?;
        
        Ok(Decl {
            ty,
            name,
            init,
            is_static: false,
            is_extern: false,
            is_const: false,
        })
    }
    
    fn parse_typedef(&mut self) -> Result<TypedefDecl, String> {
        self.expect(Token::KwTypedef)?;
        let ty = self.parse_type()?;
        let name = self.parse_ident()?;
        self.expect(Token::Semicolon)?;
        Ok(TypedefDecl { ty, name })
    }
    
    fn parse_struct(&mut self) -> Result<StructDecl, String> {
        let is_union = matches!(self.peek(), Token::KwUnion);
        self.advance();
        
        let name = if matches!(self.peek(), Token::Ident(_)) {
            Some(self.parse_ident()?)
        } else {
            None
        };
        
        let fields = if matches!(self.peek(), Token::LBrace) {
            self.advance();
            let mut fields = Vec::new();
            while !matches!(self.peek(), Token::RBrace | Token::Eof) {
                let ty = self.parse_type()?;
                let name = self.parse_ident()?;
                self.expect(Token::Semicolon)?;
                fields.push(Decl { ty, name, init: None, is_static: false, is_extern: false, is_const: false });
            }
            self.expect(Token::RBrace)?;
            fields
        } else {
            Vec::new()
        };
        
        self.expect(Token::Semicolon)?;
        Ok(StructDecl { name, fields, is_union })
    }
    
    fn parse_enum(&mut self) -> Result<EnumDecl, String> {
        self.expect(Token::KwEnum)?;
        
        let name = if matches!(self.peek(), Token::Ident(_)) {
            Some(self.parse_ident()?)
        } else {
            None
        };
        
        let mut variants = Vec::new();
        if matches!(self.peek(), Token::LBrace) {
            self.advance();
            let mut val = 0i64;
            while !matches!(self.peek(), Token::RBrace | Token::Eof) {
                let vname = self.parse_ident()?;
                if matches!(self.peek(), Token::Eq) {
                    self.advance();
                    if let Token::IntLit(v) = self.advance() {
                        val = v;
                    }
                }
                variants.push((vname, Some(val)));
                val += 1;
                if matches!(self.peek(), Token::Comma) { self.advance(); }
            }
            self.expect(Token::RBrace)?;
        }
        
        self.expect(Token::Semicolon)?;
        Ok(EnumDecl { name, variants })
    }
    
    fn parse_block(&mut self) -> Result<Vec<Stmt>, String> {
        self.expect(Token::LBrace)?;
        let mut stmts = Vec::new();
        
        while !matches!(self.peek(), Token::RBrace | Token::Eof) {
            self.skip_newlines();
            if matches!(self.peek(), Token::RBrace) { break; }
            stmts.push(self.parse_stmt()?);
        }
        
        self.expect(Token::RBrace)?;
        Ok(stmts)
    }
    
    fn parse_stmt(&mut self) -> Result<Stmt, String> {
        self.skip_newlines();
        
        match self.peek() {
            Token::KwReturn => {
                self.advance();
                let expr = if !matches!(self.peek(), Token::Semicolon) {
                    Some(self.parse_expr()?)
                } else {
                    None
                };
                self.expect(Token::Semicolon)?;
                Ok(Stmt::Return(expr))
            }
            Token::KwIf => {
                self.advance();
                self.expect(Token::LParen)?;
                let cond = self.parse_expr()?;
                self.expect(Token::RParen)?;
                let then = Box::new(self.parse_stmt()?);
                let else_ = if matches!(self.peek(), Token::KwElse) {
                    self.advance();
                    Some(Box::new(self.parse_stmt()?))
                } else {
                    None
                };
                Ok(Stmt::If(cond, then, else_))
            }
            Token::KwWhile => {
                self.advance();
                self.expect(Token::LParen)?;
                let cond = self.parse_expr()?;
                self.expect(Token::RParen)?;
                let body = Box::new(self.parse_stmt()?);
                Ok(Stmt::While(cond, body))
            }
            Token::KwFor => {
                self.advance();
                self.expect(Token::LParen)?;
                let init = if !matches!(self.peek(), Token::Semicolon) {
                    Some(Box::new(self.parse_stmt()?))
                } else {
                    self.advance();
                    None
                };
                let cond = if !matches!(self.peek(), Token::Semicolon) {
                    Some(self.parse_expr()?)
                } else {
                    None
                };
                self.expect(Token::Semicolon)?;
                let inc = if !matches!(self.peek(), Token::RParen) {
                    Some(self.parse_expr()?)
                } else {
                    None
                };
                self.expect(Token::RParen)?;
                let body = Box::new(self.parse_stmt()?);
                Ok(Stmt::For(init, cond, inc, body))
            }
            Token::KwBreak => { self.advance(); self.expect(Token::Semicolon)?; Ok(Stmt::Break) }
            Token::KwContinue => { self.advance(); self.expect(Token::Semicolon)?; Ok(Stmt::Continue) }
            Token::LBrace => Ok(Stmt::Block(self.parse_block()?)),
            Token::Semicolon => { self.advance(); Ok(Stmt::Empty) }
            _ => {
                // Check for declaration
                if self.is_type_start() {
                    let ty = self.parse_type()?;
                    let name = self.parse_ident()?;
                    let decl = self.parse_var_decl(ty, name)?;
                    Ok(Stmt::Decl(decl))
                } else {
                    let expr = self.parse_expr()?;
                    self.expect(Token::Semicolon)?;
                    Ok(Stmt::Expr(expr))
                }
            }
        }
    }
    
    fn is_type_start(&self) -> bool {
        matches!(self.peek(), 
            Token::KwVoid | Token::KwChar | Token::KwShort | Token::KwInt | Token::KwLong |
            Token::KwFloat | Token::KwDouble | Token::KwSigned | Token::KwUnsigned |
            Token::KwStruct | Token::KwUnion | Token::KwEnum | Token::KwConst
        )
    }
    
    // ============== EXPRESSIONS ==============
    
    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_assignment()
    }
    
    fn parse_assignment(&mut self) -> Result<Expr, String> {
        let lhs = self.parse_ternary()?;
        
        if matches!(self.peek(), Token::Eq) {
            self.advance();
            let rhs = self.parse_assignment()?;
            return Ok(Expr::Assign(Box::new(lhs), Box::new(rhs)));
        }
        
        // Compound assignment
        let op = match self.peek() {
            Token::PlusEq => Some(BinOp::Add),
            Token::MinusEq => Some(BinOp::Sub),
            Token::StarEq => Some(BinOp::Mul),
            Token::SlashEq => Some(BinOp::Div),
            Token::PercentEq => Some(BinOp::Mod),
            Token::AmpEq => Some(BinOp::BitAnd),
            Token::PipeEq => Some(BinOp::BitOr),
            Token::CaretEq => Some(BinOp::BitXor),
            Token::LtLtEq => Some(BinOp::Shl),
            Token::GtGtEq => Some(BinOp::Shr),
            _ => None,
        };
        
        if let Some(binop) = op {
            self.advance();
            let rhs = self.parse_assignment()?;
            return Ok(Expr::CompoundAssign(binop, Box::new(lhs), Box::new(rhs)));
        }
        
        Ok(lhs)
    }
    
    fn parse_ternary(&mut self) -> Result<Expr, String> {
        let cond = self.parse_logical_or()?;
        
        if matches!(self.peek(), Token::Question) {
            self.advance();
            let then = self.parse_expr()?;
            self.expect(Token::Colon)?;
            let else_ = self.parse_ternary()?;
            return Ok(Expr::Ternary(Box::new(cond), Box::new(then), Box::new(else_)));
        }
        
        Ok(cond)
    }
    
    fn parse_logical_or(&mut self) -> Result<Expr, String> {
        let mut lhs = self.parse_logical_and()?;
        while matches!(self.peek(), Token::PipePipe) {
            self.advance();
            let rhs = self.parse_logical_and()?;
            lhs = Expr::Binary(BinOp::LogOr, Box::new(lhs), Box::new(rhs));
        }
        Ok(lhs)
    }
    
    fn parse_logical_and(&mut self) -> Result<Expr, String> {
        let mut lhs = self.parse_bitwise_or()?;
        while matches!(self.peek(), Token::AmpAmp) {
            self.advance();
            let rhs = self.parse_bitwise_or()?;
            lhs = Expr::Binary(BinOp::LogAnd, Box::new(lhs), Box::new(rhs));
        }
        Ok(lhs)
    }
    
    fn parse_bitwise_or(&mut self) -> Result<Expr, String> {
        let mut lhs = self.parse_bitwise_xor()?;
        while matches!(self.peek(), Token::Pipe) {
            self.advance();
            let rhs = self.parse_bitwise_xor()?;
            lhs = Expr::Binary(BinOp::BitOr, Box::new(lhs), Box::new(rhs));
        }
        Ok(lhs)
    }
    
    fn parse_bitwise_xor(&mut self) -> Result<Expr, String> {
        let mut lhs = self.parse_bitwise_and()?;
        while matches!(self.peek(), Token::Caret) {
            self.advance();
            let rhs = self.parse_bitwise_and()?;
            lhs = Expr::Binary(BinOp::BitXor, Box::new(lhs), Box::new(rhs));
        }
        Ok(lhs)
    }
    
    fn parse_bitwise_and(&mut self) -> Result<Expr, String> {
        let mut lhs = self.parse_equality()?;
        while matches!(self.peek(), Token::Ampersand) {
            self.advance();
            let rhs = self.parse_equality()?;
            lhs = Expr::Binary(BinOp::BitAnd, Box::new(lhs), Box::new(rhs));
        }
        Ok(lhs)
    }
    
    fn parse_equality(&mut self) -> Result<Expr, String> {
        let mut lhs = self.parse_relational()?;
        loop {
            let op = match self.peek() {
                Token::EqEq => BinOp::Eq,
                Token::BangEq => BinOp::Ne,
                _ => break,
            };
            self.advance();
            let rhs = self.parse_relational()?;
            lhs = Expr::Binary(op, Box::new(lhs), Box::new(rhs));
        }
        Ok(lhs)
    }
    
    fn parse_relational(&mut self) -> Result<Expr, String> {
        let mut lhs = self.parse_shift()?;
        loop {
            let op = match self.peek() {
                Token::Lt => BinOp::Lt,
                Token::Gt => BinOp::Gt,
                Token::LtEq => BinOp::Le,
                Token::GtEq => BinOp::Ge,
                _ => break,
            };
            self.advance();
            let rhs = self.parse_shift()?;
            lhs = Expr::Binary(op, Box::new(lhs), Box::new(rhs));
        }
        Ok(lhs)
    }
    
    fn parse_shift(&mut self) -> Result<Expr, String> {
        let mut lhs = self.parse_additive()?;
        loop {
            let op = match self.peek() {
                Token::LtLt => BinOp::Shl,
                Token::GtGt => BinOp::Shr,
                _ => break,
            };
            self.advance();
            let rhs = self.parse_additive()?;
            lhs = Expr::Binary(op, Box::new(lhs), Box::new(rhs));
        }
        Ok(lhs)
    }
    
    fn parse_additive(&mut self) -> Result<Expr, String> {
        let mut lhs = self.parse_multiplicative()?;
        loop {
            let op = match self.peek() {
                Token::Plus => BinOp::Add,
                Token::Minus => BinOp::Sub,
                _ => break,
            };
            self.advance();
            let rhs = self.parse_multiplicative()?;
            lhs = Expr::Binary(op, Box::new(lhs), Box::new(rhs));
        }
        Ok(lhs)
    }
    
    fn parse_multiplicative(&mut self) -> Result<Expr, String> {
        let mut lhs = self.parse_unary()?;
        loop {
            let op = match self.peek() {
                Token::Star => BinOp::Mul,
                Token::Slash => BinOp::Div,
                Token::Percent => BinOp::Mod,
                _ => break,
            };
            self.advance();
            let rhs = self.parse_unary()?;
            lhs = Expr::Binary(op, Box::new(lhs), Box::new(rhs));
        }
        Ok(lhs)
    }
    
    fn parse_unary(&mut self) -> Result<Expr, String> {
        match self.peek() {
            Token::Minus => {
                self.advance();
                Ok(Expr::Unary(UnaryOp::Neg, Box::new(self.parse_unary()?)))
            }
            Token::Bang => {
                self.advance();
                Ok(Expr::Unary(UnaryOp::Not, Box::new(self.parse_unary()?)))
            }
            Token::Tilde => {
                self.advance();
                Ok(Expr::Unary(UnaryOp::BitNot, Box::new(self.parse_unary()?)))
            }
            Token::Ampersand => {
                self.advance();
                Ok(Expr::AddrOf(Box::new(self.parse_unary()?)))
            }
            Token::Star => {
                self.advance();
                Ok(Expr::Deref(Box::new(self.parse_unary()?)))
            }
            Token::PlusPlus => {
                self.advance();
                Ok(Expr::PreInc(Box::new(self.parse_unary()?)))
            }
            Token::MinusMinus => {
                self.advance();
                Ok(Expr::PreDec(Box::new(self.parse_unary()?)))
            }
            Token::KwSizeof => {
                self.advance();
                if matches!(self.peek(), Token::LParen) {
                    self.advance();
                    if self.is_type_start() {
                        let ty = self.parse_type()?;
                        self.expect(Token::RParen)?;
                        Ok(Expr::SizeofType(ty))
                    } else {
                        let expr = self.parse_expr()?;
                        self.expect(Token::RParen)?;
                        Ok(Expr::SizeofExpr(Box::new(expr)))
                    }
                } else {
                    Ok(Expr::SizeofExpr(Box::new(self.parse_unary()?)))
                }
            }
            _ => self.parse_postfix(),
        }
    }
    
    fn parse_postfix(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;
        
        loop {
            match self.peek() {
                Token::PlusPlus => {
                    self.advance();
                    expr = Expr::PostInc(Box::new(expr));
                }
                Token::MinusMinus => {
                    self.advance();
                    expr = Expr::PostDec(Box::new(expr));
                }
                Token::LBracket => {
                    self.advance();
                    let idx = self.parse_expr()?;
                    self.expect(Token::RBracket)?;
                    expr = Expr::Index(Box::new(expr), Box::new(idx));
                }
                Token::LParen => {
                    self.advance();
                    let mut args = Vec::new();
                    if !matches!(self.peek(), Token::RParen) {
                        args.push(self.parse_assignment()?);
                        while matches!(self.peek(), Token::Comma) {
                            self.advance();
                            args.push(self.parse_assignment()?);
                        }
                    }
                    self.expect(Token::RParen)?;
                    expr = Expr::Call(Box::new(expr), args);
                }
                Token::Dot => {
                    self.advance();
                    let member = self.parse_ident()?;
                    expr = Expr::Member(Box::new(expr), member);
                }
                Token::Arrow => {
                    self.advance();
                    let member = self.parse_ident()?;
                    expr = Expr::Arrow(Box::new(expr), member);
                }
                _ => break,
            }
        }
        
        Ok(expr)
    }
    
    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.advance() {
            Token::IntLit(n) => Ok(Expr::IntLit(n)),
            Token::FloatLit(f) => Ok(Expr::FloatLit(f)),
            Token::CharLit(c) => Ok(Expr::CharLit(c)),
            Token::StringLit(s) => Ok(Expr::StringLit(s)),
            Token::Ident(s) => Ok(Expr::Ident(s)),
            Token::LParen => {
                let expr = self.parse_expr()?;
                self.expect(Token::RParen)?;
                Ok(expr)
            }
            tok => Err(format!("Unexpected token in expression: {:?}", tok)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::lexer::Lexer;
    
    fn parse(src: &str) -> TranslationUnit {
        let mut lex = Lexer::new(src);
        let tokens = lex.tokenize();
        let mut parser = Parser::new(tokens);
        parser.parse().unwrap()
    }
    
    #[test]
    fn test_simple_func() {
        let unit = parse("int main() { return 0; }");
        assert_eq!(unit.items.len(), 1);
    }
    
    #[test]
    fn test_variable() {
        let unit = parse("int x = 42;");
        assert_eq!(unit.items.len(), 1);
    }
    
    #[test]
    fn test_struct() {
        let unit = parse("struct Point { int x; int y; };");
        assert_eq!(unit.items.len(), 1);
    }
}
'''

def generate_mod_rs():
    """Genera mod.rs del frontend"""
    return '''//! Frontend C99 - Lexer, Parser, AST
//! Generado automáticamente

pub mod token;
pub mod lexer;
pub mod ast;
pub mod parser;

pub use lexer::Lexer;
pub use parser::Parser;
pub use ast::*;
'''

def main():
    print("🔧 Generando frontend del compilador C99...")
    
    frontend_path = COMPILER_PATH / "frontend"
    frontend_path.mkdir(parents=True, exist_ok=True)
    
    # Generate files
    files = [
        ("mod.rs", generate_mod_rs()),
        ("token.rs", generate_token_rs()),
        ("lexer.rs", generate_lexer_rs()),
        ("ast.rs", generate_ast_rs()),
        ("parser.rs", generate_parser_rs()),
    ]
    
    for name, content in files:
        path = frontend_path / name
        path.write_text(content, encoding='utf-8')
        lines = content.count('\n')
        print(f"  ✅ {name:12s} → {lines:4d} líneas")
    
    print(f"\n📁 Frontend generado en: {frontend_path}")
    print(f"   Total: {sum(f[1].count(chr(10)) for f in files)} líneas de Rust")

if __name__ == "__main__":
    main()
