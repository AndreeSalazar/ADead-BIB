// ============================================================
// ADead-BIB ISA Layer Abstraction
// ============================================================
// Representación estructurada de instrucciones x86-64.
//
// En lugar de emitir bytes directamente (emit_bytes(&[0x55])),
// construimos una IR tipada: ADeadOp::Push { src: Reg(RBP) }
//
// Flujo: AST → ADeadIR (Vec<ADeadOp>) → Encoder → Bytes
//
// Esto permite:
// - Validación de instrucciones en tiempo de compilación
// - Optimizaciones sobre la IR antes de emitir bytes
// - Multi-target sin reescribir codegen completo
// - Debugging legible (print de instrucciones, no hex)
//
// Autor: Eddi Andreé Salazar Matos
// Email: eddi.salazar.dev@gmail.com
// ============================================================

pub mod encoder;
pub mod decoder;
pub mod isa_compiler;
pub mod optimizer;

// ============================================================
// Registers
// ============================================================

/// Registros x86-64 usados por el compilador ADead-BIB.
///
/// Incluye registros de propósito general (64-bit, 32-bit, 8-bit)
/// y registros SSE para operaciones de punto flotante.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Reg {
    // 64-bit general purpose
    RAX,
    RBX,
    RCX,
    RDX,
    RSI,
    RDI,
    RBP,
    RSP,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,

    // 32-bit (sub-registers)
    EAX,
    ECX,

    // 8-bit (sub-registers)
    AL,

    // SSE registers
    XMM0,
    XMM1,
}

impl Reg {
    /// Retorna true si el registro es de 64 bits.
    pub fn is_64bit(&self) -> bool {
        matches!(
            self,
            Reg::RAX
                | Reg::RBX
                | Reg::RCX
                | Reg::RDX
                | Reg::RSI
                | Reg::RDI
                | Reg::RBP
                | Reg::RSP
                | Reg::R8
                | Reg::R9
                | Reg::R10
                | Reg::R11
                | Reg::R12
                | Reg::R13
                | Reg::R14
                | Reg::R15
        )
    }

    /// Retorna true si el registro es de 32 bits.
    pub fn is_32bit(&self) -> bool {
        matches!(self, Reg::EAX | Reg::ECX)
    }

    /// Retorna true si el registro es de 8 bits.
    pub fn is_8bit(&self) -> bool {
        matches!(self, Reg::AL)
    }

    /// Retorna true si es un registro SSE/XMM.
    pub fn is_xmm(&self) -> bool {
        matches!(self, Reg::XMM0 | Reg::XMM1)
    }
}

impl std::fmt::Display for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Reg::RAX => "rax",
            Reg::RBX => "rbx",
            Reg::RCX => "rcx",
            Reg::RDX => "rdx",
            Reg::RSI => "rsi",
            Reg::RDI => "rdi",
            Reg::RBP => "rbp",
            Reg::RSP => "rsp",
            Reg::R8 => "r8",
            Reg::R9 => "r9",
            Reg::R10 => "r10",
            Reg::R11 => "r11",
            Reg::R12 => "r12",
            Reg::R13 => "r13",
            Reg::R14 => "r14",
            Reg::R15 => "r15",
            Reg::EAX => "eax",
            Reg::ECX => "ecx",
            Reg::AL => "al",
            Reg::XMM0 => "xmm0",
            Reg::XMM1 => "xmm1",
        };
        write!(f, "{}", name)
    }
}

// ============================================================
// Operands
// ============================================================

/// Operandos de instrucciones x86-64.
///
/// Cubre todos los modos de direccionamiento que usa codegen_v2:
/// - Registro directo
/// - Inmediatos (8, 32, 64 bits)
/// - Memoria con base + desplazamiento: `[rbp + disp]`
/// - RIP-relative: `[rip + disp]` (para IAT/tablas)
#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    /// Registro directo: rax, rbx, etc.
    Reg(Reg),
    /// Inmediato de 64 bits (mov rax, imm64)
    Imm64(u64),
    /// Inmediato de 32 bits (mov eax, imm32 / sub rsp, imm32)
    Imm32(i32),
    /// Inmediato de 8 bits (shl rax, 3 / sub rsp, 32)
    Imm8(i8),
    /// Memoria: [base + disp] (ej: [rbp - 8])
    Mem { base: Reg, disp: i32 },
    /// RIP-relative: [rip + disp] (para call indirecto via IAT)
    RipRel(i32),
}

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Reg(r) => write!(f, "{}", r),
            Operand::Imm64(v) => write!(f, "0x{:X}", v),
            Operand::Imm32(v) => write!(f, "{}", v),
            Operand::Imm8(v) => write!(f, "{}", v),
            Operand::Mem { base, disp } => {
                if *disp >= 0 {
                    write!(f, "[{}+{}]", base, disp)
                } else {
                    write!(f, "[{}{}]", base, disp)
                }
            }
            Operand::RipRel(disp) => write!(f, "[rip+{}]", disp),
        }
    }
}

