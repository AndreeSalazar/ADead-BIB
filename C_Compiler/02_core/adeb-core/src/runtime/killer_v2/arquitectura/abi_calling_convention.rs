// ============================================================================
// abi_calling_convention.rs — Win64/SysV ABI Completo con Validación
// ============================================================================
//
// FILOSOFÍA: ABI violations detectadas en compile-time.
// Genera código correcto para Win64 y SysV x86-64 calling conventions.
//
// Win64 ABI:
//   - Integer args: RCX, RDX, R8, R9 (resto en stack)
//   - Float args: XMM0, XMM1, XMM2, XMM3 (resto en stack)
//   - Shadow space: 32 bytes SIEMPRE (incluso sin args)
//   - Return: RAX (int), XMM0 (float)
//   - Callee-saved: RBX, RBP, RDI, RSI, RSP, R12-R15, XMM6-XMM15
//   - Stack alignment: 16 bytes at CALL
//
// SysV x86-64 ABI:
//   - Integer args: RDI, RSI, RDX, RCX, R8, R9 (resto en stack)
//   - Float args: XMM0-XMM7 (resto en stack)
//   - NO shadow space
//   - Red zone: 128 bytes below RSP
//   - Return: RAX (int), XMM0 (float)
//   - Callee-saved: RBX, RBP, RSP, R12-R15
//   - Stack alignment: 16 bytes at CALL
//
// Autor: Eddi Andreé Salazar Matos
// ============================================================================

/// Tipo de calling convention soportada
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallingConvention {
    /// Win64 (Microsoft x64)
    Win64,
    /// System V AMD64 (Linux, macOS, *BSD)
    SysV,
    /// Vectorcall (MSVC __vectorcall)
    VectorCall,
    /// Fastcall (legacy, 32-bit)
    FastCall,
    /// Cdecl (legacy, 32-bit)
    Cdecl,
    /// Custom (user-defined register assignment)
    Custom,
}

impl Default for CallingConvention {
    fn default() -> Self {
        // En Windows, default es Win64
        #[cfg(target_os = "windows")]
        { CallingConvention::Win64 }
        #[cfg(not(target_os = "windows"))]
        { CallingConvention::SysV }
    }
}

/// Tipo de un argumento para determinar qué registro usar
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArgType {
    /// Integer/pointer (goes in GP register or stack)
    Integer,
    /// Single precision float (goes in XMM or stack)
    Float32,
    /// Double precision float (goes in XMM or stack)
    Float64,
    /// Struct por valor (puede ir en registro si ≤ 8 bytes)
    Struct { size: usize },
}

/// Slot donde un argumento se pasa
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegisterSlot {
    /// Registro GP (e.g., "rcx", "rdi")
    Gp(String),
    /// Registro XMM (e.g., "xmm0")
    Xmm(String),
    /// Stack con offset desde RSP
    Stack { offset: usize, size: usize },
}

/// Descripción completa de un argumento en la ABI
#[derive(Debug, Clone)]
pub struct ArgLayout {
    /// Índice del argumento (0-based)
    pub index: usize,
    /// Nombre del parámetro
    pub name: String,
    /// Tipo del argumento
    pub arg_type: ArgType,
    /// Slot donde se pasa
    pub slot: RegisterSlot,
}

/// Layout completo de una llamada a función
#[derive(Debug, Clone)]
pub struct CallLayout {
    /// Calling convention usada
    pub convention: CallingConvention,
    /// Argumentos con sus slots
    pub args: Vec<ArgLayout>,
    /// Tamaño total del stack requerido para args (incluyendo shadow space y alignment)
    pub stack_size: usize,
    /// Shadow space (32 para Win64, 0 para SysV)
    pub shadow_space: usize,
    /// Registros callee-saved que deben preservarse
    pub callee_saved: Vec<String>,
    /// Registro de retorno
    pub return_reg: String,
    /// ¿El stack está 16-aligned?
    pub stack_aligned: bool,
}

/// Validador y generador de ABI
#[derive(Debug)]
pub struct AbiValidator {
    convention: CallingConvention,
}

impl AbiValidator {
    pub fn new(convention: CallingConvention) -> Self {
        Self { convention }
    }

    /// Calcular el layout completo de una llamada a función
    pub fn calculate_call_layout(
        &self,
        args: &[(String, ArgType)],
    ) -> Result<CallLayout, AbiError> {
        match self.convention {
            CallingConvention::Win64 => self.layout_win64(args),
            CallingConvention::SysV => self.layout_sysv(args),
            _ => Err(AbiError::UnsupportedConvention(self.convention)),
        }
    }

