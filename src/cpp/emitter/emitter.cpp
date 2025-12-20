#include "emitter.h"
#include <cstring>

OpcodeEmitter::OpcodeEmitter() {
    code.reserve(1024);  // Pre-allocate
}

OpcodeEmitter::~OpcodeEmitter() {
}

const std::vector<uint8_t>& OpcodeEmitter::get_code() const {
    return code;
}

size_t OpcodeEmitter::size() const {
    return code.size();
}

void OpcodeEmitter::emit_ret() {
    code.push_back(0xC3);  // ret
}

void OpcodeEmitter::emit_nop() {
    code.push_back(0x90);  // nop
}

void OpcodeEmitter::emit_mov_rax_imm64(uint64_t value) {
    // mov rax, imm64
    // REX.W prefix (48) + opcode (B8) + 8 bytes value
    code.push_back(0x48);  // REX.W
    code.push_back(0xB8);  // MOV RAX, imm64
    emit_u64(value);
}

void OpcodeEmitter::emit_mov_rdi_imm64(uint64_t value) {
    // mov rdi, imm64
    code.push_back(0x48);  // REX.W
    code.push_back(0xBF);  // MOV RDI, imm64
    emit_u64(value);
}

void OpcodeEmitter::emit_mov_rsi_imm64(uint64_t value) {
    // mov rsi, imm64
    code.push_back(0x48);  // REX.W
    code.push_back(0xBE);  // MOV RSI, imm64
    emit_u64(value);
}

void OpcodeEmitter::emit_mov_rdx_imm64(uint64_t value) {
    // mov rdx, imm64
    code.push_back(0x48);  // REX.W
    code.push_back(0xBA);  // MOV RDX, imm64
    emit_u64(value);
}

void OpcodeEmitter::emit_push_rax() {
    code.push_back(0x50);  // push rax
}

void OpcodeEmitter::emit_pop_rax() {
    code.push_back(0x58);  // pop rax
}

void OpcodeEmitter::emit_push_rbx() {
    code.push_back(0x53);  // push rbx
}

void OpcodeEmitter::emit_pop_rbx() {
    code.push_back(0x5B);  // pop rbx
}

void OpcodeEmitter::emit_add_rax_rbx() {
    // add rax, rbx
    code.push_back(0x48);  // REX.W
    code.push_back(0x01);  // ADD
    code.push_back(0xD8);  // ModR/M: rax + rbx
}

void OpcodeEmitter::emit_sub_rax_rbx() {
    // sub rax, rbx
    code.push_back(0x48);  // REX.W
    code.push_back(0x29);  // SUB
    code.push_back(0xD8);  // ModR/M: rax - rbx
}

void OpcodeEmitter::emit_call_rip_relative(int32_t offset) {
    // call [rip+offset]
    code.push_back(0xFF);  // CALL
    code.push_back(0x15);  // ModR/M: [rip+disp32]
    emit_u32(static_cast<uint32_t>(offset));
}

void OpcodeEmitter::emit_syscall() {
    code.push_back(0x0F);  // SYSCALL prefix
    code.push_back(0x05);  // SYSCALL
}

void OpcodeEmitter::emit_bytes(const uint8_t* bytes, size_t count) {
    code.insert(code.end(), bytes, bytes + count);
}

void OpcodeEmitter::emit_u8(uint8_t value) {
    code.push_back(value);
}

void OpcodeEmitter::emit_u32(uint32_t value) {
    // Little-endian
    code.push_back(value & 0xFF);
    code.push_back((value >> 8) & 0xFF);
    code.push_back((value >> 16) & 0xFF);
    code.push_back((value >> 24) & 0xFF);
}

void OpcodeEmitter::emit_u64(uint64_t value) {
    // Little-endian
    code.push_back(value & 0xFF);
    code.push_back((value >> 8) & 0xFF);
    code.push_back((value >> 16) & 0xFF);
    code.push_back((value >> 24) & 0xFF);
    code.push_back((value >> 32) & 0xFF);
    code.push_back((value >> 40) & 0xFF);
    code.push_back((value >> 48) & 0xFF);
    code.push_back((value >> 56) & 0xFF);
}

void OpcodeEmitter::clear() {
    code.clear();
}

// Helper functions
void emit_print_string(OpcodeEmitter& emitter, const char* str, size_t len) {
    // TODO: Implementar llamada a printf/puts
    // Por ahora placeholder
    emitter.emit_nop();
}

void emit_exit(OpcodeEmitter& emitter, int code) {
    // TODO: Implementar exit syscall
    emitter.emit_nop();
}

