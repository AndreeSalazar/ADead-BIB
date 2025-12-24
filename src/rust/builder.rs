// ADead-BIB Builder
// Orquestador principal del compilador
// Conecta Frontend -> Optimizer -> Backend

use crate::frontend::parser::Parser;
use crate::frontend::type_checker::TypeChecker;
use crate::frontend::ast::Program;
use crate::optimizer::branch_detector::{BranchDetector, BranchPattern};
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
        let transformer = BranchlessTransformer::new();
        
        // Optimizar cada función
        for func in &mut program.functions {
            // Detectar patrones
            let patterns = detector.analyze(&func.body);
            
            if !patterns.is_empty() {
                // Aplicar transformaciones branchless
                let mut new_body = Vec::new();
                let mut i = 0;
                
                while i < func.body.len() {
                    let mut transformed = false;
                    
                    // Buscar si el statement actual coincide con algún patrón
                    for pattern in &patterns {
                        if let Some(replacement) = Self::try_transform_stmt(&func.body[i], pattern, &transformer) {
                            new_body.extend(replacement);
                            transformed = true;
                            break;
                        }
                    }
                    
                    if !transformed {
                        // Si no se transformó, mantener el statement original
                        new_body.push(func.body[i].clone());
                    }
                    
                    i += 1;
                }
                
                func.body = new_body;
            }
        }
    }
    
    /// Intenta transformar un statement usando un patrón detectado
    fn try_transform_stmt(
        stmt: &crate::frontend::ast::Stmt,
        pattern: &crate::optimizer::branch_detector::BranchPattern,
        transformer: &BranchlessTransformer,
    ) -> Option<Vec<crate::frontend::ast::Stmt>> {
        // Verificar si el statement coincide con el patrón
        match (stmt, pattern) {
            (crate::frontend::ast::Stmt::If { .. }, BranchPattern::ReLU { .. }) |
            (crate::frontend::ast::Stmt::If { .. }, BranchPattern::Select { .. }) => {
                // Transformar el patrón
                Some(transformer.transform(pattern.clone()))
            }
            _ => None,
        }
    }
}
