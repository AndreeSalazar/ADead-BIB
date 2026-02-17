//! ADead-BIB FFI Rust Bindings
//! ============================
//! Author: Eddi Andreé Salazar Matos
//! Email: eddi.salazar.dev@gmail.com
//! Made with ❤️ in Peru 🇵🇪
//!
//! ADead-BIB como cabeza principal ABI para Rust.
//! Permite usar funciones compiladas de ADead-BIB desde Rust.

use std::process::Command;
use std::path::PathBuf;
use std::fs;

/// Wrapper FFI para ADead-BIB
pub struct ADeadBIB {
    compiler_path: PathBuf,
    base_dir: PathBuf,
}

impl ADeadBIB {
    /// Crea nueva instancia buscando el compilador
    pub fn new() -> Result<Self, String> {
        let base_dir = std::env::current_dir()
            .map_err(|e| e.to_string())?;
        
        let possible_paths = [
            base_dir.join("target/release/adeadc.exe"),
            base_dir.join("target/debug/adeadc.exe"),
        ];
        
        for path in &possible_paths {
            if path.exists() {
                return Ok(Self {
                    compiler_path: path.clone(),
                    base_dir,
                });
            }
        }
        
        Err("Compilador ADead-BIB no encontrado".to_string())
    }
    
    /// Crea instancia con ruta específica
    pub fn with_compiler(compiler_path: PathBuf) -> Self {
        let base_dir = compiler_path.parent()
            .unwrap_or(&PathBuf::from("."))
            .to_path_buf();
        
        Self { compiler_path, base_dir }
    }
    
    /// Compila y ejecuta archivo .adB
    pub fn run(&self, source_file: &str) -> Result<String, String> {
        let output = Command::new(&self.compiler_path)
            .args(["run", source_file])
            .output()
            .map_err(|e| e.to_string())?;
        
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
    
    /// Ejecuta código ADead-BIB desde string
    pub fn run_code(&self, code: &str) -> Result<String, String> {
        let temp_file = self.base_dir.join("FFI/examples/_temp.adB");
        
        fs::write(&temp_file, code)
            .map_err(|e| e.to_string())?;
        
        let result = self.run(temp_file.to_str().unwrap());
        
        let _ = fs::remove_file(&temp_file);
        
        result
    }
    
    // ============================================
    // FUNCIONES MATEMÁTICAS (usando ABI)
    // ============================================
    
    /// Suma usando ADead-BIB ABI
    pub fn sum(&self, a: i64, b: i64) -> Result<i64, String> {
        let code = format!(r#"
int main() {{
    printf({} + {})
    return 0
}}
"#, a, b);
        
        let result = self.run_code(&code)?;
        result.trim().parse::<i64>()
            .map_err(|e| e.to_string())
    }
    
    /// Multiplicación usando ADead-BIB ABI
    pub fn mul(&self, a: i64, b: i64) -> Result<i64, String> {
        let code = format!(r#"
int main() {{
    printf({} * {})
    return 0
}}
"#, a, b);
        
        let result = self.run_code(&code)?;
        result.trim().parse::<i64>()
            .map_err(|e| e.to_string())
    }
    
    /// Factorial usando ADead-BIB ABI
    pub fn factorial(&self, n: i64) -> Result<i64, String> {
        let code = format!(r#"
int factorial(int n) {{
    if n <= 1 {{
        return 1
    }}
    return n * factorial(n - 1)
}}

int main() {{
    printf(factorial({}))
    return 0
}}
"#, n);
        
        let result = self.run_code(&code)?;
        result.trim().parse::<i64>()
            .map_err(|e| e.to_string())
    }
    
    /// Versión del ABI
    pub fn version(&self) -> i32 {
        320 // v3.2.0
    }
}

impl Default for ADeadBIB {
    fn default() -> Self {
        Self::new().expect("No se pudo inicializar ADead-BIB")
    }
}

// ============================================
// EJEMPLO DE USO
// ============================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sum() {
        if let Ok(adead) = ADeadBIB::new() {
            let result = adead.sum(10, 20).unwrap_or(0);
            assert_eq!(result, 30);
        }
    }
    
    #[test]
    fn test_mul() {
        if let Ok(adead) = ADeadBIB::new() {
            let result = adead.mul(5, 6).unwrap_or(0);
            assert_eq!(result, 30);
        }
    }
    
    #[test]
    fn test_factorial() {
        if let Ok(adead) = ADeadBIB::new() {
            let result = adead.factorial(5).unwrap_or(0);
            assert_eq!(result, 120);
        }
    }
}

fn main() {
    println!("=== ADead-BIB FFI Rust Demo ===\n");
    
    match ADeadBIB::new() {
        Ok(adead) => {
            println!("✓ FFI inicializado");
            
            if let Ok(result) = adead.sum(10, 20) {
                println!("\nadead.sum(10, 20) = {}", result);
            }
            
            if let Ok(result) = adead.mul(5, 6) {
                println!("adead.mul(5, 6) = {}", result);
            }
            
            if let Ok(result) = adead.factorial(5) {
                println!("adead.factorial(5) = {}", result);
            }
            
            println!("adead.version() = {}", adead.version());
            
            println!("\n=== FFI Rust Funcionando ===");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
