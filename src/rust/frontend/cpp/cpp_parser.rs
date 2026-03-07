// ============================================================
// ADead-BIB C++ Frontend — Parser
// ============================================================
// Recursive descent parser: CppToken → C++ AST
// Supports: classes, templates, namespaces, lambdas, modern C++
//
// Sin GCC. Sin LLVM. Sin Clang. Solo ADead-BIB. 💀🦈
// ============================================================

use super::cpp_ast::*;
use super::cpp_lexer::CppToken;

pub struct CppParser {
    tokens: Vec<CppToken>,
    lines: Vec<usize>,
    pos: usize,
    type_names: std::collections::HashSet<String>,
}

impl CppParser {
    pub fn new(tokens: Vec<CppToken>, lines: Vec<usize>) -> Self {
        Self {
            tokens,
            lines,
            pos: 0,
            type_names: std::collections::HashSet::new(),
        }
    }

    pub fn current_line(&self) -> usize {
        self.lines.get(self.pos).copied().unwrap_or(0)
    }

    // ========== Token helpers ==========

    fn current(&self) -> &CppToken {
        self.tokens.get(self.pos).unwrap_or(&CppToken::Eof)
    }

    fn peek(&self) -> &CppToken {
        self.tokens.get(self.pos + 1).unwrap_or(&CppToken::Eof)
    }

    fn peek_at(&self, offset: usize) -> &CppToken {
        self.tokens.get(self.pos + offset).unwrap_or(&CppToken::Eof)
    }

    fn advance(&mut self) -> CppToken {
        let tok = self.current().clone();
        self.pos += 1;
        tok
    }

    fn expect(&mut self, expected: &CppToken) -> Result<(), String> {
        if self.current() == expected {
            self.advance();
            Ok(())
        } else {
            Err(format!(
                "Expected {:?}, got {:?} at pos {}",
                expected,
                self.current(),
                self.pos
            ))
        }
    }

