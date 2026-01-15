// ============================================================================
// gpu/mod.adB — Módulo GPU: Opcodes HEX Directos
// ============================================================================
// Funciones que emiten opcodes GPU (0xC0DA...) directamente.
// Cada función mapea 1:1 a un opcode GPU de ADead-BIB.
//
// Uso:
//   gpu::init()              // Emite: 0xC0DA0001
//   gpu::alloc(4096, reg0)   // Emite: 0xC0DA0010 + params
//   gpu::matmul(a, b, c)     // Emite: 0xC0DA0020 + params
// ============================================================================

#![exports(
    // Registros GPU
    reg0, reg1, reg2, reg3, reg4, reg5, reg6, reg7,
    // Control
    init, shutdown, sync, barrier,
    // Memoria
    alloc, free, copy_h2d, copy_d2h, copy_d2d,
    // Compute
    matmul, add, sub, mul, div, relu, softmax,
    // Tensores
    tensor_create, tensor_reshape, tensor_transpose,
    // Kernels
    kernel_launch, kernel_wait
)]

// ============================================================================
// REGISTROS GPU (constantes tipadas)
// ============================================================================

pub const reg0: GpuReg = GpuReg { id: 0, name: "reg0" }
pub const reg1: GpuReg = GpuReg { id: 1, name: "reg1" }
pub const reg2: GpuReg = GpuReg { id: 2, name: "reg2" }
pub const reg3: GpuReg = GpuReg { id: 3, name: "reg3" }
pub const reg4: GpuReg = GpuReg { id: 4, name: "reg4" }
pub const reg5: GpuReg = GpuReg { id: 5, name: "reg5" }
pub const reg6: GpuReg = GpuReg { id: 6, name: "reg6" }
pub const reg7: GpuReg = GpuReg { id: 7, name: "reg7" }

struct GpuReg {
    id: u8,
    name: string
}

// ============================================================================
// OPCODES GPU (Tabla de referencia)
// ============================================================================
// 0xC0DA0001 - GPU_INIT
// 0xC0DA0002 - GPU_SHUTDOWN
// 0xC0DA0010 - GPU_ALLOC
// 0xC0DA0011 - GPU_FREE
// 0xC0DA0012 - GPU_COPY_H2D
// 0xC0DA0013 - GPU_COPY_D2H
// 0xC0DA0014 - GPU_COPY_D2D
// 0xC0DA0020 - GPU_MATMUL
// 0xC0DA0021 - GPU_ADD
// 0xC0DA0022 - GPU_SUB
// 0xC0DA0023 - GPU_MUL
// 0xC0DA0024 - GPU_DIV
// 0xC0DA0030 - GPU_RELU
// 0xC0DA0031 - GPU_SOFTMAX
// 0xC0DA0040 - GPU_TENSOR_CREATE
// 0xC0DA0041 - GPU_TENSOR_RESHAPE
// 0xC0DA0042 - GPU_TENSOR_TRANSPOSE
// 0xC0DA0050 - GPU_KERNEL_LAUNCH
// 0xC0DA0051 - GPU_KERNEL_WAIT
// 0xC0DA00F0 - GPU_SYNC
// 0xC0DA00F1 - GPU_BARRIER
// 0xC0DAFFFF - GPU_END
// ============================================================================

// ============================================================================
// CONTROL
// ============================================================================

// init — Inicializar contexto GPU
// Opcode: 0xC0DA0001
pub fn init() {
    emit_opcode(0xC0DA0001)
}

// shutdown — Cerrar contexto GPU
// Opcode: 0xC0DA0002
pub fn shutdown() {
    emit_opcode(0xC0DA0002)
}

// sync — Sincronizar GPU (esperar a que termine)
// Opcode: 0xC0DA00F0
pub fn sync() {
    emit_opcode(0xC0DA00F0)
}

// barrier — Barrera de sincronización entre threads
// Opcode: 0xC0DA00F1
pub fn barrier() {
    emit_opcode(0xC0DA00F1)
}

