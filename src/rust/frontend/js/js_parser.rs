// ============================================================
// JsDead-BIB — JavaScript Parser
// ============================================================
// Recursive descent parser: JsToken stream → JsProgram AST
// Implícitamente estricto — semicolons obligatorios (no ASI)
// Sin Node.js. Sin V8. Sin runtime. JS → ASM directo.
// ============================================================

use super::js_ast::*;
use super::js_lexer::JsToken;

pub struct JsParser {
    tokens: Vec<JsToken>,
    lines: Vec<usize>,
    pos: usize,
}

impl JsParser {
    pub fn new(tokens: Vec<JsToken>, lines: Vec<usize>) -> Self {
        Self {
            tokens,
            lines,
            pos: 0,
        }
    }

    // ── Helpers ──────────────────────────────────────────────

    fn peek(&self) -> &JsToken {
        self.tokens.get(self.pos).unwrap_or(&JsToken::EOF)
    }

    fn peek_at(&self, offset: usize) -> &JsToken {
        self.tokens.get(self.pos + offset).unwrap_or(&JsToken::EOF)
    }

    fn advance(&mut self) -> JsToken {
        let tok = self.tokens.get(self.pos).cloned().unwrap_or(JsToken::EOF);
        self.pos += 1;
        tok
    }

    fn expect(&mut self, expected: &JsToken) -> Result<(), String> {
        let tok = self.advance();
        if std::mem::discriminant(&tok) == std::mem::discriminant(expected) {
            Ok(())
        } else {
            Err(format!(
                "JS parse error line {}: expected {:?}, got {:?}",
                self.current_line(),
                expected,
                tok
            ))
        }
    }

    fn expect_semicolon(&mut self) -> Result<(), String> {
        if *self.peek() == JsToken::Semicolon {
            self.advance();
            Ok(())
        } else {
            // Lenient: don't error on missing semicolons before } or EOF
            if matches!(self.peek(), JsToken::RBrace | JsToken::EOF) {
                Ok(())
            } else {
                Err(format!(
                    "JS parse error line {}: expected ';', got {:?}",
                    self.current_line(),
                    self.peek()
                ))
            }
        }
    }

    fn expect_ident(&mut self) -> Result<String, String> {
        match self.advance() {
            JsToken::Identifier(s) => Ok(s),
            other => Err(format!(
                "JS parse error line {}: expected identifier, got {:?}",
                self.current_line(),
                other
            )),
        }
    }

    fn current_line(&self) -> usize {
        self.lines.get(self.pos.saturating_sub(1)).copied().unwrap_or(1)
    }

    // ── Entry point ─────────────────────────────────────────

    pub fn parse(&mut self) -> Result<JsProgram, String> {
        let mut program = JsProgram::new();
        while *self.peek() != JsToken::EOF {
            let stmt = self.parse_stmt()?;
            program.stmts.push(stmt);
        }
        Ok(program)
    }

    // ── Statements ──────────────────────────────────────────

    fn parse_stmt(&mut self) -> Result<JsStmt, String> {
        match self.peek().clone() {
            JsToken::Let | JsToken::Const | JsToken::Var => self.parse_var_decl(),
            JsToken::Function => self.parse_function_decl(),
            JsToken::Async => {
                // async function declaration
                if *self.peek_at(1) == JsToken::Function {
                    self.parse_function_decl()
                } else {
                    self.parse_expr_stmt()
                }
            }
            JsToken::Class => self.parse_class_decl(),
            JsToken::If => self.parse_if(),
            JsToken::For => self.parse_for(),
            JsToken::While => self.parse_while(),
            JsToken::Do => self.parse_do_while(),
            JsToken::Return => self.parse_return(),
            JsToken::Break => self.parse_break(),
            JsToken::Continue => self.parse_continue(),
            JsToken::Throw => self.parse_throw(),
            JsToken::Try => self.parse_try_catch(),
            JsToken::Switch => self.parse_switch(),
            JsToken::Import => self.parse_import(),
            JsToken::Export => self.parse_export(),
            JsToken::LBrace => self.parse_block(),
            JsToken::Semicolon => {
                self.advance();
                Ok(JsStmt::Empty)
            }
            _ => self.parse_expr_stmt(),
        }
    }

    fn parse_var_decl(&mut self) -> Result<JsStmt, String> {
        let kind = match self.advance() {
            JsToken::Let => DeclKind::Let,
            JsToken::Const => DeclKind::Const,
            JsToken::Var => DeclKind::Var,
            _ => return Err("Expected let/const/var".into()),
        };

        let name = self.expect_ident()?;

        // Optional type annotation: let x: int = 5
        let type_ann = if *self.peek() == JsToken::Colon {
            self.advance();
            Some(self.parse_type_annotation()?)
        } else {
            None
        };

        // Optional initializer
        let init = if *self.peek() == JsToken::Eq {
            self.advance();
            Some(self.parse_expr()?)
        } else {
            None
        };

        self.expect_semicolon()?;
        Ok(JsStmt::VarDecl {
            kind,
            name,
            type_ann,
            init,
        })
    }

