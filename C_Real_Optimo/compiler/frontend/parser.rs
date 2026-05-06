//! Parser C99 - Generado automáticamente
#![allow(dead_code)]
#![allow(unused_variables)]

use super::token::Token;
use super::ast::*;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    /// B-04: typedef registry — nombres reconocidos como tipos
    typedefs: std::collections::HashSet<String>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0, typedefs: std::collections::HashSet::new() }
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
        // Handle array declarations: int arr[5];
        let final_ty = if matches!(self.peek(), Token::LBracket) {
            self.advance(); // consume '['
            let size = if let Token::IntLit(n) = self.peek().clone() {
                self.advance();
                Some(n as usize)
            } else {
                None
            };
            self.expect(Token::RBracket)?;
            Type::Array(Box::new(ty), size)
        } else {
            ty
        };

        let init = if matches!(self.peek(), Token::Eq) {
            self.advance();
            Some(self.parse_expr()?)
        } else {
            None
        };
        self.expect(Token::Semicolon)?;
        
        Ok(Decl {
            ty: final_ty,
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
        // B-04: registrar nombre para que is_type_start lo reconozca
        self.typedefs.insert(name.clone());
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
                self.skip_newlines();
                if matches!(self.peek(), Token::RBrace) { break; }
                let ty = self.parse_type()?;
                let name = self.parse_ident()?;
                // Handle array fields: int data[10];
                let final_ty = if matches!(self.peek(), Token::LBracket) {
                    self.advance();
                    let size = if let Token::IntLit(n) = self.peek().clone() {
                        self.advance();
                        Some(n as usize)
                    } else {
                        None
                    };
                    self.expect(Token::RBracket)?;
                    Type::Array(Box::new(ty), size)
                } else {
                    ty
                };
                self.expect(Token::Semicolon)?;
                fields.push(Decl { ty: final_ty, name, init: None, is_static: false, is_extern: false, is_const: false });
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
            // ============================================================
            // B-04: do-while
            // ============================================================
            Token::KwDo => {
                self.advance(); // consume 'do'
                let body = Box::new(self.parse_stmt()?);
                self.expect(Token::KwWhile)?;
                self.expect(Token::LParen)?;
                let cond = self.parse_expr()?;
                self.expect(Token::RParen)?;
                self.expect(Token::Semicolon)?;
                Ok(Stmt::DoWhile(body, cond))
            }
            // ============================================================
            // B-04: switch / case / default
            // ============================================================
            Token::KwSwitch => {
                self.advance(); // consume 'switch'
                self.expect(Token::LParen)?;
                let scrutinee = self.parse_expr()?;
                self.expect(Token::RParen)?;
                self.expect(Token::LBrace)?;
                let mut cases: Vec<SwitchCase> = Vec::new();
                while !matches!(self.peek(), Token::RBrace | Token::Eof) {
                    match self.peek() {
                        Token::KwCase => {
                            self.advance();
                            let val = self.parse_expr()?;
                            self.expect(Token::Colon)?;
                            let mut stmts: Vec<Stmt> = Vec::new();
                            while !matches!(self.peek(), Token::KwCase | Token::KwDefault | Token::RBrace | Token::Eof) {
                                stmts.push(self.parse_stmt()?);
                            }
                            cases.push(SwitchCase { value: Some(val), stmts });
                        }
                        Token::KwDefault => {
                            self.advance();
                            self.expect(Token::Colon)?;
                            let mut stmts: Vec<Stmt> = Vec::new();
                            while !matches!(self.peek(), Token::KwCase | Token::KwDefault | Token::RBrace | Token::Eof) {
                                stmts.push(self.parse_stmt()?);
                            }
                            cases.push(SwitchCase { value: None, stmts });
                        }
                        _ => { self.advance(); } // skip unexpected tokens
                    }
                }
                self.expect(Token::RBrace)?;
                Ok(Stmt::Switch(scrutinee, cases))
            }
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
        if matches!(self.peek(), 
            Token::KwVoid | Token::KwChar | Token::KwShort | Token::KwInt | Token::KwLong |
            Token::KwFloat | Token::KwDouble | Token::KwSigned | Token::KwUnsigned |
            Token::KwStruct | Token::KwUnion | Token::KwEnum | Token::KwConst
        ) {
            return true;
        }
        // B-04: identificador conocido como typedef
        if let Token::Ident(name) = self.peek() {
            return self.typedefs.contains(name);
        }
        false
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