    fn eat(&mut self, expected: &CppToken) -> bool {
        if self.current() == expected {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect_identifier(&mut self) -> Result<String, String> {
        match self.current().clone() {
            CppToken::Identifier(name) => {
                self.advance();
                Ok(name)
            }
            other => Err(format!(
                "Expected identifier, got {:?} at pos {}",
                other, self.pos
            )),
        }
    }

    // ========== Type parsing ==========

    fn is_type_start(&self) -> bool {
        match self.current() {
            CppToken::Void
            | CppToken::Char
            | CppToken::Short
            | CppToken::Int
            | CppToken::Long
            | CppToken::Float
            | CppToken::Double
            | CppToken::Signed
            | CppToken::Unsigned
            | CppToken::Bool
            | CppToken::Auto
            | CppToken::Wchar_t
            | CppToken::Char8_t
            | CppToken::Char16_t
            | CppToken::Char32_t
            | CppToken::Struct
            | CppToken::Class
            | CppToken::Enum
            | CppToken::Union
            | CppToken::Const
            | CppToken::Volatile
            | CppToken::Static
            | CppToken::Extern
            | CppToken::Inline
            | CppToken::Constexpr
            | CppToken::Mutable
            | CppToken::Typename
            | CppToken::Decltype
            | CppToken::Register
            | CppToken::Thread_local => true,
            CppToken::Identifier(name) => {
                if self.type_names.contains(name) {
                    return true;
                }
                // Recognize namespace::type patterns (e.g. std::vector, std::string)
                if *self.peek() == CppToken::Scope {
                    if let CppToken::Identifier(inner) = self.peek_at(2) {
                        return self.type_names.contains(inner);
                    }
                }
                false
            }
            _ => false,
        }
    }

    fn parse_base_type(&mut self) -> Result<CppType, String> {
        // Skip storage class specifiers and qualifiers
        let mut is_const = false;
        let mut is_volatile = false;
        let mut is_constexpr = false;
        loop {
            match self.current() {
                CppToken::Static
                | CppToken::Extern
                | CppToken::Register
                | CppToken::Inline
                | CppToken::Thread_local => {
                    self.advance();
                }
                CppToken::Const => {
                    is_const = true;
                    self.advance();
                }
                CppToken::Volatile => {
                    is_volatile = true;
                    self.advance();
                }
                CppToken::Constexpr => {
                    is_constexpr = true;
                    self.advance();
                }
                CppToken::Mutable => {
                    self.advance();
                }
                _ => break,
            }
        }

        let base = match self.current().clone() {
            CppToken::Void => {
                self.advance();
                CppType::Void
            }
            CppToken::Bool => {
                self.advance();
                CppType::Bool
            }
            CppToken::Char => {
                self.advance();
                CppType::Char
            }
            CppToken::Wchar_t => {
                self.advance();
                CppType::WChar
            }
            CppToken::Char8_t => {
                self.advance();
                CppType::Char8
            }
            CppToken::Char16_t => {
                self.advance();
                CppType::Char16
            }
            CppToken::Char32_t => {
                self.advance();
                CppType::Char32
            }
            CppToken::Short => {
                self.advance();
                self.eat(&CppToken::Int);
                CppType::Short
            }
            CppToken::Int => {
                self.advance();
                CppType::Int
            }
            CppToken::Long => {
                self.advance();
                if self.eat(&CppToken::Long) {
                    self.eat(&CppToken::Int);
                    CppType::LongLong
                } else if self.eat(&CppToken::Double) {
                    CppType::LongDouble
                } else {
                    self.eat(&CppToken::Int);
                    CppType::Long
                }
            }
            CppToken::Float => {
                self.advance();
                CppType::Float
            }
            CppToken::Double => {
                self.advance();
                CppType::Double
            }
            CppToken::Signed => {
                self.advance();
                if self.is_type_start() {
                    let inner = self.parse_base_type()?;
                    CppType::Signed(Box::new(inner))
                } else {
                    CppType::Int
                }
            }
            CppToken::Unsigned => {
                self.advance();
                if self.is_type_start() {
                    let inner = self.parse_base_type()?;
                    CppType::Unsigned(Box::new(inner))
                } else {
                    CppType::Unsigned(Box::new(CppType::Int))
                }
            }
            CppToken::Auto => {
                self.advance();
                CppType::Auto
            }
            CppToken::Decltype => {
                self.advance();
                self.expect(&CppToken::LParen)?;
                let expr = self.parse_expression()?;
                self.expect(&CppToken::RParen)?;
                CppType::Decltype(Box::new(expr))
            }
            CppToken::Struct | CppToken::Class => {
                let is_class = *self.current() == CppToken::Class;
                self.advance();
                let name = self.expect_identifier()?;
                if is_class {
                    CppType::Class(name)
                } else {
                    CppType::Struct(name)
                }
            }
            CppToken::Enum => {
                self.advance();
                self.eat(&CppToken::Class); // enum class
                let name = self.expect_identifier()?;
                CppType::Enum(name)
            }
            CppToken::Union => {
                self.advance();
                let name = self.expect_identifier()?;
                CppType::Union(name)
            }
            CppToken::Typename => {
                self.advance();
                let name = self.expect_identifier()?;
                CppType::Named(name)
            }
            CppToken::Identifier(ref name) => {
                let name = name.clone();
                self.advance();
                // Check for template arguments: Type<T>
                if *self.current() == CppToken::Lt {
                    if let Ok(args) = self.try_parse_template_args() {
                        // Recognize common STL types
                        match name.as_str() {
                            "string" | "std" => CppType::TemplateType { name, args },
                            "vector" if args.len() == 1 => {
                                CppType::StdVector(Box::new(args[0].clone()))
                            }
                            "array" if args.len() >= 1 => CppType::TemplateType { name, args },
                            "map" if args.len() == 2 => CppType::StdMap(
                                Box::new(args[0].clone()),
                                Box::new(args[1].clone()),
                            ),
                            "unordered_map" if args.len() == 2 => CppType::StdUnorderedMap(
                                Box::new(args[0].clone()),
                                Box::new(args[1].clone()),
                            ),
                            "optional" if args.len() == 1 => {
                                CppType::StdOptional(Box::new(args[0].clone()))
                            }
                            "unique_ptr" if args.len() == 1 => {
                                CppType::UniquePtr(Box::new(args[0].clone()))
                            }
                            "shared_ptr" if args.len() == 1 => {
                                CppType::SharedPtr(Box::new(args[0].clone()))
                            }
                            _ => CppType::TemplateType { name, args },
                        }
                    } else {
                        CppType::Named(name)
                    }
                } else {
                    // Check for scope: std::string
                    if *self.current() == CppToken::Scope {
                        self.advance();
                        let inner_name = self.expect_identifier()?;
                        let full = format!("{}::{}", name, inner_name);
                        match full.as_str() {
                            "std::string" => CppType::StdString,
                            "std::size_t" | "size_t" => CppType::SizeT,
                            _ => {
                                // Check for template args after scope
                                if *self.current() == CppToken::Lt {
                                    if let Ok(args) = self.try_parse_template_args() {
                                        match inner_name.as_str() {
                                            "vector" if args.len() == 1 => {
                                                CppType::StdVector(Box::new(args[0].clone()))
                                            }
                                            "map" if args.len() == 2 => CppType::StdMap(
                                                Box::new(args[0].clone()),
                                                Box::new(args[1].clone()),
                                            ),
                                            "unique_ptr" if args.len() == 1 => {
                                                CppType::UniquePtr(Box::new(args[0].clone()))
                                            }
                                            "shared_ptr" if args.len() == 1 => {
                                                CppType::SharedPtr(Box::new(args[0].clone()))
                                            }
                                            "optional" if args.len() == 1 => {
                                                CppType::StdOptional(Box::new(args[0].clone()))
                                            }
                                            _ => CppType::TemplateType { name: full, args },
                                        }
                                    } else {
                                        CppType::Named(full)
                                    }
                                } else {
                                    CppType::Named(full)
                                }
                            }
                        }
                    } else {
                        match name.as_str() {
                            "string" => CppType::StdString,
                            "size_t" => CppType::SizeT,
                            _ => CppType::Named(name),
                        }
                    }
                }
            }
            _ => {
                return Err(format!(
                    "Expected type, got {:?} at pos {}",
                    self.current(),
                    self.pos
                ))
            }
        };

        let mut result = base;
        if is_constexpr {
            result = CppType::Constexpr(Box::new(result));
        }
        if is_volatile {
            result = CppType::Volatile(Box::new(result));
        }
        if is_const {
            result = CppType::Const(Box::new(result));
        }
        Ok(result)
    }

    fn parse_type(&mut self) -> Result<CppType, String> {
        let mut base = self.parse_base_type()?;

        // Pointers, references
        loop {
            match self.current() {
                CppToken::Star => {
                    self.advance();
                    // const after * : int *const
                    while matches!(self.current(), CppToken::Const | CppToken::Volatile) {
                        self.advance();
                    }
                    base = CppType::Pointer(Box::new(base));
                }
                CppToken::Amp => {
                    // Check for && (rvalue ref) vs & (lvalue ref)
                    if *self.peek() == CppToken::Amp {
                        // This is &&, but we need to be careful — && as logical AND
                        // In type context after a type name, treat as rvalue ref
                        // Skip for now, treat as lvalue ref
                        self.advance();
                        base = CppType::Reference(Box::new(base));
                    } else {
                        self.advance();
                        base = CppType::Reference(Box::new(base));
                    }
                }
                CppToken::And => {
                    // && token — rvalue reference
                    self.advance();
                    base = CppType::RValueRef(Box::new(base));
                }
                _ => break,
            }
        }
        Ok(base)
    }

    /// Try to parse a single template argument (type or non-type integer/bool/identifier)
    fn try_parse_one_template_arg(&mut self) -> Result<CppType, String> {
        // Non-type args: integer literals → represent as Named("N") for IR
        match self.current().clone() {
            CppToken::IntLiteral(n) => {
                self.advance();
                Ok(CppType::Named(format!("{}", n)))
            }
            CppToken::True => {
                self.advance();
                Ok(CppType::Named("true".to_string()))
            }
            CppToken::False => {
                self.advance();
                Ok(CppType::Named("false".to_string()))
            }
            _ => self.parse_type(),
        }
    }

    fn try_parse_template_args(&mut self) -> Result<Vec<CppType>, String> {
        let save_pos = self.pos;
        if !self.eat(&CppToken::Lt) {
            return Err("Not a template".to_string());
        }
        let mut args = Vec::new();
        if *self.current() != CppToken::Gt {
            args.push(match self.try_parse_one_template_arg() {
                Ok(t) => t,
                Err(_) => {
                    self.pos = save_pos;
                    return Err("Not a template".to_string());
                }
            });
            while self.eat(&CppToken::Comma) {
                args.push(match self.try_parse_one_template_arg() {
                    Ok(t) => t,
                    Err(_) => {
                        self.pos = save_pos;
                        return Err("Not a template".to_string());
                    }
                });
            }
        }
        if !self.eat(&CppToken::Gt) {
            // Try >> as two >
            if *self.current() == CppToken::Shr {
                // Consume >> as one >, leave one > for the outer template
                self.advance();
                // We consumed both, but we need to "put back" one >
                // For simplicity, just accept
            } else {
                self.pos = save_pos;
                return Err("Not a template".to_string());
            }
        }
        Ok(args)
    }

    // ========== Translation unit ==========

    pub fn parse_translation_unit(&mut self) -> Result<CppTranslationUnit, String> {
        self.prescan_type_names();
        let mut unit = CppTranslationUnit::new();
        while *self.current() != CppToken::Eof {
            let decl = self.parse_top_level()?;
            match &decl {
                CppTopLevel::ClassDef { name, .. } => {
                    self.type_names.insert(name.clone());
                }
                CppTopLevel::TypeAlias { new_name, .. } => {
                    self.type_names.insert(new_name.clone());
                }
                CppTopLevel::EnumDef { name, .. } => {
                    self.type_names.insert(name.clone());
                }
                _ => {}
            }
            unit.declarations.push(decl);
        }
        Ok(unit)
    }

    fn prescan_type_names(&mut self) {
        // Pre-scan for class/struct/enum/typedef/using names
        let mut i = 0;
        while i < self.tokens.len() {
            match &self.tokens[i] {
                CppToken::Class | CppToken::Struct => {
                    if let Some(CppToken::Identifier(name)) = self.tokens.get(i + 1) {
                        self.type_names.insert(name.clone());
                    }
                    i += 1;
                }
                CppToken::Typedef => {
                    // Find semicolon, name is before it
                    let mut j = i + 1;
                    let mut depth = 0;
                    while j < self.tokens.len() {
                        match &self.tokens[j] {
                            CppToken::LBrace | CppToken::LParen => depth += 1,
                            CppToken::RBrace | CppToken::RParen => {
                                if depth > 0 {
                                    depth -= 1;
                                }
                            }
                            CppToken::Semicolon if depth == 0 => break,
                            _ => {}
                        }
                        j += 1;
                    }
                    if j > 0 && j < self.tokens.len() {
                        if let CppToken::Identifier(name) = &self.tokens[j - 1] {
                            self.type_names.insert(name.clone());
                        }
                    }
                    i = j + 1;
                }
                CppToken::Using => {
                    if let Some(CppToken::Identifier(name)) = self.tokens.get(i + 1) {
                        if self.tokens.get(i + 2) == Some(&CppToken::Assign) {
                            self.type_names.insert(name.clone());
                        }
                    }
                    i += 1;
                }
                CppToken::Enum => {
                    // enum or enum class
                    let mut j = i + 1;
                    if self.tokens.get(j) == Some(&CppToken::Class) {
                        j += 1;
                    }
                    if let Some(CppToken::Identifier(name)) = self.tokens.get(j) {
                        self.type_names.insert(name.clone());
                    }
                    i += 1;
                }
                CppToken::Namespace => {
                    // skip namespace names — not types
                    i += 1;
                }
                _ => {
                    i += 1;
                }
            }
        }
        // Common STL names
        for name in &[
            "string",
            "vector",
            "map",
            "unordered_map",
            "set",
            "list",
            "deque",
            "array",
            "pair",
            "tuple",
            "optional",
            "variant",
            "unique_ptr",
            "shared_ptr",
            "weak_ptr",
            "span",
            "size_t",
            "int8_t",
            "int16_t",
            "int32_t",
            "int64_t",
            "uint8_t",
            "uint16_t",
            "uint32_t",
            "uint64_t",
            "ptrdiff_t",
            "nullptr_t",
        ] {
            self.type_names.insert(name.to_string());
        }
    }

    // ========== Top-level parsing ==========

    fn parse_top_level(&mut self) -> Result<CppTopLevel, String> {
        // Template
        if *self.current() == CppToken::Template {
            return self.parse_template_decl();
        }

        // Namespace
        if *self.current() == CppToken::Namespace {
            return self.parse_namespace();
        }

        // Using
        if *self.current() == CppToken::Using {
            return self.parse_using_decl();
        }

        // Class / Struct definition
        if (*self.current() == CppToken::Class || *self.current() == CppToken::Struct)
            && matches!(self.peek(), CppToken::Identifier(_))
            && (*self.peek_at(2) == CppToken::LBrace
                || *self.peek_at(2) == CppToken::Colon
                || *self.peek_at(2) == CppToken::Final)
        {
            return self.parse_class_def(Vec::new());
        }

        // Enum
        if *self.current() == CppToken::Enum {
            return self.parse_enum_def();
        }

        // Typedef
        if *self.current() == CppToken::Typedef {
            return self.parse_typedef();
        }

        // Extern "C"
        if *self.current() == CppToken::Extern {
            if let CppToken::StringLiteral(ref s) = self.peek().clone() {
                if s == "C" {
                    self.advance(); // extern
                    self.advance(); // "C"
                    self.expect(&CppToken::LBrace)?;
                    let mut decls = Vec::new();
                    while *self.current() != CppToken::RBrace && *self.current() != CppToken::Eof {
                        decls.push(self.parse_top_level()?);
                    }
                    self.expect(&CppToken::RBrace)?;
                    return Ok(CppTopLevel::ExternC {
                        declarations: decls,
                    });
                }
            }
        }

        // Static assert
        if *self.current() == CppToken::Static_assert {
            return self.parse_static_assert();
        }

        // Function or global variable
        let ret_type = self.parse_type()?;

        // Check for destructor: ~ClassName
        if *self.current() == CppToken::Tilde {
            self.advance();
            let name = self.expect_identifier()?;
            // This is a destructor definition outside class
            self.expect(&CppToken::LParen)?;
            self.expect(&CppToken::RParen)?;
            let body = self.parse_block_stmts()?;
            return Ok(CppTopLevel::FunctionDef {
                return_type: CppType::Void,
                name: format!("~{}", name),
                template_params: Vec::new(),
                params: Vec::new(),
                qualifiers: CppFuncQualifiers::default(),
                body,
            });
        }

        // Operator overload
        if *self.current() == CppToken::Operator {
            self.advance();
            let op_name = self.parse_operator_name()?;
            let name = format!("operator{}", op_name);
            return self.parse_function_rest(ret_type, name, Vec::new());
        }

        let name = self.expect_identifier()?;

        // Check for scope resolution: Class::method
        if *self.current() == CppToken::Scope {
            self.advance();
            // Destructor: Class::~Class()
            if *self.current() == CppToken::Tilde {
                self.advance();
                let _dtor_name = self.expect_identifier()?;
                self.expect(&CppToken::LParen)?;
                self.expect(&CppToken::RParen)?;
                let body = self.parse_block_stmts()?;
                return Ok(CppTopLevel::FunctionDef {
                    return_type: CppType::Void,
                    name: format!("{}::~{}", name, _dtor_name),
                    template_params: Vec::new(),
                    params: Vec::new(),
                    qualifiers: CppFuncQualifiers::default(),
                    body,
                });
            }
            let method_name = self.expect_identifier()?;
            let full_name = format!("{}::{}", name, method_name);
            return self.parse_function_rest(ret_type, full_name, Vec::new());
        }

        self.parse_function_or_var(ret_type, name, Vec::new())
    }

    fn parse_function_or_var(
        &mut self,
        ret_type: CppType,
        name: String,
        tp: Vec<CppTemplateParam>,
    ) -> Result<CppTopLevel, String> {
        if *self.current() == CppToken::LParen {
            self.parse_function_rest(ret_type, name, tp)
        } else {
            // Global variable
            let mut declarators = Vec::new();
            let first = self.parse_declarator_rest(name)?;
            declarators.push(first);
            while self.eat(&CppToken::Comma) {
                let n = self.expect_identifier()?;
                let d = self.parse_declarator_rest(n)?;
                declarators.push(d);
            }
            self.expect(&CppToken::Semicolon)?;
            Ok(CppTopLevel::GlobalVar {
                type_spec: ret_type,
                declarators,
            })
        }
    }

    fn parse_function_rest(
        &mut self,
        ret_type: CppType,
        name: String,
        tp: Vec<CppTemplateParam>,
    ) -> Result<CppTopLevel, String> {
        self.expect(&CppToken::LParen)?;
        let params = self.parse_param_list()?;
        self.expect(&CppToken::RParen)?;

        let mut quals = CppFuncQualifiers::default();
        // Parse trailing qualifiers
        loop {
            match self.current() {
                CppToken::Const => {
                    quals.is_const = true;
                    self.advance();
                }
                CppToken::Noexcept => {
                    quals.is_noexcept = true;
                    self.advance();
                }
                CppToken::Override => {
                    quals.is_override = true;
                    self.advance();
                }
                CppToken::Final => {
                    quals.is_final = true;
                    self.advance();
                }
                _ => break,
            }
        }

        // = 0, = default, = delete
        if self.eat(&CppToken::Assign) {
            match self.current() {
                CppToken::IntLiteral(0) => {
                    quals.is_pure_virtual = true;
                    self.advance();
                }
                CppToken::Default => {
                    quals.is_default = true;
                    self.advance();
                }
                CppToken::Delete => {
                    quals.is_delete = true;
                    self.advance();
                }
                _ => {}
            }
        }

        // Arrow return type: auto f() -> int
        if *self.current() == CppToken::Arrow {
            self.advance();
            let _trailing_ret = self.parse_type()?;
        }

        if *self.current() == CppToken::LBrace {
            let body = self.parse_block_stmts()?;
            Ok(CppTopLevel::FunctionDef {
                return_type: ret_type,
                name,
                template_params: tp,
                params,
                qualifiers: quals,
                body,
            })
        } else {
            self.expect(&CppToken::Semicolon)?;
            Ok(CppTopLevel::FunctionDecl {
                return_type: ret_type,
                name,
                template_params: tp,
                params,
                qualifiers: quals,
            })
        }
    }

    fn parse_declarator_rest(&mut self, name: String) -> Result<CppDeclarator, String> {
        let mut derived = Vec::new();
        // Array: name[N]
        if *self.current() == CppToken::LBracket {
            self.advance();
            let size = if let CppToken::IntLiteral(n) = self.current().clone() {
                self.advance();
                Some(n as usize)
            } else {
                None
            };
            self.expect(&CppToken::RBracket)?;
            derived.push(CppDerivedType::Array(size));
        }

        // Initializer
        let init = if self.eat(&CppToken::Assign) {
            Some(self.parse_expression()?)
        } else if *self.current() == CppToken::LParen {
            // Direct initialization: int x(5) or Dog rex(5, 30, 10)
            self.advance();
            let mut args = Vec::new();
            if *self.current() != CppToken::RParen {
                args.push(self.parse_assignment_expr()?);
                while self.eat(&CppToken::Comma) {
                    args.push(self.parse_assignment_expr()?);
                }
            }
            self.expect(&CppToken::RParen)?;
            let expr = if args.len() == 1 {
                args.into_iter().next().unwrap()
            } else {
                CppExpr::InitList(args)
            };
            Some(expr)
        } else if *self.current() == CppToken::LBrace {
            // Brace initialization: int x{5} or vector<int> v = {1,2,3} or int arr{1,2,3}
            self.advance();
            let expr = if *self.current() != CppToken::RBrace {
                let mut items = Vec::new();
                items.push(self.parse_assignment_expr()?);
                while self.eat(&CppToken::Comma) {
                    if *self.current() == CppToken::RBrace {
                        break;
                    } // trailing comma
                    items.push(self.parse_assignment_expr()?);
                }
                if items.len() == 1 {
                    items.into_iter().next().unwrap()
                } else {
                    CppExpr::InitList(items)
                }
            } else {
                CppExpr::InitList(Vec::new())
            };
            self.expect(&CppToken::RBrace)?;
            Some(expr)
        } else {
            None
        };

        Ok(CppDeclarator {
            name,
            derived_type: derived,
            initializer: init,
        })
    }

    fn parse_param_list(&mut self) -> Result<Vec<CppParam>, String> {
        let mut params = Vec::new();
        if *self.current() == CppToken::RParen {
            return Ok(params);
        }
        // void as single param
        if *self.current() == CppToken::Void && *self.peek() == CppToken::RParen {
            self.advance();
            return Ok(params);
        }

        params.push(self.parse_param()?);
        while self.eat(&CppToken::Comma) {
            if *self.current() == CppToken::Ellipsis {
                self.advance();
                params.push(CppParam {
                    param_type: CppType::Void,
                    name: None,
                    default_value: None,
                    is_variadic: true,
                });
                break;
            }
            params.push(self.parse_param()?);
        }
        Ok(params)
    }

    fn parse_param(&mut self) -> Result<CppParam, String> {
        let param_type = self.parse_type()?;
        let name = if let CppToken::Identifier(_) = self.current() {
            Some(self.expect_identifier()?)
        } else {
            None
        };
        // Array param: type name[]
        if *self.current() == CppToken::LBracket {
            self.advance();
            if let CppToken::IntLiteral(_) = self.current() {
                self.advance();
            }
            self.expect(&CppToken::RBracket)?;
        }
        let default_value = if self.eat(&CppToken::Assign) {
            Some(self.parse_assignment_expr()?)
        } else {
            None
        };
        Ok(CppParam {
            param_type,
            name,
            default_value,
            is_variadic: false,
        })
    }

    // ========== Class parsing ==========

    fn parse_class_def(
        &mut self,
        template_params: Vec<CppTemplateParam>,
    ) -> Result<CppTopLevel, String> {
        let is_struct = *self.current() == CppToken::Struct;
        self.advance(); // skip class/struct
        let name = self.expect_identifier()?;
        self.type_names.insert(name.clone());

        // final
        self.eat(&CppToken::Final);

        // Base classes
        let mut bases = Vec::new();
        if self.eat(&CppToken::Colon) {
            loop {
                let mut access = if is_struct {
                    CppAccess::Public
                } else {
                    CppAccess::Private
                };
                let mut is_virtual = false;
                if self.eat(&CppToken::Virtual) {
                    is_virtual = true;
                }
                match self.current() {
                    CppToken::Public => {
                        access = CppAccess::Public;
                        self.advance();
                    }
                    CppToken::Protected => {
                        access = CppAccess::Protected;
                        self.advance();
                    }
                    CppToken::Private => {
                        access = CppAccess::Private;
                        self.advance();
                    }
                    _ => {}
                }
                if self.eat(&CppToken::Virtual) {
                    is_virtual = true;
                }
                let base_name = self.expect_identifier()?;
                let template_args = if *self.current() == CppToken::Lt {
                    self.try_parse_template_args().unwrap_or_default()
                } else {
                    Vec::new()
                };
                bases.push(CppBaseClass {
                    access,
                    name: base_name,
                    is_virtual,
                    template_args,
                });
                if !self.eat(&CppToken::Comma) {
                    break;
                }
            }
        }

        // Body
        self.expect(&CppToken::LBrace)?;
        let members = self.parse_class_members(is_struct)?;
        self.expect(&CppToken::RBrace)?;
        self.expect(&CppToken::Semicolon)?;

        Ok(CppTopLevel::ClassDef {
            name,
            template_params,
            bases,
            members,
            is_struct,
        })
    }

    fn parse_class_members(&mut self, is_struct: bool) -> Result<Vec<CppClassMember>, String> {
        let mut members = Vec::new();
        let mut current_access = if is_struct {
            CppAccess::Public
        } else {
            CppAccess::Private
        };

        while *self.current() != CppToken::RBrace && *self.current() != CppToken::Eof {
            // Access specifiers
            match self.current() {
                CppToken::Public => {
                    self.advance();
                    self.expect(&CppToken::Colon)?;
                    current_access = CppAccess::Public;
                    members.push(CppClassMember::AccessSpec(CppAccess::Public));
                    continue;
                }
                CppToken::Protected => {
                    self.advance();
                    self.expect(&CppToken::Colon)?;
                    current_access = CppAccess::Protected;
                    members.push(CppClassMember::AccessSpec(CppAccess::Protected));
                    continue;
                }
                CppToken::Private => {
                    self.advance();
                    self.expect(&CppToken::Colon)?;
                    current_access = CppAccess::Private;
                    members.push(CppClassMember::AccessSpec(CppAccess::Private));
                    continue;
                }
                _ => {}
            }

            // Friend
            if *self.current() == CppToken::Friend {
                self.advance();
                // Skip friend declaration until semicolon
                let mut friend_name = String::new();
                while *self.current() != CppToken::Semicolon && *self.current() != CppToken::Eof {
                    if let CppToken::Identifier(ref n) = self.current().clone() {
                        friend_name = n.clone();
                    }
                    self.advance();
                }
                self.expect(&CppToken::Semicolon)?;
                members.push(CppClassMember::FriendDecl(friend_name));
                continue;
            }

            // Using declaration inside class
            if *self.current() == CppToken::Using {
                self.advance();
                let target = self.expect_identifier()?;
                // Skip rest until ;
                while *self.current() != CppToken::Semicolon && *self.current() != CppToken::Eof {
                    self.advance();
                }
                self.expect(&CppToken::Semicolon)?;
                members.push(CppClassMember::UsingDecl(target));
                continue;
            }

            // Destructor: ~ClassName()
            if *self.current() == CppToken::Tilde
                || (*self.current() == CppToken::Virtual && *self.peek() == CppToken::Tilde)
            {
                let is_virtual = self.eat(&CppToken::Virtual);
                self.expect(&CppToken::Tilde)?;
                let _dtor_name = self.expect_identifier()?;
                self.expect(&CppToken::LParen)?;
                self.expect(&CppToken::RParen)?;
                // qualifiers
                let mut _noexcept = false;
                if self.eat(&CppToken::Noexcept) {
                    _noexcept = true;
                }
                // = default / = delete
                if self.eat(&CppToken::Assign) {
                    match self.current() {
                        CppToken::Default | CppToken::IntLiteral(0) => {
                            self.advance();
                        }
                        _ => {}
                    }
                }
                let body = if *self.current() == CppToken::LBrace {
                    Some(self.parse_block_stmts()?)
                } else {
                    self.expect(&CppToken::Semicolon)?;
                    None
                };
                members.push(CppClassMember::Destructor {
                    access: current_access,
                    is_virtual,
                    body,
                });
                continue;
            }

            // Gather qualifiers
            let mut quals = CppFuncQualifiers::default();
            let mut is_explicit = false;
            loop {
                match self.current() {
                    CppToken::Virtual => {
                        quals.is_virtual = true;
                        self.advance();
                    }
                    CppToken::Static => {
                        quals.is_static = true;
                        self.advance();
                    }
                    CppToken::Inline => {
                        quals.is_inline = true;
                        self.advance();
                    }
                    CppToken::Constexpr => {
                        quals.is_constexpr = true;
                        self.advance();
                    }
                    CppToken::Explicit => {
                        is_explicit = true;
                        self.advance();
                    }
                    _ => break,
                }
            }

            // Constructor check: ClassName(...)
            // A constructor has no return type — the identifier IS the class name and next is (
            if let CppToken::Identifier(ref ident) = self.current().clone() {
                if *self.peek() == CppToken::LParen && self.type_names.contains(ident) {
                    // This could be a constructor or a method with return type = ClassName
                    // Heuristic: if the name matches a class name and it's followed by (
                    // and there's no type before it, it's likely a constructor
                    let _ctor_name = ident.clone();
                    self.advance(); // skip name
                    self.expect(&CppToken::LParen)?;
                    let params = self.parse_param_list()?;
                    self.expect(&CppToken::RParen)?;

                    // Check for initializer list: : member(val), Base(a, b), ...
                    let mut init_list = Vec::new();
                    if self.eat(&CppToken::Colon) {
                        loop {
                            let member = self.expect_identifier()?;
                            self.expect(&CppToken::LParen)?;
                            // Parse arguments (may be multiple comma-separated)
                            let mut args = Vec::new();
                            if *self.current() != CppToken::RParen {
                                args.push(self.parse_assignment_expr()?);
                                while self.eat(&CppToken::Comma) {
                                    args.push(self.parse_assignment_expr()?);
                                }
                            }
                            self.expect(&CppToken::RParen)?;
                            // Use first arg as the init value (or a placeholder)
                            let val = if args.len() == 1 {
                                args.into_iter().next().unwrap()
                            } else if args.is_empty() {
                                CppExpr::IntLiteral(0)
                            } else {
                                CppExpr::Call {
                                    callee: Box::new(CppExpr::Identifier(member.clone())),
                                    args,
                                }
                            };
                            init_list.push((member, val));
                            if !self.eat(&CppToken::Comma) {
                                break;
                            }
                        }
                    }

                    // = default / = delete
                    if self.eat(&CppToken::Assign) {
                        match self.current() {
                            CppToken::Default | CppToken::Delete => {
                                self.advance();
                            }
                            _ => {}
                        }
                    }

                    let body = if *self.current() == CppToken::LBrace {
                        Some(self.parse_block_stmts()?)
                    } else {
                        self.expect(&CppToken::Semicolon)?;
                        None
                    };

                    members.push(CppClassMember::Constructor {
                        access: current_access,
                        params,
                        initializer_list: init_list,
                        body,
                        is_explicit,
                    });
                    continue;
                }
            }

            // Regular member: type name; or type name(...) { ... }
            let member_type = self.parse_type()?;

            // Operator overload
            if *self.current() == CppToken::Operator {
                self.advance();
                let op_name = self.parse_operator_name()?;
                let name = format!("operator{}", op_name);
                self.expect(&CppToken::LParen)?;
                let params = self.parse_param_list()?;
                self.expect(&CppToken::RParen)?;
                // trailing qualifiers
                loop {
                    match self.current() {
                        CppToken::Const => {
                            quals.is_const = true;
                            self.advance();
                        }
                        CppToken::Noexcept => {
                            quals.is_noexcept = true;
                            self.advance();
                        }
                        CppToken::Override => {
                            quals.is_override = true;
                            self.advance();
                        }
                        _ => break,
                    }
                }
                let body = if *self.current() == CppToken::LBrace {
                    Some(self.parse_block_stmts()?)
                } else {
                    // = 0, = default
                    if self.eat(&CppToken::Assign) {
                        match self.current() {
                            CppToken::IntLiteral(0) => {
                                quals.is_pure_virtual = true;
                                self.advance();
                            }
                            CppToken::Default => {
                                quals.is_default = true;
                                self.advance();
                            }
                            _ => {}
                        }
                    }
                    self.expect(&CppToken::Semicolon)?;
                    None
                };
                members.push(CppClassMember::Method {
                    access: current_access,
                    return_type: member_type,
                    name,
                    template_params: Vec::new(),
                    params,
                    qualifiers: quals,
                    body,
                });
                continue;
            }

            let member_name = self.expect_identifier()?;

            if *self.current() == CppToken::LParen {
                // Method
                self.advance();
                let params = self.parse_param_list()?;
                self.expect(&CppToken::RParen)?;
                loop {
                    match self.current() {
                        CppToken::Const => {
                            quals.is_const = true;
                            self.advance();
                        }
                        CppToken::Noexcept => {
                            quals.is_noexcept = true;
                            self.advance();
                        }
                        CppToken::Override => {
                            quals.is_override = true;
                            self.advance();
                        }
                        CppToken::Final => {
                            quals.is_final = true;
                            self.advance();
                        }
                        _ => break,
                    }
                }
                if self.eat(&CppToken::Assign) {
                    match self.current() {
                        CppToken::IntLiteral(0) => {
                            quals.is_pure_virtual = true;
                            self.advance();
                        }
                        CppToken::Default => {
                            quals.is_default = true;
                            self.advance();
                        }
                        CppToken::Delete => {
                            quals.is_delete = true;
                            self.advance();
                        }
                        _ => {}
                    }
                }
                // Arrow return type
                if *self.current() == CppToken::Arrow {
                    self.advance();
                    let _trailing = self.parse_type()?;
                }
                let body = if *self.current() == CppToken::LBrace {
                    Some(self.parse_block_stmts()?)
                } else {
                    self.expect(&CppToken::Semicolon)?;
                    None
                };
                members.push(CppClassMember::Method {
                    access: current_access,
                    return_type: member_type,
                    name: member_name,
                    template_params: Vec::new(),
                    params,
                    qualifiers: quals,
                    body,
                });
            } else {
                // Field — handle array dimensions: int data[32];
                let mut field_type = member_type;
                while *self.current() == CppToken::LBracket {
                    self.advance();
                    let arr_size = if *self.current() != CppToken::RBracket {
                        let size_expr = self.parse_assignment_expr()?;
                        match size_expr {
                            CppExpr::IntLiteral(n) => Some(n as usize),
                            _ => None,
                        }
                    } else {
                        None
                    };
                    self.expect(&CppToken::RBracket)?;
                    field_type = CppType::Array(Box::new(field_type), arr_size);
                }
                let default_value = if self.eat(&CppToken::Assign) {
                    Some(self.parse_expression()?)
                } else if *self.current() == CppToken::LBrace {
                    self.advance();
                    let expr = self.parse_expression()?;
                    self.expect(&CppToken::RBrace)?;
                    Some(expr)
                } else {
                    None
                };
                self.expect(&CppToken::Semicolon)?;
                members.push(CppClassMember::Field {
                    access: current_access,
                    type_spec: field_type,
                    name: member_name,
                    default_value,
                    is_static: quals.is_static,
                });
            }
        }
        Ok(members)
    }

    // ========== Enum, Namespace, Using, Template, Typedef ==========

    fn parse_enum_def(&mut self) -> Result<CppTopLevel, String> {
        self.advance(); // skip enum
        let is_class = self.eat(&CppToken::Class) || self.eat(&CppToken::Struct);
        let name = self.expect_identifier()?;
        self.type_names.insert(name.clone());
        let underlying = if self.eat(&CppToken::Colon) {
            Some(self.parse_type()?)
        } else {
            None
        };
        self.expect(&CppToken::LBrace)?;
        let mut values = Vec::new();
        while *self.current() != CppToken::RBrace && *self.current() != CppToken::Eof {
            let ident = self.expect_identifier()?;
            let val = if self.eat(&CppToken::Assign) {
                Some(self.parse_expression()?)
            } else {
                None
            };
            values.push((ident, val));
            if !self.eat(&CppToken::Comma) {
                break;
            }
        }
        self.expect(&CppToken::RBrace)?;
        self.expect(&CppToken::Semicolon)?;
        Ok(CppTopLevel::EnumDef {
            name,
            is_class,
            underlying_type: underlying,
            values,
        })
    }

    fn parse_namespace(&mut self) -> Result<CppTopLevel, String> {
        self.advance(); // skip namespace

        // C++17 nested namespace: namespace A::B::C { ... }
        let mut names = vec![self.expect_identifier()?];
        while self.eat(&CppToken::Scope) {
            names.push(self.expect_identifier()?);
        }

        self.expect(&CppToken::LBrace)?;
        let mut decls = Vec::new();
        while *self.current() != CppToken::RBrace && *self.current() != CppToken::Eof {
            decls.push(self.parse_top_level()?);
        }
        self.expect(&CppToken::RBrace)?;

        // Build nested namespaces from inside out
        let mut result = CppTopLevel::Namespace {
            name: names.last().unwrap().clone(),
            declarations: decls,
        };
        for name in names.iter().rev().skip(1) {
            result = CppTopLevel::Namespace {
                name: name.clone(),
                declarations: vec![result],
            };
        }
        Ok(result)
    }

    fn parse_using_decl(&mut self) -> Result<CppTopLevel, String> {
        self.advance(); // skip using
        if *self.current() == CppToken::Namespace {
            self.advance();
            let mut ns = self.expect_identifier()?;
            while self.eat(&CppToken::Scope) {
                ns.push_str("::");
                ns.push_str(&self.expect_identifier()?);
            }
            self.expect(&CppToken::Semicolon)?;
            return Ok(CppTopLevel::UsingNamespace(ns));
        }
        let name = self.expect_identifier()?;
        self.type_names.insert(name.clone());
        self.expect(&CppToken::Assign)?;
        let original = self.parse_type()?;
        self.expect(&CppToken::Semicolon)?;
        Ok(CppTopLevel::TypeAlias {
            new_name: name,
            original,
            template_params: Vec::new(),
        })
    }

    fn parse_typedef(&mut self) -> Result<CppTopLevel, String> {
        self.advance(); // skip typedef
        let original = self.parse_type()?;
        let new_name = self.expect_identifier()?;
        self.type_names.insert(new_name.clone());
        self.expect(&CppToken::Semicolon)?;
        Ok(CppTopLevel::TypeAlias {
            new_name,
            original,
            template_params: Vec::new(),
        })
    }

    fn parse_template_decl(&mut self) -> Result<CppTopLevel, String> {
        self.advance(); // skip template
        self.expect(&CppToken::Lt)?;
        let params = self.parse_template_params()?;
        self.expect(&CppToken::Gt)?;

        let is_full_specialization = params.is_empty();

        // Register template type parameter names so they are recognized as types
        for p in &params {
            match p {
                CppTemplateParam::TypeParam { name, .. } => {
                    self.type_names.insert(name.clone());
                }
                CppTemplateParam::VariadicType { name } => {
                    self.type_names.insert(name.clone());
                }
                _ => {}
            }
        }

        // Template type alias: template<typename T> using Vec = vector<T>;
        if *self.current() == CppToken::Using {
            self.advance();
            let new_name = self.expect_identifier()?;
            self.type_names.insert(new_name.clone());
            self.expect(&CppToken::Assign)?;
            let original = self.parse_type()?;
            self.expect(&CppToken::Semicolon)?;
            return Ok(CppTopLevel::TypeAlias {
                new_name,
                original,
                template_params: params,
            });
        }

        // What follows: class, struct, function
        if *self.current() == CppToken::Class || *self.current() == CppToken::Struct {
            if matches!(self.peek(), CppToken::Identifier(_)) {
                let is_struct = *self.current() == CppToken::Struct;
                // Peek ahead: is this a specialization? Name<args> { ... }
                // For full specialization: template<> class Foo<int> { ... }
                // For partial specialization: template<typename T> class Foo<T*> { ... }
                if is_full_specialization || self.is_template_specialization_ahead() {
                    return self.parse_template_class_specialization(params, is_struct);
                }
                return self.parse_class_def(params);
            }
        }

        // Template function (possibly specialization)
        let ret_type = self.parse_type()?;
        let name = self.expect_identifier()?;

        // Check for function specialization: template<> int max<int>(...)
        if is_full_specialization {
            if let Ok(spec_args) = self.try_parse_template_args() {
                // Full function specialization
                self.expect(&CppToken::LParen)?;
                let func_params = self.parse_param_list()?;
                self.expect(&CppToken::RParen)?;
                let body = self.parse_block_stmts()?;
                return Ok(CppTopLevel::TemplateFuncSpecialization {
                    name,
                    specialized_args: spec_args,
                    template_params: params,
                    return_type: ret_type,
                    params: func_params,
                    body,
                });
            }
        }

        self.parse_function_or_var(ret_type, name, params)
    }

    /// Check if the next tokens look like ClassName<Args> { (specialization pattern)
    fn is_template_specialization_ahead(&self) -> bool {
        // We're at class/struct, peek is Identifier
        // Check if peek_at(2) is < (specialization args)
        *self.peek_at(2) == CppToken::Lt
    }

    /// Parse template class specialization: template<> class Foo<int> { ... }
    /// or partial: template<typename T> class Foo<T*> { ... }
    fn parse_template_class_specialization(
        &mut self,
        template_params: Vec<CppTemplateParam>,
        is_struct: bool,
    ) -> Result<CppTopLevel, String> {
        self.advance(); // skip class/struct
        let name = self.expect_identifier()?;
        self.type_names.insert(name.clone());

        // Parse specialization args: <int> or <T*>
        let spec_args = self.try_parse_template_args()
            .unwrap_or_default();

        // Parse optional base classes
        if self.eat(&CppToken::Colon) {
            // Skip base classes for specializations (simplified)
            while *self.current() != CppToken::LBrace && *self.current() != CppToken::Eof {
                self.advance();
            }
        }

        // Parse class body
        self.expect(&CppToken::LBrace)?;
        let members = self.parse_class_members(is_struct)?;
        self.expect(&CppToken::RBrace)?;
        self.eat(&CppToken::Semicolon);

        Ok(CppTopLevel::TemplateSpecialization {
            name,
            specialized_args: spec_args,
            template_params,
            members,
            is_struct,
        })
    }

    fn parse_template_params(&mut self) -> Result<Vec<CppTemplateParam>, String> {
        let mut params = Vec::new();
        if *self.current() == CppToken::Gt {
            return Ok(params);
        }
        loop {
            if *self.current() == CppToken::Typename || *self.current() == CppToken::Class {
                self.advance();
                if *self.current() == CppToken::Ellipsis {
                    self.advance();
                    let name = if let CppToken::Identifier(_) = self.current() {
                        self.expect_identifier()?
                    } else {
                        "Args".to_string()
                    };
                    params.push(CppTemplateParam::VariadicType { name });
                } else {
                    let name = if let CppToken::Identifier(_) = self.current() {
                        self.expect_identifier()?
                    } else {
                        format!("T{}", params.len())
                    };
                    let default_type = if self.eat(&CppToken::Assign) {
                        Some(self.parse_type()?)
                    } else {
                        None
                    };
                    params.push(CppTemplateParam::TypeParam { name, default_type });
                }
            } else {
                // Non-type template parameter
                let pt = self.parse_type()?;
                let name = self.expect_identifier()?;
                let default_value = if self.eat(&CppToken::Assign) {
                    // Use parse_primary to avoid consuming > as greater-than
                    Some(self.parse_primary()?)
                } else {
                    None
                };
                params.push(CppTemplateParam::NonTypeParam {
                    param_type: pt,
                    name,
                    default_value,
                });
            }
            if !self.eat(&CppToken::Comma) {
                break;
            }
        }
        Ok(params)
    }

    fn parse_static_assert(&mut self) -> Result<CppTopLevel, String> {
        self.advance(); // static_assert
        self.expect(&CppToken::LParen)?;
        let condition = self.parse_expression()?;
        let message = if self.eat(&CppToken::Comma) {
            if let CppToken::StringLiteral(s) = self.current().clone() {
                self.advance();
                Some(s)
            } else {
                None
            }
        } else {
            None
        };
        self.expect(&CppToken::RParen)?;
        self.expect(&CppToken::Semicolon)?;
        Ok(CppTopLevel::StaticAssert { condition, message })
    }

    fn parse_operator_name(&mut self) -> Result<String, String> {
        let name = match self.current() {
            CppToken::Plus => "+",
            CppToken::Minus => "-",
            CppToken::Star => "*",
            CppToken::Slash => "/",
            CppToken::Percent => "%",
            CppToken::Eq => "==",
            CppToken::Ne => "!=",
            CppToken::Lt => "<",
            CppToken::Gt => ">",
            CppToken::Le => "<=",
            CppToken::Ge => ">=",
            CppToken::And => "&&",
            CppToken::Or => "||",
            CppToken::Not => "!",
            CppToken::Amp => "&",
            CppToken::Pipe => "|",
            CppToken::Caret => "^",
            CppToken::Tilde => "~",
            CppToken::Shl => "<<",
            CppToken::Shr => ">>",
            CppToken::Assign => "=",
            CppToken::PlusAssign => "+=",
            CppToken::MinusAssign => "-=",
            CppToken::StarAssign => "*=",
            CppToken::Increment => "++",
            CppToken::Decrement => "--",
            CppToken::Arrow => "->",
            CppToken::LParen => {
                self.advance();
                self.expect(&CppToken::RParen)?;
                return Ok("()".to_string());
            }
            CppToken::LBracket => {
                self.advance();
                self.expect(&CppToken::RBracket)?;
                return Ok("[]".to_string());
            }
            CppToken::Spaceship => "<=>",
            _ => return Err(format!("Unknown operator {:?}", self.current())),
        };
        let s = name.to_string();
        self.advance();
        Ok(s)
    }

    // ========== Statement parsing ==========

    fn parse_block_stmts(&mut self) -> Result<Vec<CppStmt>, String> {
        self.expect(&CppToken::LBrace)?;
        let mut stmts = Vec::new();
        while *self.current() != CppToken::RBrace && *self.current() != CppToken::Eof {
            let line = self.current_line();
            stmts.push(CppStmt::LineMarker(line));
            stmts.push(self.parse_statement()?);
        }
        self.expect(&CppToken::RBrace)?;
        Ok(stmts)
    }

    fn parse_statement(&mut self) -> Result<CppStmt, String> {
        match self.current().clone() {
            CppToken::LBrace => {
                let stmts = self.parse_block_stmts()?;
                Ok(CppStmt::Block(stmts))
            }
            CppToken::Return => {
                self.advance();
                if *self.current() == CppToken::Semicolon {
                    self.advance();
                    Ok(CppStmt::Return(None))
                } else {
                    let expr = self.parse_expression()?;
                    self.expect(&CppToken::Semicolon)?;
                    Ok(CppStmt::Return(Some(expr)))
                }
            }
            CppToken::If => self.parse_if(),
            CppToken::While => self.parse_while(),
            CppToken::Do => self.parse_do_while(),
            CppToken::For => self.parse_for(),
            CppToken::Switch => self.parse_switch(),
            CppToken::Break => {
                self.advance();
                self.expect(&CppToken::Semicolon)?;
                Ok(CppStmt::Break)
            }
            CppToken::Continue => {
                self.advance();
                self.expect(&CppToken::Semicolon)?;
                Ok(CppStmt::Continue)
            }
            CppToken::Goto => {
                self.advance();
                let label = self.expect_identifier()?;
                self.expect(&CppToken::Semicolon)?;
                Ok(CppStmt::Goto(label))
            }
            CppToken::Try => self.parse_try(),
            CppToken::Throw => {
                self.advance();
                if *self.current() == CppToken::Semicolon {
                    self.advance();
                    Ok(CppStmt::Throw(None))
                } else {
                    let expr = self.parse_expression()?;
                    self.expect(&CppToken::Semicolon)?;
                    Ok(CppStmt::Throw(Some(expr)))
                }
            }
            CppToken::Co_return => {
                self.advance();
                if *self.current() == CppToken::Semicolon {
                    self.advance();
                    Ok(CppStmt::CoReturn(None))
                } else {
                    let expr = self.parse_expression()?;
                    self.expect(&CppToken::Semicolon)?;
                    Ok(CppStmt::CoReturn(Some(expr)))
                }
            }
            CppToken::Semicolon => {
                self.advance();
                Ok(CppStmt::Empty)
            }
            _ => {
                // Variable declaration or expression statement
                if self.is_type_start() && !self.is_cast_expr() {
                    self.parse_var_decl_stmt()
                } else {
                    // Label check: ident :
                    if let CppToken::Identifier(ref name) = self.current().clone() {
                        if *self.peek() == CppToken::Colon {
                            let label = name.clone();
                            self.advance();
                            self.advance();
                            let stmt = self.parse_statement()?;
                            return Ok(CppStmt::Label(label, Box::new(stmt)));
                        }
                    }
                    let expr = self.parse_expression()?;
                    self.expect(&CppToken::Semicolon)?;
                    Ok(CppStmt::Expr(expr))
                }
            }
        }
    }

    fn is_cast_expr(&self) -> bool {
        false // declarations take priority
    }

    fn parse_var_decl_stmt(&mut self) -> Result<CppStmt, String> {
        let type_spec = self.parse_type()?;
        let mut declarators = Vec::new();
        let name = self.expect_identifier()?;
        let first = self.parse_declarator_rest(name)?;
        declarators.push(first);
        while self.eat(&CppToken::Comma) {
            let n = self.expect_identifier()?;
            let d = self.parse_declarator_rest(n)?;
            declarators.push(d);
        }
        self.expect(&CppToken::Semicolon)?;
        Ok(CppStmt::VarDecl {
            type_spec,
            declarators,
        })
    }

    fn parse_if(&mut self) -> Result<CppStmt, String> {
        self.advance(); // skip if
                        // C++17 constexpr if
        let is_constexpr = self.eat(&CppToken::Constexpr);
        self.expect(&CppToken::LParen)?;
        let condition = self.parse_expression()?;
        self.expect(&CppToken::RParen)?;
        let then_body = Box::new(self.parse_statement()?);
        let else_body = if self.eat(&CppToken::Else) {
            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };
        Ok(CppStmt::If {
            init: None,
            condition,
            then_body,
            else_body,
            is_constexpr,
        })
    }

    fn parse_while(&mut self) -> Result<CppStmt, String> {
        self.advance();
        self.expect(&CppToken::LParen)?;
        let condition = self.parse_expression()?;
        self.expect(&CppToken::RParen)?;
        let body = Box::new(self.parse_statement()?);
        Ok(CppStmt::While { condition, body })
    }

    fn parse_do_while(&mut self) -> Result<CppStmt, String> {
        self.advance();
        let body = Box::new(self.parse_statement()?);
        self.expect(&CppToken::While)?;
        self.expect(&CppToken::LParen)?;
        let condition = self.parse_expression()?;
        self.expect(&CppToken::RParen)?;
        self.expect(&CppToken::Semicolon)?;
        Ok(CppStmt::DoWhile { body, condition })
    }

    fn parse_for(&mut self) -> Result<CppStmt, String> {
        self.advance();
        self.expect(&CppToken::LParen)?;

        // Range-for: for (type var : iterable)
        // Try to detect range-for by looking ahead
        let save = self.pos;
        if self.is_type_start() {
            let type_spec = self.parse_type();
            if let Ok(ts) = type_spec {
                if let Ok(name) = self.expect_identifier() {
                    if self.eat(&CppToken::Colon) {
                        let iterable = self.parse_expression()?;
                        self.expect(&CppToken::RParen)?;
                        let body = Box::new(self.parse_statement()?);
                        return Ok(CppStmt::RangeFor {
                            type_spec: ts,
                            name,
                            iterable,
                            body,
                        });
                    }
                }
            }
            self.pos = save;
        }

        // Regular for
        let init = if *self.current() == CppToken::Semicolon {
            self.advance();
            None
        } else {
            let stmt = if self.is_type_start() {
                self.parse_var_decl_stmt()?
            } else {
                let expr = self.parse_expression()?;
                self.expect(&CppToken::Semicolon)?;
                CppStmt::Expr(expr)
            };
            Some(Box::new(stmt))
        };
        let condition = if *self.current() == CppToken::Semicolon {
            None
        } else {
            Some(self.parse_expression()?)
        };
        self.expect(&CppToken::Semicolon)?;
        let increment = if *self.current() == CppToken::RParen {
            None
        } else {
            Some(self.parse_expression()?)
        };
        self.expect(&CppToken::RParen)?;
        let body = Box::new(self.parse_statement()?);
        Ok(CppStmt::For {
            init,
            condition,
            increment,
            body,
        })
    }

    fn parse_switch(&mut self) -> Result<CppStmt, String> {
        self.advance();
        self.expect(&CppToken::LParen)?;
        let expr = self.parse_expression()?;
        self.expect(&CppToken::RParen)?;
        self.expect(&CppToken::LBrace)?;
        let mut cases = Vec::new();
        let mut default = None;
        while *self.current() != CppToken::RBrace && *self.current() != CppToken::Eof {
            if self.eat(&CppToken::Case) {
                let value = self.parse_expression()?;
                self.expect(&CppToken::Colon)?;
                let mut body = Vec::new();
                while !matches!(
                    self.current(),
                    CppToken::Case | CppToken::Default | CppToken::RBrace
                ) {
                    body.push(self.parse_statement()?);
                }
                cases.push(CppSwitchCase { value, body });
            } else if self.eat(&CppToken::Default) {
                self.expect(&CppToken::Colon)?;
                let mut body = Vec::new();
                while !matches!(
                    self.current(),
                    CppToken::Case | CppToken::Default | CppToken::RBrace
                ) {
                    body.push(self.parse_statement()?);
                }
                default = Some(body);
            } else {
                return Err(format!(
                    "Expected case or default, got {:?}",
                    self.current()
                ));
            }
        }
        self.expect(&CppToken::RBrace)?;
        Ok(CppStmt::Switch {
            expr,
            cases,
            default,
        })
    }

    fn parse_try(&mut self) -> Result<CppStmt, String> {
        self.advance(); // try
        let body = self.parse_block_stmts()?;
        let mut catches = Vec::new();
        while *self.current() == CppToken::Catch {
            self.advance();
            self.expect(&CppToken::LParen)?;
            let (param_type, param_name) = if *self.current() == CppToken::Ellipsis {
                self.advance();
                (None, None)
            } else {
                let t = self.parse_type()?;
                let n = if let CppToken::Identifier(_) = self.current() {
                    Some(self.expect_identifier()?)
                } else {
                    None
                };
                (Some(t), n)
            };
            self.expect(&CppToken::RParen)?;
            let catch_body = self.parse_block_stmts()?;
            catches.push(CppCatch {
                param_type,
                param_name,
                body: catch_body,
            });
        }
        Ok(CppStmt::Try { body, catches })
    }

    // ========== Expression parsing ==========

    fn parse_expression(&mut self) -> Result<CppExpr, String> {
        self.parse_assignment_expr()
    }

    fn parse_assignment_expr(&mut self) -> Result<CppExpr, String> {
        let lhs = self.parse_ternary()?;

        match self.current() {
            CppToken::Assign => {
                self.advance();
                let rhs = self.parse_assignment_expr()?;
                Ok(CppExpr::Assign {
                    target: Box::new(lhs),
                    value: Box::new(rhs),
                })
            }
            CppToken::PlusAssign
            | CppToken::MinusAssign
            | CppToken::StarAssign
            | CppToken::SlashAssign
            | CppToken::PercentAssign
            | CppToken::AmpAssign
            | CppToken::PipeAssign
            | CppToken::CaretAssign
            | CppToken::ShlAssign
            | CppToken::ShrAssign => {
                let op = match self.advance() {
                    CppToken::PlusAssign => CppBinOp::Add,
                    CppToken::MinusAssign => CppBinOp::Sub,
                    CppToken::StarAssign => CppBinOp::Mul,
                    CppToken::SlashAssign => CppBinOp::Div,
                    CppToken::PercentAssign => CppBinOp::Mod,
                    CppToken::AmpAssign => CppBinOp::BitAnd,
                    CppToken::PipeAssign => CppBinOp::BitOr,
                    CppToken::CaretAssign => CppBinOp::BitXor,
                    CppToken::ShlAssign => CppBinOp::Shl,
                    CppToken::ShrAssign => CppBinOp::Shr,
                    _ => unreachable!(),
                };
                let rhs = self.parse_assignment_expr()?;
                Ok(CppExpr::CompoundAssign {
                    op,
                    target: Box::new(lhs),
                    value: Box::new(rhs),
                })
            }
            _ => Ok(lhs),
        }
    }

    fn parse_ternary(&mut self) -> Result<CppExpr, String> {
        let cond = self.parse_logical_or()?;
        if self.eat(&CppToken::Question) {
            let then_expr = self.parse_expression()?;
            self.expect(&CppToken::Colon)?;
            let else_expr = self.parse_ternary()?;
            Ok(CppExpr::Ternary {
                condition: Box::new(cond),
                then_expr: Box::new(then_expr),
                else_expr: Box::new(else_expr),
            })
        } else {
            Ok(cond)
        }
    }

    fn parse_logical_or(&mut self) -> Result<CppExpr, String> {
        let mut left = self.parse_logical_and()?;
        while self.eat(&CppToken::Or) {
            let right = self.parse_logical_and()?;
            left = CppExpr::BinaryOp {
                op: CppBinOp::Or,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_logical_and(&mut self) -> Result<CppExpr, String> {
        let mut left = self.parse_bitwise_or()?;
        while self.eat(&CppToken::And) {
            let right = self.parse_bitwise_or()?;
            left = CppExpr::BinaryOp {
                op: CppBinOp::And,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_bitwise_or(&mut self) -> Result<CppExpr, String> {
        let mut left = self.parse_bitwise_xor()?;
        while self.eat(&CppToken::Pipe) {
            let right = self.parse_bitwise_xor()?;
            left = CppExpr::BinaryOp {
                op: CppBinOp::BitOr,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_bitwise_xor(&mut self) -> Result<CppExpr, String> {
        let mut left = self.parse_bitwise_and()?;
        while self.eat(&CppToken::Caret) {
            let right = self.parse_bitwise_and()?;
            left = CppExpr::BinaryOp {
                op: CppBinOp::BitXor,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_bitwise_and(&mut self) -> Result<CppExpr, String> {
        let mut left = self.parse_equality()?;
        while *self.current() == CppToken::Amp {
            self.advance();
            let right = self.parse_equality()?;
            left = CppExpr::BinaryOp {
                op: CppBinOp::BitAnd,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<CppExpr, String> {
        let mut left = self.parse_relational()?;
        loop {
            let op = match self.current() {
                CppToken::Eq => CppBinOp::Eq,
                CppToken::Ne => CppBinOp::Ne,
                _ => break,
            };
            self.advance();
            let right = self.parse_relational()?;
            left = CppExpr::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_relational(&mut self) -> Result<CppExpr, String> {
        let mut left = self.parse_shift()?;
        loop {
            let op = match self.current() {
                CppToken::Lt => CppBinOp::Lt,
                CppToken::Gt => CppBinOp::Gt,
                CppToken::Le => CppBinOp::Le,
                CppToken::Ge => CppBinOp::Ge,
                CppToken::Spaceship => CppBinOp::Spaceship,
                _ => break,
            };
            self.advance();
            let right = self.parse_shift()?;
            left = CppExpr::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_shift(&mut self) -> Result<CppExpr, String> {
        let mut left = self.parse_additive()?;
        loop {
            let op = match self.current() {
                CppToken::Shl => CppBinOp::Shl,
                CppToken::Shr => CppBinOp::Shr,
                _ => break,
            };
            self.advance();
            let right = self.parse_additive()?;
            left = CppExpr::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_additive(&mut self) -> Result<CppExpr, String> {
        let mut left = self.parse_multiplicative()?;
        loop {
            let op = match self.current() {
                CppToken::Plus => CppBinOp::Add,
                CppToken::Minus => CppBinOp::Sub,
                _ => break,
            };
            self.advance();
            let right = self.parse_multiplicative()?;
            left = CppExpr::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<CppExpr, String> {
        let mut left = self.parse_unary()?;
        loop {
            let op = match self.current() {
                CppToken::Star => CppBinOp::Mul,
                CppToken::Slash => CppBinOp::Div,
                CppToken::Percent => CppBinOp::Mod,
                _ => break,
            };
            self.advance();
            let right = self.parse_unary()?;
            left = CppExpr::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<CppExpr, String> {
        match self.current().clone() {
            CppToken::Minus => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(CppExpr::UnaryOp {
                    op: CppUnaryOp::Neg,
                    expr: Box::new(expr),
                    is_prefix: true,
                })
            }
            CppToken::Not => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(CppExpr::UnaryOp {
                    op: CppUnaryOp::Not,
                    expr: Box::new(expr),
                    is_prefix: true,
                })
            }
            CppToken::Tilde => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(CppExpr::UnaryOp {
                    op: CppUnaryOp::BitNot,
                    expr: Box::new(expr),
                    is_prefix: true,
                })
            }
            CppToken::Increment => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(CppExpr::UnaryOp {
                    op: CppUnaryOp::PreInc,
                    expr: Box::new(expr),
                    is_prefix: true,
                })
            }
            CppToken::Decrement => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(CppExpr::UnaryOp {
                    op: CppUnaryOp::PreDec,
                    expr: Box::new(expr),
                    is_prefix: true,
                })
            }
            CppToken::Star => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(CppExpr::Deref(Box::new(expr)))
            }
            CppToken::Amp => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(CppExpr::AddressOf(Box::new(expr)))
            }
            CppToken::Sizeof => {
                self.advance();
                self.expect(&CppToken::LParen)?;
                // Try type first
                if self.is_type_start() {
                    let t = self.parse_type()?;
                    self.expect(&CppToken::RParen)?;
                    Ok(CppExpr::SizeOf(CppSizeOfArg::Type(t)))
                } else {
                    let expr = self.parse_expression()?;
                    self.expect(&CppToken::RParen)?;
                    Ok(CppExpr::SizeOf(CppSizeOfArg::Expr(Box::new(expr))))
                }
            }
            CppToken::New => {
                self.advance();
                let is_array = false;
                let t = self.parse_type()?;
                if *self.current() == CppToken::LParen {
                    self.advance();
                    let mut args = Vec::new();
                    if *self.current() != CppToken::RParen {
                        args.push(self.parse_expression()?);
                        while self.eat(&CppToken::Comma) {
                            args.push(self.parse_expression()?);
                        }
                    }
                    self.expect(&CppToken::RParen)?;
                    Ok(CppExpr::New {
                        type_name: t,
                        args,
                        is_array,
                        array_size: None,
                    })
                } else if *self.current() == CppToken::LBracket {
                    self.advance();
                    let size = self.parse_expression()?;
                    self.expect(&CppToken::RBracket)?;
                    Ok(CppExpr::New {
                        type_name: t,
                        args: Vec::new(),
                        is_array: true,
                        array_size: Some(Box::new(size)),
                    })
                } else {
                    Ok(CppExpr::New {
                        type_name: t,
                        args: Vec::new(),
                        is_array,
                        array_size: None,
                    })
                }
            }
            CppToken::Delete => {
                self.advance();
                let is_array = if *self.current() == CppToken::LBracket {
                    self.advance();
                    self.expect(&CppToken::RBracket)?;
                    true
                } else {
                    false
                };
                let expr = self.parse_unary()?;
                Ok(CppExpr::Delete {
                    expr: Box::new(expr),
                    is_array,
                })
            }
            CppToken::Throw => {
                self.advance();
                if *self.current() == CppToken::Semicolon {
                    Ok(CppExpr::Throw(None))
                } else {
                    let expr = self.parse_assignment_expr()?;
                    Ok(CppExpr::Throw(Some(Box::new(expr))))
                }
            }
            CppToken::Co_await => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(CppExpr::CoAwait(Box::new(expr)))
            }
            // C++ style casts
            CppToken::StaticCast
            | CppToken::DynamicCast
            | CppToken::ConstCast
            | CppToken::ReinterpretCast => {
                let kind = match self.advance() {
                    CppToken::StaticCast => CppCastKind::StaticCast,
                    CppToken::DynamicCast => CppCastKind::DynamicCast,
                    CppToken::ConstCast => CppCastKind::ConstCast,
                    CppToken::ReinterpretCast => CppCastKind::ReinterpretCast,
                    _ => unreachable!(),
                };
                self.expect(&CppToken::Lt)?;
                let target_type = self.parse_type()?;
                self.expect(&CppToken::Gt)?;
                self.expect(&CppToken::LParen)?;
                let expr = self.parse_expression()?;
                self.expect(&CppToken::RParen)?;
                Ok(CppExpr::Cast {
                    cast_type: kind,
                    target_type,
                    expr: Box::new(expr),
                })
            }
            // C-style cast or parenthesized expr
            CppToken::LParen => {
                // Try cast
                let save = self.pos;
                self.advance(); // skip (
                if self.is_type_start() {
                    if let Ok(t) = self.parse_type() {
                        if self.eat(&CppToken::RParen) {
                            let expr = self.parse_unary()?;
                            return Ok(CppExpr::Cast {
                                cast_type: CppCastKind::CStyle,
                                target_type: t,
                                expr: Box::new(expr),
                            });
                        }
                    }
                }
                // Not a cast — parenthesized expression
                self.pos = save;
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(&CppToken::RParen)?;
                self.parse_postfix_on(expr)
            }
            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> Result<CppExpr, String> {
        let mut expr = self.parse_primary()?;
        expr = self.parse_postfix_on(expr)?;
        Ok(expr)
    }

    fn parse_postfix_on(&mut self, mut expr: CppExpr) -> Result<CppExpr, String> {
        loop {
            match self.current() {
                // Template function call: func<Type>(args)
                // Speculatively try template args when identifier is followed by <
                CppToken::Lt => {
                    if let CppExpr::Identifier(_) | CppExpr::ScopedIdentifier { .. } = &expr {
                        let save_pos = self.pos;
                        if let Ok(targs) = self.try_parse_template_args() {
                            // Must be followed by ( to be a template call
                            if *self.current() == CppToken::LParen {
                                // Mangle the callee name with template args
                                let callee_name = match &expr {
                                    CppExpr::Identifier(n) => n.clone(),
                                    CppExpr::ScopedIdentifier { scope, name } => {
                                        format!("{}::{}", scope.join("::"), name)
                                    }
                                    _ => unreachable!(),
                                };
                                let mangled = format!("{}<{}>", callee_name,
                                    targs.iter().map(|t| format!("{:?}", t)).collect::<Vec<_>>().join(", "));
                                expr = CppExpr::Identifier(mangled);
                                // The ( will be consumed in the LParen arm below
                                continue;
                            }
                        }
                        // Not a template call — backtrack
                        self.pos = save_pos;
                        break;
                    } else {
                        break;
                    }
                }
                CppToken::LBracket => {
                    self.advance();
                    let index = self.parse_expression()?;
                    self.expect(&CppToken::RBracket)?;
                    expr = CppExpr::Index {
                        object: Box::new(expr),
                        index: Box::new(index),
                    };
                }
                CppToken::LParen => {
                    self.advance();
                    let mut args = Vec::new();
                    if *self.current() != CppToken::RParen {
                        args.push(self.parse_assignment_expr()?);
                        while self.eat(&CppToken::Comma) {
                            args.push(self.parse_assignment_expr()?);
                        }
                    }
                    self.expect(&CppToken::RParen)?;
                    expr = CppExpr::Call {
                        callee: Box::new(expr),
                        args,
                    };
                }
                CppToken::Dot => {
                    self.advance();
                    let member = self.expect_identifier()?;
                    expr = CppExpr::MemberAccess {
                        object: Box::new(expr),
                        member,
                    };
                }
                CppToken::Arrow => {
                    self.advance();
                    let member = self.expect_identifier()?;
                    expr = CppExpr::ArrowAccess {
                        pointer: Box::new(expr),
                        member,
                    };
                }
                CppToken::Increment => {
                    self.advance();
                    expr = CppExpr::UnaryOp {
                        op: CppUnaryOp::PostInc,
                        expr: Box::new(expr),
                        is_prefix: false,
                    };
                }
                CppToken::Decrement => {
                    self.advance();
                    expr = CppExpr::UnaryOp {
                        op: CppUnaryOp::PostDec,
                        expr: Box::new(expr),
                        is_prefix: false,
                    };
                }
                CppToken::Scope => {
                    self.advance();
                    let member = self.expect_identifier()?;
                    // Turn into scoped identifier
                    if let CppExpr::Identifier(ref scope_name) = expr {
                        expr = CppExpr::ScopedIdentifier {
                            scope: vec![scope_name.clone()],
                            name: member,
                        };
                    } else {
                        expr = CppExpr::MemberAccess {
                            object: Box::new(expr),
                            member,
                        };
                    }
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<CppExpr, String> {
        match self.current().clone() {
            CppToken::IntLiteral(n) => {
                self.advance();
                Ok(CppExpr::IntLiteral(n))
            }
            CppToken::UIntLiteral(n) => {
                self.advance();
                Ok(CppExpr::UIntLiteral(n))
            }
            CppToken::FloatLiteral(f) => {
                self.advance();
                Ok(CppExpr::FloatLiteral(f))
            }
            CppToken::StringLiteral(s) => {
                let mut result = s;
                self.advance();
                // Concatenate adjacent string literals
                while let CppToken::StringLiteral(ref next) = self.current().clone() {
                    result.push_str(next);
                    self.advance();
                }
                Ok(CppExpr::StringLiteral(result))
            }
            CppToken::CharLiteral(c) => {
                self.advance();
                Ok(CppExpr::CharLiteral(c))
            }
            CppToken::True => {
                self.advance();
                Ok(CppExpr::BoolLiteral(true))
            }
            CppToken::False => {
                self.advance();
                Ok(CppExpr::BoolLiteral(false))
            }
            CppToken::Nullptr => {
                self.advance();
                Ok(CppExpr::NullptrLiteral)
            }
            CppToken::This => {
                self.advance();
                Ok(CppExpr::This)
            }
            CppToken::Identifier(name) => {
                self.advance();
                Ok(CppExpr::Identifier(name))
            }
            CppToken::LBrace => {
                // Initializer list (C++11/C++20 designated initializers)
                self.advance();
                let mut items = Vec::new();
                if *self.current() != CppToken::RBrace {
                    loop {
                        // C++20 designated initializer: .field = expr
                        if *self.current() == CppToken::Dot {
                            self.advance(); // skip .
                            let _field = self.expect_identifier()?;
                            self.expect(&CppToken::Assign)?;
                        }
                        items.push(self.parse_assignment_expr()?);
                        if !self.eat(&CppToken::Comma) {
                            break;
                        }
                        if *self.current() == CppToken::RBrace {
                            break;
                        }
                    }
                }
                self.expect(&CppToken::RBrace)?;
                Ok(CppExpr::InitList(items))
            }
            CppToken::LBracket => {
                // Lambda: [captures](params) { body }
                self.parse_lambda()
            }
            other => Err(format!(
                "Unexpected token in expression: {:?} at pos {}",
                other, self.pos
            )),
        }
    }

    fn parse_lambda(&mut self) -> Result<CppExpr, String> {
        self.expect(&CppToken::LBracket)?;
        let mut captures = Vec::new();
        if *self.current() != CppToken::RBracket {
            loop {
                match self.current() {
                    CppToken::Assign => {
                        self.advance();
                        captures.push(CppCapture::DefaultByValue);
                    }
                    CppToken::Amp => {
                        self.advance();
                        if let CppToken::Identifier(name) = self.current().clone() {
                            self.advance();
                            captures.push(CppCapture::ByRef(name));
                        } else {
                            captures.push(CppCapture::DefaultByRef);
                        }
                    }
                    CppToken::This => {
                        self.advance();
                        captures.push(CppCapture::ThisByRef);
                    }
                    CppToken::Identifier(ref name) => {
                        let n = name.clone();
                        self.advance();
                        captures.push(CppCapture::ByValue(n));
                    }
                    _ => break,
                }
                if !self.eat(&CppToken::Comma) {
                    break;
                }
            }
        }
        self.expect(&CppToken::RBracket)?;

        // Parameters (optional)
        let params = if *self.current() == CppToken::LParen {
            self.advance();
            let p = self.parse_param_list()?;
            self.expect(&CppToken::RParen)?;
            p
        } else {
            Vec::new()
        };

        // Return type (optional)
        let return_type = if *self.current() == CppToken::Arrow {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };

        // Mutable/noexcept qualifiers
        self.eat(&CppToken::Mutable);
        self.eat(&CppToken::Noexcept);

        let body = self.parse_block_stmts()?;

        Ok(CppExpr::Lambda {
            captures,
            params,
            return_type,
            body,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::cpp_lexer::CppLexer;
    use super::*;

    fn parse(src: &str) -> CppTranslationUnit {
        let (tokens, lines) = CppLexer::new(src).tokenize();
        CppParser::new(tokens, lines).parse_translation_unit().unwrap()
    }

    #[test]
    fn test_simple_function() {
        let unit = parse("int main() { return 0; }");
        assert_eq!(unit.declarations.len(), 1);
        match &unit.declarations[0] {
            CppTopLevel::FunctionDef { name, .. } => assert_eq!(name, "main"),
            _ => panic!("Expected FunctionDef"),
        }
    }

    #[test]
    fn test_class_def() {
        let unit = parse(
            r#"
            class Animal {
            public:
                virtual void speak() = 0;
                int age;
            };
        "#,
        );
        assert_eq!(unit.declarations.len(), 1);
        match &unit.declarations[0] {
            CppTopLevel::ClassDef { name, members, .. } => {
                assert_eq!(name, "Animal");
                assert!(members.len() >= 2);
            }
            _ => panic!("Expected ClassDef"),
        }
    }

    #[test]
    fn test_template_function() {
        let unit = parse(
            r#"
            template<typename T>
            T add(T a, T b) { return a + b; }
        "#,
        );
        assert_eq!(unit.declarations.len(), 1);
        match &unit.declarations[0] {
            CppTopLevel::FunctionDef {
                name,
                template_params,
                ..
            } => {
                assert_eq!(name, "add");
                assert_eq!(template_params.len(), 1);
            }
            _ => panic!("Expected FunctionDef"),
        }
    }

    #[test]
    fn test_namespace() {
        let unit = parse(
            r#"
            namespace math {
                int add(int a, int b) { return a + b; }
            }
        "#,
        );
        assert_eq!(unit.declarations.len(), 1);
        match &unit.declarations[0] {
            CppTopLevel::Namespace { name, declarations } => {
                assert_eq!(name, "math");
                assert_eq!(declarations.len(), 1);
            }
            _ => panic!("Expected Namespace"),
        }
    }

    #[test]
    fn test_enum_class() {
        let unit = parse(
            r#"
            enum class Color : int { Red = 0, Green, Blue };
        "#,
        );
        match &unit.declarations[0] {
            CppTopLevel::EnumDef {
                name,
                is_class,
                values,
                ..
            } => {
                assert_eq!(name, "Color");
                assert!(*is_class);
                assert_eq!(values.len(), 3);
            }
            _ => panic!("Expected EnumDef"),
        }
    }

    #[test]
    fn test_using_alias() {
        let unit = parse("using StringVec = int;");
        match &unit.declarations[0] {
            CppTopLevel::TypeAlias { new_name, .. } => {
                assert_eq!(new_name, "StringVec");
            }
            _ => panic!("Expected TypeAlias"),
        }
    }
}