    fn parse_type_annotation(&mut self) -> Result<JsType, String> {
        match self.peek().clone() {
            JsToken::Identifier(ref s) => {
                let type_name = s.clone();
                self.advance();
                // Check for array type: int[], float[], string[]
                if *self.peek() == JsToken::LBracket && *self.peek_at(1) == JsToken::RBracket {
                    self.advance(); // [
                    self.advance(); // ]
                    let inner = match type_name.as_str() {
                        "int" => JsType::Int,
                        "float" => JsType::Float,
                        "number" => JsType::Number,
                        "string" => JsType::String,
                        "boolean" | "bool" => JsType::Boolean,
                        _ => JsType::Named(type_name),
                    };
                    return Ok(JsType::Array(Box::new(inner)));
                }
                match type_name.as_str() {
                    "int" => Ok(JsType::Int),
                    "float" | "double" => Ok(JsType::Float),
                    "number" => Ok(JsType::Number),
                    "string" | "str" => Ok(JsType::String),
                    "boolean" | "bool" => Ok(JsType::Boolean),
                    "void" => Ok(JsType::Void),
                    "null" => Ok(JsType::Null),
                    "undefined" => Ok(JsType::Undefined),
                    _ => Ok(JsType::Named(type_name)),
                }
            }
            JsToken::Void => {
                self.advance();
                Ok(JsType::Void)
            }
            _ => Ok(JsType::Inferred),
        }
    }

    fn parse_function_decl(&mut self) -> Result<JsStmt, String> {
        let is_async = if *self.peek() == JsToken::Async {
            self.advance();
            true
        } else {
            false
        };
        self.expect(&JsToken::Function)?;
        let name = self.expect_ident()?;
        let params = self.parse_params()?;

        // Optional return type: function foo(): int { ... }
        let return_type = if *self.peek() == JsToken::Colon {
            self.advance();
            Some(self.parse_type_annotation()?)
        } else {
            None
        };

        let body = self.parse_block_body()?;
        Ok(JsStmt::FuncDecl {
            name,
            params,
            return_type,
            body,
            is_async,
        })
    }

    fn parse_params(&mut self) -> Result<Vec<JsParam>, String> {
        self.expect(&JsToken::LParen)?;
        let mut params = Vec::new();
        while *self.peek() != JsToken::RParen && *self.peek() != JsToken::EOF {
            let is_rest = if *self.peek() == JsToken::DotDotDot {
                self.advance();
                true
            } else {
                false
            };
            let name = self.expect_ident()?;
            let type_ann = if *self.peek() == JsToken::Colon {
                self.advance();
                Some(self.parse_type_annotation()?)
            } else {
                None
            };
            let default = if *self.peek() == JsToken::Eq {
                self.advance();
                Some(self.parse_assignment_expr()?)
            } else {
                None
            };
            params.push(JsParam {
                name,
                type_ann,
                default,
                is_rest,
            });
            if *self.peek() == JsToken::Comma {
                self.advance();
            }
        }
        self.expect(&JsToken::RParen)?;
        Ok(params)
    }

    fn parse_class_decl(&mut self) -> Result<JsStmt, String> {
        self.expect(&JsToken::Class)?;
        let name = self.expect_ident()?;
        let super_class = if *self.peek() == JsToken::Extends {
            self.advance();
            Some(self.expect_ident()?)
        } else {
            None
        };
        self.expect(&JsToken::LBrace)?;
        let mut body = Vec::new();
        while *self.peek() != JsToken::RBrace && *self.peek() != JsToken::EOF {
            body.push(self.parse_class_member()?);
        }
        self.expect(&JsToken::RBrace)?;
        Ok(JsStmt::ClassDecl {
            name,
            super_class,
            body,
        })
    }

