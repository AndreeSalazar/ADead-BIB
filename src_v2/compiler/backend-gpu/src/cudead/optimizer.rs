// ============================================================
// CUDead-BIB — GPU Optimizer
// ============================================================
// Optimizaciones específicas para GPU
// - Warp efficiency analysis
// - Memory coalescing optimizer
// - Shared memory optimizer
// - Occupancy maximizer
// ============================================================

use super::ir::{CudeadIR, CudeadOp, IrBlock, IrType, KernelIR};
use super::GpuArch;

/// GPU Optimizer
pub struct GpuOptimizer {
    arch: GpuArch,
}

impl GpuOptimizer {
    pub fn new(arch: GpuArch) -> Self {
        Self { arch }
    }

    /// Optimize IR
    pub fn optimize(&self, ir: &CudeadIR) -> Result<CudeadIR, super::CudeadError> {
        let mut optimized = ir.clone();

        for kernel in &mut optimized.kernels {
            self.optimize_kernel(kernel)?;
        }

        for func in &mut optimized.device_functions {
            self.optimize_kernel(func)?;
        }

        Ok(optimized)
    }

    fn optimize_kernel(&self, kernel: &mut KernelIR) -> Result<(), super::CudeadError> {
        // Pass 1: Constant folding
        self.constant_folding(kernel);

        // Pass 2: Dead code elimination
        self.dead_code_elimination(kernel);

        // Pass 3: Memory coalescing optimization
        self.optimize_memory_access(kernel);

        // Pass 4: Instruction combining
        self.instruction_combining(kernel);

        // Pass 5: Loop unrolling hints
        self.analyze_loops(kernel);

        Ok(())
    }

    /// Constant folding
    fn constant_folding(&self, kernel: &mut KernelIR) {
        for block in &mut kernel.blocks {
            let mut i = 0;
            while i < block.ops.len() {
                if let Some(folded) = self.try_fold_constant(&block.ops, i) {
                    block.ops[i] = folded;
                }
                i += 1;
            }
        }
    }

    fn try_fold_constant(&self, ops: &[CudeadOp], idx: usize) -> Option<CudeadOp> {
        // Look for patterns like: const + const -> const
        // This is a simplified version
        None // TODO: Implement full constant folding
    }

    /// Dead code elimination
    fn dead_code_elimination(&self, kernel: &mut KernelIR) {
        // Track which registers are used
        let mut used_regs = std::collections::HashSet::new();

        // First pass: collect all used registers (from reads)
        for block in &kernel.blocks {
            for op in &block.ops {
                self.collect_used_regs(op, &mut used_regs);
            }
        }

        // Second pass: remove writes to unused registers
        for block in &mut kernel.blocks {
            block.ops.retain(|op| self.is_op_used(op, &used_regs));
        }
    }

    fn collect_used_regs(&self, op: &CudeadOp, used: &mut std::collections::HashSet<u32>) {
        match op {
            CudeadOp::Add { src1, src2, .. }
            | CudeadOp::Sub { src1, src2, .. }
            | CudeadOp::Mul { src1, src2, .. }
            | CudeadOp::Div { src1, src2, .. } => {
                used.insert(*src1);
                used.insert(*src2);
            }
            CudeadOp::Fma { src1, src2, src3, .. } => {
                used.insert(*src1);
                used.insert(*src2);
                used.insert(*src3);
            }
            CudeadOp::Neg { src, .. } => {
                used.insert(*src);
            }
            CudeadOp::Load { addr, .. } | CudeadOp::LoadShared { addr, .. } => {
                used.insert(*addr);
            }
            CudeadOp::Store { addr, src, .. } | CudeadOp::StoreShared { addr, src, .. } => {
                used.insert(*addr);
                used.insert(*src);
            }
            CudeadOp::BranchCond { cond, .. } => {
                used.insert(*cond);
            }
            CudeadOp::Cmp { src1, src2, .. } => {
                used.insert(*src1);
                used.insert(*src2);
            }
            CudeadOp::Convert { src, .. } => {
                used.insert(*src);
            }
            _ => {}
        }
    }

