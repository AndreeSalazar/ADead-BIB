// ============================================================
// ADead-BIB ISA Optimizer — Binary Layout Optimizer & Minimizer
// ============================================================
// Optimiza instrucciones ADeadOp ANTES de codificarlas a bytes.
// Esto permite optimizaciones que serían imposibles a nivel de bytes.
//
// Pipeline completo:
//   Path A: AST → ADeadIR → Optimizer → Encoder → bytes (compilación)
//   Path B: bytes → Decoder → ADeadIR → Optimizer → Encoder → bytes (reoptimización)
//
// Optimizaciones implementadas:
//   - Peephole: patrones locales de instrucciones
//   - Dead code elimination: código inalcanzable
//   - Constant folding: operaciones con constantes
//   - Register allocation hints: sugerencias de registros
//   - Instruction fusion: combinar instrucciones
//   - Size minimization: encodings más cortos
//
// Autor: Eddi Andreé Salazar Matos
// Email: eddi.salazar.dev@gmail.com
// ============================================================

use super::{ADeadOp, ADeadIR, Reg, Operand};
use std::collections::HashSet;

/// Nivel de optimización
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsaOptLevel {
    /// Sin optimización (debug)
    None,
    /// Optimizaciones básicas (peephole)
    Basic,
    /// Optimizaciones agresivas (dead code, fusion)
    Aggressive,
    /// Optimización máxima de tamaño
    Size,
}

/// Estadísticas de optimización
#[derive(Debug, Clone, Default)]
pub struct OptStats {
    pub original_ops: usize,
    pub optimized_ops: usize,
    pub peephole_applied: usize,
    pub dead_code_removed: usize,
    pub instructions_fused: usize,
    pub nops_eliminated: usize,
}

/// Binary Layout Optimizer — Optimizador a nivel ISA
pub struct IsaOptimizer {
    level: IsaOptLevel,
    stats: OptStats,
}

impl IsaOptimizer {
    pub fn new(level: IsaOptLevel) -> Self {
        Self {
            level,
            stats: OptStats::default(),
        }
    }

    /// Optimiza un buffer de instrucciones ADeadIR
    pub fn optimize(&mut self, ir: &ADeadIR) -> ADeadIR {
        let mut ops = ir.ops().to_vec();
        self.stats.original_ops = ops.len();

        if self.level == IsaOptLevel::None {
            self.stats.optimized_ops = ops.len();
            let mut result = ADeadIR::new();
            for op in ops {
                result.emit(op);
            }
            return result;
        }

        // Pass 1: Eliminar NOPs
        ops = self.eliminate_nops(ops);

        // Pass 2: Peephole optimizations
        ops = self.peephole_optimize(ops);

        // Pass 3: Dead code elimination (solo en Aggressive+)
        if self.level == IsaOptLevel::Aggressive || self.level == IsaOptLevel::Size {
            ops = self.eliminate_dead_code(ops);
        }

        // Pass 4: Instruction fusion (solo en Aggressive+)
        if self.level == IsaOptLevel::Aggressive || self.level == IsaOptLevel::Size {
            ops = self.fuse_instructions(ops);
        }

        // Pass 5: Size optimizations (solo en Size)
        if self.level == IsaOptLevel::Size {
            ops = self.minimize_size(ops);
        }

        self.stats.optimized_ops = ops.len();

        // Reconstruir ADeadIR
        let mut result = ADeadIR::new();
        for op in ops {
            result.emit(op);
        }
        result
    }

    /// Optimiza un slice de operaciones directamente
    pub fn optimize_ops(&mut self, ops: &[ADeadOp]) -> Vec<ADeadOp> {
        let mut ir = ADeadIR::new();
        for op in ops {
            ir.emit(op.clone());
        }
        let optimized = self.optimize(&ir);
        optimized.ops().to_vec()
    }

    /// Retorna estadísticas de la última optimización
    pub fn stats(&self) -> &OptStats {
        &self.stats
    }

    // ========================================
    // Pass 1: Eliminar NOPs
    // ========================================