    fn parse_class_member(&mut self) -> Result<JsClassMember, String> {
        let is_static = if *self.peek() == JsToken::Static {
            self.advance();
            true
        } else {
            false
        };

        let is_async = if *self.peek() == JsToken::Async {
            self.advance();
            true
        } else {
            false
        };

        // Constructor
        if let JsToken::Identifier(ref s) = self.peek().clone() {
            if s == "constructor" {
                self.advance();
                let params = self.parse_params()?;
                let body = self.parse_block_body()?;
                return Ok(JsClassMember::Constructor { params, body });
            }
        }

        // Getter
        if *self.peek() == JsToken::Get {
            let next = self.peek_at(1);
            if let JsToken::Identifier(_) = next {
                self.advance();
                let name = self.expect_ident()?;
                self.expect(&JsToken::LParen)?;
                self.expect(&JsToken::RParen)?;
                let body = self.parse_block_body()?;
                return Ok(JsClassMember::Getter { name, body });
            }
        }

        // Setter
        if *self.peek() == JsToken::Set {
            let next = self.peek_at(1);
            if let JsToken::Identifier(_) = next {
                self.advance();
                let name = self.expect_ident()?;
                self.expect(&JsToken::LParen)?;
                let param = self.expect_ident()?;
                self.expect(&JsToken::RParen)?;
                let body = self.parse_block_body()?;
                return Ok(JsClassMember::Setter { name, param, body });
            }
        }

        // Method or property
        let name = self.expect_ident()?;

        if *self.peek() == JsToken::LParen {
            // Method
            let params = self.parse_params()?;
            let return_type = if *self.peek() == JsToken::Colon {
                self.advance();
                Some(self.parse_type_annotation()?)
            } else {
                None
            };
            let body = self.parse_block_body()?;
            Ok(JsClassMember::Method {
                name,
                params,
                return_type,
                body,
                is_static,
                is_async,
            })
        } else {
            // Property
            let type_ann = if *self.peek() == JsToken::Colon {
                self.advance();
                Some(self.parse_type_annotation()?)
            } else {
                None
            };
            let init = if *self.peek() == JsToken::Eq {
                self.advance();
                Some(self.parse_expr()?)
            } else {
                None
            };
            // Optional semicolon after property
            if *self.peek() == JsToken::Semicolon {
                self.advance();
            }
            Ok(JsClassMember::Property {
                name,
                type_ann,
                init,
                is_static,
            })
        }
    }

    fn parse_if(&mut self) -> Result<JsStmt, String> {
        self.expect(&JsToken::If)?;
        self.expect(&JsToken::LParen)?;
        let cond = self.parse_expr()?;
        self.expect(&JsToken::RParen)?;
        let then_body = Box::new(self.parse_stmt()?);
        let else_body = if *self.peek() == JsToken::Else {
            self.advance();
            Some(Box::new(self.parse_stmt()?))
        } else {
            None
        };
        Ok(JsStmt::If {
            cond,
            then_body,
            else_body,
        })
    }

    fn parse_for(&mut self) -> Result<JsStmt, String> {
        self.expect(&JsToken::For)?;
        self.expect(&JsToken::LParen)?;

        // Check for for-of / for-in
        if matches!(self.peek(), JsToken::Let | JsToken::Const | JsToken::Var) {
            let saved = self.pos;
            let _kind = self.advance();
            if let JsToken::Identifier(name) = self.peek().clone() {
                self.advance();
                if *self.peek() == JsToken::Identifier("of".into()) || *self.peek() == JsToken::Of
                {
                    self.advance();
                    let iter = self.parse_expr()?;
                    self.expect(&JsToken::RParen)?;
                    let body = Box::new(self.parse_stmt()?);
                    return Ok(JsStmt::ForOf {
                        decl: name,
                        iter,
                        body,
                    });
                }
                if *self.peek() == JsToken::In {
                    self.advance();
                    let iter = self.parse_expr()?;
                    self.expect(&JsToken::RParen)?;
                    let body = Box::new(self.parse_stmt()?);
                    return Ok(JsStmt::ForIn {
                        decl: name,
                        iter,
                        body,
                    });
                }
            }
            // Not for-of/for-in, restore position
            self.pos = saved;
        }

        // Standard for loop
        let init = if *self.peek() == JsToken::Semicolon {
            None
        } else {
            Some(Box::new(self.parse_stmt()?))
        };
        // init already consumed its semicolon via parse_stmt→parse_var_decl or parse_expr_stmt
        // But if init was None, we need to consume the first semicolon
        if init.is_none() {
            self.expect(&JsToken::Semicolon)?;
        }

        let cond = if *self.peek() == JsToken::Semicolon {
            None
        } else {
            Some(self.parse_expr()?)
        };
        self.expect(&JsToken::Semicolon)?;

        let update = if *self.peek() == JsToken::RParen {
            None
        } else {
            Some(self.parse_expr()?)
        };
        self.expect(&JsToken::RParen)?;

        let body = Box::new(self.parse_stmt()?);
        Ok(JsStmt::For {
            init,
            cond,
            update,
            body,
        })
    }

    fn parse_while(&mut self) -> Result<JsStmt, String> {
        self.expect(&JsToken::While)?;
        self.expect(&JsToken::LParen)?;
        let cond = self.parse_expr()?;
        self.expect(&JsToken::RParen)?;
        let body = Box::new(self.parse_stmt()?);
        Ok(JsStmt::While { cond, body })
    }

    fn parse_do_while(&mut self) -> Result<JsStmt, String> {
        self.expect(&JsToken::Do)?;
        let body = Box::new(self.parse_stmt()?);
        self.expect(&JsToken::While)?;
        self.expect(&JsToken::LParen)?;
        let cond = self.parse_expr()?;
        self.expect(&JsToken::RParen)?;
        self.expect_semicolon()?;
        Ok(JsStmt::DoWhile { body, cond })
    }