    fn is_op_used(&self, op: &CudeadOp, used: &std::collections::HashSet<u32>) -> bool {
        // Keep all side-effecting operations
        match op {
            CudeadOp::Store { .. }
            | CudeadOp::StoreShared { .. }
            | CudeadOp::SyncThreads
            | CudeadOp::MemoryFence
            | CudeadOp::Return
            | CudeadOp::Branch { .. }
            | CudeadOp::BranchCond { .. } => true,

            // Check if destination is used
            CudeadOp::Add { dst, .. }
            | CudeadOp::Sub { dst, .. }
            | CudeadOp::Mul { dst, .. }
            | CudeadOp::Div { dst, .. }
            | CudeadOp::Fma { dst, .. }
            | CudeadOp::Neg { dst, .. }
            | CudeadOp::Load { dst, .. }
            | CudeadOp::LoadShared { dst, .. }
            | CudeadOp::ThreadIdxX { dst }
            | CudeadOp::ThreadIdxY { dst }
            | CudeadOp::ThreadIdxZ { dst }
            | CudeadOp::BlockIdxX { dst }
            | CudeadOp::BlockIdxY { dst }
            | CudeadOp::BlockIdxZ { dst }
            | CudeadOp::BlockDimX { dst }
            | CudeadOp::BlockDimY { dst }
            | CudeadOp::BlockDimZ { dst }
            | CudeadOp::GridDimX { dst }
            | CudeadOp::GridDimY { dst }
            | CudeadOp::GridDimZ { dst }
            | CudeadOp::Cmp { dst, .. }
            | CudeadOp::Convert { dst, .. }
            | CudeadOp::Const { dst, .. }
            | CudeadOp::LoadParam { dst, .. }
            | CudeadOp::Phi { dst, .. } => used.contains(dst),

            CudeadOp::Label { .. } => true,
        }
    }

    /// Memory access optimization for coalescing
    fn optimize_memory_access(&self, kernel: &mut KernelIR) {
        // Analyze memory access patterns
        for block in &mut kernel.blocks {
            self.analyze_memory_pattern(block);
        }
    }

    fn analyze_memory_pattern(&self, block: &mut IrBlock) {
        // Look for sequential loads/stores that can be vectorized
        let mut load_sequences: Vec<(usize, Vec<usize>)> = Vec::new();
        let mut current_seq: Vec<usize> = Vec::new();
        let mut seq_start = 0;

        for (i, op) in block.ops.iter().enumerate() {
            match op {
                CudeadOp::Load { .. } | CudeadOp::Store { .. } => {
                    if current_seq.is_empty() {
                        seq_start = i;
                    }
                    current_seq.push(i);
                }
                _ => {
                    if current_seq.len() >= 4 {
                        load_sequences.push((seq_start, current_seq.clone()));
                    }
                    current_seq.clear();
                }
            }
        }

        // Mark sequences for vectorization (TODO: actually vectorize)
        for (start, seq) in load_sequences {
            // Could convert 4 x f32 loads to 1 x float4 load
            let _ = (start, seq);
        }
    }

    /// Instruction combining (peephole optimization)
    fn instruction_combining(&self, kernel: &mut KernelIR) {
        for block in &mut kernel.blocks {
            let mut i = 0;
            while i + 1 < block.ops.len() {
                if let Some(combined) = self.try_combine(&block.ops[i], &block.ops[i + 1]) {
                    block.ops[i] = combined;
                    block.ops.remove(i + 1);
                } else {
                    i += 1;
                }
            }
        }
    }

    fn try_combine(&self, op1: &CudeadOp, op2: &CudeadOp) -> Option<CudeadOp> {
        // Look for mul + add -> fma
        match (op1, op2) {
            (
                CudeadOp::Mul {
                    dst: mul_dst,
                    src1: mul_src1,
                    src2: mul_src2,
                    ty: mul_ty,
                },
                CudeadOp::Add {
                    dst: add_dst,
                    src1: add_src1,
                    src2: add_src2,
                    ty: add_ty,
                },
            ) if mul_dst == add_src1 && mul_ty == add_ty && matches!(mul_ty, IrType::F32 | IrType::F64) => {
                Some(CudeadOp::Fma {
                    dst: *add_dst,
                    src1: *mul_src1,
                    src2: *mul_src2,
                    src3: *add_src2,
                    ty: *mul_ty,
                })
            }
            _ => None,
        }
    }

    /// Analyze loops for unrolling opportunities
    fn analyze_loops(&self, kernel: &mut KernelIR) {
        // Look for small loops that could benefit from unrolling
        // This is a heuristic analysis
        for block in &kernel.blocks {
            let mut branch_count = 0;
            let mut op_count = 0;

            for op in &block.ops {
                op_count += 1;
                if matches!(op, CudeadOp::Branch { .. } | CudeadOp::BranchCond { .. }) {
                    branch_count += 1;
                }
            }

            // Small loops (< 20 ops) with single back-edge are good candidates
            if op_count < 20 && branch_count == 1 {
                // Mark for unrolling (TODO: implement)
            }
        }
    }
}

impl Default for GpuOptimizer {
    fn default() -> Self {
        Self::new(GpuArch::Ampere)
    }
}

/// Occupancy calculator
pub struct OccupancyCalculator {
    arch: GpuArch,
}

impl OccupancyCalculator {
    pub fn new(arch: GpuArch) -> Self {
        Self { arch }
    }