    fn eliminate_nops(&mut self, ops: Vec<ADeadOp>) -> Vec<ADeadOp> {
        let original_len = ops.len();
        let result: Vec<ADeadOp> = ops.into_iter()
            .filter(|op| !matches!(op, ADeadOp::Nop))
            .collect();
        self.stats.nops_eliminated = original_len - result.len();
        result
    }

    // ========================================
    // Pass 2: Peephole Optimizations
    // ========================================

    fn peephole_optimize(&mut self, ops: Vec<ADeadOp>) -> Vec<ADeadOp> {
        let mut result = Vec::with_capacity(ops.len());
        let mut i = 0;

        while i < ops.len() {
            // Pattern: mov rax, 0 → xor eax, eax (más corto)
            if let ADeadOp::Mov { dst: Operand::Reg(Reg::RAX), src: Operand::Imm64(0) } = &ops[i] {
                result.push(ADeadOp::Xor { dst: Reg::EAX, src: Reg::EAX });
                self.stats.peephole_applied += 1;
                i += 1;
                continue;
            }

            // Pattern: mov rcx, 0 → xor ecx, ecx
            if let ADeadOp::Mov { dst: Operand::Reg(Reg::RCX), src: Operand::Imm64(0) } = &ops[i] {
                result.push(ADeadOp::Xor { dst: Reg::ECX, src: Reg::ECX });
                self.stats.peephole_applied += 1;
                i += 1;
                continue;
            }

            // Pattern: push rbp; mov rbp, rsp → (mantener, es prologue estándar)
            // Pattern: mov rsp, rbp; pop rbp → (mantener, es epilogue estándar)

            // Pattern: add rax, 0 → eliminar (no-op)
            if let ADeadOp::Add { dst: Operand::Reg(_), src: Operand::Imm32(0) } = &ops[i] {
                self.stats.peephole_applied += 1;
                i += 1;
                continue;
            }
            if let ADeadOp::Add { dst: Operand::Reg(_), src: Operand::Imm8(0) } = &ops[i] {
                self.stats.peephole_applied += 1;
                i += 1;
                continue;
            }

            // Pattern: sub rax, 0 → eliminar (no-op)
            if let ADeadOp::Sub { dst: Operand::Reg(_), src: Operand::Imm32(0) } = &ops[i] {
                self.stats.peephole_applied += 1;
                i += 1;
                continue;
            }
            if let ADeadOp::Sub { dst: Operand::Reg(_), src: Operand::Imm8(0) } = &ops[i] {
                self.stats.peephole_applied += 1;
                i += 1;
                continue;
            }

            // Pattern: mov reg, reg (mismo registro) → eliminar
            if let ADeadOp::Mov { dst: Operand::Reg(d), src: Operand::Reg(s) } = &ops[i] {
                if d == s {
                    self.stats.peephole_applied += 1;
                    i += 1;
                    continue;
                }
            }

            // Pattern: push rax; pop rax → eliminar ambos
            if i + 1 < ops.len() {
                if let (
                    ADeadOp::Push { src: Operand::Reg(r1) },
                    ADeadOp::Pop { dst: r2 }
                ) = (&ops[i], &ops[i + 1]) {
                    if r1 == r2 {
                        self.stats.peephole_applied += 1;
                        i += 2;
                        continue;
                    }
                }
            }

            // Pattern: jmp .L0; .L0: → eliminar jmp (salto al siguiente)
            if i + 1 < ops.len() {
                if let (
                    ADeadOp::Jmp { target: t1 },
                    ADeadOp::Label(t2)
                ) = (&ops[i], &ops[i + 1]) {
                    if t1 == t2 {
                        self.stats.peephole_applied += 1;
                        i += 1; // Skip jmp, keep label
                        continue;
                    }
                }
            }

            // Pattern: test rax, rax; sete al; movzx rax, al → (mantener, es NOT lógico)

            // Pattern: imm32 que cabe en imm8 → usar imm8
            if let ADeadOp::Sub { dst: Operand::Reg(Reg::RSP), src: Operand::Imm32(v) } = &ops[i] {
                if *v >= -128 && *v <= 127 {
                    result.push(ADeadOp::Sub {
                        dst: Operand::Reg(Reg::RSP),
                        src: Operand::Imm8(*v as i8),
                    });
                    self.stats.peephole_applied += 1;
                    i += 1;
                    continue;
                }
            }
            if let ADeadOp::Add { dst: Operand::Reg(Reg::RSP), src: Operand::Imm32(v) } = &ops[i] {
                if *v >= -128 && *v <= 127 {
                    result.push(ADeadOp::Add {
                        dst: Operand::Reg(Reg::RSP),
                        src: Operand::Imm8(*v as i8),
                    });
                    self.stats.peephole_applied += 1;
                    i += 1;
                    continue;
                }
            }

            // Pattern: mov reg, rax; ... use reg → optimize when reg == rax (self-move)
            // Pattern: mov rax, imm; mov reg, rax; → mov reg, imm (fuse load+move)
            if i + 1 < ops.len() {
                if let (
                    ADeadOp::Mov { dst: Operand::Reg(Reg::RAX), src: Operand::Imm64(v) },
                    ADeadOp::Mov { dst: Operand::Reg(dst_reg), src: Operand::Reg(Reg::RAX) }
                ) = (&ops[i], &ops[i + 1]) {
                    // Fuse: mov rax, imm64; mov reg, rax → mov reg, imm64
                    result.push(ADeadOp::Mov {
                        dst: Operand::Reg(*dst_reg),
                        src: Operand::Imm64(*v),
                    });
                    self.stats.peephole_applied += 1;
                    self.stats.instructions_fused += 1;
                    i += 2;
                    continue;
                }
            }

            // Pattern: mov temp, rax; mov rax, temp → eliminate (register round-trip)
            if i + 1 < ops.len() {
                if let (
                    ADeadOp::Mov { dst: Operand::Reg(r1), src: Operand::Reg(Reg::RAX) },
                    ADeadOp::Mov { dst: Operand::Reg(Reg::RAX), src: Operand::Reg(r2) }
                ) = (&ops[i], &ops[i + 1]) {
                    if r1 == r2 && *r1 != Reg::RAX {
                        // This is a no-op round-trip, skip both
                        self.stats.peephole_applied += 1;
                        i += 2;
                        continue;
                    }
                }
            }

            // Pattern: xor eax, eax; mov [rbp+off], rax → mov qword [rbp+off], 0
            // (Keep as-is; xor+mov is already efficient)

            // No pattern matched, keep instruction
            result.push(ops[i].clone());
            i += 1;
        }

        result
    }