    fn parse_return(&mut self) -> Result<JsStmt, String> {
        self.expect(&JsToken::Return)?;
        let value = if *self.peek() == JsToken::Semicolon
            || *self.peek() == JsToken::RBrace
            || *self.peek() == JsToken::EOF
        {
            None
        } else {
            Some(self.parse_expr()?)
        };
        self.expect_semicolon()?;
        Ok(JsStmt::Return(value))
    }

    fn parse_break(&mut self) -> Result<JsStmt, String> {
        self.expect(&JsToken::Break)?;
        let label = if let JsToken::Identifier(s) = self.peek().clone() {
            self.advance();
            Some(s)
        } else {
            None
        };
        self.expect_semicolon()?;
        Ok(JsStmt::Break(label))
    }

    fn parse_continue(&mut self) -> Result<JsStmt, String> {
        self.expect(&JsToken::Continue)?;
        let label = if let JsToken::Identifier(s) = self.peek().clone() {
            self.advance();
            Some(s)
        } else {
            None
        };
        self.expect_semicolon()?;
        Ok(JsStmt::Continue(label))
    }

    fn parse_throw(&mut self) -> Result<JsStmt, String> {
        self.expect(&JsToken::Throw)?;
        let expr = self.parse_expr()?;
        self.expect_semicolon()?;
        Ok(JsStmt::Throw(expr))
    }

    fn parse_try_catch(&mut self) -> Result<JsStmt, String> {
        self.expect(&JsToken::Try)?;
        let try_body = self.parse_block_body()?;

        let (catch_param, catch_body) = if *self.peek() == JsToken::Catch {
            self.advance();
            let param = if *self.peek() == JsToken::LParen {
                self.advance();
                let p = self.expect_ident()?;
                self.expect(&JsToken::RParen)?;
                Some(p)
            } else {
                None
            };
            let body = self.parse_block_body()?;
            (param, Some(body))
        } else {
            (None, None)
        };

        let finally_body = if *self.peek() == JsToken::Finally {
            self.advance();
            Some(self.parse_block_body()?)
        } else {
            None
        };

        Ok(JsStmt::TryCatch {
            try_body,
            catch_param,
            catch_body,
            finally_body,
        })
    }

    fn parse_import(&mut self) -> Result<JsStmt, String> {
        self.expect(&JsToken::Import)?;

        // import { a, b, c } from "module"
        // import name from "module"  
        // import "module"
        if *self.peek() == JsToken::LBrace {
            // Named imports: import { a, b } from "module"
            self.advance(); // {
            let mut items = Vec::new();
            while *self.peek() != JsToken::RBrace && *self.peek() != JsToken::EOF {
                let name = self.expect_ident()?;
                let alias = if let JsToken::Identifier(ref s) = self.peek().clone() {
                    if s == "as" {
                        self.advance(); // as
                        Some(self.expect_ident()?)
                    } else {
                        None
                    }
                } else {
                    None
                };
                items.push(JsImportItem::Named { name, alias });
                if *self.peek() == JsToken::Comma {
                    self.advance();
                }
            }
            self.expect(&JsToken::RBrace)?;

            // expect "from"
            if let JsToken::From = self.peek() {
                self.advance();
            } else if let JsToken::Identifier(ref s) = self.peek().clone() {
                if s == "from" {
                    self.advance();
                } else {
                    return Err(format!(
                        "JS parse error line {}: expected 'from', got {:?}",
                        self.current_line(),
                        self.peek()
                    ));
                }
            }

            let module = match self.advance() {
                JsToken::StringLiteral(s) => s,
                other => {
                    return Err(format!(
                        "JS parse error line {}: expected module string, got {:?}",
                        self.current_line(),
                        other
                    ))
                }
            };
            self.expect_semicolon()?;
            Ok(JsStmt::Import {
                items,
                from: module,
            })
        } else if let JsToken::StringLiteral(module) = self.peek().clone() {
            // Side-effect import: import "module"
            self.advance();
            self.expect_semicolon()?;
            Ok(JsStmt::Import {
                items: Vec::new(),
                from: module,
            })
        } else {
            // Default import: import name from "module"
            let name = self.expect_ident()?;
            if let JsToken::From = self.peek() {
                self.advance();
            } else if let JsToken::Identifier(ref s) = self.peek().clone() {
                if s == "from" {
                    self.advance();
                }
            }
            let module = match self.advance() {
                JsToken::StringLiteral(s) => s,
                other => {
                    return Err(format!(
                        "JS parse error line {}: expected module string, got {:?}",
                        self.current_line(),
                        other
                    ))
                }
            };
            self.expect_semicolon()?;
            Ok(JsStmt::Import {
                items: vec![JsImportItem::Default(name)],
                from: module,
            })
        }
    }

    fn parse_export(&mut self) -> Result<JsStmt, String> {
        self.expect(&JsToken::Export)?;

        // export default expr
        if *self.peek() == JsToken::Default {
            self.advance();
            let expr = self.parse_expr()?;
            self.expect_semicolon()?;
            return Ok(JsStmt::ExportDefault(expr));
        }

        // export function/class/let/const
        let stmt = self.parse_stmt()?;
        Ok(JsStmt::Export {
            item: Box::new(stmt),
        })
    }

