# 🗺️ Rutas para Generar Binarios Puros - ADead-BIB

## 🎯 Objetivo: Binarios Ejecutables Puros (NO ASM)

Este documento describe **TODOS los enfoques posibles** para generar binarios ejecutables puros escribiendo opcodes directamente, sin pasar por assembly.

---

## 📊 Mapa de Rutas

```
┌─────────────────────────────────────────────────────────────┐
│                    CÓDIGO FUENTE                            │
└───────────────────────┬─────────────────────────────────────┘
                        │
        ┌───────────────┼───────────────┐
        │               │               │
        ▼               ▼               ▼
    [PARSER]      [PARSER]        [PARSER]
    (Rust/nom)    (C++)          (Manual)
        │               │               │
        └───────┬───────┴───────┬───────┘
                │               │
                ▼               ▼
            [AST/IR]      [AST/IR]
                │               │
        ┌───────┼───────┬───────┼───────┐
        │       │       │       │       │
        ▼       ▼       ▼       ▼       ▼
    [RUTA 1] [RUTA 2] [RUTA 3] [RUTA 4] [RUTA 5]
    Bytecode  Directo  Híbrido  JIT     Packer
```

---

## 🛤️ RUTA 1: Bytecode Intermedio → Opcodes → Binario

### Descripción
Generar un bytecode intermedio primero, luego convertir a opcodes de CPU.

### Flujo
```
Código → AST → Bytecode (stack-based) → Opcodes x86-64 → PE/ELF
```

### Ventajas
- ✅ Separación clara de concerns
- ✅ Fácil de optimizar el bytecode
- ✅ Puedes cambiar arquitectura fácilmente
- ✅ Más fácil de debuggear

### Desventajas
- ❌ Paso extra (bytecode intermedio)
- ❌ Más complejidad

### Implementación

**Paso 1: Bytecode Stack-Based**
```rust
// Rust: Generar bytecode
enum Bytecode {
    Push(i64),
    Add,
    Sub,
    Mul,
    Div,
    Store(String),
    Load(String),
    Call(String),
    Ret,
}
```

**Paso 2: Bytecode → Opcodes**
```cpp
// C++: Convertir bytecode a opcodes x86-64
void emit_bytecode_to_opcodes(const Bytecode& bc, OpcodeEmitter& emitter) {
    switch(bc) {
        case Push(val):
            emitter.emit_mov_rax_imm64(val);
            emitter.emit_push_rax();
            break;
        case Add:
            emitter.emit_pop_rbx();
            emitter.emit_pop_rax();
            emitter.emit_add_rax_rbx();
            emitter.emit_push_rax();
            break;
        // ...
    }
}
```

**Paso 3: Opcodes → Binario**
```rust
// Rust: Escribir opcodes en PE/ELF
fn write_binary(opcodes: &[u8]) -> Result<()> {
    let mut pe = PEBuilder::new();
    pe.add_section(".text", opcodes, SectionFlags::EXECUTABLE);
    pe.write("output.exe")?;
    Ok(())
}
```

### Casos de Uso
- Compiladores tradicionales
- Lenguajes de alto nivel
- Cuando necesitas portabilidad

---

## 🛤️ RUTA 2: Directo AST → Opcodes → Binario

### Descripción
Convertir AST directamente a opcodes, sin bytecode intermedio.

### Flujo
```
Código → AST → Opcodes x86-64 → PE/ELF
```

### Ventajas
- ✅ Menos pasos, más directo
- ✅ Más eficiente
- ✅ Menos overhead
- ✅ Binarios más pequeños

### Desventajas
- ❌ Más complejo (necesitas conocer opcodes bien)
- ❌ Menos flexible para cambiar arquitectura

### Implementación

