// ADead-BIB Builder
// Orquestador principal del compilador
// Conecta Frontend -> Optimizer -> Backend
// v1.5.0: Module system support

use crate::frontend::parser::Parser;
use crate::frontend::type_checker::TypeChecker;
use crate::frontend::ast::{Program, Function};
use crate::optimizer::branch_detector::{BranchDetector, BranchPattern};
use crate::optimizer::branchless::BranchlessTransformer;
use crate::optimizer::binary_optimizer::{BinaryOptimizer, OptLevel};
use crate::isa::isa_compiler::{IsaCompiler, Target};
use crate::backend::{pe, elf};
use std::fs;
use std::path::Path;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct BuildOptions {
    pub target: Target,
    pub optimize: bool,
    pub output_path: String,
    pub verbose: bool,
    pub opt_level: OptLevel,
    pub size_optimize: bool,
}

impl Default for BuildOptions {
    fn default() -> Self {
        Self {
            target: Target::Windows,
            optimize: true,
            output_path: "output.exe".to_string(),
            verbose: false,
            opt_level: OptLevel::Basic,
            size_optimize: false,
        }
    }
}

pub struct Builder;

impl Builder {
    /// Compila código fuente ADead-BIB a un binario ejecutable
    pub fn build(source: &str, options: BuildOptions) -> Result<(), Box<dyn std::error::Error>> {
        Self::build_with_base_path(source, options, None)
    }
    
    /// Compila con path base para resolver imports
    pub fn build_with_base_path(source: &str, options: BuildOptions, base_path: Option<&Path>) -> Result<(), Box<dyn std::error::Error>> {
        if options.verbose {
            println!("Starting build for target: {:?}", options.target);
        }

        // 1. Frontend: Lexing & Parsing
        if options.verbose { println!("Step 1: Parsing..."); }
        let mut program = Parser::parse_program(source)?;
        
        // 1.5: Resolve imports (v1.5.0)
        if !program.imports.is_empty() {
            if options.verbose { println!("Step 1.5: Resolving imports..."); }
            Self::resolve_imports(&mut program, base_path, options.verbose)?;
        }
        
        // 1.6: Convert Python-style classes to functions (v1.6.0)
        if !program.classes.is_empty() {
            Self::convert_classes_to_functions(&mut program);
        }

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

        // 4. Backend: ISA Compilation (ADead ISA → bytes)
        if options.verbose { println!("Step 4: ISA Compilation..."); }
        let mut compiler = IsaCompiler::new(options.target);
        let (opcodes, data) = compiler.compile(&program);

        // 4.5. Binary Optimization (new!)
        let final_opcodes = if options.size_optimize {
            if options.verbose { println!("Step 4.5: Binary Optimization (level: {:?})...", options.opt_level); }
            let mut binary_opt = BinaryOptimizer::new(options.opt_level);
            let optimized = binary_opt.optimize(&opcodes);
            if options.verbose {
                let stats = binary_opt.get_stats();
                println!("   Original: {} bytes, Optimized: {} bytes, Saved: {} bytes ({:.1}%)",
                    stats.original_size, stats.optimized_size, stats.bytes_saved,
                    if stats.original_size > 0 { (stats.bytes_saved as f64 / stats.original_size as f64) * 100.0 } else { 0.0 }
                );
            }
            optimized
        } else {
            opcodes
        };

        // 5. Linking / Binary Generation
        if options.verbose { println!("Step 5: Writing Binary to {}...", options.output_path); }
        match options.target {
            Target::Windows => pe::generate_pe(&final_opcodes, &data, &options.output_path)?,
            Target::Linux => elf::generate_elf(&final_opcodes, &data, &options.output_path)?,
            Target::Raw => fs::write(&options.output_path, &final_opcodes)?,
        }

        if options.verbose { println!("Build successful!"); }
        Ok(())
    }

    /// Construye desde un archivo
    pub fn build_file(path: &str, options: BuildOptions) -> Result<(), Box<dyn std::error::Error>> {
        let source = fs::read_to_string(path)?;
        let base_path = Path::new(path).parent();
        Self::build_with_base_path(&source, options, base_path)
    }
    
    /// Resuelve imports y agrega funciones de módulos al programa (v1.5.0)
    fn resolve_imports(program: &mut Program, base_path: Option<&Path>, verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut resolved_modules: HashSet<String> = HashSet::new();
        let mut imported_functions: Vec<Function> = Vec::new();
        
        for import in &program.imports {
            let module_name = &import.module;
            
            // Skip if already resolved
            if resolved_modules.contains(module_name) {
                continue;
            }
            
            // Find module file
            let module_path = Self::find_module(module_name, base_path)?;
            
            if verbose {
                println!("  Importing module: {} from {:?}", module_name, module_path);
            }
            
            // Parse module
            let module_source = fs::read_to_string(&module_path)?;
            let module_program = Parser::parse_program(&module_source)?;
            
            // Import specific items or all
            if import.items.is_empty() {
                // import module - import all public functions
                for func in module_program.functions {
                    imported_functions.push(func);
                }
            } else {
                // from module import item1, item2
                for item in &import.items {
                    if let Some(func) = module_program.functions.iter().find(|f| &f.name == item) {
                        imported_functions.push(func.clone());
                    } else {
                        eprintln!("Warning: '{}' not found in module '{}'", item, module_name);
                    }
                }
            }
            
            resolved_modules.insert(module_name.clone());
        }
        
        // Add imported functions to program (at the beginning)
        let mut all_functions = imported_functions;
        all_functions.extend(program.functions.drain(..));
        program.functions = all_functions;
        
        Ok(())
    }
    
    /// Encuentra el archivo de un módulo
    fn find_module(module_name: &str, base_path: Option<&Path>) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
        // Module name can be:
        // - "math" -> look for math.adB
        // - "std::math" -> look for std/math.adB
        // - "./mymodule" -> look for mymodule.adB relative to current file
        
        let module_file = if module_name.contains("::") {
            // std::math -> std/math.adB
            module_name.replace("::", "/") + ".adB"
        } else if module_name.starts_with("./") {
            // Relative path
            module_name[2..].to_string() + ".adB"
        } else {
            // Simple name
            module_name.to_string() + ".adB"
        };
        
        // Search paths:
        // 1. Relative to current file
        // 2. In std/ directory (for standard library)
        // 3. In current working directory
        
        let search_paths = [
            base_path.map(|p| p.join(&module_file)),
            Some(std::path::PathBuf::from(&module_file)),
            Some(std::path::PathBuf::from(format!("std/{}", module_file.trim_start_matches("std/")))),
        ];
        
        for path_opt in search_paths.iter().flatten() {
            if path_opt.exists() {
                return Ok(path_opt.clone());
            }
        }
        
        Err(format!("Module '{}' not found. Searched for: {}", module_name, module_file).into())
    }

    /// Convierte clases Python-style a funciones (v1.6.0)
    fn convert_classes_to_functions(program: &mut Program) {
        for class in &program.classes {
            // Convert each method to a function with Class::method name
            for method in &class.methods {
                let func_name = format!("{}::{}", class.name, method.name);
                let func = Function {
                    name: func_name,
                    params: method.params.clone(),
                    return_type: method.return_type.clone(),
                    body: method.body.clone(),
                };
                program.functions.push(func);
            }
            
            // Convert constructor if present
            if let Some(ref constructor) = class.constructor {
                let func_name = format!("{}::__init__", class.name);
                let func = Function {
                    name: func_name,
                    params: constructor.params.clone(),
                    return_type: constructor.return_type.clone(),
                    body: constructor.body.clone(),
                };
                program.functions.push(func);
            }
        }
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
