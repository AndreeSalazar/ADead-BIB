# 🔥 Ideas-7: ADead-BIB Sin Límites - HEX + Binario Puro

> **Autor:** Eddi Andreé Salazar Matos | **Hecho con ❤️ en Perú** 🇵🇪

---

## 🎯 Objetivo

Eliminar TODAS las limitaciones del compilador ADead-BIB para generar **binarios puros** sin dependencias externas.

**Filosofía:** HEX + Binario = Control Total

---

## 🚫 Limitaciones Actuales

| Limitación | Problema | Solución |
|------------|----------|----------|
| Dependencia de printf | Requiere msvcrt.dll | Syscalls directos |
| Solo Windows PE | No Linux/macOS | Multi-target |
| Stack 256 bytes | Variables limitadas | Stack dinámico |
| Solo main() | No múltiples funciones | Call table |
| No syscalls | Depende de DLLs | Syscalls nativos |
| No standalone | Necesita runtime | Binario puro |

---

## 🏗️ Arquitectura Mejorada

```
┌─────────────────────────────────────────────────────────────────────┐
│                     ADead-BIB Compiler v2.0                          │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐                  │
│  │   Lexer     │→ │   Parser    │→ │    AST      │                  │
│  └─────────────┘  └─────────────┘  └─────────────┘                  │
│                                          │                          │
│                                          ▼                          │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │                    CODEGEN v2.0 (Sin Límites)                 │   │
│  │  ┌────────────┐  ┌────────────┐  ┌────────────┐              │   │
│  │  │  Syscalls  │  │  Multi-    │  │  Stack     │              │   │
│  │  │  Directos  │  │  Function  │  │  Dinámico  │              │   │
│  │  └────────────┘  └────────────┘  └────────────┘              │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                          │                                          │
│         ┌────────────────┼────────────────┐                        │
│         ▼                ▼                ▼                        │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐                 │
│  │   PE Gen    │  │   ELF Gen   │  │   Raw Bin   │                 │
│  │  (Windows)  │  │  (Linux)    │  │  (Bare)     │                 │
│  └─────────────┘  └─────────────┘  └─────────────┘                 │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 🔧 Mejoras Propuestas

### 1. Syscalls Directos (Sin DLLs)

#### Windows (x64)
```asm
; WriteConsole via syscall (NtWriteFile)
mov r10, rcx
mov eax, 0x08        ; NtWriteFile syscall number
syscall
```

#### Linux (x64)
```asm
; write(1, buf, len)
mov rax, 1           ; sys_write
mov rdi, 1           ; stdout
mov rsi, buf         ; buffer
mov rdx, len         ; length
syscall
```

### 2. Print Sin printf

```rust
// Implementación directa de print
fn emit_print_direct(&mut self, expr: &Expr) {
    if let Expr::String(s) = expr {
        let string_addr = self.get_string_address(s);
        let len = s.len();
        
        // Windows: GetStdHandle + WriteConsoleA
        // mov rcx, -11 (STD_OUTPUT_HANDLE)
        self.emit_bytes(&[0x48, 0xC7, 0xC1, 0xF5, 0xFF, 0xFF, 0xFF]);
        // call GetStdHandle (inline o syscall)
        self.emit_get_std_handle();
        // mov rcx, rax (handle)
        self.emit_bytes(&[0x48, 0x89, 0xC1]);
        // mov rdx, string_addr
        self.emit_bytes(&[0x48, 0xBA]);
        self.emit_u64(string_addr);
        // mov r8, len
        self.emit_bytes(&[0x49, 0xC7, 0xC0]);
        self.emit_u32(len as u32);
        // call WriteConsoleA
        self.emit_write_console();
    }
}
```

### 3. Stack Dinámico

```rust
fn emit_function_v2(&mut self, func: &Function) {
    // Calcular espacio necesario
    let stack_size = self.calculate_stack_size(func);
    let aligned_size = (stack_size + 15) & !15; // Alinear a 16 bytes
    
    // Prologue dinámico
    self.emit_bytes(&[0x55]);                    // push rbp
    self.emit_bytes(&[0x48, 0x89, 0xE5]);        // mov rbp, rsp
    
    // sub rsp, aligned_size (32-bit immediate)
    self.emit_bytes(&[0x48, 0x81, 0xEC]);
    self.emit_u32(aligned_size as u32);
    
    // ... body ...
    
    // Epilogue
    self.emit_bytes(&[0x48, 0x89, 0xEC]);        // mov rsp, rbp
    self.emit_bytes(&[0x5D]);                    // pop rbp
    self.emit_bytes(&[0xC3]);                    // ret
}
```

### 4. Múltiples Funciones

```rust
struct FunctionTable {
    functions: HashMap<String, u64>,  // nombre -> offset en código
}