// ============================================================================
// MEMORIA
// ============================================================================

// alloc — Reservar memoria en GPU
// Opcode: 0xC0DA0010 [size:u32] [dest_reg:u8]
pub fn alloc(size: u32, dest: GpuReg) {
    emit_opcode(0xC0DA0010)
    emit_u32(size)
    emit![dest.id]
}

// free — Liberar memoria en GPU
// Opcode: 0xC0DA0011 [reg:u8]
pub fn free(reg: GpuReg) {
    emit_opcode(0xC0DA0011)
    emit![reg.id]
}

// copy_h2d — Copiar de Host (CPU) a Device (GPU)
// Opcode: 0xC0DA0012 [src_ptr:u64] [dest_reg:u8] [size:u32]
pub fn copy_h2d(src_ptr: u64, dest: GpuReg, size: u32) {
    emit_opcode(0xC0DA0012)
    emit_u64(src_ptr)
    emit![dest.id]
    emit_u32(size)
}

// copy_d2h — Copiar de Device (GPU) a Host (CPU)
// Opcode: 0xC0DA0013 [src_reg:u8] [dest_ptr:u64] [size:u32]
pub fn copy_d2h(src: GpuReg, dest_ptr: u64, size: u32) {
    emit_opcode(0xC0DA0013)
    emit![src.id]
    emit_u64(dest_ptr)
    emit_u32(size)
}

// copy_d2d — Copiar de Device a Device
// Opcode: 0xC0DA0014 [src_reg:u8] [dest_reg:u8] [size:u32]
pub fn copy_d2d(src: GpuReg, dest: GpuReg, size: u32) {
    emit_opcode(0xC0DA0014)
    emit![src.id, dest.id]
    emit_u32(size)
}

// ============================================================================
// COMPUTE (Operaciones Matemáticas)
// ============================================================================

// matmul — Multiplicación de matrices: C = A * B
// Opcode: 0xC0DA0020 [a_reg:u8] [b_reg:u8] [c_reg:u8] [m:u32] [n:u32] [k:u32]
pub fn matmul(a: GpuReg, b: GpuReg, c: GpuReg, m: u32, n: u32, k: u32) {
    emit_opcode(0xC0DA0020)
    emit![a.id, b.id, c.id]
    emit_u32(m)
    emit_u32(n)
    emit_u32(k)
}

// add — Suma de tensores: C = A + B
// Opcode: 0xC0DA0021 [a_reg:u8] [b_reg:u8] [c_reg:u8] [size:u32]
pub fn add(a: GpuReg, b: GpuReg, c: GpuReg, size: u32) {
    emit_opcode(0xC0DA0021)
    emit![a.id, b.id, c.id]
    emit_u32(size)
}

// sub — Resta de tensores: C = A - B
// Opcode: 0xC0DA0022 [a_reg:u8] [b_reg:u8] [c_reg:u8] [size:u32]
pub fn sub(a: GpuReg, b: GpuReg, c: GpuReg, size: u32) {
    emit_opcode(0xC0DA0022)
    emit![a.id, b.id, c.id]
    emit_u32(size)
}

// mul — Multiplicación elemento a elemento: C = A * B
// Opcode: 0xC0DA0023 [a_reg:u8] [b_reg:u8] [c_reg:u8] [size:u32]
pub fn mul(a: GpuReg, b: GpuReg, c: GpuReg, size: u32) {
    emit_opcode(0xC0DA0023)
    emit![a.id, b.id, c.id]
    emit_u32(size)
}

// div — División elemento a elemento: C = A / B
// Opcode: 0xC0DA0024 [a_reg:u8] [b_reg:u8] [c_reg:u8] [size:u32]
pub fn div(a: GpuReg, b: GpuReg, c: GpuReg, size: u32) {
    emit_opcode(0xC0DA0024)
    emit![a.id, b.id, c.id]
    emit_u32(size)
}

// ============================================================================
// ACTIVACIONES (Deep Learning)
// ============================================================================