// ============================================================
// Conditions (for Jcc, SetCC)
// ============================================================

/// Condiciones para saltos condicionales y set condicional.
///
/// Mapean directamente a los códigos de condición x86-64:
/// - Equal → ZF=1 (je/sete)
/// - Less → SF≠OF (jl/setl)
/// - etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Condition {
    /// ZF=1 — je / sete
    Equal,
    /// ZF=0 — jne / setne
    NotEqual,
    /// SF≠OF — jl / setl
    Less,
    /// ZF=1 OR SF≠OF — jle / setle
    LessEq,
    /// ZF=0 AND SF=OF — jg / setg
    Greater,
    /// SF=OF — jge / setge
    GreaterEq,
    /// Incondicional (usado en Jmp genérico)
    Always,
}

impl std::fmt::Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Condition::Equal => "e",
            Condition::NotEqual => "ne",
            Condition::Less => "l",
            Condition::LessEq => "le",
            Condition::Greater => "g",
            Condition::GreaterEq => "ge",
            Condition::Always => "",
        };
        write!(f, "{}", name)
    }
}

// ============================================================
// Labels
// ============================================================

/// Label para saltos y targets de call.
///
/// Identificador numérico único generado por `ADeadIR::new_label()`.
/// Se resuelve a un offset concreto durante la fase de encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Label(pub u32);

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, ".L{}", self.0)
    }
}

// ============================================================
// Call Target
// ============================================================

/// Target de una instrucción CALL.
///
/// - `Relative(Label)`: call a una función interna (rel32)
/// - `RipRelative(i32)`: call indirecto via IAT `[rip+disp]`
/// - `Name(String)`: call a función por nombre (se resuelve después)
#[derive(Debug, Clone, PartialEq)]
pub enum CallTarget {
    /// Call relativo a un label interno (call rel32)
    Relative(Label),
    /// Call indirecto via RIP-relative (call [rip+disp], para IAT)
    RipRelative(i32),
    /// Call a función por nombre (resolución diferida)
    Name(String),
}

impl std::fmt::Display for CallTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CallTarget::Relative(label) => write!(f, "{}", label),
            CallTarget::RipRelative(disp) => write!(f, "[rip+{}]", disp),
            CallTarget::Name(name) => write!(f, "{}", name),
        }
    }
}

// ============================================================
// ADeadOp — Instruction Set
// ============================================================