fn emit_call_user_function(&mut self, name: &str, args: &[Expr]) {
    // Evaluar argumentos (calling convention)
    for (i, arg) in args.iter().enumerate() {
        self.emit_expression(arg);
        match i {
            0 => self.emit_bytes(&[0x48, 0x89, 0xC1]), // mov rcx, rax
            1 => self.emit_bytes(&[0x48, 0x89, 0xC2]), // mov rdx, rax
            2 => self.emit_bytes(&[0x49, 0x89, 0xC0]), // mov r8, rax
            3 => self.emit_bytes(&[0x49, 0x89, 0xC1]), // mov r9, rax
            _ => self.emit_bytes(&[0x50]),             // push rax
        }
    }
    
    // Call relativo
    let target_offset = self.functions.get(name).unwrap();
    let rel_offset = target_offset - (self.code.len() + 5);
    self.emit_bytes(&[0xE8]);
    self.emit_i32(rel_offset as i32);
}
```

### 5. Generación Multi-Target

```rust
enum Target {
    WindowsPE,
    LinuxELF,
    MachoMachO,
    RawBinary,
}

impl CodeGenerator {
    pub fn generate_for_target(&mut self, program: &Program, target: Target) -> Vec<u8> {
        match target {
            Target::WindowsPE => self.generate_pe(program),
            Target::LinuxELF => self.generate_elf(program),
            Target::MachoMachO => self.generate_macho(program),
            Target::RawBinary => self.generate_raw(program),
        }
    }
}
```

---

## 📊 Syscalls de Referencia

### Windows NT Syscalls (x64)

| Syscall | Number | Description |
|---------|--------|-------------|
| NtWriteFile | 0x08 | Write to file/console |
| NtReadFile | 0x06 | Read from file |
| NtAllocateVirtualMemory | 0x18 | Allocate memory |
| NtFreeVirtualMemory | 0x1E | Free memory |
| NtTerminateProcess | 0x2C | Exit process |

### Linux Syscalls (x64)

| Syscall | Number | Description |
|---------|--------|-------------|
| sys_write | 1 | Write to fd |
| sys_read | 0 | Read from fd |
| sys_mmap | 9 | Map memory |
| sys_munmap | 11 | Unmap memory |
| sys_exit | 60 | Exit process |

---

## 🔥 Opcodes HEX Directos

### Print String (Windows, sin DLL)

```
; Inline WriteConsole implementation
48 C7 C1 F5 FF FF FF    ; mov rcx, -11 (STD_OUTPUT_HANDLE)
48 B8 xx xx xx xx xx xx xx xx  ; mov rax, GetStdHandle_addr
FF D0                   ; call rax
48 89 C1                ; mov rcx, rax (handle)
48 BA yy yy yy yy yy yy yy yy  ; mov rdx, string_addr
49 C7 C0 zz 00 00 00    ; mov r8, length
4D 31 C9                ; xor r9, r9 (lpNumberOfCharsWritten = NULL)
48 83 EC 28             ; sub rsp, 40
48 B8 ww ww ww ww ww ww ww ww  ; mov rax, WriteConsoleA_addr
FF D0                   ; call rax
48 83 C4 28             ; add rsp, 40
```

### Print String (Linux, syscall directo)

```
; sys_write(1, buf, len)
48 C7 C0 01 00 00 00    ; mov rax, 1 (sys_write)
48 C7 C7 01 00 00 00    ; mov rdi, 1 (stdout)
48 BE xx xx xx xx xx xx xx xx  ; mov rsi, string_addr
48 BA yy 00 00 00 00 00 00 00  ; mov rdx, length
0F 05                   ; syscall
```

### Exit (Linux)

```
; sys_exit(0)
48 C7 C0 3C 00 00 00    ; mov rax, 60 (sys_exit)
48 31 FF                ; xor rdi, rdi (exit code 0)
0F 05                   ; syscall
```

---

## 📁 Estructura de Archivos Propuesta

```
src/rust/backend/
├── codegen.rs           # Codegen actual (mantener)
├── codegen_v2.rs        # 🆕 Codegen sin límites
├── syscalls.rs          # 🆕 Syscalls directos
├── targets/             # 🆕 Generadores por target
│   ├── mod.rs
│   ├── pe.rs            # Windows PE (mejorado)
│   ├── elf.rs           # Linux ELF
│   ├── macho.rs         # macOS Mach-O
│   └── raw.rs           # Binario puro
└── functions.rs         # 🆕 Tabla de funciones
```

---

## 🚀 Fases de Implementación

### Fase 1: Syscalls Directos
- [ ] Implementar syscalls Windows
- [ ] Implementar syscalls Linux
- [ ] Print sin printf
- [ ] Exit sin ExitProcess

### Fase 2: Stack Dinámico
- [ ] Calcular tamaño de stack por función
- [ ] Alineación automática
- [ ] Variables locales ilimitadas

### Fase 3: Múltiples Funciones
- [ ] Tabla de funciones
- [ ] Llamadas entre funciones
- [ ] Recursión

### Fase 4: Multi-Target
- [ ] Mejorar PE generator
- [ ] Implementar ELF generator
- [ ] Binario raw

### Fase 5: Optimizaciones
- [ ] Inline pequeñas funciones
- [ ] Eliminar código muerto
- [ ] Optimizar registros

---

## 📈 Beneficios

| Antes | Después |
|-------|---------|
| Depende de msvcrt.dll | **Standalone** |
| Solo Windows | **Windows + Linux + Raw** |
| Stack 256 bytes | **Stack ilimitado** |
| Solo main() | **Múltiples funciones** |
| ~1.5 KB binario | **< 1 KB posible** |

---

## 🎯 Ejemplo: Hello World Sin Dependencias

### Código ADead-BIB
```python
def main():
    print("Hello, World!")
