// ============================================================
// CUDead-BIB — GPU Intermediate Representation
// ============================================================
// IR específico para operaciones GPU
// SSA-form con análisis de hilos y memoria
// ============================================================

use super::primitives::{Dim3, KernelDef, KernelType, ParamType};

/// GPU IR Operation
#[derive(Debug, Clone)]
pub enum CudeadOp {
    // Arithmetic
    Add { dst: u32, src1: u32, src2: u32, ty: IrType },
    Sub { dst: u32, src1: u32, src2: u32, ty: IrType },
    Mul { dst: u32, src1: u32, src2: u32, ty: IrType },
    Div { dst: u32, src1: u32, src2: u32, ty: IrType },
    Fma { dst: u32, src1: u32, src2: u32, src3: u32, ty: IrType },
    Neg { dst: u32, src: u32, ty: IrType },

    // Memory
    Load { dst: u32, addr: u32, ty: IrType },
    Store { addr: u32, src: u32, ty: IrType },
    LoadShared { dst: u32, addr: u32, ty: IrType },
    StoreShared { addr: u32, src: u32, ty: IrType },

    // Thread indexing
    ThreadIdxX { dst: u32 },
    ThreadIdxY { dst: u32 },
    ThreadIdxZ { dst: u32 },
    BlockIdxX { dst: u32 },
    BlockIdxY { dst: u32 },
    BlockIdxZ { dst: u32 },
    BlockDimX { dst: u32 },
    BlockDimY { dst: u32 },
    BlockDimZ { dst: u32 },
    GridDimX { dst: u32 },
    GridDimY { dst: u32 },
    GridDimZ { dst: u32 },

    // Control flow
    Branch { target: u32 },
    BranchCond { cond: u32, target_true: u32, target_false: u32 },
    Label { id: u32 },
    Return,

    // Synchronization
    SyncThreads,
    MemoryFence,

    // Comparison
    Cmp { dst: u32, src1: u32, src2: u32, op: CmpOp, ty: IrType },

    // Conversion
    Convert { dst: u32, src: u32, from: IrType, to: IrType },

    // Constants
    Const { dst: u32, value: IrConst },

    // Parameter load
    LoadParam { dst: u32, param_idx: u32, ty: IrType },

    // Phi node (SSA)
    Phi { dst: u32, sources: Vec<(u32, u32)>, ty: IrType }, // (value, block)
}

/// IR types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrType {
    I32,
    I64,
    U32,
    U64,
    F32,
    F64,
    Pred, // Predicate (boolean)
    Ptr,  // Pointer
}

impl IrType {
    pub fn size(&self) -> usize {
        match self {
            IrType::I32 | IrType::U32 | IrType::F32 => 4,
            IrType::I64 | IrType::U64 | IrType::F64 | IrType::Ptr => 8,
            IrType::Pred => 1,
        }
    }

    pub fn to_ptx(&self) -> &'static str {
        match self {
            IrType::I32 => ".s32",
            IrType::I64 => ".s64",
            IrType::U32 => ".u32",
            IrType::U64 => ".u64",
            IrType::F32 => ".f32",
            IrType::F64 => ".f64",
            IrType::Pred => ".pred",
            IrType::Ptr => ".u64",
        }
    }
}

impl From<ParamType> for IrType {
    fn from(pt: ParamType) -> Self {
        match pt {
            ParamType::Float => IrType::F32,
            ParamType::Double => IrType::F64,
            ParamType::Int => IrType::I32,
            ParamType::Long => IrType::I64,
            ParamType::Uint => IrType::U32,
            ParamType::Ulong => IrType::U64,
            ParamType::Void => IrType::U32, // Default
        }
    }
}

/// Comparison operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CmpOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

impl CmpOp {
    pub fn to_ptx(&self) -> &'static str {
        match self {
            CmpOp::Eq => ".eq",
            CmpOp::Ne => ".ne",
            CmpOp::Lt => ".lt",
            CmpOp::Le => ".le",
            CmpOp::Gt => ".gt",
            CmpOp::Ge => ".ge",
        }
    }
}

/// IR constant values
#[derive(Debug, Clone)]
pub enum IrConst {
    I32(i32),
    I64(i64),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
}

impl IrConst {
    pub fn ir_type(&self) -> IrType {
        match self {
            IrConst::I32(_) => IrType::I32,
            IrConst::I64(_) => IrType::I64,
            IrConst::U32(_) => IrType::U32,
            IrConst::U64(_) => IrType::U64,
            IrConst::F32(_) => IrType::F32,
            IrConst::F64(_) => IrType::F64,
        }
    }
}

/// Basic block in IR
#[derive(Debug, Clone)]
pub struct IrBlock {
    pub id: u32,
    pub ops: Vec<CudeadOp>,
    pub predecessors: Vec<u32>,
    pub successors: Vec<u32>,
}

impl IrBlock {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            ops: Vec::new(),
            predecessors: Vec::new(),
            successors: Vec::new(),
        }
    }

    pub fn emit(&mut self, op: CudeadOp) {
        self.ops.push(op);
    }
}

/// Kernel IR
#[derive(Debug, Clone)]
pub struct KernelIR {
    pub name: String,
    pub kernel_type: KernelType,
    pub params: Vec<(String, IrType, bool)>, // (name, type, is_pointer)
    pub blocks: Vec<IrBlock>,
    pub shared_memory: usize,
    pub next_reg: u32,
    pub next_block: u32,
}