/// Instrucción x86-64 en representación estructurada.
///
/// Cada variante corresponde a una o más instrucciones x86-64
/// que codegen_v2.rs emitía como bytes directos. Ahora se
/// construyen como datos tipados y se codifican en la fase de
/// encoding.
///
/// # Ejemplo
/// ```text
/// // Antes (codegen_v2):
/// emit_bytes(&[0x55]);                    // push rbp
/// emit_bytes(&[0x48, 0x89, 0xE5]);       // mov rbp, rsp
///
/// // Ahora (ISA layer):
/// ir.emit(ADeadOp::Push { src: Operand::Reg(Reg::RBP) });
/// ir.emit(ADeadOp::Mov { dst: Operand::Reg(Reg::RBP), src: Operand::Reg(Reg::RSP) });
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum ADeadOp {
    // ---- Data Movement ----
    /// MOV dst, src — Movimiento de datos entre registros, memoria e inmediatos
    Mov { dst: Operand, src: Operand },

    /// MOVZX dst, src — Zero-extend (ej: movzx rax, al)
    MovZx { dst: Reg, src: Reg },

    /// LEA dst, [base+disp] — Load effective address
    Lea { dst: Reg, src: Operand },

    // ---- Arithmetic ----
    /// ADD dst, src
    Add { dst: Operand, src: Operand },

    /// SUB dst, src
    Sub { dst: Operand, src: Operand },

    /// IMUL dst, src — Multiplicación con signo
    Mul { dst: Reg, src: Reg },

    /// IDIV src — División con signo (RDX:RAX / src → RAX, RDX)
    /// Implica CQO antes de la división.
    Div { src: Reg },

    /// AND dst, src — Bitwise AND
    And { dst: Reg, src: Reg },

    /// OR dst, src — Bitwise OR
    Or { dst: Reg, src: Reg },

    /// XOR dst, src — Bitwise XOR (también usado para zeroing: xor eax, eax)
    Xor { dst: Reg, src: Reg },

    /// INC dst — Incremento
    Inc { dst: Operand },

    /// DEC dst — Decremento
    Dec { dst: Operand },

    /// NEG dst — Negación aritmética (two's complement)
    Neg { dst: Reg },

    /// NOT lógico — Implementado como test+sete+movzx
    Not { dst: Reg },

    /// SHL dst, amount — Shift left
    Shl { dst: Reg, amount: u8 },

    // ---- Comparison & Flags ----
    /// CMP left, right — Comparación (sets flags)
    Cmp { left: Operand, right: Operand },

    /// TEST left, right — AND lógico sin guardar resultado (sets flags)
    Test { left: Reg, right: Reg },

    /// SETcc dst — Set byte según condición (ej: sete al)
    SetCC { cond: Condition, dst: Reg },

    // ---- Stack ----
    /// PUSH src — Push al stack
    Push { src: Operand },

    /// POP dst — Pop del stack
    Pop { dst: Reg },

    // ---- Control Flow ----
    /// CALL target — Llamada a función
    Call { target: CallTarget },

    /// JMP target — Salto incondicional
    Jmp { target: Label },

    /// Jcc target — Salto condicional
    Jcc { cond: Condition, target: Label },

    /// RET — Retorno de función
    Ret,

    /// SYSCALL — Llamada al sistema (Linux)
    Syscall,

    // ---- SSE / Floating Point ----
    /// CVTSI2SD dst, src — Convertir entero a double (int → xmm)
    CvtSi2Sd { dst: Reg, src: Reg },

    /// MOVQ dst, src — Mover entre registro GP y XMM (64-bit)
    MovQ { dst: Reg, src: Reg },

    // ---- Pseudo-instructions ----
    /// Pseudo-instrucción: marca la posición de un label.
    /// No emite bytes, solo registra el offset para resolución de saltos.
    Label(Label),

    /// NOP — No operation
    Nop,

    /// Escape hatch: bytes crudos para casos no cubiertos.
    /// Usar solo cuando no existe una variante tipada equivalente.
    RawBytes(Vec<u8>),

    /// Call indirecto via IAT (Import Address Table) para Windows.
    /// El encoder calcula el offset RIP-relative automáticamente.
    /// iat_rva: RVA del slot IAT (ej: 0x2040 para printf, 0x2048 para scanf)
    CallIAT { iat_rva: u32 },
}