```

### Binario Generado (Linux, ~200 bytes)
```
7F 45 4C 46 02 01 01 00  ; ELF header
...
48 C7 C0 01 00 00 00     ; mov rax, 1 (sys_write)
48 C7 C7 01 00 00 00     ; mov rdi, 1 (stdout)
48 BE xx xx xx xx xx xx  ; mov rsi, "Hello, World!\n"
48 C7 C2 0E 00 00 00     ; mov rdx, 14
0F 05                    ; syscall
48 C7 C0 3C 00 00 00     ; mov rax, 60 (sys_exit)
48 31 FF                 ; xor rdi, rdi
0F 05                    ; syscall
48 65 6C 6C 6F ...       ; "Hello, World!\n"
```

**Total: ~200 bytes** vs ~1.5 KB actual

---

## 🔗 Integración con Runtime

El runtime universal (`runtime/`) puede usarse opcionalmente:

```
┌─────────────────────────────────────────────────────────────────────┐
│                        ADead-BIB Modes                               │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  Mode 1: PURE (Sin runtime)                                         │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │  .adB → Compiler → Syscalls directos → Binario puro         │    │
│  │  Tamaño: < 1 KB | Dependencias: NINGUNA                     │    │
│  └─────────────────────────────────────────────────────────────┘    │
│                                                                      │
│  Mode 2: RUNTIME (Con runtime)                                      │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │  .adB → Compiler → Runtime calls → Binario + libadead       │    │
│  │  Tamaño: ~10 KB | Features: GPU, Vulkan, AI                 │    │
│  └─────────────────────────────────────────────────────────────┘    │
│                                                                      │
│  Mode 3: HYBRID (Mixto)                                             │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │  .adB → Compiler → Syscalls + Runtime selectivo             │    │
│  │  Tamaño: Variable | Best of both worlds                     │    │
│  └─────────────────────────────────────────────────────────────┘    │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

---

