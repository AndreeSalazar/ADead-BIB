// ============================================================
// x86-64 Instruction Decoder — Bytes → ABIB Instructions
// ============================================================
// Minimal linear-sweep decoder for x86-64 machine code.
// Converts raw bytes into ABIB_Instruction with opcode + operands.
//
// Supports the most common instructions:
//   MOV, PUSH, POP, ADD, SUB, XOR, CMP, TEST, LEA,
//   CALL, RET, JMP, Jcc, NOP, INT, SYSCALL, HLT
// ============================================================

use crate::core::ir::*;

/// Decoded result from one instruction
pub struct DecodedInst {
    pub instruction: ABIB_Instruction,
    pub size: usize,
}

/// Decode a single x86-64 instruction at the given offset
pub fn decode_one(code: &[u8], offset: usize, base_addr: u64) -> Option<DecodedInst> {
    if offset >= code.len() { return None; }

    let addr = base_addr + offset as u64;
    let bytes = &code[offset..];
    if bytes.is_empty() { return None; }

    // Track REX prefix
    let mut pos = 0;
    let mut rex: u8 = 0;
    let mut has_rex = false;

    // Check for REX prefix (0x40-0x4F)
    if bytes[pos] >= 0x40 && bytes[pos] <= 0x4F {
        rex = bytes[pos];
        has_rex = true;
        pos += 1;
        if pos >= bytes.len() { return None; }
    }

    let rex_w = has_rex && (rex & 0x08) != 0;
    let rex_r = has_rex && (rex & 0x04) != 0;
    let rex_b = has_rex && (rex & 0x01) != 0;

    let opbyte = bytes[pos];
    pos += 1;

    match opbyte {
        // NOP
        0x90 => Some(make_simple(Opcode::Nop, addr, pos, &bytes[..pos])),

        // RET
        0xC3 => Some(make_simple(Opcode::Ret, addr, pos, &bytes[..pos])),

        // INT3
        0xCC => Some(make_inst(Opcode::Int, vec![Operand::Imm32(3)], addr, pos, &bytes[..pos])),

        // INT imm8
        0xCD => {
            if pos >= bytes.len() { return None; }
            let imm = bytes[pos] as i32;
            pos += 1;
            Some(make_inst(Opcode::Int, vec![Operand::Imm32(imm)], addr, pos, &bytes[..pos]))
        }

        // HLT
        0xF4 => Some(make_simple(Opcode::Hlt, addr, pos, &bytes[..pos])),

        // PUSH r64 (50+rd)
        0x50..=0x57 => {
            let reg_id = (opbyte - 0x50) + if rex_b { 8 } else { 0 };
            let reg = Register::from_x86_reg(reg_id);
            Some(make_inst(Opcode::Push, vec![Operand::Reg(reg)], addr, pos, &bytes[..pos]))
        }

        // POP r64 (58+rd)
        0x58..=0x5F => {
            let reg_id = (opbyte - 0x58) + if rex_b { 8 } else { 0 };
            let reg = Register::from_x86_reg(reg_id);
            Some(make_inst(Opcode::Pop, vec![Operand::Reg(reg)], addr, pos, &bytes[..pos]))
        }

        // MOV r64, imm64 (REX.W + B8+rd)
        0xB8..=0xBF if rex_w => {
            let reg_id = (opbyte - 0xB8) + if rex_b { 8 } else { 0 };
            let reg = Register::from_x86_reg(reg_id);
            if pos + 8 > bytes.len() { return None; }
            let imm = i64::from_le_bytes(bytes[pos..pos+8].try_into().unwrap());
            pos += 8;
            Some(make_inst(Opcode::Mov, vec![Operand::Reg(reg), Operand::Imm64(imm)], addr, pos, &bytes[..pos]))
        }

        // MOV r32, imm32 (B8+rd)
        0xB8..=0xBF => {
            let reg_id = (opbyte - 0xB8) + if rex_b { 8 } else { 0 };
            let reg = Register::from_x86_reg(reg_id);
            if pos + 4 > bytes.len() { return None; }
            let imm = i32::from_le_bytes(bytes[pos..pos+4].try_into().unwrap());
            pos += 4;
            Some(make_inst(Opcode::Mov, vec![Operand::Reg(reg), Operand::Imm32(imm)], addr, pos, &bytes[..pos]))
        }

        // XOR r/m, r (31 /r) or XOR r, r/m (33 /r)
        0x31 | 0x33 => {
            if pos >= bytes.len() { return None; }
            let modrm = bytes[pos]; pos += 1;
            let (reg1, reg2) = decode_modrm_regs(modrm, rex_r, rex_b);
            if opbyte == 0x31 {
                Some(make_inst(Opcode::Xor, vec![Operand::Reg(reg2), Operand::Reg(reg1)], addr, pos, &bytes[..pos]))
            } else {
                Some(make_inst(Opcode::Xor, vec![Operand::Reg(reg1), Operand::Reg(reg2)], addr, pos, &bytes[..pos]))
            }
        }

        // MOV r/m, r (89 /r) or MOV r, r/m (8B /r)
        0x89 | 0x8B => {
            if pos >= bytes.len() { return None; }
            let modrm = bytes[pos]; pos += 1;
            let (reg1, reg2) = decode_modrm_regs(modrm, rex_r, rex_b);
            if opbyte == 0x89 {
                Some(make_inst(Opcode::Mov, vec![Operand::Reg(reg2), Operand::Reg(reg1)], addr, pos, &bytes[..pos]))
            } else {
                Some(make_inst(Opcode::Mov, vec![Operand::Reg(reg1), Operand::Reg(reg2)], addr, pos, &bytes[..pos]))
            }
        }

        // SUB/ADD/CMP r/m, imm8 (83 /5, /0, /7)
        0x83 => {
            if pos >= bytes.len() { return None; }
            let modrm = bytes[pos]; pos += 1;
            let op_ext = (modrm >> 3) & 7;
            let rm = (modrm & 7) + if rex_b { 8 } else { 0 };
            let reg = Register::from_x86_reg(rm);
            if pos >= bytes.len() { return None; }
            let imm = bytes[pos] as i8 as i32;
            pos += 1;
            let opcode = match op_ext {
                0 => Opcode::Add,
                5 => Opcode::Sub,
                7 => Opcode::Cmp,
                1 => Opcode::Or,
                4 => Opcode::And,
                6 => Opcode::Xor,
                _ => Opcode::RawBytes,
            };
            Some(make_inst(opcode, vec![Operand::Reg(reg), Operand::Imm32(imm)], addr, pos, &bytes[..pos]))
        }

        // CALL rel32 (E8)
        0xE8 => {
            if pos + 4 > bytes.len() { return None; }
            let rel = i32::from_le_bytes(bytes[pos..pos+4].try_into().unwrap());
            pos += 4;
            let target = addr.wrapping_add(pos as u64).wrapping_add(rel as i64 as u64);
            Some(make_inst(Opcode::Call, vec![Operand::Imm64(target as i64)], addr, pos, &bytes[..pos]))
        }

        // JMP rel32 (E9)
        0xE9 => {
            if pos + 4 > bytes.len() { return None; }
            let rel = i32::from_le_bytes(bytes[pos..pos+4].try_into().unwrap());
            pos += 4;
            let target = addr.wrapping_add(pos as u64).wrapping_add(rel as i64 as u64);
            Some(make_inst(Opcode::Jmp, vec![Operand::Imm64(target as i64)], addr, pos, &bytes[..pos]))
        }

        // JMP rel8 (EB)
        0xEB => {
            if pos >= bytes.len() { return None; }
            let rel = bytes[pos] as i8 as i64;
            pos += 1;
            let target = addr.wrapping_add(pos as u64).wrapping_add(rel as u64);
            Some(make_inst(Opcode::Jmp, vec![Operand::Imm64(target as i64)], addr, pos, &bytes[..pos]))
        }

        // Jcc rel8 (70-7F)
        0x70..=0x7F => {
            if pos >= bytes.len() { return None; }
            let rel = bytes[pos] as i8 as i64;
            pos += 1;
            let target = addr.wrapping_add(pos as u64).wrapping_add(rel as u64);
            let opcode = match opbyte {
                0x74 => Opcode::Je,
                0x75 => Opcode::Jne,
                0x7F => Opcode::Jg,
                0x7D => Opcode::Jge,
                0x7C => Opcode::Jl,
                0x7E => Opcode::Jle,
                0x77 => Opcode::Ja,
                0x73 => Opcode::Jae,
                0x72 => Opcode::Jb,
                0x76 => Opcode::Jbe,
                _    => Opcode::Jmp,
            };
            Some(make_inst(opcode, vec![Operand::Imm64(target as i64)], addr, pos, &bytes[..pos]))
        }

        // LEA r, m (8D /r)
        0x8D => {
            if pos >= bytes.len() { return None; }
            let modrm = bytes[pos]; pos += 1;
            let (reg1, reg2) = decode_modrm_regs(modrm, rex_r, rex_b);
            Some(make_inst(Opcode::Lea, vec![Operand::Reg(reg1), Operand::Reg(reg2)], addr, pos, &bytes[..pos]))
        }

        // TEST r/m, r (85 /r)
        0x85 => {
            if pos >= bytes.len() { return None; }
            let modrm = bytes[pos]; pos += 1;
            let (reg1, reg2) = decode_modrm_regs(modrm, rex_r, rex_b);
            Some(make_inst(Opcode::Test, vec![Operand::Reg(reg2), Operand::Reg(reg1)], addr, pos, &bytes[..pos]))
        }

        // Two-byte opcodes (0F xx)
        0x0F => {
            if pos >= bytes.len() { return None; }
            let op2 = bytes[pos]; pos += 1;
            match op2 {
                // SYSCALL
                0x05 => Some(make_simple(Opcode::Syscall, addr, pos, &bytes[..pos])),

                // Jcc rel32 (0F 80-8F)
                0x80..=0x8F => {
                    if pos + 4 > bytes.len() { return None; }
                    let rel = i32::from_le_bytes(bytes[pos..pos+4].try_into().unwrap());
                    pos += 4;
                    let target = addr.wrapping_add(pos as u64).wrapping_add(rel as i64 as u64);
                    let opcode = match op2 {
                        0x84 => Opcode::Je,
                        0x85 => Opcode::Jne,
                        0x8F => Opcode::Jg,
                        0x8D => Opcode::Jge,
                        0x8C => Opcode::Jl,
                        0x8E => Opcode::Jle,
                        0x87 => Opcode::Ja,
                        0x83 => Opcode::Jae,
                        0x82 => Opcode::Jb,
                        0x86 => Opcode::Jbe,
                        _    => Opcode::Jmp,
                    };
                    Some(make_inst(opcode, vec![Operand::Imm64(target as i64)], addr, pos, &bytes[..pos]))
                }

                // MOVZX (0F B6/B7)
                0xB6 | 0xB7 => {
                    if pos >= bytes.len() { return None; }
                    let modrm = bytes[pos]; pos += 1;
                    let (reg1, reg2) = decode_modrm_regs(modrm, rex_r, rex_b);
                    Some(make_inst(Opcode::Movzx, vec![Operand::Reg(reg1), Operand::Reg(reg2)], addr, pos, &bytes[..pos]))
                }

                // UD2
                0x0B => Some(make_simple(Opcode::Ud2, addr, pos, &bytes[..pos])),

                // Unknown 0F xx — emit as raw
                _ => {
                    let raw = bytes[..pos].to_vec();
                    Some(make_raw(raw, addr, pos))
                }
            }
        }

        // FF group (CALL/JMP indirect, PUSH, INC, DEC)
        0xFF => {
            if pos >= bytes.len() { return None; }
            let modrm = bytes[pos]; pos += 1;
            let op_ext = (modrm >> 3) & 7;
            let rm = (modrm & 7) + if rex_b { 8 } else { 0 };

            // Check for RIP-relative (mod=00, rm=5)
            let _mode = modrm >> 6;

            match op_ext {
                2 => { // CALL r/m
                    let reg = Register::from_x86_reg(rm);
                    Some(make_inst(Opcode::Call, vec![Operand::Reg(reg)], addr, pos, &bytes[..pos]))
                }
                4 => { // JMP r/m
                    let reg = Register::from_x86_reg(rm);
                    Some(make_inst(Opcode::Jmp, vec![Operand::Reg(reg)], addr, pos, &bytes[..pos]))
                }
                6 => { // PUSH r/m
                    let reg = Register::from_x86_reg(rm);
                    Some(make_inst(Opcode::Push, vec![Operand::Reg(reg)], addr, pos, &bytes[..pos]))
                }
                0 => { // INC r/m
                    let reg = Register::from_x86_reg(rm);
                    Some(make_inst(Opcode::Inc, vec![Operand::Reg(reg)], addr, pos, &bytes[..pos]))
                }
                1 => { // DEC r/m
                    let reg = Register::from_x86_reg(rm);
                    Some(make_inst(Opcode::Dec, vec![Operand::Reg(reg)], addr, pos, &bytes[..pos]))
                }
                _ => {
                    let raw = bytes[..pos].to_vec();
                    Some(make_raw(raw, addr, pos))
                }
            }
        }

        // LEAVE (C9)
        0xC9 => Some(make_simple(Opcode::Leave, addr, pos, &bytes[..pos])),

        // Unknown — emit as raw byte
        _ => {
            let raw = bytes[..pos].to_vec();
            Some(make_raw(raw, addr, pos))
        }
    }
}