impl std::fmt::Display for ADeadOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ADeadOp::Mov { dst, src } => write!(f, "mov {}, {}", dst, src),
            ADeadOp::MovZx { dst, src } => write!(f, "movzx {}, {}", dst, src),
            ADeadOp::Lea { dst, src } => write!(f, "lea {}, {}", dst, src),
            ADeadOp::Add { dst, src } => write!(f, "add {}, {}", dst, src),
            ADeadOp::Sub { dst, src } => write!(f, "sub {}, {}", dst, src),
            ADeadOp::Mul { dst, src } => write!(f, "imul {}, {}", dst, src),
            ADeadOp::Div { src } => write!(f, "cqo; idiv {}", src),
            ADeadOp::And { dst, src } => write!(f, "and {}, {}", dst, src),
            ADeadOp::Or { dst, src } => write!(f, "or {}, {}", dst, src),
            ADeadOp::Xor { dst, src } => write!(f, "xor {}, {}", dst, src),
            ADeadOp::Inc { dst } => write!(f, "inc {}", dst),
            ADeadOp::Dec { dst } => write!(f, "dec {}", dst),
            ADeadOp::Neg { dst } => write!(f, "neg {}", dst),
            ADeadOp::Not { dst } => write!(f, "not.logical {}", dst),
            ADeadOp::Shl { dst, amount } => write!(f, "shl {}, {}", dst, amount),
            ADeadOp::Cmp { left, right } => write!(f, "cmp {}, {}", left, right),
            ADeadOp::Test { left, right } => write!(f, "test {}, {}", left, right),
            ADeadOp::SetCC { cond, dst } => write!(f, "set{} {}", cond, dst),
            ADeadOp::Push { src } => write!(f, "push {}", src),
            ADeadOp::Pop { dst } => write!(f, "pop {}", dst),
            ADeadOp::Call { target } => write!(f, "call {}", target),
            ADeadOp::Jmp { target } => write!(f, "jmp {}", target),
            ADeadOp::Jcc { cond, target } => write!(f, "j{} {}", cond, target),
            ADeadOp::Ret => write!(f, "ret"),
            ADeadOp::Syscall => write!(f, "syscall"),
            ADeadOp::CvtSi2Sd { dst, src } => write!(f, "cvtsi2sd {}, {}", dst, src),
            ADeadOp::MovQ { dst, src } => write!(f, "movq {}, {}", dst, src),
            ADeadOp::Label(label) => write!(f, "{}:", label),
            ADeadOp::Nop => write!(f, "nop"),
            ADeadOp::RawBytes(bytes) => {
                write!(f, "db ")?;
                for (i, b) in bytes.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "0x{:02X}", b)?;
                }
                Ok(())
            }
            ADeadOp::CallIAT { iat_rva } => write!(f, "call [iat:0x{:04X}]", iat_rva),
        }
    }
}

// ============================================================
// ADeadIR — Instruction Buffer
// ============================================================

/// Buffer de instrucciones ISA para el compilador ADead-BIB.
///
/// Acumula instrucciones `ADeadOp` en orden, gestiona labels
/// para saltos, y mantiene una tabla de strings.
///
/// # Uso
/// ```text
/// let mut ir = ADeadIR::new();
/// let loop_start = ir.new_label();
///
/// ir.emit(ADeadOp::Push { src: Operand::Reg(Reg::RBP) });
/// ir.emit(ADeadOp::Mov {
///     dst: Operand::Reg(Reg::RBP),
///     src: Operand::Reg(Reg::RSP),
/// });
/// ir.emit(ADeadOp::Label(loop_start));
/// ir.emit(ADeadOp::Jmp { target: loop_start });
/// ```
#[derive(Debug, Clone)]
pub struct ADeadIR {
    /// Instrucciones emitidas en orden
    ops: Vec<ADeadOp>,
    /// Contador de labels (cada new_label() incrementa)
    label_counter: u32,
    /// Tabla de strings (para datos estáticos referenciados por el código)
    string_table: Vec<String>,
}

impl ADeadIR {
    /// Crea un nuevo buffer de instrucciones vacío.
    pub fn new() -> Self {
        Self {
            ops: Vec::new(),
            label_counter: 0,
            string_table: Vec::new(),
        }
    }

    /// Emite una instrucción al final del buffer.
    pub fn emit(&mut self, op: ADeadOp) {
        self.ops.push(op);
    }

    /// Genera un nuevo label único para saltos y targets.
    pub fn new_label(&mut self) -> Label {
        let id = self.label_counter;
        self.label_counter += 1;
        Label(id)
    }

    /// Retorna una referencia a las instrucciones emitidas.
    pub fn ops(&self) -> &[ADeadOp] {
        &self.ops
    }

    /// Retorna una referencia mutable a las instrucciones emitidas.
    pub fn ops_mut(&mut self) -> &mut Vec<ADeadOp> {
        &mut self.ops
    }

    /// Agrega un string a la tabla de strings y retorna su índice.
    pub fn add_string(&mut self, s: String) -> usize {
        let idx = self.string_table.len();
        self.string_table.push(s);
        idx
    }

    /// Retorna la tabla de strings.
    pub fn string_table(&self) -> &[String] {
        &self.string_table
    }

    /// Retorna el número total de instrucciones emitidas.
    pub fn len(&self) -> usize {
        self.ops.len()
    }

    /// Retorna true si no se han emitido instrucciones.
    pub fn is_empty(&self) -> bool {
        self.ops.is_empty()
    }
}

impl Default for ADeadIR {
    fn default() -> Self {
        Self::new()
    }
}