**Creado por:** Eddi Andreé Salazar Matos  
**Email:** eddi.salazar.dev@gmail.com  
**Hecho con ❤️ en Perú** 🇵🇪

**ADead-BIB: HEX + Binario = Sin Límites 🚀**

---

## ✅ Implementación Completada

### Archivos Creados

| Archivo | Descripción | Estado |
|---------|-------------|--------|
| `src/rust/backend/syscalls.rs` | Syscalls directos Windows/Linux | ✅ |
| `src/rust/backend/codegen_v2.rs` | CodeGen mejorado multi-target | ✅ |
| `src/rust/backend/elf.rs` | Generador ELF completo | ✅ |

### Características Implementadas

1. **Syscalls Directos** (`syscalls.rs`)
   - Linux: sys_write, sys_exit, sys_mmap
   - Windows: NT syscalls preparados
   - Opcodes x86-64 helpers

2. **CodeGen v2** (`codegen_v2.rs`)
   - Soporte multi-función
   - Stack dinámico (sin límite de 256 bytes)
   - Multi-target: Windows, Linux, Raw
   - Resolución de llamadas entre funciones

3. **ELF Generator** (`elf.rs`)
   - Binarios Linux x86-64 puros
   - Sin dependencias (no libc)
   - Syscalls directos para I/O

### Próximos Pasos

- [ ] Integrar codegen_v2 en main.rs con flag --target
- [ ] Agregar flag --pure para binarios sin DLLs
- [ ] Implementar más syscalls (file I/O, networking)
- [ ] Optimizaciones de código generado

---

## 🚀 FASE 6: AUTO-DETECCIÓN CPU + GPU (EXPRIMIR AL MÁXIMO)

### 6.1 Auto-Detección de CPU via CPUID

```asm
; CPUID - Detectar características del procesador
mov eax, 0          ; Get vendor string
cpuid
; EBX:EDX:ECX = "GenuineIntel" o "AuthenticAMD"

mov eax, 1          ; Get features
cpuid
; ECX bit 28 = AVX
; ECX bit 19 = SSE4.1
; EDX bit 26 = SSE2

mov eax, 7          ; Extended features
mov ecx, 0
cpuid
; EBX bit 5 = AVX2
; EBX bit 16 = AVX-512F
```

#### Opcodes CPUID en HEX
```
0F A2                 ; cpuid
```

#### Estructura de Detección CPU
```rust
#[repr(C)]
pub struct CPUFeatures {
    pub vendor: [u8; 12],      // "GenuineIntel" o "AuthenticAMD"
    pub has_sse2: bool,
    pub has_sse4_1: bool,
    pub has_avx: bool,
    pub has_avx2: bool,
    pub has_avx512: bool,
    pub has_fma: bool,
    pub cores: u32,
    pub threads: u32,
    pub cache_l1: u32,
    pub cache_l2: u32,
    pub cache_l3: u32,
}

impl CPUFeatures {
    pub fn detect() -> Self {
        // Usar CPUID para detectar todo
        unsafe { Self::detect_cpuid() }
    }
    
    pub fn best_simd_width(&self) -> u32 {
        if self.has_avx512 { 512 }
        else if self.has_avx2 { 256 }
        else if self.has_avx { 256 }
        else if self.has_sse2 { 128 }
        else { 64 }
    }
}
```

### 6.2 Auto-Detección de GPU

#### Vulkan Detection (Sin dependencias)
```rust
pub struct GPUFeatures {
    pub available: bool,
    pub vendor: GPUVendor,
    pub name: String,
    pub vram_mb: u32,
    pub compute_units: u32,
    pub max_workgroup_size: u32,
    pub supports_fp16: bool,
    pub supports_fp64: bool,
    pub supports_int8: bool,
}

pub enum GPUVendor {
    NVIDIA,
    AMD,
    Intel,
    Unknown,
}

impl GPUFeatures {
    pub fn detect_vulkan() -> Option<Self> {
        // Cargar vulkan-1.dll / libvulkan.so dinámicamente
        // vkEnumeratePhysicalDevices
        // vkGetPhysicalDeviceProperties
        // vkGetPhysicalDeviceMemoryProperties
    }
    
    pub fn detect_cuda() -> Option<Self> {
        // Cargar nvcuda.dll dinámicamente
        // cuDeviceGetCount
        // cuDeviceGetName
        // cuDeviceTotalMem
    }
}
```

