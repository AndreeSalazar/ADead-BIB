// ============================================================
// ADead-BIB — C++ Language Driver
// ============================================================
// Full C++17/20 compilation pipeline:
//   Phase 0: Preprocessor  (#include, #define, #ifdef)
//   Phase 1: Lexer         (tokenization)
//   Phase 2: Parser        (C++ AST)
//   Phase 3: Semantic      (symbol table, class hierarchy)
//   Phase 4: UB Detection  (C++ specific: dangling refs, slicing, etc.)
//   Phase 5: IR Generation (CppAST → Program IR)
//   Phase 6: Code Gen      (x86-64 machine code)
//   Phase 7: PE Output     (Windows executable)
//
// Sin GCC. Sin LLVM. Sin Clang. Solo ADead-BIB. 💀🦈
// ============================================================

use crate::cli::term;
use adeb_backend_x64::isa::isa_compiler::{IsaCompiler, Target};
use adeb_core::ast::Program;
use adeb_frontend_cpp::ast::*;
use adeb_frontend_cpp::lower::cpp_to_ir::CppToIR;
use adeb_frontend_cpp::parse::lexer::CppLexer;
use adeb_frontend_cpp::parse::parser::CppParser;
use adeb_frontend_cpp::preprocessor::CppPreprocessor;
use std::fs;

// ── Public types ────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct CppPipelineArtifacts {
    pub preprocessed: String,
    pub tokens: Vec<adeb_frontend_cpp::parse::lexer::CppToken>,
    pub token_lines: Vec<usize>,
    pub unit: CppTranslationUnit,
    pub ub_report: CppUBReport,
    pub program: Program,
}

// ── C++ UB Detection ────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum CppUBKind {
    // Inherited from C
    NullPointerDereference,
    DivisionByZero,
    ShiftOverflow,
    SignedIntegerOverflow,
    // C++ specific
    DanglingReference,
    ObjectSlicing,
    UseAfterMove,
    DoubleFree,
    DeleteMismatch,       // delete vs delete[]
    VirtualInConstructor, // calling virtual in ctor/dtor
    UninitialisedMember,
    ThrowInDestructor,
    InfiniteRecursion,
    NarrowingConversion,
}

