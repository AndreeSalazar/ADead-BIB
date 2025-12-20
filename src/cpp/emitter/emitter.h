#pragma once

#include <vector>
#include <cstdint>
#include <string>

// OpcodeEmitter - Emite opcodes x86-64 directamente
class OpcodeEmitter {
private:
    std::vector<uint8_t> code;
    
public:
    OpcodeEmitter();
    ~OpcodeEmitter();
    
    // Obtener código generado
    const std::vector<uint8_t>& get_code() const;
    size_t size() const;
    
    // Instrucciones básicas
    void emit_ret();                    // ret (C3)
    void emit_nop();                    // nop (90)
    
    // Movimientos
    void emit_mov_rax_imm64(uint64_t value);  // mov rax, imm64
    void emit_mov_rdi_imm64(uint64_t value);  // mov rdi, imm64
    void emit_mov_rsi_imm64(uint64_t value);  // mov rsi, imm64
    void emit_mov_rdx_imm64(uint64_t value);  // mov rdx, imm64
    
    // Stack operations
    void emit_push_rax();               // push rax (50)
    void emit_pop_rax();                // pop rax (58)
    void emit_push_rbx();               // push rbx (53)
    void emit_pop_rbx();                // pop rbx (5B)
    
    // Aritméticas
    void emit_add_rax_rbx();            // add rax, rbx (48 01 D8)
    void emit_sub_rax_rbx();            // sub rax, rbx (48 29 D8)
    
    // Llamadas
    void emit_call_rip_relative(int32_t offset);  // call [rip+offset]
    void emit_syscall();                // syscall (0F 05)
    
    // Utilidades
    void emit_bytes(const uint8_t* bytes, size_t count);
    void emit_u8(uint8_t value);
    void emit_u32(uint32_t value);
    void emit_u64(uint64_t value);
    
    // Limpiar
    void clear();
};

// Funciones helper para generar código común
void emit_print_string(OpcodeEmitter& emitter, const char* str, size_t len);
void emit_exit(OpcodeEmitter& emitter, int code);

