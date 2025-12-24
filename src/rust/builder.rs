// ADead-BIB Builder
// Orquestador principal del compilador
// Conecta Frontend -> Optimizer -> Backend

use crate::frontend::parser::Parser;
use crate::frontend::type_checker::TypeChecker;
use crate::frontend::ast::Program;
use crate::optimizer::branch_detector::BranchDetector;
use crate::optimizer::branchless::BranchlessTransformer;
use crate::backend::codegen_v2::{CodeGeneratorV2, Target};
use crate::backend::{pe, elf};
// use crate::backend::{pe_minimal as pe, elf};
use std::fs;

#[derive(Clone, Debug)]
pub struct BuildOptions {
    pub target: Target,
    pub optimize: bool,
    pub output_path: String,
    pub verbose: bool,
}

impl Default for BuildOptions {
    fn default() -> Self {
        Self {
            target: Target::Windows,
            optimize: true,
            output_path: "output.exe".to_string(),
            verbose: false,
        }
    }
}

pub struct Builder;

impl Builder {
    /// Compila código fuente ADead-BIB a un binario ejecutable
    pub fn build(source: &str, options: BuildOptions) -> Result<(), Box<dyn std::error::Error>> {
        if options.verbose {
            println!("Starting build for target: {:?}", options.target);
        }

        // 1. Frontend: Lexing & Parsing
        if options.verbose { println!("Step 1: Parsing..."); }
        let mut program = Parser::parse_program(source)?;

        // 2. Type Checking (Static Analysis)
        if options.verbose { println!("Step 2: Type Checking..."); }
        let mut type_checker = TypeChecker::new();
        // Check types (ignoring errors for now, just inference)
        let _types = type_checker.check_program(&program);

        // 3. Optimization
        if options.optimize {
            if options.verbose { println!("Step 3: Optimizing..."); }
            Self::apply_optimizations(&mut program);
        }

        // 4. Backend: Code Generation
        if options.verbose { println!("Step 4: Code Generation..."); }
        let mut codegen = CodeGeneratorV2::new(options.target);
        let (opcodes, data) = codegen.generate(&program);

        // 5. Linking / Binary Generation
        if options.verbose { println!("Step 5: Writing Binary to {}...", options.output_path); }
        match options.target {
            Target::Windows => pe::generate_pe(&opcodes, &data, &options.output_path)?,
            Target::Linux => elf::generate_elf(&opcodes, &data, &options.output_path)?,
            Target::Raw => fs::write(&options.output_path, &opcodes)?,
        }

        if options.verbose { println!("Build successful!"); }
        Ok(())
    }

    /// Construye desde un archivo
    pub fn build_file(path: &str, options: BuildOptions) -> Result<(), Box<dyn std::error::Error>> {
        let source = fs::read_to_string(path)?;
        Self::build(&source, options)
    }

    /// Aplica optimizaciones al AST
    fn apply_optimizations(program: &mut Program) {
        let detector = BranchDetector::new();
        let _transformer = BranchlessTransformer::new(); // Prefix with _ to suppress unused warning
        
        // Optimizar cada función
        for func in &mut program.functions {
            // Detectar patrones
            let patterns = detector.analyze(&func.body);
            
            if !patterns.is_empty() {
                // Aquí deberíamos aplicar las transformaciones.
                // Por ahora, como BranchlessTransformer retorna nuevos Stmt,
                // necesitaríamos un mecanismo para reemplazar los Stmt originales en el árbol.
                // Esta es una implementación simplificada.
                
                // TODO: Implementar reemplazo en el AST
                // func.body = transformer.transform_all(func.body, patterns);
            }
        }
    }
}