**Paso 1: AST → Opcodes Directo**
```cpp
// C++: Emitir opcodes directamente desde AST
void emit_ast_to_opcodes(AST* node, OpcodeEmitter& emitter) {
    if (node->type == NUMBER) {
        // mov rax, value
        emitter.emit_mov_rax_imm64(node->value);
    }
    else if (node->type == ADD) {
        // Emitir left
        emit_ast_to_opcodes(node->left, emitter);
        emitter.emit_push_rax();  // Guardar resultado
        
        // Emitir right
        emit_ast_to_opcodes(node->right, emitter);
        emitter.emit_pop_rbx();   // Cargar left
        
        // add rax, rbx
        emitter.emit_add_rax_rbx();
    }
    else if (node->type == CALL) {
        // Preparar argumentos según calling convention
        // x86-64: rcx, rdx, r8, r9, stack
        emit_call_prepare_args(node->args, emitter);
        
        // call function
        emitter.emit_call_rip_relative(function_offset);
    }
}
```

**Paso 2: Opcodes → Binario**
```rust
// Rust: Mismo que Ruta 1
```

### Casos de Uso
- Compiladores de bajo nivel
- Cuando performance es crítica
- Lenguajes tipo C

---

## 🛤️ RUTA 3: Híbrido - IR Optimizado → Opcodes

### Descripción
Usar una IR (Intermediate Representation) optimizada, luego convertir a opcodes.

### Flujo
```
Código → AST → IR → Optimizaciones → Opcodes x86-64 → PE/ELF
```

### Ventajas
- ✅ Mejor para optimizaciones
- ✅ IR puede ser arquitectura-agnóstica
- ✅ Balance entre flexibilidad y performance

### Desventajas
- ❌ Más complejo
- ❌ Más pasos

### Implementación

**Paso 1: AST → IR**
```rust
// Rust: Convertir AST a IR
enum IR {
    Load(Register, Memory),
    Store(Memory, Register),
    Add(Register, Register, Register),
    Imm(Register, i64),
    Call(String, Vec<Register>),
}
```

**Paso 2: Optimizar IR**
```rust
// Rust: Optimizaciones
fn optimize_ir(ir: &mut Vec<IR>) {
    constant_folding(ir);
    dead_code_elimination(ir);
    register_allocation(ir);
}
```

**Paso 3: IR → Opcodes**
```cpp
// C++: Convertir IR optimizado a opcodes
void emit_ir_to_opcodes(const IR& ir, OpcodeEmitter& emitter) {
    match ir {
        Load(reg, mem) => {
            emitter.emit_mov_reg_mem(reg, mem);
        },
        Add(dst, src1, src2) => {
            emitter.emit_mov_reg_reg(dst, src1);
            emitter.emit_add_reg_reg(dst, src2);
        },
        // ...
    }
}
```

### Casos de Uso
- Compiladores optimizadores
- LLVM-like approach
- Cuando necesitas muchas optimizaciones

---

## 🛤️ RUTA 4: JIT - Generación Dinámica de Opcodes

### Descripción
Generar opcodes en tiempo de ejecución, escribir en memoria ejecutable.

### Flujo
```
Código → AST → Opcodes (en memoria) → Ejecutar directamente
```

### Ventajas
- ✅ Máxima flexibilidad
- ✅ Optimizaciones dinámicas
- ✅ Puedes recompilar en runtime

### Desventajas
- ❌ Más complejo (gestión de memoria ejecutable)
- ❌ Requiere permisos especiales
- ❌ Menos portable

### Implementación

**Paso 1: Allocar Memoria Ejecutable**
```cpp
// C++: Allocar memoria ejecutable
void* allocate_executable_memory(size_t size) {
    #ifdef _WIN32
        return VirtualAlloc(NULL, size, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE);
    #else
        void* mem = mmap(NULL, size, PROT_READ | PROT_WRITE | PROT_EXEC, 
                        MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
        return mem;
    #endif
}
```

