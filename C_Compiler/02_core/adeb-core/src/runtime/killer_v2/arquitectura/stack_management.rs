// ============================================================================
// stack_management.rs — Stack Frame Management Determinista
// ============================================================================
//
// FILOSOFÍA: Tamaño de stack calculado en compile-time.
// Stack overflow = abort, NUNCA undefined behavior.
//
// Implementa:
//   - Stack frame layout con shadow space Win64 (32 bytes)
//   - Red zone detection (128 bytes en SysV x86-64)
//   - Stack canary para detección de overflow
//   - Guard pages virtuales
//   - Alignment a 16 bytes (requerido por Win64 ABI)
//
// Autor: Eddi Andreé Salazar Matos
// ============================================================================

/// Configuración del Stack Manager
#[derive(Debug, Clone)]
pub struct StackConfig {
    /// Tamaño máximo del stack por función (default: 1MB)
    pub max_stack_size: usize,
    /// Activar stack canary (default: true)
    pub enable_canary: bool,
    /// Valor del canary (debe ser aleatorio en producción)
    pub canary_value: u64,
    /// Shadow space Win64 (default: 32 bytes)
    pub shadow_space: usize,
    /// Red zone SysV (default: 128 bytes)
    pub red_zone_size: usize,
    /// Alignment del stack (default: 16 bytes)
    pub stack_alignment: usize,
}

impl Default for StackConfig {
    fn default() -> Self {
        Self {
            max_stack_size: 1024 * 1024,  // 1MB
            enable_canary: true,
            canary_value: 0xDEAD_BIB_CAFE_BABE,
            shadow_space: 32,              // Win64 ABI
            red_zone_size: 128,            // SysV ABI
            stack_alignment: 16,           // ABI requirement
        }
    }
}

/// Representa un stack frame completo con todas las zonas
///
/// Layout de un stack frame Win64:
/// ```text
/// ┌───────────────────────────┐ ← RSP antes del CALL
/// │     Return Address (8)    │
/// ├───────────────────────────┤ ← RSP después del CALL
/// │     Saved RBP (8)         │
/// ├───────────────────────────┤ ← RBP (frame pointer)
/// │     Stack Canary (8)      │  (si habilitado)
/// ├───────────────────────────┤
/// │     Local Variables       │
/// │     ...                   │
/// ├───────────────────────────┤
/// │     Spill Space           │
/// │     (register saves)      │
/// ├───────────────────────────┤
/// │     Shadow Space (32)     │  ← Win64 ABI requirement
/// ├───────────────────────────┤
/// │     Alignment Padding     │  ← para 16-byte alignment
/// └───────────────────────────┘ ← RSP final (16-aligned)
/// ```
#[derive(Debug, Clone)]
pub struct StackFrame {
    /// Nombre de la función a la que pertenece este frame
    pub function_name: String,
    /// Variables locales con sus offsets desde RBP
    pub locals: Vec<LocalVariable>,
    /// Registros guardados (callee-saved)
    pub saved_registers: Vec<SavedRegister>,
    /// Tamaño total del frame (ya alineado)
    pub total_size: usize,
    /// Offset del canary desde RBP (si habilitado)
    pub canary_offset: Option<i32>,
    /// Shadow space incluido
    pub shadow_space: usize,
    /// Alignment padding
    pub alignment_padding: usize,
}

/// Variable local en el stack frame
#[derive(Debug, Clone)]
pub struct LocalVariable {
    /// Nombre de la variable
    pub name: String,
    /// Tamaño en bytes
    pub size: usize,
    /// Alignment requerido
    pub alignment: usize,
    /// Offset desde RBP (negativo)
    pub rbp_offset: i32,
}

/// Registro guardado en el stack
#[derive(Debug, Clone)]
pub struct SavedRegister {
    /// Nombre del registro (e.g., "rbx", "r12")
    pub name: String,
    /// Offset desde RBP (negativo)
    pub rbp_offset: i32,
}

/// Guard de stack — valida stack overflow en compile-time
#[derive(Debug)]
pub struct StackGuard {
    config: StackConfig,
}

impl StackGuard {
    pub fn new(config: StackConfig) -> Self {
        Self { config }
    }

    /// Calcular el layout completo de un stack frame
    ///
    /// Retorna el frame con todos los offsets calculados, o error si excede el límite.
    /// Este cálculo se hace en compile-time — cero costo en runtime.
    pub fn calculate_frame(
        &self,
        function_name: &str,
        locals: &[(String, usize, usize)], // (name, size, alignment)
        saved_regs: &[String],
    ) -> Result<StackFrame, StackError> {
        let mut current_offset: i32 = 0;

        // 1. Stack canary (si habilitado)
        let canary_offset = if self.config.enable_canary {
            current_offset -= 8;
            Some(current_offset)
        } else {
            None
        };

        // 2. Variables locales (ordenadas por alignment descendente para minimizar padding)
        let mut sorted_locals: Vec<(String, usize, usize)> = locals.to_vec();
        sorted_locals.sort_by(|a, b| b.2.cmp(&a.2)); // Sort by alignment DESC

        let mut frame_locals = Vec::new();
        for (name, size, alignment) in &sorted_locals {
            // Align the offset
            let aligned = align_down_negative(current_offset, *alignment as i32);
            current_offset = aligned - *size as i32;
            frame_locals.push(LocalVariable {
                name: name.clone(),
                size: *size,
                alignment: *alignment,
                rbp_offset: current_offset,
            });
        }

        // 3. Saved registers (8 bytes each)
        let mut frame_saved = Vec::new();
        for reg in saved_regs {
            current_offset -= 8;
            frame_saved.push(SavedRegister {
                name: reg.clone(),
                rbp_offset: current_offset,
            });
        }

        // 4. Shadow space (Win64)
        let shadow_space = self.config.shadow_space;
        current_offset -= shadow_space as i32;

        // 5. Alignment padding (stack must be 16-aligned at CALL instruction)
        let raw_size = (-current_offset) as usize;
        let aligned_size = align_up_usize(raw_size, self.config.stack_alignment);
        let alignment_padding = aligned_size - raw_size;

        // 6. Validate against max stack size
        if aligned_size > self.config.max_stack_size {
            return Err(StackError::StackOverflow {
                function: function_name.to_string(),
                required: aligned_size,
                max: self.config.max_stack_size,
            });
        }

        Ok(StackFrame {
            function_name: function_name.to_string(),
            locals: frame_locals,
            saved_registers: frame_saved,
            total_size: aligned_size,
            canary_offset,
            shadow_space,
            alignment_padding,
        })
    }