### 6.3 Sistema de Dispatch Automático

```rust
pub enum ComputeBackend {
    CPU_Scalar,      // Sin SIMD
    CPU_SSE2,        // 128-bit SIMD
    CPU_AVX2,        // 256-bit SIMD
    CPU_AVX512,      // 512-bit SIMD
    GPU_Vulkan,      // Vulkan compute
    GPU_CUDA,        // CUDA (NVIDIA)
}

pub struct AutoDispatcher {
    cpu: CPUFeatures,
    gpu: Option<GPUFeatures>,
    threshold_gpu: usize,  // Tamaño mínimo para usar GPU
}

impl AutoDispatcher {
    pub fn new() -> Self {
        Self {
            cpu: CPUFeatures::detect(),
            gpu: GPUFeatures::detect_vulkan().or_else(|| GPUFeatures::detect_cuda()),
            threshold_gpu: 1024 * 1024,  // 1M elementos
        }
    }
    
    /// Selecciona el mejor backend para una operación
    pub fn select_backend(&self, op: &str, size: usize) -> ComputeBackend {
        // GPU para operaciones grandes
        if size >= self.threshold_gpu && self.gpu.is_some() {
            if let Some(gpu) = &self.gpu {
                match gpu.vendor {
                    GPUVendor::NVIDIA => return ComputeBackend::GPU_CUDA,
                    _ => return ComputeBackend::GPU_Vulkan,
                }
            }
        }
        
        // CPU con mejor SIMD disponible
        if self.cpu.has_avx512 { ComputeBackend::CPU_AVX512 }
        else if self.cpu.has_avx2 { ComputeBackend::CPU_AVX2 }
        else if self.cpu.has_sse2 { ComputeBackend::CPU_SSE2 }
        else { ComputeBackend::CPU_Scalar }
    }
}
```

---

## 🔥 FASE 7: SIMD AUTOMÁTICO (AVX2/AVX-512)

### 7.1 Vectorización Automática de Loops

```rust
// Antes (escalar)
for i in 0..n {
    c[i] = a[i] + b[i];
}

// Después (AVX2 - 8 floats a la vez)
// vmovups ymm0, [rsi]      ; Load 8 floats from a
// vmovups ymm1, [rdx]      ; Load 8 floats from b
// vaddps ymm2, ymm0, ymm1  ; Add 8 floats
// vmovups [rdi], ymm2      ; Store 8 floats to c
```

### 7.2 Opcodes SIMD en HEX

```
; AVX2 (256-bit)
C5 FC 10 06           ; vmovups ymm0, [rsi]
C5 FC 10 0A           ; vmovups ymm1, [rdx]
C5 FC 58 D0           ; vaddps ymm2, ymm0, ymm1
C5 FC 11 17           ; vmovups [rdi], ymm2

; AVX-512 (512-bit)
62 F1 7C 48 10 06     ; vmovups zmm0, [rsi]
62 F1 7C 48 58 C1     ; vaddps zmm0, zmm0, zmm1

; FMA (Fused Multiply-Add)
C4 E2 7D B8 C1        ; vfmadd231ps ymm0, ymm0, ymm1  ; a = a*b + c
```

### 7.3 Generador SIMD Automático