**Paso 2: Emitir Opcodes en Memoria**
```cpp
// C++: Escribir opcodes directamente en memoria
void* jit_compile(AST* ast) {
    OpcodeEmitter emitter;
    emit_ast_to_opcodes(ast, emitter);
    
    void* exec_mem = allocate_executable_memory(emitter.size());
    memcpy(exec_mem, emitter.data(), emitter.size());
    
    // Hacer memoria ejecutable
    #ifdef _WIN32
        DWORD old_protect;
        VirtualProtect(exec_mem, emitter.size(), PAGE_EXECUTE_READ, &old_protect);
    #else
        mprotect(exec_mem, emitter.size(), PROT_READ | PROT_EXEC);
    #endif
    
    return exec_mem;
}
```

**Paso 3: Ejecutar**
```cpp
// C++: Ejecutar código generado
typedef int (*GeneratedFunc)();
GeneratedFunc func = (GeneratedFunc)jit_compile(ast);
int result = func();  // Ejecutar directamente
```

### Casos de Uso
- JIT compilers (V8, SpiderMonkey)
- Interpreters con JIT
- Scripting engines

---

## 🛤️ RUTA 5: Packer/Loader - Opcodes Empaquetados

### Descripción
Generar opcodes, comprimirlos/empaquetarlos, y descomprimirlos en runtime.

### Flujo
```
Código → AST → Opcodes → Comprimir → PE/ELF (con loader) → Descomprimir → Ejecutar
```

### Ventajas
- ✅ Binarios más pequeños
- ✅ Ofuscación
- ✅ Puedes agregar protección

### Desventajas
- ❌ Más complejo
- ❌ Overhead en runtime
- ❌ Puede ser detectado por antivirus

### Implementación

**Paso 1: Generar Opcodes**
```cpp
// C++: Generar opcodes normalmente
OpcodeEmitter emitter;
emit_ast_to_opcodes(ast, emitter);
```

**Paso 2: Comprimir**
```rust
// Rust: Comprimir opcodes
use flate2::Compression;
use flate2::write::DeflateEncoder;

fn compress_opcodes(opcodes: &[u8]) -> Vec<u8> {
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(opcodes).unwrap();
    encoder.finish().unwrap()
}
```

**Paso 3: Crear PE con Loader**
```rust
// Rust: Crear PE con loader embebido
fn create_packed_binary(compressed_opcodes: &[u8]) -> Result<()> {
    // 1. Cargar loader (código que descomprime y ejecuta)
    let loader_code = load_loader_opcodes();
    
    // 2. Crear PE con:
    //    - .text: loader code
    //    - .data: compressed_opcodes
    let mut pe = PEBuilder::new();
    pe.add_section(".text", loader_code, EXECUTABLE);
    pe.add_section(".data", compressed_opcodes, READABLE);
    pe.write("packed.exe")?;
    Ok(())
}
```

**Paso 4: Loader en Runtime**
```cpp
// C++: Loader que se ejecuta primero
void loader_entry_point() {
    // 1. Leer compressed_opcodes de .data
    void* compressed = get_data_section();
    size_t compressed_size = get_data_section_size();
    
    // 2. Descomprimir
    void* decompressed = decompress(compressed, compressed_size);
    
    // 3. Allocar memoria ejecutable
    void* exec_mem = allocate_executable_memory(decompressed_size);
    memcpy(exec_mem, decompressed, decompressed_size);
    
    // 4. Ejecutar
    ((void(*)())exec_mem)();
}
```

### Casos de Uso
- Packers (UPX, VMProtect)
- Software protection
- Reducir tamaño de binarios

---

## 🛤️ RUTA 6: VM Engine - Bytecode Personalizado

### Descripción
Crear una VM que ejecuta bytecode personalizado, el bytecode se genera a opcodes.

### Flujo
```
Código → AST → Bytecode Personalizado → Opcodes (VM) → PE/ELF (con VM) → Ejecutar
```

### Ventajas
- ✅ Portabilidad (mismo bytecode en todas las plataformas)
- ✅ Seguridad (puedes validar bytecode)
- ✅ Flexibilidad (puedes cambiar VM)

### Desventajas
- ❌ Overhead de VM
- ❌ Más complejo
- ❌ Binarios más grandes

### Implementación