    /// Win64 ABI layout
    fn layout_win64(&self, args: &[(String, ArgType)]) -> Result<CallLayout, AbiError> {
        let gp_regs = ["rcx", "rdx", "r8", "r9"];
        let xmm_regs = ["xmm0", "xmm1", "xmm2", "xmm3"];

        let mut arg_layouts = Vec::new();
        let mut stack_offset: usize = 0;

        for (i, (name, arg_type)) in args.iter().enumerate() {
            let slot = if i < 4 {
                // Win64: Posición i determina el registro, no el tipo
                // Los primeros 4 args van en registros basados en su POSICIÓN
                match arg_type {
                    ArgType::Float32 | ArgType::Float64 => {
                        RegisterSlot::Xmm(xmm_regs[i].to_string())
                    }
                    ArgType::Integer | ArgType::Struct { .. } => {
                        RegisterSlot::Gp(gp_regs[i].to_string())
                    }
                }
            } else {
                // Args 5+ van en stack
                let size = match arg_type {
                    ArgType::Integer | ArgType::Float64 => 8,
                    ArgType::Float32 => 4,
                    ArgType::Struct { size } => align_up(*size, 8),
                };
                let offset = stack_offset;
                stack_offset += align_up(size, 8);
                RegisterSlot::Stack { offset, size }
            };

            arg_layouts.push(ArgLayout {
                index: i,
                name: name.clone(),
                arg_type: *arg_type,
                slot,
            });
        }

        // Shadow space: SIEMPRE 32 bytes (incluso sin argumentos)
        let shadow_space = 32;
        let total_stack = align_up(stack_offset + shadow_space, 16);

        Ok(CallLayout {
            convention: CallingConvention::Win64,
            args: arg_layouts,
            stack_size: total_stack,
            shadow_space,
            callee_saved: vec![
                "rbx".into(), "rbp".into(), "rdi".into(), "rsi".into(),
                "r12".into(), "r13".into(), "r14".into(), "r15".into(),
                "xmm6".into(), "xmm7".into(), "xmm8".into(), "xmm9".into(),
                "xmm10".into(), "xmm11".into(), "xmm12".into(), "xmm13".into(),
                "xmm14".into(), "xmm15".into(),
            ],
            return_reg: "rax".into(),
            stack_aligned: true,
        })
    }

    /// SysV x86-64 ABI layout
    fn layout_sysv(&self, args: &[(String, ArgType)]) -> Result<CallLayout, AbiError> {
        let gp_regs = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
        let xmm_regs = ["xmm0", "xmm1", "xmm2", "xmm3", "xmm4", "xmm5", "xmm6", "xmm7"];

        let mut arg_layouts = Vec::new();
        let mut gp_idx: usize = 0;
        let mut xmm_idx: usize = 0;
        let mut stack_offset: usize = 0;

        for (i, (name, arg_type)) in args.iter().enumerate() {
            let slot = match arg_type {
                ArgType::Integer | ArgType::Struct { .. } => {
                    if gp_idx < gp_regs.len() {
                        let reg = gp_regs[gp_idx].to_string();
                        gp_idx += 1;
                        RegisterSlot::Gp(reg)
                    } else {
                        let size = match arg_type {
                            ArgType::Struct { size } => align_up(*size, 8),
                            _ => 8,
                        };
                        let offset = stack_offset;
                        stack_offset += align_up(size, 8);
                        RegisterSlot::Stack { offset, size }
                    }
                }
                ArgType::Float32 | ArgType::Float64 => {
                    if xmm_idx < xmm_regs.len() {
                        let reg = xmm_regs[xmm_idx].to_string();
                        xmm_idx += 1;
                        RegisterSlot::Xmm(reg)
                    } else {
                        let size = if matches!(arg_type, ArgType::Float32) { 4 } else { 8 };
                        let offset = stack_offset;
                        stack_offset += align_up(size, 8);
                        RegisterSlot::Stack { offset, size }
                    }
                }
            };

            arg_layouts.push(ArgLayout {
                index: i,
                name: name.clone(),
                arg_type: *arg_type,
                slot,
            });
        }

        let total_stack = align_up(stack_offset, 16);

        Ok(CallLayout {
            convention: CallingConvention::SysV,
            args: arg_layouts,
            stack_size: total_stack,
            shadow_space: 0, // SysV no tiene shadow space
            callee_saved: vec![
                "rbx".into(), "rbp".into(),
                "r12".into(), "r13".into(), "r14".into(), "r15".into(),
            ],
            return_reg: "rax".into(),
            stack_aligned: true,
        })
    }

    /// Generar instrucciones de setup para una llamada
    pub fn generate_call_setup(&self, layout: &CallLayout) -> Vec<String> {
        let mut asm = Vec::new();

        // Allocar stack space (shadow + stack args + alignment)
        if layout.stack_size > 0 {
            asm.push(format!("sub rsp, {}", layout.stack_size));
        }

        // Mover argumentos a sus registros/stack slots
        for arg in &layout.args {
            match &arg.slot {
                RegisterSlot::Gp(reg) => {
                    asm.push(format!("; arg{}: {} → {}", arg.index, arg.name, reg));
                }
                RegisterSlot::Xmm(reg) => {
                    asm.push(format!("; arg{}: {} → {}", arg.index, arg.name, reg));
                }
                RegisterSlot::Stack { offset, size } => {
                    let stack_pos = offset + layout.shadow_space;
                    let size_str = match size {
                        4 => "DWORD",
                        8 => "QWORD",
                        _ => "BYTE",
                    };
                    asm.push(format!(
                        "; arg{}: {} → {} PTR [rsp+{}]",
                        arg.index, arg.name, size_str, stack_pos
                    ));
                }
            }
        }

        asm
    }