    fn parse_switch(&mut self) -> Result<JsStmt, String> {
        self.expect(&JsToken::Switch)?;
        self.expect(&JsToken::LParen)?;
        let expr = self.parse_expr()?;
        self.expect(&JsToken::RParen)?;
        self.expect(&JsToken::LBrace)?;

        let mut cases = Vec::new();
        while *self.peek() != JsToken::RBrace && *self.peek() != JsToken::EOF {
            let test = if *self.peek() == JsToken::Case {
                self.advance();
                let e = self.parse_expr()?;
                self.expect(&JsToken::Colon)?;
                Some(e)
            } else if *self.peek() == JsToken::Default {
                self.advance();
                self.expect(&JsToken::Colon)?;
                None
            } else {
                return Err(format!(
                    "JS parse error line {}: expected 'case' or 'default'",
                    self.current_line()
                ));
            };

            let mut body = Vec::new();
            while !matches!(
                self.peek(),
                JsToken::Case | JsToken::Default | JsToken::RBrace | JsToken::EOF
            ) {
                body.push(self.parse_stmt()?);
            }
            cases.push(JsCase { test, body });
        }
        self.expect(&JsToken::RBrace)?;
        Ok(JsStmt::Switch { expr, cases })
    }

    fn parse_block(&mut self) -> Result<JsStmt, String> {
        let body = self.parse_block_body()?;
        Ok(JsStmt::Block(body))
    }

    fn parse_block_body(&mut self) -> Result<Vec<JsStmt>, String> {
        self.expect(&JsToken::LBrace)?;
        let mut stmts = Vec::new();
        while *self.peek() != JsToken::RBrace && *self.peek() != JsToken::EOF {
            stmts.push(self.parse_stmt()?);
        }
        self.expect(&JsToken::RBrace)?;
        Ok(stmts)
    }

    fn parse_expr_stmt(&mut self) -> Result<JsStmt, String> {
        let expr = self.parse_expr()?;
        self.expect_semicolon()?;
        Ok(JsStmt::Expr(expr))
    }

    // ── Expressions (Pratt-style precedence) ────────────────

    fn parse_expr(&mut self) -> Result<JsExpr, String> {
        self.parse_assignment_expr()
    }

    fn parse_assignment_expr(&mut self) -> Result<JsExpr, String> {
        let expr = self.parse_ternary()?;

        // Assignment operators
        let op = match self.peek() {
            JsToken::Eq => Some(JsAssignOp::Eq),
            JsToken::PlusEq => Some(JsAssignOp::Add),
            JsToken::MinusEq => Some(JsAssignOp::Sub),
            JsToken::StarEq => Some(JsAssignOp::Mul),
            JsToken::SlashEq => Some(JsAssignOp::Div),
            JsToken::PercentEq => Some(JsAssignOp::Mod),
            _ => None,
        };

        if let Some(op) = op {
            self.advance();
            let value = self.parse_assignment_expr()?;
            Ok(JsExpr::Assign {
                target: Box::new(expr),
                op,
                value: Box::new(value),
            })
        } else {
            Ok(expr)
        }
    }

    fn parse_ternary(&mut self) -> Result<JsExpr, String> {
        let expr = self.parse_nullish()?;
        if *self.peek() == JsToken::Question {
            self.advance();
            let then_expr = self.parse_assignment_expr()?;
            self.expect(&JsToken::Colon)?;
            let else_expr = self.parse_assignment_expr()?;
            Ok(JsExpr::Ternary {
                cond: Box::new(expr),
                then_expr: Box::new(then_expr),
                else_expr: Box::new(else_expr),
            })
        } else {
            Ok(expr)
        }
    }

