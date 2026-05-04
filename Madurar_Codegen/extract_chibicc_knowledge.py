#!/usr/bin/env python3
import re
import json

# Read chibicc codegen.c
with open('chibicc/codegen.c', 'r') as f:
    content = f.read()

knowledge = {
    "prologue": {
        "description": "Function prologue - setup stack frame",
        "pattern": "push rbp; mov rbp, rsp; sub rsp, N",
        "chibicc_pattern": "Not found in chibicc (uses different approach)",
        "current_implementation": "Already correct in codegen.rs"
    },
    "epilogue": {
        "description": "Function epilogue - restore stack frame",
        "pattern": "mov rsp, rbp; pop rbp; ret",
        "chibicc_pattern": "Not found in chibicc",
        "current_implementation": "Already correct in codegen.rs"
    },
    "stack_allocation": {
        "description": "Local variable stack allocation",
        "pattern": "sub rsp, N (N = 16-byte aligned)",
        "chibicc_pattern": "Uses alloca_bottom pointer approach",
        "current_implementation": "Simplified direct slot allocation in codegen.rs"
    },
    "arithmetic": {
        "add": "add %s, %s",
        "sub": "sub %s, %s",
        "imul": "imul %s, %s",
        "idiv": {
            "signed_64bit": "cqo; idiv %s",
            "signed_32bit": "cdq; idiv %s",
            "unsigned": "mov $0, %%rdx; div %s",
            "mod_result": "mov %%rdx, %%rax"
        }
    },
    "comparison": {
        "description": "Comparison with SETcc - CRITICAL PATTERN",
        "pattern": "cmp %s, %s; setcc %%al; movzb %%al, %%rax",
        "variants": {
            "sete": "sete %%al (equal)",
            "setne": "setne %%al (not equal)",
            "setl": "setl %%al (signed less)",
            "setle": "setle %%al (signed less or equal)",
            "setg": "setg %%al (signed greater)",
            "setge": "setge %%al (signed greater or equal)",
            "setb": "setb %%al (unsigned below)",
            "setbe": "setbe %%al (unsigned below or equal)",
            "seta": "seta %%al (unsigned above)",
            "setae": "setae %%al (unsigned above or equal)"
        },
        "critical_note": "SETcc only sets AL byte. MUST use MOVZX (0x48 0x0F 0xB6 0xC0) to zero-extend to full RAX before using as 64-bit value.",
        "chibicc_implementation": "println(\"  cmp %s, %s\", di, ax); println(\"  sete %%al\"); println(\"  movzb %%al, %%rax\");"
    },
    "jumps": {
        "unconditional": "jmp .L.label",
        "conditional": {
            "je": "je .L.label (jump if equal, ZF=1)",
            "jne": "jne .L.label (jump if not equal, ZF=0)",
            "jl": "jl .L.label (jump if less, signed)",
            "jle": "jle .L.label (jump if less or equal, signed)",
            "jg": "jg .L.label (jump if greater, signed)",
            "jge": "jge .L.label (jump if greater or equal, signed)",
            "jb": "jb .L.label (jump if below, unsigned)",
            "jbe": "jbe .L.label (jump if below or equal, unsigned)",
            "ja": "ja .L.label (jump if above, unsigned)",
            "jae": "jae .L.label (jump if above or equal, unsigned)"
        },
        "label_format": ".L.label:",
        "if_else_pattern": "gen_expr(cond); cmp_zero; je .L.else.N; gen_expr(then); jmp .L.end.N; .L.else.N: gen_expr(els); .L.end.N:",
        "loop_pattern": ".L.begin.N: gen_expr(cond); je .L.end.N; gen_expr(body); jmp .L.begin.N; .L.end.N:"
    },
    "bitwise": {
        "and": "and %s, %s",
        "or": "or %s, %s",
        "xor": "xor %s, %s",
        "not": "not %s",
        "shl": "mov %%rdi, %%rcx; shl %%cl, %s",
        "shr": "mov %%rdi, %%rcx; shr %%cl, %s (unsigned)",
        "sar": "mov %%rdi, %%rcx; sar %%cl, %s (signed)"
    },
    "load_store": {
        "load_from_stack": "mov %rax, [rbp-offset]",
        "store_to_stack": "mov [rbp-offset], %rax",
        "load_from_ptr": "mov %rax, [%rax]",
        "store_to_ptr": "mov [%rax], %rcx"
    },
    "call_ret": {
        "call": "call func",
        "ret": "ret"
    },
    "critical_fixes_needed": {
        "setcc_zero_extend": {
            "issue": "SETcc only sets AL byte, upper bytes of RAX contain garbage",
            "fix": "Add MOVZX AL, AL (0x48 0x0F 0xB6 0xC0) after every SETcc",
            "encoder_method": "Need movzx_rr method in encoder.rs",
            "current_status": "Manually added in emit_compare, but should be encoder method"
        },
        "imul_idiv": {
            "issue": "Need to verify IMUL/IDIV encoding is correct",
            "chibicc_pattern": "imul %s, %s; cqo/cdq; idiv %s",
            "check_needed": "Test arithmetic operations individually"
        },
        "jump_patching": {
            "issue": "Forward jumps need patching after code generation",
            "current_implementation": "pending_jumps with patch_jumps() - looks correct",
            "chibicc_pattern": "Uses label counter (.L.N) and direct label emission"
        }
    },
    "encoder_methods_needed": {
        "movzx_rr": "movzx r64, r8 (zero-extend 8-bit to 64-bit) - CRITICAL for SETcc",
        "movsx_rr": "movsx r64, r8 (sign-extend 8-bit to 64-bit) - for signed comparisons",
        "cdq": "cdq (sign-extend EAX to EDX:EAX)",
        "cqo": "cqo (sign-extend RAX to RDX:RAX)"
    },
    "byte_encoding_reference": {
        "movzx_al_to_eax": "0x0F 0xB6 0xC0 (MOVZX EAX, AL)",
        "movzx_al_to_rax": "0x48 0x0F 0xB6 0xC0 (REX.W + MOVZX RAX, AL)",
        "cdq": "0x99",
        "cqo": "0x48 0x99 (REX.W + CDQ)",
        "setcc_al": "0x0F 0x9X (where X is condition code: E=0x94, NE=0x95, L=0x9C, LE=0x9E, G=0x9F, GE=0x9D, B=0x92, BE=0x96, A=0x97, AE=0x93)"
    }
}