**Paso 1: Diseñar Bytecode**
```rust
// Rust: Bytecode personalizado
enum VMBytecode {
    PushI64(i64),
    PushF64(f64),
    Add,
    Sub,
    Call(u32),  // índice de función
    Ret,
    LoadLocal(u32),
    StoreLocal(u32),
}
```

**Paso 2: Generar Bytecode**
```rust
// Rust: AST → Bytecode
fn ast_to_bytecode(ast: &AST) -> Vec<VMBytecode> {
    // ...
}
```

**Paso 3: Compilar VM a Opcodes**
```cpp
// C++: VM interpreter compilado a opcodes
void emit_vm_interpreter(OpcodeEmitter& emitter) {
    // Emitir código que interpreta bytecode
    // switch(bytecode) {
    //   case PushI64: ...
    //   case Add: ...
    // }
}
```

**Paso 4: Crear Binario con VM + Bytecode**
```rust
// Rust: PE con VM + bytecode
fn create_vm_binary(bytecode: &[u8]) -> Result<()> {
    let vm_opcodes = compile_vm_interpreter();
    
    let mut pe = PEBuilder::new();
    pe.add_section(".text", vm_opcodes, EXECUTABLE);
    pe.add_section(".data", bytecode, READABLE);
    pe.set_entry_point(vm_entry_point);
    pe.write("vm_program.exe")?;
    Ok(())
}
```

### Casos de Uso
- Lenguajes interpretados
- Sandboxing
- Cross-platform languages

---

## 🛤️ RUTA 7: Extremo - Sin libc, Solo Syscalls

### Descripción
Generar binarios que hacen syscalls directamente, sin ninguna librería.

### Flujo
```
Código → AST → Opcodes (syscalls directos) → PE/ELF mínimo
```

### Ventajas
- ✅ Binarios ultra-pequeños (<1KB posible)
- ✅ Sin dependencias
- ✅ Control total
- ✅ Aprendizaje máximo

### Desventajas
- ❌ Muy complejo
- ❌ Específico por OS
- ❌ Difícil de mantener

### Implementación

**Paso 1: Emitir Syscalls Directos**
```cpp
// C++: Emitir syscalls directamente
void emit_syscall_write(OpcodeEmitter& emitter, int fd, const char* str, size_t len) {
    // Linux x86-64 syscall convention:
    // rax = syscall number (1 = write)
    // rdi = fd (1 = stdout)
    // rsi = buffer
    // rdx = length
    
    // mov rax, 1  (sys_write)
    emitter.emit_mov_rax_imm64(1);
    
    // mov rdi, 1  (stdout)
    emitter.emit_mov_rdi_imm64(1);
    
    // mov rsi, address_of_string
    emitter.emit_mov_rsi_imm64(string_address);
    
    // mov rdx, len
    emitter.emit_mov_rdx_imm64(len);
    
    // syscall
    emitter.emit_syscall();
}

void emit_syscall_exit(OpcodeEmitter& emitter, int code) {
    // mov rax, 60  (sys_exit)
    emitter.emit_mov_rax_imm64(60);
    
    // mov rdi, code
    emitter.emit_mov_rdi_imm64(code);
    
    // syscall
    emitter.emit_syscall();
}
```

**Paso 2: Crear PE/ELF Mínimo**
```rust
// Rust: PE mínimo sin dependencias
fn create_minimal_elf(opcodes: &[u8]) -> Result<()> {
    // ELF mínimo:
    // - Header ELF
    // - Program header (LOAD)
    // - .text section con opcodes
    // - Entry point apunta a opcodes
    
    let mut elf = ELFBuilder::minimal();
    elf.add_load_segment(opcodes, 0x400000);  // Base address
    elf.set_entry_point(0x400000);
    elf.write("minimal")?;
    Ok(())
}
```

**Resultado:**
- Binario de ~200-500 bytes
- Ejecuta directamente
- Sin dependencias
- Solo syscalls

### Casos de Uso
- Demos de tamaño mínimo
- Aprendizaje profundo
- Embedded systems
- Bootloaders

---

## 📊 Comparación de Rutas