impl KernelIR {
    pub fn new(def: &KernelDef) -> Self {
        let params = def
            .params
            .iter()
            .map(|p| (p.name.clone(), IrType::from(p.param_type), p.is_pointer))
            .collect();

        Self {
            name: def.name.clone(),
            kernel_type: def.kernel_type,
            params,
            blocks: vec![IrBlock::new(0)],
            shared_memory: def.shared_memory,
            next_reg: 0,
            next_block: 1,
        }
    }

    /// Allocate a new virtual register
    pub fn alloc_reg(&mut self) -> u32 {
        let reg = self.next_reg;
        self.next_reg += 1;
        reg
    }

    /// Create a new basic block
    pub fn new_block(&mut self) -> u32 {
        let id = self.next_block;
        self.next_block += 1;
        self.blocks.push(IrBlock::new(id));
        id
    }

    /// Get current block
    pub fn current_block(&mut self) -> &mut IrBlock {
        self.blocks.last_mut().unwrap()
    }

    /// Emit to current block
    pub fn emit(&mut self, op: CudeadOp) {
        self.current_block().emit(op);
    }

    /// Total instruction count
    pub fn instruction_count(&self) -> usize {
        self.blocks.iter().map(|b| b.ops.len()).sum()
    }
}

/// Complete CUDead IR (multiple kernels)
#[derive(Debug, Clone)]
pub struct CudeadIR {
    pub kernels: Vec<KernelIR>,
    pub device_functions: Vec<KernelIR>,
    pub shared_memory_total: usize,
}

impl CudeadIR {
    pub fn new() -> Self {
        Self {
            kernels: Vec::new(),
            device_functions: Vec::new(),
            shared_memory_total: 0,
        }
    }

    pub fn add_kernel(&mut self, kernel: KernelIR) {
        self.shared_memory_total += kernel.shared_memory;
        match kernel.kernel_type {
            KernelType::Kernel => self.kernels.push(kernel),
            KernelType::Device => self.device_functions.push(kernel),
        }
    }
}

impl Default for CudeadIR {
    fn default() -> Self {
        Self::new()
    }
}

/// IR Generator from AST
pub struct CudeadIRGenerator {
    current_kernel: Option<KernelIR>,
}

impl CudeadIRGenerator {
    pub fn new() -> Self {
        Self {
            current_kernel: None,
        }
    }

    pub fn generate(&self, ast: &super::parser::CudeadAst) -> Result<CudeadIR, super::CudeadError> {
        let mut ir = CudeadIR::new();

        for kernel_ast in &ast.kernels {
            let kernel_def = KernelDef {
                name: kernel_ast.name.clone(),
                kernel_type: kernel_ast.kernel_type,
                params: kernel_ast.params.clone(),
                body: kernel_ast.body.clone(),
                shared_memory: kernel_ast.shared_memory,
                registers_used: 0,
            };

            let mut kernel_ir = KernelIR::new(&kernel_def);

            // Generate IR for kernel body
            self.generate_kernel_body(&mut kernel_ir, &kernel_ast.body)?;

            ir.add_kernel(kernel_ir);
        }

        Ok(ir)
    }

    fn generate_kernel_body(
        &self,
        kernel: &mut KernelIR,
        body: &str,
    ) -> Result<(), super::CudeadError> {
        // Parse body and generate IR
        // For now, generate a simple vectorAdd pattern

        // Load thread index
        let tid = kernel.alloc_reg();
        kernel.emit(CudeadOp::ThreadIdxX { dst: tid });

        // Load block index
        let bid = kernel.alloc_reg();
        kernel.emit(CudeadOp::BlockIdxX { dst: bid });

        // Load block dim
        let bdim = kernel.alloc_reg();
        kernel.emit(CudeadOp::BlockDimX { dst: bdim });

        // Compute global index: i = blockIdx.x * blockDim.x + threadIdx.x
        let tmp = kernel.alloc_reg();
        kernel.emit(CudeadOp::Mul {
            dst: tmp,
            src1: bid,
            src2: bdim,
            ty: IrType::I32,
        });

        let idx = kernel.alloc_reg();
        kernel.emit(CudeadOp::Add {
            dst: idx,
            src1: tmp,
            src2: tid,
            ty: IrType::I32,
        });

        // Return
        kernel.emit(CudeadOp::Return);

        let _ = body; // Will be used for real parsing
        Ok(())
    }
}

impl Default for CudeadIRGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ir_type_size() {
        assert_eq!(IrType::F32.size(), 4);
        assert_eq!(IrType::F64.size(), 8);
        assert_eq!(IrType::Ptr.size(), 8);
    }

    #[test]
    fn test_kernel_ir() {
        let def = KernelDef::new("test", KernelType::Kernel);
        let mut kernel = KernelIR::new(&def);

        let r0 = kernel.alloc_reg();
        let r1 = kernel.alloc_reg();
        assert_eq!(r0, 0);
        assert_eq!(r1, 1);

        kernel.emit(CudeadOp::ThreadIdxX { dst: r0 });
        assert_eq!(kernel.instruction_count(), 1);
    }

    #[test]
    fn test_cudead_ir() {
        let mut ir = CudeadIR::new();
        let def = KernelDef::new("vectorAdd", KernelType::Kernel);
        let kernel = KernelIR::new(&def);
        ir.add_kernel(kernel);
        assert_eq!(ir.kernels.len(), 1);
    }
}