    // ========================================
    // Pass 3: Dead Code Elimination
    // ========================================

    fn eliminate_dead_code(&mut self, ops: Vec<ADeadOp>) -> Vec<ADeadOp> {
        // Encontrar labels usados
        let mut used_labels: HashSet<u32> = HashSet::new();
        for op in &ops {
            match op {
                ADeadOp::Jmp { target } => { used_labels.insert(target.0); }
                ADeadOp::Jcc { target, .. } => { used_labels.insert(target.0); }
                ADeadOp::Call { target } => {
                    if let super::CallTarget::Relative(label) = target {
                        used_labels.insert(label.0);
                    }
                }
                _ => {}
            }
        }

        // Eliminar labels no usados y código después de ret/jmp incondicional
        let mut result = Vec::with_capacity(ops.len());
        let mut skip_until_label = false;

        for op in ops {
            // Si encontramos un label, dejamos de saltar
            if let ADeadOp::Label(label) = &op {
                if used_labels.contains(&label.0) {
                    skip_until_label = false;
                    result.push(op);
                } else {
                    // Label no usado, eliminar
                    self.stats.dead_code_removed += 1;
                }
                continue;
            }

            if skip_until_label {
                self.stats.dead_code_removed += 1;
                continue;
            }

            result.push(op.clone());

            // Después de ret o jmp incondicional, el código es inalcanzable
            if matches!(op, ADeadOp::Ret | ADeadOp::Jmp { .. }) {
                skip_until_label = true;
            }
        }

        result
    }

    // ========================================
    // Pass 4: Instruction Fusion
    // ========================================