```rust
pub struct SIMDCodeGen {
    width: u32,  // 128, 256, o 512
}

impl SIMDCodeGen {
    pub fn emit_vector_add_f32(&mut self, code: &mut Vec<u8>, dst: u8, src1: u8, src2: u8) {
        match self.width {
            512 => {
                // AVX-512: vaddps zmm, zmm, zmm
                code.extend_from_slice(&[0x62, 0xF1, 0x7C, 0x48, 0x58]);
                code.push(0xC0 | (dst << 3) | src2);
            }
            256 => {
                // AVX2: vaddps ymm, ymm, ymm
                code.extend_from_slice(&[0xC5, 0xFC, 0x58]);
                code.push(0xC0 | (dst << 3) | src2);
            }
            128 => {
                // SSE: addps xmm, xmm
                code.extend_from_slice(&[0x0F, 0x58]);
                code.push(0xC0 | (dst << 3) | src2);
            }
            _ => panic!("Unsupported SIMD width"),
        }
    }
    
    pub fn emit_vector_mul_f32(&mut self, code: &mut Vec<u8>, dst: u8, src1: u8, src2: u8) {
        match self.width {
            512 => code.extend_from_slice(&[0x62, 0xF1, 0x7C, 0x48, 0x59, 0xC0 | (dst << 3) | src2]),
            256 => code.extend_from_slice(&[0xC5, 0xFC, 0x59, 0xC0 | (dst << 3) | src2]),
            128 => code.extend_from_slice(&[0x0F, 0x59, 0xC0 | (dst << 3) | src2]),
            _ => {}
        }
    }
    
    pub fn emit_fma_f32(&mut self, code: &mut Vec<u8>, dst: u8, mul: u8, add: u8) {
        // vfmadd231ps: dst = dst * mul + add
        code.extend_from_slice(&[0xC4, 0xE2, 0x7D, 0xB8]);
        code.push(0xC0 | (dst << 3) | add);
    }
}
```

---

## ⚡ FASE 8: MATMUL OPTIMIZADO (CPU + GPU)

### 8.1 MatMul CPU con SIMD + Cache Blocking

```rust
pub fn matmul_avx2(a: &[f32], b: &[f32], c: &mut [f32], m: usize, n: usize, k: usize) {
    const BLOCK: usize = 64;  // Cache-friendly block size
    
    for i0 in (0..m).step_by(BLOCK) {
        for j0 in (0..n).step_by(BLOCK) {
            for k0 in (0..k).step_by(BLOCK) {
                // Micro-kernel con AVX2
                for i in i0..min(i0 + BLOCK, m) {
                    for j in (j0..min(j0 + BLOCK, n)).step_by(8) {
                        // Cargar 8 elementos de C
                        let mut acc = _mm256_loadu_ps(&c[i * n + j]);
                        
                        for kk in k0..min(k0 + BLOCK, k) {
                            let a_val = _mm256_broadcast_ss(&a[i * k + kk]);
                            let b_vec = _mm256_loadu_ps(&b[kk * n + j]);
                            acc = _mm256_fmadd_ps(a_val, b_vec, acc);
                        }
                        
                        _mm256_storeu_ps(&mut c[i * n + j], acc);
                    }
                }
            }
        }
    }
}
```

### 8.2 MatMul GPU con Vulkan Compute

```glsl
// matmul_tiled.comp - Optimizado con shared memory
#version 450
#define TILE_SIZE 16

layout(local_size_x = TILE_SIZE, local_size_y = TILE_SIZE) in;

layout(binding = 0) readonly buffer A { float a[]; };
layout(binding = 1) readonly buffer B { float b[]; };
layout(binding = 2) writeonly buffer C { float c[]; };

layout(push_constant) uniform Dims { uint M, N, K; };

shared float tileA[TILE_SIZE][TILE_SIZE];
shared float tileB[TILE_SIZE][TILE_SIZE];

void main() {
    uint row = gl_GlobalInvocationID.y;
    uint col = gl_GlobalInvocationID.x;
    uint localRow = gl_LocalInvocationID.y;
    uint localCol = gl_LocalInvocationID.x;
    
    float sum = 0.0;
    
    for (uint t = 0; t < (K + TILE_SIZE - 1) / TILE_SIZE; t++) {
        // Cargar tiles a shared memory
        uint aIdx = row * K + t * TILE_SIZE + localCol;
        uint bIdx = (t * TILE_SIZE + localRow) * N + col;
        
        tileA[localRow][localCol] = (aIdx < M * K) ? a[aIdx] : 0.0;
        tileB[localRow][localCol] = (bIdx < K * N) ? b[bIdx] : 0.0;
        
        barrier();
        
        // Compute
        for (uint k = 0; k < TILE_SIZE; k++) {
            sum += tileA[localRow][k] * tileB[k][localCol];
        }
        
        barrier();
    }
    
    if (row < M && col < N) {
        c[row * N + col] = sum;
    }
}
```