// relu — Rectified Linear Unit: out = max(0, in)
// Opcode: 0xC0DA0030 [in_reg:u8] [out_reg:u8] [size:u32]
pub fn relu(input: GpuReg, output: GpuReg, size: u32) {
    emit_opcode(0xC0DA0030)
    emit![input.id, output.id]
    emit_u32(size)
}

// softmax — Softmax activation
// Opcode: 0xC0DA0031 [in_reg:u8] [out_reg:u8] [size:u32]
pub fn softmax(input: GpuReg, output: GpuReg, size: u32) {
    emit_opcode(0xC0DA0031)
    emit![input.id, output.id]
    emit_u32(size)
}

// ============================================================================
// TENSORES
// ============================================================================

// tensor_create — Crear tensor con dimensiones
// Opcode: 0xC0DA0040 [dest_reg:u8] [dims:u8] [d0:u32] [d1:u32] ...
pub fn tensor_create(dest: GpuReg, dims: u8, d0: u32, d1: u32, d2: u32, d3: u32) {
    emit_opcode(0xC0DA0040)
    emit![dest.id, dims]
    emit_u32(d0)
    emit_u32(d1)
    emit_u32(d2)
    emit_u32(d3)
}

// tensor_reshape — Cambiar forma del tensor
// Opcode: 0xC0DA0041 [reg:u8] [new_dims:u8] [d0:u32] [d1:u32] ...
pub fn tensor_reshape(reg: GpuReg, dims: u8, d0: u32, d1: u32, d2: u32, d3: u32) {
    emit_opcode(0xC0DA0041)
    emit![reg.id, dims]
    emit_u32(d0)
    emit_u32(d1)
    emit_u32(d2)
    emit_u32(d3)
}

// tensor_transpose — Transponer tensor
// Opcode: 0xC0DA0042 [in_reg:u8] [out_reg:u8]
pub fn tensor_transpose(input: GpuReg, output: GpuReg) {
    emit_opcode(0xC0DA0042)
    emit![input.id, output.id]
}

// ============================================================================
// KERNELS
// ============================================================================

// kernel_launch — Lanzar kernel personalizado
// Opcode: 0xC0DA0050 [kernel_id:u32] [grid_x:u32] [grid_y:u32] [grid_z:u32]
pub fn kernel_launch(kernel_id: u32, grid_x: u32, grid_y: u32, grid_z: u32) {
    emit_opcode(0xC0DA0050)
    emit_u32(kernel_id)
    emit_u32(grid_x)
    emit_u32(grid_y)
    emit_u32(grid_z)
}

// kernel_wait — Esperar a que termine un kernel
// Opcode: 0xC0DA0051 [kernel_id:u32]
pub fn kernel_wait(kernel_id: u32) {
    emit_opcode(0xC0DA0051)
    emit_u32(kernel_id)
}

// ============================================================================
// HELPERS INTERNOS
// ============================================================================

fn emit_opcode(opcode: u32) {
    emit![
        ((opcode >> 24) & 0xFF) as u8,
        ((opcode >> 16) & 0xFF) as u8,
        ((opcode >> 8) & 0xFF) as u8,
        (opcode & 0xFF) as u8
    ]
}

fn emit_u32(value: u32) {
    emit![
        (value & 0xFF) as u8,
        ((value >> 8) & 0xFF) as u8,
        ((value >> 16) & 0xFF) as u8,
        ((value >> 24) & 0xFF) as u8
    ]
}

fn emit_u64(value: u64) {
    emit![
        (value & 0xFF) as u8,
        ((value >> 8) & 0xFF) as u8,
        ((value >> 16) & 0xFF) as u8,
        ((value >> 24) & 0xFF) as u8,
        ((value >> 32) & 0xFF) as u8,
        ((value >> 40) & 0xFF) as u8,
        ((value >> 48) & 0xFF) as u8,
        ((value >> 56) & 0xFF) as u8
    ]
}