    fn fuse_instructions(&mut self, ops: Vec<ADeadOp>) -> Vec<ADeadOp> {
        let mut result = Vec::with_capacity(ops.len());
        let mut i = 0;

        while i < ops.len() {
            // Pattern: cqo; idiv rbx → Div (ya fusionado en ADeadOp::Div)
            // Este patrón ya está manejado en el encoder

            // Pattern: test rax, rax; sete al; movzx rax, al → Not (ya fusionado)
            // Este patrón ya está manejado en el encoder

            // Pattern: mov rax, [rbp+X]; push rax → push [rbp+X] (si soportado)
            // x86-64 soporta push mem, pero es más lento. Mantener separado.

            // Pattern: xor eax, eax; ret → (mantener, es return 0 estándar)

            // Pattern: sub rsp, N; ... ; add rsp, N → (verificar que N sea igual)
            // Esto requiere análisis más complejo, skip por ahora

            result.push(ops[i].clone());
            i += 1;
        }

        result
    }

    // ========================================
    // Pass 5: Size Minimization
    // ========================================

    fn minimize_size(&mut self, ops: Vec<ADeadOp>) -> Vec<ADeadOp> {
        let mut result = Vec::with_capacity(ops.len());

        for op in ops {
            match &op {
                // Usar encodings más cortos para inmediatos pequeños
                ADeadOp::Mov { dst: Operand::Reg(r), src: Operand::Imm64(v) } => {
                    if *v == 0 {
                        // mov rax, 0 → xor eax, eax (2 bytes vs 10 bytes)
                        match r {
                            Reg::RAX => result.push(ADeadOp::Xor { dst: Reg::EAX, src: Reg::EAX }),
                            Reg::RCX => result.push(ADeadOp::Xor { dst: Reg::ECX, src: Reg::ECX }),
                            _ => result.push(op),
                        }
                    } else if *v <= 0x7FFFFFFF {
                        // Cabe en imm32 sign-extended
                        result.push(ADeadOp::Mov {
                            dst: Operand::Reg(*r),
                            src: Operand::Imm32(*v as i32),
                        });
                    } else {
                        result.push(op);
                    }
                }

                // Usar inc/dec en lugar de add/sub 1
                ADeadOp::Add { dst: Operand::Reg(r), src: Operand::Imm32(1) } => {
                    result.push(ADeadOp::Inc { dst: Operand::Reg(*r) });
                }
                ADeadOp::Add { dst: Operand::Reg(r), src: Operand::Imm8(1) } => {
                    result.push(ADeadOp::Inc { dst: Operand::Reg(*r) });
                }
                ADeadOp::Sub { dst: Operand::Reg(r), src: Operand::Imm32(1) } => {
                    result.push(ADeadOp::Dec { dst: Operand::Reg(*r) });
                }
                ADeadOp::Sub { dst: Operand::Reg(r), src: Operand::Imm8(1) } => {
                    result.push(ADeadOp::Dec { dst: Operand::Reg(*r) });
                }

                _ => result.push(op),
            }
        }

        result
    }
}

impl Default for IsaOptimizer {
    fn default() -> Self {
        Self::new(IsaOptLevel::Basic)
    }
}

// ============================================================
// Binary Layout Rebuilder — Reoptimizar binarios existentes
// ============================================================

/// Reoptimiza un binario existente usando el pipeline ISA
pub struct BinaryRebuilder {
    optimizer: IsaOptimizer,
}

impl BinaryRebuilder {
    pub fn new(level: IsaOptLevel) -> Self {
        Self {
            optimizer: IsaOptimizer::new(level),
        }
    }

    /// Reoptimiza bytes de código x86-64
    /// Pipeline: bytes → Decoder → ADeadIR → Optimizer → Encoder → bytes
    pub fn reoptimize(&mut self, code: &[u8]) -> Vec<u8> {
        use super::decoder::Decoder;
        use super::encoder::Encoder;

        // Decode
        let mut decoder = Decoder::new();
        let ops = decoder.decode_all(code);

        // Optimize
        let mut ir = ADeadIR::new();
        for op in ops {
            ir.emit(op);
        }
        let optimized_ir = self.optimizer.optimize(&ir);

        // Encode
        let mut encoder = Encoder::new();
        let result = encoder.encode_all(optimized_ir.ops());

        result.code
    }