# Extract specific patterns from chibicc
patterns_found = []

# Find all arithmetic patterns
arithmetic_pattern = r'println\(\s*"\\s+(add|sub|imul|and|or|xor)\s+%[^,]+,\s*%[^"]+"\s*\);'
for match in re.finditer(arithmetic_pattern, content):
    patterns_found.append({"type": "arithmetic", "code": match.group(0)})

# Find setcc patterns with movzx
setcc_pattern = r'println\(\s*"\\s+(set[nelebg]+)\s+%%[^"]+"\s*\);\s*println\(\s*"\\s+movz[bw]\s+%%[al],\s*%%[^"]+"\s*\);'
for match in re.finditer(setcc_pattern, content):
    patterns_found.append({"type": "setcc_with_movzx", "code": match.group(0)})

# Find jump patterns
jump_pattern = r'println\(\s*"\\s+(j[e|ne|l|le|g|ge|b|be|a|ae]+)\s+\.?L?\.?[^"]+"\s*\);'
for match in re.finditer(jump_pattern, content):
    patterns_found.append({"type": "jump", "code": match.group(0)})

knowledge["extracted_patterns"] = patterns_found

# Save to JSON
with open('knowledge_codegen_complete.json', 'w') as f:
    json.dump(knowledge, f, indent=2)

print("Extracted complete knowledge from chibicc:")
print(f"- Arithmetic patterns: {len([p for p in patterns_found if p['type'] == 'arithmetic'])}")
print(f"- SETCC patterns: {len([p for p in patterns_found if p['type'] == 'setcc_with_movzx'])}")
print(f"- Jump patterns: {len([p for p in patterns_found if p['type'] == 'jump'])}")
print(f"\nSaved to knowledge_codegen_complete.json")