    /// Generar cleanup después de la llamada
    pub fn generate_call_cleanup(&self, layout: &CallLayout) -> Vec<String> {
        let mut asm = Vec::new();

        if layout.stack_size > 0 {
            asm.push(format!("add rsp, {}", layout.stack_size));
        }

        asm
    }

    /// Validar que una llamada no viola la ABI
    pub fn validate_call(
        &self,
        expected_args: &[(String, ArgType)],
        actual_args: &[(String, ArgType)],
    ) -> Vec<AbiError> {
        let mut errors = Vec::new();

        if expected_args.len() != actual_args.len() {
            errors.push(AbiError::ArgCountMismatch {
                expected: expected_args.len(),
                actual: actual_args.len(),
            });
            return errors;
        }

        for (i, (expected, actual)) in expected_args.iter().zip(actual_args.iter()).enumerate() {
            if expected.1 != actual.1 {
                errors.push(AbiError::ArgTypeMismatch {
                    index: i,
                    expected: format!("{:?}", expected.1),
                    actual: format!("{:?}", actual.1),
                });
            }
        }

        errors
    }
}

/// Errores de ABI
#[derive(Debug, Clone)]
pub enum AbiError {
    /// Calling convention no soportada
    UnsupportedConvention(CallingConvention),
    /// Número de argumentos no coincide
    ArgCountMismatch { expected: usize, actual: usize },
    /// Tipo de argumento no coincide
    ArgTypeMismatch { index: usize, expected: String, actual: String },
    /// Stack no alineado
    StackMisalignment { expected: usize, actual: usize },
}

impl std::fmt::Display for AbiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AbiError::UnsupportedConvention(cc) => write!(f, "Unsupported calling convention: {:?}", cc),
            AbiError::ArgCountMismatch { expected, actual } => {
                write!(f, "Argument count mismatch: expected {}, got {}", expected, actual)
            }
            AbiError::ArgTypeMismatch { index, expected, actual } => {
                write!(f, "Argument {} type mismatch: expected {}, got {}", index, expected, actual)
            }
            AbiError::StackMisalignment { expected, actual } => {
                write!(f, "Stack misalignment: expected {} bytes, got {}", expected, actual)
            }
        }
    }
}

impl std::error::Error for AbiError {}

/// Align up helper
#[inline(always)]
const fn align_up(value: usize, alignment: usize) -> usize {
    (value + alignment - 1) & !(alignment - 1)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_win64_basic_layout() {
        let abi = AbiValidator::new(CallingConvention::Win64);
        let layout = abi.calculate_call_layout(&[
            ("a".into(), ArgType::Integer),
            ("b".into(), ArgType::Integer),
            ("c".into(), ArgType::Float64),
        ]).unwrap();

        assert_eq!(layout.shadow_space, 32);
        assert_eq!(layout.args[0].slot, RegisterSlot::Gp("rcx".into()));
        assert_eq!(layout.args[1].slot, RegisterSlot::Gp("rdx".into()));
        assert_eq!(layout.args[2].slot, RegisterSlot::Xmm("xmm2".into()));
    }

    #[test]
    fn test_win64_stack_args() {
        let abi = AbiValidator::new(CallingConvention::Win64);
        let layout = abi.calculate_call_layout(&[
            ("a".into(), ArgType::Integer),
            ("b".into(), ArgType::Integer),
            ("c".into(), ArgType::Integer),
            ("d".into(), ArgType::Integer),
            ("e".into(), ArgType::Integer), // 5th arg → stack
        ]).unwrap();

        assert!(matches!(layout.args[4].slot, RegisterSlot::Stack { .. }));
        assert!(layout.stack_size >= 40); // 32 shadow + 8 for 5th arg
    }

    #[test]
    fn test_sysv_layout() {
        let abi = AbiValidator::new(CallingConvention::SysV);
        let layout = abi.calculate_call_layout(&[
            ("a".into(), ArgType::Integer),
            ("b".into(), ArgType::Float64),
            ("c".into(), ArgType::Integer),
        ]).unwrap();

        assert_eq!(layout.shadow_space, 0);
        assert_eq!(layout.args[0].slot, RegisterSlot::Gp("rdi".into()));
        assert_eq!(layout.args[1].slot, RegisterSlot::Xmm("xmm0".into()));
        assert_eq!(layout.args[2].slot, RegisterSlot::Gp("rsi".into()));
    }

    #[test]
    fn test_abi_validation() {
        let abi = AbiValidator::new(CallingConvention::Win64);
        let expected = vec![
            ("x".into(), ArgType::Integer),
            ("y".into(), ArgType::Float64),
        ];
        let actual = vec![
            ("x".into(), ArgType::Integer),
            ("y".into(), ArgType::Integer), // Wrong! Should be Float64
        ];

        let errors = abi.validate_call(&expected, &actual);
        assert_eq!(errors.len(), 1);
    }
}