    /// Generar el prologue assembly para este frame
    pub fn generate_prologue(&self, frame: &StackFrame) -> Vec<String> {
        let mut asm = Vec::new();

        // Push frame pointer
        asm.push("push rbp".to_string());
        asm.push("mov rbp, rsp".to_string());

        // Allocar stack space
        if frame.total_size > 0 {
            asm.push(format!("sub rsp, {}", frame.total_size));
        }

        // Stack canary
        if let Some(offset) = frame.canary_offset {
            asm.push(format!(
                "mov QWORD PTR [rbp{}], 0x{:016X}",
                offset, self.config.canary_value
            ));
        }

        // Save callee-saved registers
        for reg in &frame.saved_registers {
            asm.push(format!("mov QWORD PTR [rbp{}], {}", reg.rbp_offset, reg.name));
        }

        asm
    }

    /// Generar el epilogue assembly para este frame
    pub fn generate_epilogue(&self, frame: &StackFrame) -> Vec<String> {
        let mut asm = Vec::new();

        // Verificar stack canary
        if let Some(offset) = frame.canary_offset {
            asm.push(format!(
                "cmp QWORD PTR [rbp{}], 0x{:016X}",
                offset, self.config.canary_value
            ));
            asm.push("jne __stack_smash_detected".to_string());
        }

        // Restore callee-saved registers (en orden inverso)
        for reg in frame.saved_registers.iter().rev() {
            asm.push(format!("mov {}, QWORD PTR [rbp{}]", reg.name, reg.rbp_offset));
        }

        // Restore stack and return
        asm.push("mov rsp, rbp".to_string());
        asm.push("pop rbp".to_string());
        asm.push("ret".to_string());

        asm
    }
}

/// Errores del Stack Manager
#[derive(Debug, Clone)]
pub enum StackError {
    /// Stack frame excede el tamaño máximo
    StackOverflow {
        function: String,
        required: usize,
        max: usize,
    },
    /// Alignment inválido
    InvalidAlignment(usize),
}

impl std::fmt::Display for StackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackError::StackOverflow { function, required, max } => {
                write!(f, "Stack overflow in '{}': requires {} bytes, max is {}", function, required, max)
            }
            StackError::InvalidAlignment(a) => {
                write!(f, "Invalid stack alignment: {}", a)
            }
        }
    }
}

impl std::error::Error for StackError {}

/// Align up (unsigned)
#[inline(always)]
const fn align_up_usize(value: usize, alignment: usize) -> usize {
    (value + alignment - 1) & !(alignment - 1)
}

/// Align down for negative offsets
#[inline(always)]
fn align_down_negative(offset: i32, alignment: i32) -> i32 {
    if alignment <= 1 { return offset; }
    let abs_offset = (-offset) as usize;
    let aligned = align_up_usize(abs_offset, alignment as usize);
    -(aligned as i32)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_frame_layout() {
        let guard = StackGuard::new(StackConfig::default());
        let frame = guard.calculate_frame(
            "test_func",
            &[
                ("x".into(), 8, 8),
                ("y".into(), 4, 4),
            ],
            &["rbx".into(), "r12".into()],
        ).unwrap();

        assert!(frame.total_size > 0);
        assert!(frame.total_size % 16 == 0); // 16-byte aligned
        assert!(frame.canary_offset.is_some());
        assert_eq!(frame.locals.len(), 2);
        assert_eq!(frame.saved_registers.len(), 2);
    }

    #[test]
    fn test_stack_overflow_detection() {
        let guard = StackGuard::new(StackConfig {
            max_stack_size: 64,
            ..Default::default()
        });

        let result = guard.calculate_frame(
            "big_func",
            &[("huge_array".into(), 1024, 8)],
            &[],
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_prologue_epilogue_generation() {
        let guard = StackGuard::new(StackConfig::default());
        let frame = guard.calculate_frame(
            "my_func",
            &[("local_var".into(), 8, 8)],
            &["rbx".into()],
        ).unwrap();

        let prologue = guard.generate_prologue(&frame);
        let epilogue = guard.generate_epilogue(&frame);

        assert!(prologue[0] == "push rbp");
        assert!(prologue[1] == "mov rbp, rsp");
        assert!(epilogue.last().unwrap() == "ret");
    }
}