---

## 🎯 FASE 9: RUNTIME HÍBRIDO CPU+GPU

### 9.1 Tensor con Storage Unificado

```rust
pub struct Tensor {
    shape: Vec<usize>,
    data_cpu: Option<Vec<f32>>,
    data_gpu: Option<VulkanBuffer>,
    location: TensorLocation,
    dirty_cpu: bool,
    dirty_gpu: bool,
}

pub enum TensorLocation {
    CPU,
    GPU,
    Both,
}

impl Tensor {
    /// Asegura que los datos estén en CPU
    pub fn to_cpu(&mut self) {
        if self.dirty_gpu && self.data_gpu.is_some() {
            // Copiar GPU -> CPU
            self.sync_from_gpu();
        }
        self.location = TensorLocation::CPU;
    }
    
    /// Asegura que los datos estén en GPU
    pub fn to_gpu(&mut self) {
        if self.dirty_cpu && self.data_cpu.is_some() {
            // Copiar CPU -> GPU
            self.sync_to_gpu();
        }
        self.location = TensorLocation::GPU;
    }
    
    /// Ejecuta operación en el mejor dispositivo
    pub fn matmul(&self, other: &Tensor, dispatcher: &AutoDispatcher) -> Tensor {
        let size = self.shape[0] * self.shape[1] * other.shape[1];
        
        match dispatcher.select_backend("matmul", size) {
            ComputeBackend::GPU_Vulkan | ComputeBackend::GPU_CUDA => {
                self.matmul_gpu(other)
            }
            ComputeBackend::CPU_AVX512 => self.matmul_avx512(other),
            ComputeBackend::CPU_AVX2 => self.matmul_avx2(other),
            _ => self.matmul_scalar(other),
        }
    }
}
```

### 9.2 Pipeline de Ejecución Automático

```rust
pub struct ExecutionPipeline {
    dispatcher: AutoDispatcher,
    gpu_queue: Vec<GPUOperation>,
    cpu_queue: Vec<CPUOperation>,
}

impl ExecutionPipeline {
    /// Ejecuta un grafo de operaciones optimizando CPU/GPU
    pub fn execute(&mut self, graph: &ComputeGraph) {
        // Fase 1: Analizar grafo y decidir ubicación de cada tensor
        let placement = self.analyze_placement(graph);
        
        // Fase 2: Agrupar operaciones por dispositivo
        let (gpu_ops, cpu_ops) = self.partition_operations(graph, &placement);
        
        // Fase 3: Ejecutar en paralelo CPU y GPU
        std::thread::scope(|s| {
            // GPU thread
            s.spawn(|| {
                for op in &gpu_ops {
                    self.execute_gpu(op);
                }
            });
            
            // CPU thread (con SIMD)
            s.spawn(|| {
                for op in &cpu_ops {
                    self.execute_cpu_simd(op);
                }
            });
        });
        
        // Fase 4: Sincronizar resultados
        self.synchronize();
    }
}
```

---

## 📊 FASE 10: BENCHMARKS Y PROFILING INTEGRADO

### 10.1 Profiler de Bajo Nivel

```rust
pub struct Profiler {
    events: Vec<ProfileEvent>,
    rdtsc_start: u64,
}

pub struct ProfileEvent {
    name: String,
    start_cycles: u64,
    end_cycles: u64,
    backend: ComputeBackend,
    bytes_processed: usize,
}

impl Profiler {
    /// Lee el contador de ciclos del CPU (RDTSC)
    #[inline]
    pub fn rdtsc() -> u64 {
        unsafe {
            let lo: u32;
            let hi: u32;
            std::arch::asm!("rdtsc", out("eax") lo, out("edx") hi);
            ((hi as u64) << 32) | (lo as u64)
        }
    }
    
    pub fn report(&self) {
        println!("╔════════════════════════════════════════════════════════════╗");
        println!("║                    PROFILER REPORT                          ║");
        println!("╠════════════════════════════════════════════════════════════╣");
        
        for event in &self.events {
            let cycles = event.end_cycles - event.start_cycles;
            let gflops = if event.bytes_processed > 0 {
                (event.bytes_processed as f64 * 2.0) / (cycles as f64 / 3.5e9) / 1e9
            } else { 0.0 };
            
            println!("║ {:20} | {:10} cycles | {:6.2} GFLOPS | {:?}",
                     event.name, cycles, gflops, event.backend);
        }
        
        println!("╚════════════════════════════════════════════════════════════╝");
    }
}
```