    fn parse_nullish(&mut self) -> Result<JsExpr, String> {
        let mut left = self.parse_or()?;
        while *self.peek() == JsToken::QuestionQuestion {
            self.advance();
            let right = self.parse_or()?;
            left = JsExpr::BinaryOp {
                op: JsBinOp::Nullish,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_or(&mut self) -> Result<JsExpr, String> {
        let mut left = self.parse_and()?;
        while *self.peek() == JsToken::PipePipe {
            self.advance();
            let right = self.parse_and()?;
            left = JsExpr::BinaryOp {
                op: JsBinOp::Or,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> Result<JsExpr, String> {
        let mut left = self.parse_bitwise_or()?;
        while *self.peek() == JsToken::AmpAmp {
            self.advance();
            let right = self.parse_bitwise_or()?;
            left = JsExpr::BinaryOp {
                op: JsBinOp::And,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_bitwise_or(&mut self) -> Result<JsExpr, String> {
        let mut left = self.parse_bitwise_xor()?;
        while *self.peek() == JsToken::Pipe {
            self.advance();
            let right = self.parse_bitwise_xor()?;
            left = JsExpr::BinaryOp {
                op: JsBinOp::BitOr,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_bitwise_xor(&mut self) -> Result<JsExpr, String> {
        let mut left = self.parse_bitwise_and()?;
        while *self.peek() == JsToken::Caret {
            self.advance();
            let right = self.parse_bitwise_and()?;
            left = JsExpr::BinaryOp {
                op: JsBinOp::BitXor,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_bitwise_and(&mut self) -> Result<JsExpr, String> {
        let mut left = self.parse_equality()?;
        while *self.peek() == JsToken::Amp {
            self.advance();
            let right = self.parse_equality()?;
            left = JsExpr::BinaryOp {
                op: JsBinOp::BitAnd,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<JsExpr, String> {
        let mut left = self.parse_comparison()?;
        loop {
            let op = match self.peek() {
                JsToken::EqEqEq => JsBinOp::EqStrict,
                JsToken::BangEqEq => JsBinOp::NeStrict,
                JsToken::EqEq => JsBinOp::Eq,
                JsToken::BangEq => JsBinOp::Ne,
                _ => break,
            };
            self.advance();
            let right = self.parse_comparison()?;
            left = JsExpr::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<JsExpr, String> {
        let mut left = self.parse_shift()?;
        loop {
            let op = match self.peek() {
                JsToken::Lt => JsBinOp::Lt,
                JsToken::Gt => JsBinOp::Gt,
                JsToken::LtEq => JsBinOp::Le,
                JsToken::GtEq => JsBinOp::Ge,
                JsToken::Instanceof => JsBinOp::Instanceof,
                JsToken::In => JsBinOp::In,
                _ => break,
            };
            self.advance();
            let right = self.parse_shift()?;
            left = JsExpr::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_shift(&mut self) -> Result<JsExpr, String> {
        let mut left = self.parse_additive()?;
        loop {
            let op = match self.peek() {
                JsToken::LtLt => JsBinOp::Shl,
                JsToken::GtGt => JsBinOp::Shr,
                JsToken::GtGtGt => JsBinOp::UShr,
                _ => break,
            };
            self.advance();
            let right = self.parse_additive()?;
            left = JsExpr::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_additive(&mut self) -> Result<JsExpr, String> {
        let mut left = self.parse_multiplicative()?;
        loop {
            let op = match self.peek() {
                JsToken::Plus => JsBinOp::Add,
                JsToken::Minus => JsBinOp::Sub,
                _ => break,
            };
            self.advance();
            let right = self.parse_multiplicative()?;
            left = JsExpr::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<JsExpr, String> {
        let mut left = self.parse_exponent()?;
        loop {
            let op = match self.peek() {
                JsToken::Star => JsBinOp::Mul,
                JsToken::Slash => JsBinOp::Div,
                JsToken::Percent => JsBinOp::Mod,
                _ => break,
            };
            self.advance();
            let right = self.parse_exponent()?;
            left = JsExpr::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_exponent(&mut self) -> Result<JsExpr, String> {
        let base = self.parse_unary()?;
        if *self.peek() == JsToken::StarStar {
            self.advance();
            let exp = self.parse_exponent()?; // right-associative
            Ok(JsExpr::BinaryOp {
                op: JsBinOp::Pow,
                left: Box::new(base),
                right: Box::new(exp),
            })
        } else {
            Ok(base)
        }
    }

    fn parse_unary(&mut self) -> Result<JsExpr, String> {
        match self.peek().clone() {
            JsToken::Minus => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(JsExpr::UnaryOp {
                    op: JsUnOp::Neg,
                    expr: Box::new(expr),
                })
            }
            JsToken::Bang => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(JsExpr::UnaryOp {
                    op: JsUnOp::Not,
                    expr: Box::new(expr),
                })
            }
            JsToken::Tilde => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(JsExpr::UnaryOp {
                    op: JsUnOp::BitNot,
                    expr: Box::new(expr),
                })
            }
            JsToken::Typeof => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(JsExpr::Typeof(Box::new(expr)))
            }
            JsToken::Void => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(JsExpr::VoidExpr(Box::new(expr)))
            }
            JsToken::Delete => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(JsExpr::UnaryOp {
                    op: JsUnOp::Delete,
                    expr: Box::new(expr),
                })
            }
            JsToken::PlusPlus => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(JsExpr::UnaryOp {
                    op: JsUnOp::PreInc,
                    expr: Box::new(expr),
                })
            }
            JsToken::MinusMinus => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(JsExpr::UnaryOp {
                    op: JsUnOp::PreDec,
                    expr: Box::new(expr),
                })
            }
            JsToken::Await => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(JsExpr::Await(Box::new(expr)))
            }
            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> Result<JsExpr, String> {
        let mut expr = self.parse_call_member()?;
        loop {
            match self.peek() {
                JsToken::PlusPlus => {
                    self.advance();
                    expr = JsExpr::UnaryOp {
                        op: JsUnOp::PostInc,
                        expr: Box::new(expr),
                    };
                }
                JsToken::MinusMinus => {
                    self.advance();
                    expr = JsExpr::UnaryOp {
                        op: JsUnOp::PostDec,
                        expr: Box::new(expr),
                    };
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_call_member(&mut self) -> Result<JsExpr, String> {
        let mut expr = self.parse_primary()?;
        loop {
            match self.peek().clone() {
                JsToken::Dot => {
                    self.advance();
                    let prop = self.expect_ident()?;
                    expr = JsExpr::MemberAccess {
                        object: Box::new(expr),
                        property: prop,
                        optional: false,
                    };
                }
                JsToken::OptionalChain => {
                    self.advance();
                    let prop = self.expect_ident()?;
                    expr = JsExpr::MemberAccess {
                        object: Box::new(expr),
                        property: prop,
                        optional: true,
                    };
                }
                JsToken::LBracket => {
                    self.advance();
                    let index = self.parse_expr()?;
                    self.expect(&JsToken::RBracket)?;
                    expr = JsExpr::ComputedAccess {
                        object: Box::new(expr),
                        index: Box::new(index),
                    };
                }
                JsToken::LParen => {
                    self.advance();
                    let mut args = Vec::new();
                    while *self.peek() != JsToken::RParen && *self.peek() != JsToken::EOF {
                        if *self.peek() == JsToken::DotDotDot {
                            self.advance();
                            let e = self.parse_assignment_expr()?;
                            args.push(JsExpr::Spread(Box::new(e)));
                        } else {
                            args.push(self.parse_assignment_expr()?);
                        }
                        if *self.peek() == JsToken::Comma {
                            self.advance();
                        }
                    }
                    self.expect(&JsToken::RParen)?;
                    expr = JsExpr::Call {
                        callee: Box::new(expr),
                        args,
                    };
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<JsExpr, String> {
        match self.peek().clone() {
            JsToken::NumberInt(n) => {
                self.advance();
                Ok(JsExpr::NumberInt(n))
            }
            JsToken::NumberFloat(f) => {
                self.advance();
                Ok(JsExpr::NumberFloat(f))
            }
            JsToken::StringLiteral(s) => {
                self.advance();
                Ok(JsExpr::StringLit(s))
            }
            JsToken::TemplateLiteral(s) => {
                self.advance();
                Ok(JsExpr::TemplateLit(vec![TemplateSegment::Str(s)]))
            }
            JsToken::BoolLiteral(b) => {
                self.advance();
                Ok(JsExpr::Bool(b))
            }
            JsToken::NullLiteral => {
                self.advance();
                Ok(JsExpr::Null)
            }
            JsToken::UndefinedLiteral => {
                self.advance();
                Ok(JsExpr::Undefined)
            }
            JsToken::This => {
                self.advance();
                Ok(JsExpr::This)
            }
            JsToken::New => {
                self.advance();
                let callee = self.parse_call_member()?;
                // new may have been parsed with call args already
                match callee {
                    JsExpr::Call { callee: c, args } => Ok(JsExpr::New { callee: c, args }),
                    other => Ok(JsExpr::New {
                        callee: Box::new(other),
                        args: Vec::new(),
                    }),
                }
            }
            JsToken::LParen => {
                // Could be arrow function or grouped expression
                // Try arrow function: (params) => body
                let saved = self.pos;
                if self.try_parse_arrow_func().is_some() {
                    // Restore and parse properly
                    self.pos = saved;
                    return self.parse_arrow_func();
                }
                self.pos = saved;
                // Grouped expression
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(&JsToken::RParen)?;
                Ok(expr)
            }
            JsToken::LBracket => {
                // Array literal
                self.advance();
                let mut elements = Vec::new();
                while *self.peek() != JsToken::RBracket && *self.peek() != JsToken::EOF {
                    if *self.peek() == JsToken::DotDotDot {
                        self.advance();
                        let e = self.parse_assignment_expr()?;
                        elements.push(JsExpr::Spread(Box::new(e)));
                    } else {
                        elements.push(self.parse_assignment_expr()?);
                    }
                    if *self.peek() == JsToken::Comma {
                        self.advance();
                    }
                }
                self.expect(&JsToken::RBracket)?;
                Ok(JsExpr::ArrayLit(elements))
            }
            JsToken::LBrace => {
                // Object literal
                self.advance();
                let mut props = Vec::new();
                while *self.peek() != JsToken::RBrace && *self.peek() != JsToken::EOF {
                    let key = match self.peek().clone() {
                        JsToken::Identifier(s) => {
                            self.advance();
                            s
                        }
                        JsToken::StringLiteral(s) => {
                            self.advance();
                            s
                        }
                        JsToken::NumberInt(n) => {
                            self.advance();
                            n.to_string()
                        }
                        _ => {
                            return Err(format!(
                                "JS parse error line {}: expected property name",
                                self.current_line()
                            ));
                        }
                    };
                    if *self.peek() == JsToken::Colon {
                        self.advance();
                        let val = self.parse_assignment_expr()?;
                        props.push((key, val));
                    } else {
                        // Shorthand: { x } → { x: x }
                        props.push((key.clone(), JsExpr::Identifier(key)));
                    }
                    if *self.peek() == JsToken::Comma {
                        self.advance();
                    }
                }
                self.expect(&JsToken::RBrace)?;
                Ok(JsExpr::ObjectLit(props))
            }
            JsToken::Function => {
                // Function expression
                self.advance();
                let name = if let JsToken::Identifier(s) = self.peek().clone() {
                    self.advance();
                    Some(s)
                } else {
                    None
                };
                let params = self.parse_params()?;
                let body = self.parse_block_body()?;
                Ok(JsExpr::FuncExpr { name, params, body })
            }
            JsToken::Identifier(name) => {
                self.advance();
                Ok(JsExpr::Identifier(name))
            }
            other => Err(format!(
                "JS parse error line {}: unexpected token {:?}",
                self.current_line(),
                other
            )),
        }
    }

    // ── Arrow function detection ────────────────────────────

    fn try_parse_arrow_func(&mut self) -> Option<()> {
        // Peek ahead to see if this is (params) => ...
        if *self.peek() != JsToken::LParen {
            return None;
        }
        self.advance(); // (
        let mut depth = 1;
        while depth > 0 && *self.peek() != JsToken::EOF {
            match self.peek() {
                JsToken::LParen => depth += 1,
                JsToken::RParen => depth -= 1,
                _ => {}
            }
            self.advance();
        }
        if *self.peek() == JsToken::Arrow {
            Some(())
        } else {
            None
        }
    }

    fn parse_arrow_func(&mut self) -> Result<JsExpr, String> {
        let params = self.parse_params()?;
        self.expect(&JsToken::Arrow)?;
        let body = if *self.peek() == JsToken::LBrace {
            JsArrowBody::Block(self.parse_block_body()?)
        } else {
            JsArrowBody::Expr(Box::new(self.parse_assignment_expr()?))
        };
        Ok(JsExpr::ArrowFunc { params, body })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::js_lexer::JsLexer;

    fn parse_js(source: &str) -> JsProgram {
        let mut lexer = JsLexer::new(source);
        let (tokens, lines) = lexer.tokenize();
        let mut parser = JsParser::new(tokens, lines);
        parser.parse().unwrap()
    }

    #[test]
    fn test_var_decl() {
        let prog = parse_js("let x = 5;");
        assert_eq!(prog.stmts.len(), 1);
        if let JsStmt::VarDecl { name, kind, .. } = &prog.stmts[0] {
            assert_eq!(name, "x");
            assert_eq!(*kind, DeclKind::Let);
        } else {
            panic!("Expected VarDecl");
        }
    }

    #[test]
    fn test_typed_var_decl() {
        let prog = parse_js("let x: int = 5;");
        if let JsStmt::VarDecl { type_ann, .. } = &prog.stmts[0] {
            assert_eq!(*type_ann, Some(JsType::Int));
        } else {
            panic!("Expected VarDecl");
        }
    }

    #[test]
    fn test_function_decl() {
        let prog = parse_js("function add(a: int, b: int): int { return a + b; }");
        assert_eq!(prog.stmts.len(), 1);
        if let JsStmt::FuncDecl { name, params, .. } = &prog.stmts[0] {
            assert_eq!(name, "add");
            assert_eq!(params.len(), 2);
        } else {
            panic!("Expected FuncDecl");
        }
    }

    #[test]
    fn test_class_decl() {
        let prog = parse_js(
            "class Point { x: int; y: int; constructor(x: int, y: int) { this.x = x; this.y = y; } }",
        );
        if let JsStmt::ClassDecl { name, body, .. } = &prog.stmts[0] {
            assert_eq!(name, "Point");
            assert!(body.len() >= 2); // properties + constructor
        } else {
            panic!("Expected ClassDecl");
        }
    }

    #[test]
    fn test_if_else() {
        let prog = parse_js("if (x > 5) { return 1; } else { return 0; }");
        assert_eq!(prog.stmts.len(), 1);
        if let JsStmt::If { else_body, .. } = &prog.stmts[0] {
            assert!(else_body.is_some());
        } else {
            panic!("Expected If");
        }
    }

    #[test]
    fn test_for_loop() {
        let prog = parse_js("for (let i = 0; i < 10; i++) { x = x + 1; }");
        assert_eq!(prog.stmts.len(), 1);
        assert!(matches!(prog.stmts[0], JsStmt::For { .. }));
    }

    #[test]
    fn test_console_log() {
        let prog = parse_js("console.log(42);");
        assert_eq!(prog.stmts.len(), 1);
        if let JsStmt::Expr(JsExpr::Call { callee, args }) = &prog.stmts[0] {
            assert_eq!(args.len(), 1);
            if let JsExpr::MemberAccess { property, .. } = callee.as_ref() {
                assert_eq!(property, "log");
            }
        } else {
            panic!("Expected Call expr");
        }
    }
}