| Ruta | Complejidad | Tamaño Binario | Performance | Portabilidad | Caso de Uso |
|------|-------------|----------------|--------------|--------------|-------------|
| 1. Bytecode | Media | Medio | Media | Alta | Compiladores tradicionales |
| 2. Directo | Alta | Pequeño | Alta | Baja | Compiladores de bajo nivel |
| 3. IR Optimizado | Muy Alta | Medio | Muy Alta | Media | Compiladores optimizadores |
| 4. JIT | Muy Alta | N/A (runtime) | Muy Alta | Media | JIT compilers |
| 5. Packer | Alta | Muy Pequeño | Media | Media | Packers, protección |
| 6. VM | Alta | Grande | Baja | Muy Alta | Lenguajes interpretados |
| 7. Extremo | Muy Alta | Ultra Pequeño | Alta | Muy Baja | Demos, aprendizaje |

---

## 🎯 Recomendación por Nivel

### Nivel 1: Principiante
**Ruta 1: Bytecode Intermedio**
- Más fácil de entender
- Separación clara
- Fácil de debuggear

### Nivel 2: Intermedio
**Ruta 2: Directo AST → Opcodes**
- Más eficiente
- Aprendes opcodes directamente
- Binarios más pequeños

### Nivel 3: Avanzado
**Ruta 3: IR Optimizado** o **Ruta 4: JIT**
- Optimizaciones avanzadas
- Performance máxima
- Flexibilidad

### Nivel 4: Extremo
**Ruta 7: Sin libc, Solo Syscalls**
- Control absoluto
- Binarios mínimos
- Aprendizaje máximo

---

## ⭐ RECOMENDACIÓN PRINCIPAL: Casos Generales y Trabajos Pesados

### 🏆 Ruta 2: Directo AST → Opcodes (LA MEJOR OPCIÓN)

**¿Por qué Ruta 2 para casos generales?**

✅ **Eficiencia Máxima**
- Sin overhead de bytecode intermedio
- Directo a opcodes que la CPU ejecuta
- Binarios más pequeños y rápidos

✅ **Performance Excelente para Trabajos Pesados**
- Sin capas adicionales que ralenticen
- Opcodes optimizados directamente
- La CPU ejecuta exactamente lo que necesitas

✅ **Perfecto como "Enzima de ASM"**
- Reemplaza ASM completamente
- Escribes opcodes directamente (más control que ASM)
- Sin dependencias de assembler
- Control total sobre cada byte

✅ **Casos de Uso Generales**
- Aplicaciones de sistema
- Compiladores
- Herramientas de bajo nivel
- Cualquier trabajo donde performance importa

✅ **Sin Conflictos en CPU**
- Opcodes válidos y optimizados
- Respetas calling conventions
- Alineación correcta de datos
- Instrucciones eficientes

**Flujo:**
```
Código (.adB) → AST → Opcodes x86-64 → PE/ELF → CPU ejecuta directamente
```

**Ventajas específicas:**
- ✅ Menos pasos = menos errores
- ✅ Binarios más pequeños (menos overhead)
- ✅ Más rápido (directo a CPU)
- ✅ Control total (cada byte es tuyo)
- ✅ Perfecto para producción

**Comparación con ASM:**
- ❌ ASM: Texto → Assembler → Objeto → Linker → Binario
- ✅ ADead-BIB: Código → Opcodes → Binario (directo)
- **Más control, menos pasos, mejor performance**

### 🥈 Alternativa: Ruta 3 (Si Necesitas Optimizaciones Avanzadas)

Si necesitas optimizaciones muy agresivas (dead code elimination, constant folding avanzado, register allocation complejo), entonces **Ruta 3 (IR Optimizado)** es mejor, pero es más compleja.

**Recomendación**: Empieza con **Ruta 2**, luego migra a **Ruta 3** si necesitas optimizaciones avanzadas.

---

## 🚀 Plan de Implementación Sugerido

### ⭐ Fase 1: Ruta 2 (Directo) - RECOMENDADO PARA EMPEZAR
**Objetivo**: Generar binarios puros para casos generales