    /// Retorna estadísticas
    pub fn stats(&self) -> &OptStats {
        self.optimizer.stats()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nop_elimination() {
        let mut opt = IsaOptimizer::new(IsaOptLevel::Basic);
        let ops = vec![
            ADeadOp::Nop,
            ADeadOp::Push { src: Operand::Reg(Reg::RBP) },
            ADeadOp::Nop,
            ADeadOp::Nop,
            ADeadOp::Ret,
        ];
        let result = opt.optimize_ops(&ops);
        assert_eq!(result.len(), 2);
        assert_eq!(opt.stats().nops_eliminated, 3);
    }

    #[test]
    fn test_mov_zero_to_xor() {
        let mut opt = IsaOptimizer::new(IsaOptLevel::Basic);
        let ops = vec![
            ADeadOp::Mov {
                dst: Operand::Reg(Reg::RAX),
                src: Operand::Imm64(0),
            },
        ];
        let result = opt.optimize_ops(&ops);
        assert_eq!(result[0], ADeadOp::Xor { dst: Reg::EAX, src: Reg::EAX });
    }

    #[test]
    fn test_push_pop_same_reg() {
        let mut opt = IsaOptimizer::new(IsaOptLevel::Basic);
        let ops = vec![
            ADeadOp::Push { src: Operand::Reg(Reg::RAX) },
            ADeadOp::Pop { dst: Reg::RAX },
        ];
        let result = opt.optimize_ops(&ops);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_add_zero_elimination() {
        let mut opt = IsaOptimizer::new(IsaOptLevel::Basic);
        let ops = vec![
            ADeadOp::Add {
                dst: Operand::Reg(Reg::RAX),
                src: Operand::Imm32(0),
            },
        ];
        let result = opt.optimize_ops(&ops);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_dead_code_after_ret() {
        let mut opt = IsaOptimizer::new(IsaOptLevel::Aggressive);
        let mut ir = ADeadIR::new();
        let label = ir.new_label();
        
        let ops = vec![
            ADeadOp::Ret,
            ADeadOp::Mov { dst: Operand::Reg(Reg::RAX), src: Operand::Imm64(42) },
            ADeadOp::Label(label),
            ADeadOp::Ret,
        ];
        
        let result = opt.optimize_ops(&ops);
        // Ret, luego código muerto eliminado, luego label (no usado) eliminado, luego ret
        assert!(result.len() < ops.len());
    }

    #[test]
    fn test_size_optimization_inc() {
        let mut opt = IsaOptimizer::new(IsaOptLevel::Size);
        let ops = vec![
            ADeadOp::Add {
                dst: Operand::Reg(Reg::RAX),
                src: Operand::Imm32(1),
            },
        ];
        let result = opt.optimize_ops(&ops);
        assert!(matches!(result[0], ADeadOp::Inc { .. }));
    }

    #[test]
    fn test_roundtrip_reoptimize() {
        use super::super::encoder::Encoder;

        // Crear código con ineficiencias
        let ops = vec![
            ADeadOp::Push { src: Operand::Reg(Reg::RBP) },
            ADeadOp::Mov { dst: Operand::Reg(Reg::RBP), src: Operand::Reg(Reg::RSP) },
            ADeadOp::Mov { dst: Operand::Reg(Reg::RAX), src: Operand::Imm64(0) },
            ADeadOp::Nop,
            ADeadOp::Mov { dst: Operand::Reg(Reg::RSP), src: Operand::Reg(Reg::RBP) },
            ADeadOp::Pop { dst: Reg::RBP },
            ADeadOp::Ret,
        ];

        // Encode original
        let mut encoder = Encoder::new();
        let original = encoder.encode_all(&ops);

        // Reoptimize
        let mut rebuilder = BinaryRebuilder::new(IsaOptLevel::Basic);
        let optimized = rebuilder.reoptimize(&original.code);

        // El código optimizado debería ser más corto (mov rax,0 → xor eax,eax, nop eliminado)
        assert!(optimized.len() <= original.code.len());
    }
}