### 10.2 Auto-Tuning

```rust
pub struct AutoTuner {
    cache: HashMap<(String, usize), ComputeBackend>,
}

impl AutoTuner {
    /// Encuentra el mejor backend para una operación específica
    pub fn tune(&mut self, op: &str, size: usize, dispatcher: &AutoDispatcher) -> ComputeBackend {
        let key = (op.to_string(), size);
        
        if let Some(&backend) = self.cache.get(&key) {
            return backend;
        }
        
        // Benchmark cada backend disponible
        let backends = [
            ComputeBackend::CPU_Scalar,
            ComputeBackend::CPU_AVX2,
            ComputeBackend::CPU_AVX512,
            ComputeBackend::GPU_Vulkan,
        ];
        
        let mut best_backend = ComputeBackend::CPU_Scalar;
        let mut best_time = u64::MAX;
        
        for backend in backends {
            if self.is_available(backend, dispatcher) {
                let time = self.benchmark(op, size, backend);
                if time < best_time {
                    best_time = time;
                    best_backend = backend;
                }
            }
        }
        
        self.cache.insert(key, best_backend);
        best_backend
    }
}
```

---

## 🎮 FASE 11: API UNIFICADA PARA EXPRIMIR TODO

### 11.1 API Simple pero Potente

```rust
// Uso simple - el runtime decide todo automáticamente
let rt = ADeadRuntime::new();  // Auto-detecta CPU y GPU

let a = rt.tensor([1024, 1024]).randn();
let b = rt.tensor([1024, 1024]).randn();

// MatMul - automáticamente usa GPU si es grande, CPU+AVX si es pequeño
let c = a.matmul(&b);

// Forzar dispositivo específico
let c_gpu = a.to_gpu().matmul(&b.to_gpu());
let c_cpu = a.to_cpu().matmul_avx2(&b.to_cpu());

// Pipeline de operaciones (fusionadas automáticamente)
let result = rt.pipeline()
    .matmul(&a, &b)
    .relu()
    .softmax()
    .execute();

// Profiling
rt.enable_profiling();
let result = a.matmul(&b);
rt.print_profile();
```

### 11.2 Configuración Avanzada

```rust
let rt = ADeadRuntime::builder()
    .prefer_gpu(true)
    .gpu_threshold(1024 * 1024)  // Usar GPU para >1M elementos
    .simd_width(256)             // Forzar AVX2
    .num_threads(6)              // Threads para CPU
    .enable_profiling(true)
    .build();
```

---

## 📈 MÉTRICAS OBJETIVO

| Operación | Tamaño | CPU Scalar | CPU AVX2 | CPU AVX-512 | GPU |
|-----------|--------|------------|----------|-------------|-----|
| MatMul | 1024x1024 | 2000ms | 250ms | 125ms | **15ms** |
| Add | 10M | 50ms | 12ms | 6ms | **2ms** |
| Softmax | 1M | 30ms | 8ms | 4ms | **1ms** |
| Attention | 512x512 | 500ms | 60ms | 30ms | **5ms** |

**Objetivo: 100x speedup vs scalar con auto-dispatch**

---

## 🔧 ARCHIVOS A CREAR

```
src/rust/
├── runtime/
│   ├── mod.rs
│   ├── cpu_detect.rs      # CPUID y features
│   ├── gpu_detect.rs      # Vulkan/CUDA detection
│   ├── dispatcher.rs      # Auto-dispatch CPU/GPU
│   ├── simd/
│   │   ├── mod.rs
│   │   ├── sse2.rs
│   │   ├── avx2.rs
│   │   └── avx512.rs
│   ├── tensor.rs          # Tensor unificado
│   └── profiler.rs        # Profiling integrado
```

---

**ADead-BIB: EXPRIMIR CPU + GPU AL MÁXIMO** 🚀🔥🇵🇪