/// Decode all instructions in a code buffer
pub fn decode_all(code: &[u8], base_addr: u64) -> Vec<ABIB_Instruction> {
    let mut result = Vec::new();
    let mut offset = 0;
    while offset < code.len() {
        match decode_one(code, offset, base_addr) {
            Some(decoded) => {
                offset += decoded.size;
                result.push(decoded.instruction);
            }
            None => {
                // Skip unknown byte
                let mut inst = ABIB_Instruction::new(Opcode::RawBytes);
                inst.source_addr = base_addr + offset as u64;
                inst.source_size = 1;
                inst.raw = vec![code[offset]];
                result.push(inst);
                offset += 1;
            }
        }
    }
    result
}

// ============================================================
// Helpers
// ============================================================

fn decode_modrm_regs(modrm: u8, rex_r: bool, rex_b: bool) -> (Register, Register) {
    let reg = ((modrm >> 3) & 7) + if rex_r { 8 } else { 0 };
    let rm = (modrm & 7) + if rex_b { 8 } else { 0 };
    (Register::from_x86_reg(reg), Register::from_x86_reg(rm))
}

fn make_simple(opcode: Opcode, addr: u64, size: usize, raw: &[u8]) -> DecodedInst {
    DecodedInst {
        instruction: ABIB_Instruction {
            opcode,
            operands: Vec::new(),
            source_addr: addr,
            source_size: size as u8,
            raw: raw.to_vec(),
        },
        size,
    }
}

fn make_inst(opcode: Opcode, operands: Vec<Operand>, addr: u64, size: usize, raw: &[u8]) -> DecodedInst {
    DecodedInst {
        instruction: ABIB_Instruction {
            opcode,
            operands,
            source_addr: addr,
            source_size: size as u8,
            raw: raw.to_vec(),
        },
        size,
    }
}

fn make_raw(raw: Vec<u8>, addr: u64, size: usize) -> DecodedInst {
    DecodedInst {
        instruction: ABIB_Instruction {
            opcode: Opcode::RawBytes,
            operands: Vec::new(),
            source_addr: addr,
            source_size: size as u8,
            raw,
        },
        size,
    }
}