1. Implementar parser para `.adB` (Rust con `nom`)
2. AST → Opcodes directo (C++)
3. Generar PE/ELF básico (Rust)
4. **Resultado**: `hello_world.adB` → `hello_world.exe` funciona
5. **Extensión**: `.adB` = ADead-BIB

**Ejemplo:**
```adB
// hello_world.adB
fn main() {
    print("Hello, World!");
}
```
→ Compila directamente a opcodes → Binario ejecutable

### Fase 2: Optimizaciones Básicas
1. Optimizar opcodes emitidos
2. Mejorar calling conventions
3. Register allocation básico
4. **Resultado**: Binarios más eficientes

### Fase 3: Características Avanzadas
1. Funciones con parámetros
2. Variables locales
3. Estructuras de control (if, while, for)
4. **Resultado**: Lenguaje completo

### Fase 4 (Opcional): Ruta 3 (IR Optimizado)
Si necesitas optimizaciones muy avanzadas:
1. Agregar capa IR
2. Implementar optimizaciones avanzadas
3. IR → Opcodes
4. **Resultado**: Compilador optimizador completo

### Fase 5 (Opcional): Ruta 7 (Extremo)
Para binarios ultra-mínimos:
1. Eliminar dependencias de libc
2. Syscalls directos
3. PE/ELF mínimo
4. **Resultado**: Binario <1KB

---

## 📚 Recursos por Ruta

### Ruta 1-3: Opcodes x86-64
- [Intel Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html)
- [x86-64 Instruction Encoding](https://wiki.osdev.org/X86-64_Instruction_Encoding)

### Ruta 4: JIT
- [JIT Compilation Techniques](https://eli.thegreenplace.net/2013/11/05/how-to-jit-an-introduction)
- [Memory Protection](https://en.wikipedia.org/wiki/Memory_protection)

### Ruta 5: Packers
- [PE Format](https://docs.microsoft.com/en-us/windows/win32/debug/pe-format)
- [Compression Algorithms](https://en.wikipedia.org/wiki/Lossless_compression)

### Ruta 7: Syscalls
- [Linux Syscalls](https://filippo.io/linux-syscall-table/)
- [Windows API](https://docs.microsoft.com/en-us/windows/win32/api/)

---

## ✅ Checklist por Ruta

### Ruta 1: Bytecode
- [ ] Diseñar formato de bytecode
- [ ] Implementar emisor de bytecode
- [ ] Implementar bytecode → opcodes
- [ ] Generar PE/ELF con opcodes

### Ruta 2: Directo ⭐ (RECOMENDADO)
- [ ] Implementar parser para `.adB`
- [ ] Implementar AST → opcodes directo
- [ ] Manejar calling conventions (x86-64: rcx, rdx, r8, r9, stack)
- [ ] Generar PE/ELF
- [ ] Tests con `hello_world.adB`

### Ruta 3: IR Optimizado
- [ ] Diseñar IR
- [ ] Implementar optimizaciones
- [ ] IR → opcodes
- [ ] Generar binario

### Ruta 4: JIT
- [ ] Allocar memoria ejecutable
- [ ] Emitir opcodes en memoria
- [ ] Proteger memoria
- [ ] Ejecutar código

### Ruta 5: Packer
- [ ] Comprimir opcodes
- [ ] Implementar loader
- [ ] Crear PE con loader
- [ ] Descomprimir en runtime

### Ruta 6: VM
- [ ] Diseñar bytecode VM
- [ ] Implementar VM interpreter
- [ ] Compilar VM a opcodes
- [ ] Crear binario con VM

### Ruta 7: Extremo
- [ ] Emitir syscalls directos
- [ ] Crear PE/ELF mínimo
- [ ] Sin dependencias
- [ ] Binario <1KB

---

**¡Elige tu ruta y comienza a construir binarios puros! 🔥**

Cada ruta te enseñará algo diferente sobre cómo funcionan los binarios a nivel de bytes.