#[derive(Debug, Clone)]
pub struct CppUBWarning {
    pub kind: CppUBKind,
    pub severity: &'static str,
    pub message: String,
    pub location: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct CppUBReport {
    pub warnings: Vec<CppUBWarning>,
}

impl CppUBReport {
    pub fn has_errors(&self) -> bool {
        self.warnings.iter().any(|w| w.severity == "error")
    }
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
}

// ── Pipeline ────────────────────────────────────────────────

pub fn compile_cpp_pipeline(source: &str, strict: bool) -> Result<CppPipelineArtifacts, String> {
    // C++ is IMPLICITLY STRICT — bits are respected, UB = error
    let effective_strict = true; // C++ is always strict in ADead-BIB

    // Phase 0: Preprocess
    let mut pp = CppPreprocessor::new();
    let preprocessed = pp.process(source);

    // Phase 1: Lex
    let (tokens, token_lines) = CppLexer::new(&preprocessed).tokenize();

    // Phase 2: Parse
    let unit = CppParser::new(tokens.clone(), token_lines.clone()).parse_translation_unit()?;

    // Phase 3+4: UB Detection (always strict)
    let mut ub_report = detect_cpp_ub(&unit);
    detect_cpp_strict_violations(&unit, &mut ub_report);

    // In strict mode (always for C++), all warnings → errors
    if effective_strict || strict {
        for w in &mut ub_report.warnings {
            if w.severity == "warning" {
                w.severity = "error";
            }
        }
    }

    // Phase 5: Lower to IR
    let mut lower = CppToIR::new();
    let program = lower.convert(&unit)?;

    Ok(CppPipelineArtifacts {
        preprocessed,
        tokens,
        token_lines,
        unit,
        ub_report,
        program,
    })
}

/// Compile a .cpp file to a PE executable
pub fn compile_cpp_file(
    input_file: &str,
    output_file: &str,
    step_mode: bool,
    strict: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // C++ is ALWAYS strict — bits are respected
    println!("  ADead-BIB C++ Compiler v2.0 [STRICT — bits respected]");
    println!("   Source: {}", input_file);
    println!("   Target: {}", output_file);

    let source = fs::read_to_string(input_file)
        .map_err(|e| format!("Cannot read '{}': {}", input_file, e))?;

    let pipeline = compile_cpp_pipeline(&source, strict)
        .map_err(|e| format!("C++ pipeline error: {}", e))?;

    if step_mode {
        print_cpp_step_mode(input_file, &source, &pipeline);
    } else {
        println!("   Phase 1: Parsing C++...");
    }

    // Report UB warnings
    if pipeline.ub_report.has_warnings() {
        println!();
        for w in &pipeline.ub_report.warnings {
            let loc = match &w.location {
                Some(l) => format!(" in {}", l),
                None => String::new(),
            };
            match w.severity {
                "error" => eprintln!("   {} UB{}: {}", term::error_text("ERROR"), loc, w.message),
                "warning" => println!("   {} UB{}: {}", term::warn("WARN"), loc, w.message),
                _ => println!("   {} UB{}: {}", term::info("NOTE"), loc, w.message),
            }
        }
        println!();
    }

    // If strict mode found errors, refuse to compile
    if strict && pipeline.ub_report.has_errors() {
        return Err("Strict mode: UB detected — refusing to emit binary".into());
    }

    println!("   Phase 6: Compiling to native code...");
    let mut compiler = IsaCompiler::new(Target::Windows);
    let (code, data, iat_offsets, string_offsets) = compiler.compile(&pipeline.program);

    println!("   Phase 7: Generating PE binary...");
    adeb_backend_x64::pe::generate_pe_with_offsets(
        &code, &data, output_file, &iat_offsets, &string_offsets,
    )?;

    let meta = fs::metadata(output_file)
        .map_err(|e| format!("Post-build: cannot stat '{}': {}", output_file, e))?;
    if meta.len() == 0 {
        return Err(format!("Post-build: output '{}' is empty", output_file).into());
    }
    if cfg!(target_os = "windows") {
        let bytes = fs::read(output_file)
            .map_err(|e| format!("Post-build: cannot read '{}': {}", output_file, e))?;
        if bytes.len() < 2 || &bytes[0..2] != b"MZ" {
            return Err(format!("Post-build: '{}' is not a PE (missing MZ)", output_file).into());
        }
    }

    println!("   Build complete: {} ({} bytes)", output_file, meta.len());
    println!("   Post-build validation OK");
    Ok(())
}

// ── Step mode output ────────────────────────────────────────

fn print_cpp_step_mode(file: &str, source: &str, arts: &CppPipelineArtifacts) {
    println!();
    println!("{}", term::phase_bar(0, "Preprocessor", "C++"));
    println!("   Source lines: {}", source.lines().count());
    println!("   After preprocessing: {} lines", arts.preprocessed.lines().count());
    println!();
    println!("{}", term::phase_bar(1, "Lexer", "C++"));
    println!("   Tokens: {}", arts.tokens.len());
    println!();
    println!("{}", term::phase_bar(2, "Parser", "C++"));
    println!("   Top-level declarations: {}", arts.unit.declarations.len());
    let funcs = arts.unit.declarations.iter().filter(|d| matches!(d, CppTopLevel::FunctionDef { .. })).count();
    let classes = arts.unit.declarations.iter().filter(|d| matches!(d, CppTopLevel::ClassDef { .. })).count();
    let namespaces = arts.unit.declarations.iter().filter(|d| matches!(d, CppTopLevel::Namespace { .. })).count();
    println!("   Functions: {}, Classes: {}, Namespaces: {}", funcs, classes, namespaces);
    println!();
    println!("{}", term::phase_bar(3, "UB Detection", "C++"));
    if arts.ub_report.warnings.is_empty() {
        println!("   No UB detected");
    } else {
        println!("   {} UB issue(s) found", arts.ub_report.warnings.len());
    }
    println!();
    println!("{}", term::phase_bar(4, "IR Generation", "C++"));
    println!("   IR functions: {}", arts.program.functions.len());
    println!("   IR structs: {}", arts.program.structs.len());
    println!("   IR statements: {}", arts.program.statements.len());
}

// ── C++ UB Detector ─────────────────────────────────────────

fn detect_cpp_ub(unit: &CppTranslationUnit) -> CppUBReport {
    let mut report = CppUBReport::default();

    for decl in &unit.declarations {
        detect_ub_in_top_level(decl, &mut report);
    }

    report
}

fn detect_ub_in_top_level(decl: &CppTopLevel, report: &mut CppUBReport) {
    match decl {
        CppTopLevel::FunctionDef { name, body, .. } => {
            for stmt in body {
                detect_ub_in_stmt(stmt, report, Some(name));
            }
        }
        CppTopLevel::ClassDef { name, members, .. } => {
            for m in members {
                match m {
                    CppClassMember::Method { name: mname, body: Some(body), .. } => {
                        let loc = format!("{}::{}", name, mname);
                        for s in body { detect_ub_in_stmt(s, report, Some(&loc)); }
                    }
                    CppClassMember::Constructor { body: Some(body), .. } => {
                        let loc = format!("{}::constructor", name);
                        for s in body {
                            // Virtual call in constructor
                            check_virtual_in_ctor(s, report, &loc);
                            detect_ub_in_stmt(s, report, Some(&loc));
                        }
                    }
                    CppClassMember::Destructor { body: Some(body), .. } => {
                        let loc = format!("{}::destructor", name);
                        for s in body {
                            // Throw in destructor
                            if matches!(s, CppStmt::Throw(_)) {
                                report.warnings.push(CppUBWarning {
                                    kind: CppUBKind::ThrowInDestructor,
                                    severity: "warning",
                                    message: format!("throw in destructor of '{}'", name),
                                    location: Some(loc.clone()),
                                });
                            }
                            detect_ub_in_stmt(s, report, Some(&loc));
                        }
                    }
                    _ => {}
                }
            }
        }
        CppTopLevel::Namespace { declarations, .. } => {
            for d in declarations { detect_ub_in_top_level(d, report); }
        }
        _ => {}
    }
}

fn detect_ub_in_stmt(stmt: &CppStmt, report: &mut CppUBReport, ctx: Option<&str>) {
    match stmt {
        CppStmt::Expr(e) => detect_ub_in_expr(e, report, ctx),
        CppStmt::Return(Some(e)) => detect_ub_in_expr(e, report, ctx),
        CppStmt::VarDecl { declarators, .. } => {
            for d in declarators {
                if let Some(init) = &d.initializer {
                    detect_ub_in_expr(init, report, ctx);
                }
            }
        }
        CppStmt::Block(stmts) => {
            for s in stmts { detect_ub_in_stmt(s, report, ctx); }
        }
        CppStmt::If { condition, then_body, else_body, .. } => {
            detect_ub_in_expr(condition, report, ctx);
            detect_ub_in_stmt(then_body, report, ctx);
            if let Some(eb) = else_body { detect_ub_in_stmt(eb, report, ctx); }
        }
        CppStmt::While { condition, body } => {
            detect_ub_in_expr(condition, report, ctx);
            detect_ub_in_stmt(body, report, ctx);
        }
        CppStmt::For { init, condition, increment, body } => {
            if let Some(i) = init { detect_ub_in_stmt(i, report, ctx); }
            if let Some(c) = condition { detect_ub_in_expr(c, report, ctx); }
            if let Some(inc) = increment { detect_ub_in_expr(inc, report, ctx); }
            detect_ub_in_stmt(body, report, ctx);
        }
        CppStmt::Switch { expr, cases, default } => {
            detect_ub_in_expr(expr, report, ctx);
            for c in cases { for s in &c.body { detect_ub_in_stmt(s, report, ctx); } }
            if let Some(d) = default { for s in d { detect_ub_in_stmt(s, report, ctx); } }
        }
        CppStmt::DoWhile { body, condition } => {
            detect_ub_in_stmt(body, report, ctx);
            detect_ub_in_expr(condition, report, ctx);
        }
        _ => {}
    }
}

fn detect_ub_in_expr(expr: &CppExpr, report: &mut CppUBReport, ctx: Option<&str>) {
    match expr {
        // Division by zero
        CppExpr::BinaryOp { op: CppBinOp::Div, right, .. }
        | CppExpr::BinaryOp { op: CppBinOp::Mod, right, .. } => {
            if matches!(right.as_ref(), CppExpr::IntLiteral(0)) {
                report.warnings.push(CppUBWarning {
                    kind: CppUBKind::DivisionByZero,
                    severity: "error",
                    message: "division by zero".into(),
                    location: ctx.map(|s| s.into()),
                });
            }
        }
        // Shift overflow
        CppExpr::BinaryOp { op: CppBinOp::Shl, right, .. }
        | CppExpr::BinaryOp { op: CppBinOp::Shr, right, .. } => {
            if let CppExpr::IntLiteral(n) = right.as_ref() {
                if *n < 0 || *n >= 64 {
                    report.warnings.push(CppUBWarning {
                        kind: CppUBKind::ShiftOverflow,
                        severity: "warning",
                        message: format!("shift amount {} is out of range", n),
                        location: ctx.map(|s| s.into()),
                    });
                }
            }
        }
        // Nullptr dereference
        CppExpr::Deref(inner) if matches!(inner.as_ref(), CppExpr::NullptrLiteral) => {
            report.warnings.push(CppUBWarning {
                kind: CppUBKind::NullPointerDereference,
                severity: "error",
                message: "dereference of nullptr".into(),
                location: ctx.map(|s| s.into()),
            });
        }
        // delete/delete[] mismatch detection (heuristic)
        CppExpr::Delete { is_array: true, expr: inner } => {
            // If deleting something allocated with `new` (not new[]), warn
            if let CppExpr::Identifier(_) = inner.as_ref() {
                // Can't statically verify — note only
            }
        }
        // Recurse into subexpressions
        CppExpr::BinaryOp { left, right, .. } => {
            detect_ub_in_expr(left, report, ctx);
            detect_ub_in_expr(right, report, ctx);
        }
        CppExpr::UnaryOp { expr: inner, .. } => detect_ub_in_expr(inner, report, ctx),
        CppExpr::Call { callee, args } => {
            detect_ub_in_expr(callee, report, ctx);
            for a in args { detect_ub_in_expr(a, report, ctx); }
        }
        CppExpr::Assign { target, value } => {
            detect_ub_in_expr(target, report, ctx);
            detect_ub_in_expr(value, report, ctx);
        }
        CppExpr::Ternary { condition, then_expr, else_expr } => {
            detect_ub_in_expr(condition, report, ctx);
            detect_ub_in_expr(then_expr, report, ctx);
            detect_ub_in_expr(else_expr, report, ctx);
        }
        _ => {}
    }
}

fn check_virtual_in_ctor(stmt: &CppStmt, report: &mut CppUBReport, ctx: &str) {
    if let CppStmt::Expr(CppExpr::Call { callee, .. }) = stmt {
        // Heuristic: calling `this->method()` in a constructor
        if let CppExpr::ArrowAccess { pointer, member } = callee.as_ref() {
            if matches!(pointer.as_ref(), CppExpr::This) {
                report.warnings.push(CppUBWarning {
                    kind: CppUBKind::VirtualInConstructor,
                    severity: "warning",
                    message: format!("calling this->{}() in constructor — may invoke wrong vtable", member),
                    location: Some(ctx.into()),
                });
            }
        }
    }
}

// ── Strict Mode: Bit-width enforcement & narrowing ──────────

fn detect_cpp_strict_violations(unit: &CppTranslationUnit, report: &mut CppUBReport) {
    for decl in &unit.declarations {
        strict_check_top_level(decl, report);
    }
}

fn strict_check_top_level(decl: &CppTopLevel, report: &mut CppUBReport) {
    match decl {
        CppTopLevel::FunctionDef { name, body, .. } => {
            for s in body { strict_check_stmt(s, report, Some(name)); }
        }
        CppTopLevel::ClassDef { name, members, .. } => {
            for m in members {
                match m {
                    CppClassMember::Method { name: mname, body: Some(body), .. } => {
                        let loc = format!("{}::{}", name, mname);
                        for s in body { strict_check_stmt(s, report, Some(&loc)); }
                    }
                    CppClassMember::Constructor { body: Some(body), .. } => {
                        let loc = format!("{}::constructor", name);
                        for s in body { strict_check_stmt(s, report, Some(&loc)); }
                    }
                    CppClassMember::Destructor { body: Some(body), .. } => {
                        let loc = format!("{}::destructor", name);
                        for s in body { strict_check_stmt(s, report, Some(&loc)); }
                    }
                    _ => {}
                }
            }
        }
        CppTopLevel::Namespace { declarations, .. } => {
            for d in declarations { strict_check_top_level(d, report); }
        }
        _ => {}
    }
}

fn strict_check_stmt(stmt: &CppStmt, report: &mut CppUBReport, ctx: Option<&str>) {
    match stmt {
        CppStmt::VarDecl { type_spec, declarators } => {
            for d in declarators {
                if let Some(init) = &d.initializer {
                    // Check bit-width: value must fit declared type
                    check_bit_width(type_spec, init, &d.name, report, ctx);
                    strict_check_expr(init, report, ctx);
                }
            }
        }
        CppStmt::Expr(e) => strict_check_expr(e, report, ctx),
        CppStmt::Return(Some(e)) => strict_check_expr(e, report, ctx),
        CppStmt::Block(stmts) => { for s in stmts { strict_check_stmt(s, report, ctx); } }
        CppStmt::If { condition, then_body, else_body, .. } => {
            strict_check_expr(condition, report, ctx);
            strict_check_stmt(then_body, report, ctx);
            if let Some(eb) = else_body { strict_check_stmt(eb, report, ctx); }
        }
        CppStmt::While { condition, body } => {
            strict_check_expr(condition, report, ctx);
            strict_check_stmt(body, report, ctx);
        }
        CppStmt::For { init, condition, increment, body } => {
            if let Some(i) = init { strict_check_stmt(i, report, ctx); }
            if let Some(c) = condition { strict_check_expr(c, report, ctx); }
            if let Some(inc) = increment { strict_check_expr(inc, report, ctx); }
            strict_check_stmt(body, report, ctx);
        }
        CppStmt::DoWhile { body, condition } => {
            strict_check_stmt(body, report, ctx);
            strict_check_expr(condition, report, ctx);
        }
        _ => {}
    }
}

fn strict_check_expr(expr: &CppExpr, report: &mut CppUBReport, ctx: Option<&str>) {
    match expr {
        // Signed integer overflow in literal arithmetic
        CppExpr::BinaryOp { op: CppBinOp::Add, left, right, .. } => {
            if let (CppExpr::IntLiteral(a), CppExpr::IntLiteral(b)) = (left.as_ref(), right.as_ref()) {
                if a.checked_add(*b).is_none() {
                    report.warnings.push(CppUBWarning {
                        kind: CppUBKind::SignedIntegerOverflow,
                        severity: "warning",
                        message: format!("signed overflow: {} + {} overflows i64", a, b),
                        location: ctx.map(|s| s.into()),
                    });
                }
            }
            strict_check_expr(left, report, ctx);
            strict_check_expr(right, report, ctx);
        }
        CppExpr::BinaryOp { op: CppBinOp::Mul, left, right, .. } => {
            if let (CppExpr::IntLiteral(a), CppExpr::IntLiteral(b)) = (left.as_ref(), right.as_ref()) {
                if a.checked_mul(*b).is_none() {
                    report.warnings.push(CppUBWarning {
                        kind: CppUBKind::SignedIntegerOverflow,
                        severity: "warning",
                        message: format!("signed overflow: {} * {} overflows i64", a, b),
                        location: ctx.map(|s| s.into()),
                    });
                }
            }
            strict_check_expr(left, report, ctx);
            strict_check_expr(right, report, ctx);
        }
        // Narrowing conversion detection in casts
        CppExpr::Cast { target_type, expr: inner, .. } => {
            if let CppExpr::IntLiteral(v) = inner.as_ref() {
                let fits = match type_range(strip_qualifiers(target_type)) {
                    Some((min, max)) => *v >= min && *v <= max,
                    None => true,
                };
                if !fits {
                    report.warnings.push(CppUBWarning {
                        kind: CppUBKind::NarrowingConversion,
                        severity: "warning",
                        message: format!("narrowing: value {} does not fit target type", v),
                        location: ctx.map(|s| s.into()),
                    });
                }
            }
            strict_check_expr(inner, report, ctx);
        }
        CppExpr::BinaryOp { left, right, .. } => {
            strict_check_expr(left, report, ctx);
            strict_check_expr(right, report, ctx);
        }
        CppExpr::UnaryOp { expr: inner, .. } => strict_check_expr(inner, report, ctx),
        CppExpr::Call { callee, args } => {
            strict_check_expr(callee, report, ctx);
            for a in args { strict_check_expr(a, report, ctx); }
        }
        CppExpr::Assign { target, value } => {
            strict_check_expr(target, report, ctx);
            strict_check_expr(value, report, ctx);
        }
        _ => {}
    }
}

/// Check if a literal value fits the declared type's bit-width
fn check_bit_width(ty: &CppType, init: &CppExpr, var_name: &str, report: &mut CppUBReport, ctx: Option<&str>) {
    let val = match init {
        CppExpr::IntLiteral(v) => *v,
        CppExpr::UnaryOp { op: CppUnaryOp::Neg, expr, .. } => {
            if let CppExpr::IntLiteral(v) = expr.as_ref() { -v } else { return; }
        }
        _ => return,
    };

    let range = type_range(strip_qualifiers(ty));
    let (min, max) = match range {
        Some(r) => r,
        None => return,
    };

    if val < min || val > max {
        report.warnings.push(CppUBWarning {
            kind: CppUBKind::NarrowingConversion,
            severity: "warning",
            message: format!("bit-width violation: '{}' = {} does not fit [{}, {}]", var_name, val, min, max),
            location: ctx.map(|s| s.into()),
        });
    }
}

fn type_range(ty: &CppType) -> Option<(i64, i64)> {
    match ty {
        CppType::Bool => Some((0, 1)),
        CppType::Char => Some((-128, 127)),
        CppType::Short => Some((-32768, 32767)),
        CppType::Int => Some((i32::MIN as i64, i32::MAX as i64)),
        CppType::Long => Some((i32::MIN as i64, i32::MAX as i64)),
        CppType::LongLong => Some((i64::MIN, i64::MAX)),
        CppType::Signed(inner) => type_range(inner),
        CppType::Unsigned(inner) => {
            match inner.as_ref() {
                CppType::Char => Some((0, 255)),
                CppType::Short => Some((0, 65535)),
                CppType::Int => Some((0, u32::MAX as i64)),
                CppType::Long => Some((0, u32::MAX as i64)),
                CppType::LongLong => Some((0, i64::MAX)), // approx u64
                _ => None,
            }
        }
        _ => None,
    }
}

fn strip_qualifiers(ty: &CppType) -> &CppType {
    match ty {
        CppType::Const(inner) | CppType::Volatile(inner)
        | CppType::Mutable(inner) | CppType::Constexpr(inner) => strip_qualifiers(inner),
        _ => ty,
    }
}

// ── Tests ───────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpp_pipeline_hello() {
        let result = compile_cpp_pipeline("int main() { return 0; }", false);
        assert!(result.is_ok());
        let arts = result.unwrap();
        assert_eq!(arts.program.functions.len(), 1);
        assert_eq!(arts.program.functions[0].name, "main");
    }

    #[test]
    fn test_cpp_pipeline_class() {
        let result = compile_cpp_pipeline(r#"
            class Point {
            public:
                int x;
                int y;
            };
            int main() { return 0; }
        "#, false);
        assert!(result.is_ok());
        let arts = result.unwrap();
        assert!(arts.program.structs.iter().any(|s| s.name == "Point"));
    }

    #[test]
    fn test_cpp_pipeline_namespace() {
        let result = compile_cpp_pipeline(r#"
            namespace math {
                int square(int x) { return x * x; }
            }
            int main() { return 0; }
        "#, false);
        assert!(result.is_ok());
        let arts = result.unwrap();
        assert!(arts.program.functions.iter().any(|f| f.name == "math::square"));
    }

    #[test]
    fn test_cpp_ub_division_by_zero() {
        let result = compile_cpp_pipeline("int main() { int x = 5 / 0; return 0; }", false);
        assert!(result.is_ok());
        let arts = result.unwrap();
        assert!(arts.ub_report.has_errors());
        assert!(arts.ub_report.warnings.iter().any(|w| matches!(w.kind, CppUBKind::DivisionByZero)));
    }

    #[test]
    fn test_cpp_ub_nullptr_deref() {
        let result = compile_cpp_pipeline("int main() { int* p = nullptr; int x = *nullptr; return 0; }", false);
        assert!(result.is_ok());
        let arts = result.unwrap();
        assert!(arts.ub_report.warnings.iter().any(|w| matches!(w.kind, CppUBKind::NullPointerDereference)));
    }

    #[test]
    fn test_cpp_strict_mode() {
        let result = compile_cpp_pipeline("int main() { int x = 5 / 0; return 0; }", true);
        assert!(result.is_ok());
        let arts = result.unwrap();
        assert!(arts.ub_report.has_errors());
    }

    #[test]
    fn test_cpp_pipeline_clean() {
        let result = compile_cpp_pipeline("int main() { int x = 42; return x; }", false);
        assert!(result.is_ok());
        let arts = result.unwrap();
        assert!(!arts.ub_report.has_warnings());
    }

    // ── Fixture integration tests ───────────────────────────

    fn fixture_path(name: &str) -> String {
        let manifest = env!("CARGO_MANIFEST_DIR");
        format!("{}/../../../../../tests/cpp/fixtures/{}", manifest, name)
    }

    fn parse_fixture(name: &str) -> CppPipelineArtifacts {
        let path = fixture_path(name);
        let source = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("Cannot read fixture '{}': {}", path, e));
        compile_cpp_pipeline(&source, false)
            .unwrap_or_else(|e| panic!("Fixture '{}' failed to compile: {}", name, e))
    }

    #[test]
    fn test_fixture_01_hello_world() {
        let arts = parse_fixture("01_hello_world.cpp");
        assert!(arts.program.functions.iter().any(|f| f.name == "main"));
        assert!(!arts.ub_report.has_errors());
    }

    #[test]
    fn test_fixture_02_class_basic() {
        let arts = parse_fixture("02_class_basic.cpp");
        assert!(arts.program.structs.iter().any(|s| s.name == "Counter"));
        assert!(arts.program.functions.iter().any(|f| f.name == "main"));
        assert!(arts.program.functions.iter().any(|f| f.name == "Counter::get"));
        assert!(arts.program.functions.iter().any(|f| f.name == "Counter::increment"));
        assert!(arts.program.functions.iter().any(|f| f.name == "Counter::__init"));
    }

    #[test]
    fn test_fixture_03_namespace() {
        let arts = parse_fixture("03_namespace.cpp");
        assert!(arts.program.functions.iter().any(|f| f.name == "math::square"));
        assert!(arts.program.functions.iter().any(|f| f.name == "math::cube"));
    }

    #[test]
    fn test_fixture_04_enum() {
        let arts = parse_fixture("04_enum.cpp");
        assert!(arts.program.functions.iter().any(|f| f.name == "main"));
    }

    #[test]
    fn test_fixture_05_control_flow() {
        let arts = parse_fixture("05_control_flow.cpp");
        assert!(arts.program.functions.iter().any(|f| f.name == "main"));
        assert!(!arts.ub_report.has_errors());
    }

    #[test]
    fn test_fixture_06_assignment() {
        let arts = parse_fixture("06_assignment.cpp");
        assert!(arts.program.functions.iter().any(|f| f.name == "main"));
        // Should have Assign/CompoundAssign stmts in main body
        let main_fn = arts.program.functions.iter().find(|f| f.name == "main").unwrap();
        assert!(main_fn.body.len() >= 3); // at least x=10, x=20, x+=5
    }

    #[test]
    fn test_fixture_07_lambda() {
        let arts = parse_fixture("07_lambda.cpp");
        assert!(arts.program.functions.iter().any(|f| f.name == "main"));
        assert!(arts.program.functions.iter().any(|f| f.name == "apply"));
    }

    #[test]
    fn test_fixture_08_inheritance() {
        let arts = parse_fixture("08_inheritance.cpp");
        assert!(arts.program.structs.iter().any(|s| s.name == "Shape"));
        assert!(arts.program.structs.iter().any(|s| s.name == "Rectangle"));
        // Rectangle should have __base_Shape field
        let rect = arts.program.structs.iter().find(|s| s.name == "Rectangle").unwrap();
        assert!(rect.fields.iter().any(|f| f.name == "__base_Shape"));
    }

    #[test]
    fn test_fixture_09_ub_strict() {
        let arts = parse_fixture("09_ub_strict.cpp");
        // Clean code — no UB should be detected
        assert!(!arts.ub_report.warnings.iter().any(|w|
            matches!(w.kind, CppUBKind::NarrowingConversion)));
    }

    #[test]
    fn test_fixture_10_new_delete() {
        let arts = parse_fixture("10_new_delete.cpp");
        assert!(arts.program.structs.iter().any(|s| s.name == "Node"));
        assert!(arts.program.functions.iter().any(|f| f.name == "Node::__init"));
    }

    // ── Phase 2: Strict bit-width tests ─────────────────────

    #[test]
    fn test_strict_bit_width_char_overflow() {
        let result = compile_cpp_pipeline("int main() { char c = 256; return 0; }", false);
        assert!(result.is_ok());
        let arts = result.unwrap();
        assert!(arts.ub_report.warnings.iter().any(|w|
            matches!(w.kind, CppUBKind::NarrowingConversion)));
    }

    #[test]
    fn test_strict_bit_width_short_overflow() {
        let result = compile_cpp_pipeline("int main() { short s = 70000; return 0; }", false);
        assert!(result.is_ok());
        let arts = result.unwrap();
        assert!(arts.ub_report.warnings.iter().any(|w|
            matches!(w.kind, CppUBKind::NarrowingConversion)));
    }

    #[test]
    fn test_strict_bit_width_clean() {
        let result = compile_cpp_pipeline("int main() { char c = 65; short s = 1000; int x = 42; return 0; }", false);
        assert!(result.is_ok());
        let arts = result.unwrap();
        assert!(!arts.ub_report.warnings.iter().any(|w|
            matches!(w.kind, CppUBKind::NarrowingConversion)));
    }

    #[test]
    fn test_implicit_strict_all_ub_are_errors() {
        // C++ is implicitly strict: all UB warnings become errors
        let result = compile_cpp_pipeline("int main() { int x = 5 / 0; return 0; }", false);
        assert!(result.is_ok());
        let arts = result.unwrap();
        // Should be errors, not warnings (implicitly strict)
        assert!(arts.ub_report.has_errors());
        assert!(arts.ub_report.warnings.iter().all(|w| w.severity == "error"));
    }

    // ── Phase 3: Fixture tests 11-15 ────────────────────────

    #[test]
    fn test_fixture_11_virtual() {
        let arts = parse_fixture("11_virtual.cpp");
        assert!(arts.program.structs.iter().any(|s| s.name == "Animal"));
        assert!(arts.program.structs.iter().any(|s| s.name == "Dog"));
        // Animal should have __vptr field (has virtual methods)
        let animal = arts.program.structs.iter().find(|s| s.name == "Animal").unwrap();
        assert!(animal.fields.iter().any(|f| f.name == "__vptr"));
        // Vtable struct should be emitted
        assert!(arts.program.structs.iter().any(|s| s.name == "__vtable_Animal"));
    }

    #[test]
    fn test_fixture_12_operator_overload() {
        let arts = parse_fixture("12_operator_overload.cpp");
        assert!(arts.program.structs.iter().any(|s| s.name == "Vec2"));
        // operator+ → operator_add, operator== → operator_eq
        assert!(arts.program.functions.iter().any(|f| f.name == "Vec2::operator_add"));
        assert!(arts.program.functions.iter().any(|f| f.name == "Vec2::operator_eq"));
    }

    #[test]
    fn test_fixture_13_static_method() {
        let arts = parse_fixture("13_static_method.cpp");
        assert!(arts.program.structs.iter().any(|s| s.name == "Config"));
        // Static methods should NOT have 'this' parameter
        let dw = arts.program.functions.iter().find(|f| f.name == "Config::default_width").unwrap();
        assert!(dw.params.is_empty(), "static method should have no params");
        // Constructor should have initializer list lowered
        let init = arts.program.functions.iter().find(|f| f.name == "Config::__init").unwrap();
        assert!(init.body.len() >= 2, "constructor should have init list stmts");
    }

    #[test]
    fn test_fixture_14_extern_c() {
        let arts = parse_fixture("14_extern_c.cpp");
        // extern "C" functions should be emitted
        assert!(arts.program.functions.iter().any(|f| f.name == "abs"));
        assert!(arts.program.functions.iter().any(|f| f.name == "atoi"));
        assert!(arts.program.functions.iter().any(|f| f.name == "helper"));
    }

    #[test]
    fn test_fixture_15_destructor() {
        let arts = parse_fixture("15_destructor.cpp");
        assert!(arts.program.structs.iter().any(|s| s.name == "Resource"));
        assert!(arts.program.functions.iter().any(|f| f.name == "Resource::__init"));
        assert!(arts.program.functions.iter().any(|f| f.name == "Resource::__destroy"));
    }

    // ── Phase 3: Unit tests for new lowerings ───────────────

    #[test]
    fn test_vtable_struct_generated() {
        let result = compile_cpp_pipeline(r#"
            class Base {
            public:
                virtual int foo() { return 1; }
                virtual int bar() { return 2; }
            };
            int main() { return 0; }
        "#, false);
        assert!(result.is_ok());
        let arts = result.unwrap();
        let base = arts.program.structs.iter().find(|s| s.name == "Base").unwrap();
        assert!(base.fields.iter().any(|f| f.name == "__vptr"));
        assert!(arts.program.structs.iter().any(|s| s.name == "__vtable_Base"));
        let vt = arts.program.structs.iter().find(|s| s.name == "__vtable_Base").unwrap();
        assert_eq!(vt.fields.len(), 2); // foo + bar
    }

    #[test]
    fn test_operator_mangling() {
        let result = compile_cpp_pipeline(r#"
            class Num {
            public:
                int v;
                int operator+(int r) { return v + r; }
                int operator==(int r) { return v == r; }
                int operator[](int i) { return i; }
            };
            int main() { return 0; }
        "#, false);
        assert!(result.is_ok());
        let arts = result.unwrap();
        assert!(arts.program.functions.iter().any(|f| f.name == "Num::operator_add"));
        assert!(arts.program.functions.iter().any(|f| f.name == "Num::operator_eq"));
        assert!(arts.program.functions.iter().any(|f| f.name == "Num::operator_index"));
    }

    #[test]
    fn test_static_method_no_this() {
        let result = compile_cpp_pipeline(r#"
            class Math {
            public:
                static int square(int x) { return x * x; }
            };
            int main() { return 0; }
        "#, false);
        assert!(result.is_ok());
        let arts = result.unwrap();
        let sq = arts.program.functions.iter().find(|f| f.name == "Math::square").unwrap();
        // Static: only 'x' param, no 'this'
        assert_eq!(sq.params.len(), 1);
        assert_eq!(sq.params[0].name, "x");
    }

    #[test]
    fn test_initializer_list_lowered() {
        let result = compile_cpp_pipeline(r#"
            class Point {
            public:
                int x;
                int y;
                Point(int a, int b) : x(a), y(b) {}
            };
            int main() { return 0; }
        "#, false);
        assert!(result.is_ok());
        let arts = result.unwrap();
        let init = arts.program.functions.iter().find(|f| f.name == "Point::__init").unwrap();
        // Should have FieldAssign for x and y from initializer list
        assert!(init.body.len() >= 2);
    }
}