    /// Calculate theoretical occupancy
    pub fn calculate(
        &self,
        threads_per_block: u32,
        registers_per_thread: u32,
        shared_memory: usize,
    ) -> OccupancyResult {
        let warp_size = self.arch.warp_size();
        let max_threads = self.arch.max_threads_per_block();
        let max_shared = self.arch.max_shared_memory() as usize;
        let regs_per_sm = self.arch.registers_per_sm();

        // Warps per block
        let warps_per_block = (threads_per_block + warp_size - 1) / warp_size;

        // Max warps per SM (64 for most architectures)
        let max_warps_per_sm = 64u32;

        // Limit by threads
        let blocks_by_threads = max_warps_per_sm / warps_per_block;

        // Limit by registers
        let regs_per_block = registers_per_thread * threads_per_block;
        let blocks_by_regs = if regs_per_block > 0 {
            regs_per_sm / regs_per_block
        } else {
            blocks_by_threads
        };

        // Limit by shared memory
        let blocks_by_shared = if shared_memory > 0 {
            (max_shared / shared_memory) as u32
        } else {
            blocks_by_threads
        };

        // Actual blocks per SM
        let blocks_per_sm = blocks_by_threads.min(blocks_by_regs).min(blocks_by_shared);

        // Actual warps per SM
        let warps_per_sm = blocks_per_sm * warps_per_block;

        // Occupancy
        let occupancy = warps_per_sm as f32 / max_warps_per_sm as f32;

        OccupancyResult {
            occupancy,
            blocks_per_sm,
            warps_per_sm,
            limiting_factor: if blocks_per_sm == blocks_by_threads {
                LimitingFactor::Threads
            } else if blocks_per_sm == blocks_by_regs {
                LimitingFactor::Registers
            } else {
                LimitingFactor::SharedMemory
            },
            threads_per_block,
            registers_per_thread,
            shared_memory,
        }
    }

    /// Suggest optimal block size
    pub fn suggest_block_size(&self, registers_per_thread: u32, shared_memory: usize) -> u32 {
        let mut best_occupancy = 0.0f32;
        let mut best_block_size = 256u32;

        for block_size in [64, 128, 192, 256, 384, 512, 768, 1024] {
            let result = self.calculate(block_size, registers_per_thread, shared_memory);
            if result.occupancy > best_occupancy {
                best_occupancy = result.occupancy;
                best_block_size = block_size;
            }
        }

        best_block_size
    }
}

/// Occupancy calculation result
#[derive(Debug, Clone)]
pub struct OccupancyResult {
    pub occupancy: f32,
    pub blocks_per_sm: u32,
    pub warps_per_sm: u32,
    pub limiting_factor: LimitingFactor,
    pub threads_per_block: u32,
    pub registers_per_thread: u32,
    pub shared_memory: usize,
}

impl std::fmt::Display for OccupancyResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Occupancy: {:.1}%", self.occupancy * 100.0)?;
        writeln!(f, "Blocks per SM: {}", self.blocks_per_sm)?;
        writeln!(f, "Warps per SM: {}", self.warps_per_sm)?;
        writeln!(f, "Limiting factor: {:?}", self.limiting_factor)?;
        Ok(())
    }
}

/// What limits occupancy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LimitingFactor {
    Threads,
    Registers,
    SharedMemory,
}

/// Warp efficiency analyzer
pub struct WarpEfficiencyAnalyzer;

impl WarpEfficiencyAnalyzer {
    /// Analyze warp efficiency of a kernel
    pub fn analyze(kernel: &KernelIR) -> WarpEfficiencyReport {
        let mut divergent_branches = 0;
        let mut total_branches = 0;
        let mut sync_points = 0;

        for block in &kernel.blocks {
            for op in &block.ops {
                match op {
                    CudeadOp::BranchCond { .. } => {
                        total_branches += 1;
                        // Heuristic: assume data-dependent branches are divergent
                        divergent_branches += 1;
                    }
                    CudeadOp::SyncThreads => {
                        sync_points += 1;
                    }
                    _ => {}
                }
            }
        }

        let efficiency = if total_branches > 0 {
            1.0 - (divergent_branches as f32 / total_branches as f32 * 0.5)
        } else {
            1.0
        };

        WarpEfficiencyReport {
            efficiency,
            divergent_branches,
            total_branches,
            sync_points,
        }
    }
}

/// Warp efficiency report
#[derive(Debug, Clone)]
pub struct WarpEfficiencyReport {
    pub efficiency: f32,
    pub divergent_branches: u32,
    pub total_branches: u32,
    pub sync_points: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_occupancy_calculator() {
        let calc = OccupancyCalculator::new(GpuArch::Ampere);
        let result = calc.calculate(256, 32, 0);
        assert!(result.occupancy > 0.0);
        assert!(result.occupancy <= 1.0);
    }

    #[test]
    fn test_suggest_block_size() {
        let calc = OccupancyCalculator::new(GpuArch::Ampere);
        let size = calc.suggest_block_size(32, 0);
        assert!(size >= 64 && size <= 1024);
    }

    #[test]
    fn test_optimizer() {
        let optimizer = GpuOptimizer::new(GpuArch::Ampere);
        let ir = CudeadIR::new();
        let result = optimizer.optimize(&ir);
        assert!(result.is_ok());
    }
}
