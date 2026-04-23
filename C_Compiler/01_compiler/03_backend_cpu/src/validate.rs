// ============================================================
// Validation Layer — Intermediate checkpoint between
// ISA compiler output and PE/binary generation
// ============================================================
//
// Purpose: Catch errors BEFORE they reach the PE generator.
// This prevents corrupt executables and provides clear diagnostics.
//
// Pipeline position:
//   ISA Compiler → [validate] → PE Generator
//
// ============================================================

use crate::iat_registry;
use std::collections::HashSet;

/// Artifacts produced by the ISA compiler, validated before PE generation.
#[derive(Debug)]
pub struct ValidatedArtifacts {
    pub code: Vec<u8>,
    pub data: Vec<u8>,
    pub iat_call_offsets: Vec<usize>,
    pub string_imm64_offsets: Vec<usize>,
    pub used_iat_slots: HashSet<usize>,
}

/// Validation error with context
#[derive(Debug)]
pub struct ValidationError {
    pub phase: &'static str,
    pub message: String,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.phase, self.message)
    }
}

impl std::error::Error for ValidationError {}

/// Validate compiler output before PE generation.
/// Returns `ValidatedArtifacts` on success, or a list of errors.
pub fn validate_for_pe(
    code: &[u8],
    data: &[u8],
    iat_call_offsets: &[usize],
    string_imm64_offsets: &[usize],
    used_iat_slots: &HashSet<usize>,
) -> Result<ValidatedArtifacts, Vec<ValidationError>> {
    let mut errors = Vec::new();

    // 1. Code must not be empty
    if code.is_empty() {
        errors.push(ValidationError {
            phase: "code",
            message: "Code section is empty — no instructions generated".into(),
        });
    }

    // 2. Code size must be reasonable for a PE section (< 256 MB)
    if code.len() > 256 * 1024 * 1024 {
        errors.push(ValidationError {
            phase: "code",
            message: format!("Code section too large: {} bytes (max 256 MB)", code.len()),
        });
    }

    // 3. Validate IAT call offsets are within code bounds
    for (i, &off) in iat_call_offsets.iter().enumerate() {
        if off + 4 > code.len() {
            errors.push(ValidationError {
                phase: "iat_offsets",
                message: format!(
                    "IAT call offset [{}] = {} exceeds code size {} (needs +4 bytes)",
                    i, off, code.len()
                ),
            });
        }
    }

    // 4. Validate string imm64 offsets are within code bounds
    for (i, &off) in string_imm64_offsets.iter().enumerate() {
        if off + 8 > code.len() {
            errors.push(ValidationError {
                phase: "string_offsets",
                message: format!(
                    "String imm64 offset [{}] = {} exceeds code size {} (needs +8 bytes)",
                    i, off, code.len()
                ),
            });
        }
    }

    // 5. Validate all used IAT slots exist in the registry
    let total_slots = iat_registry::total_function_count();
    for &slot in used_iat_slots {
        if slot >= total_slots {
            errors.push(ValidationError {
                phase: "iat_slots",
                message: format!(
                    "Used IAT slot {} exceeds registry size {} — function not registered",
                    slot, total_slots
                ),
            });
        }
    }

    // 6. Verify IAT call offsets reference actual FF 15 (call [rip+disp32]) instructions
    for (i, &off) in iat_call_offsets.iter().enumerate() {
        if off >= 2 && off + 4 <= code.len() {
            let opcode_1 = code[off - 2];
            let opcode_2 = code[off - 1];
            if opcode_1 != 0xFF || opcode_2 != 0x15 {
                errors.push(ValidationError {
                    phase: "iat_verify",
                    message: format!(
                        "IAT offset [{}] at {} does not follow FF 15 (call [rip+disp32]): found {:02X} {:02X}",
                        i, off, opcode_1, opcode_2
                    ),
                });
            }
        }
    }

    // 7. Report DLL import summary (informational, not an error)
    // This helps diagnose which DLLs will be loaded

    if errors.is_empty() {
        Ok(ValidatedArtifacts {
            code: code.to_vec(),
            data: data.to_vec(),
            iat_call_offsets: iat_call_offsets.to_vec(),
            string_imm64_offsets: string_imm64_offsets.to_vec(),
            used_iat_slots: used_iat_slots.clone(),
        })
    } else {
        Err(errors)
    }
}

/// Summarize which DLLs and how many functions are imported.
/// Useful for diagnostics and logging.
pub fn import_summary(used_iat_slots: &HashSet<usize>) -> Vec<(String, usize)> {
    let mut result = Vec::new();
    let mut slot_idx = 0usize;
    for dll in iat_registry::DLL_IMPORTS {
        let mut count = 0usize;
        for _ in dll.functions {
            if used_iat_slots.contains(&slot_idx) {
                count += 1;
            }
            slot_idx += 1;
        }
        if count > 0 {
            result.push((dll.dll.to_string(), count));
        }
    }
    result
}

/// List the specific function names used from each DLL.
pub fn import_details(used_iat_slots: &HashSet<usize>) -> Vec<(String, Vec<String>)> {
    let mut result = Vec::new();
    let mut slot_idx = 0usize;
    for dll in iat_registry::DLL_IMPORTS {
        let mut funcs = Vec::new();
        for f in dll.functions {
            if used_iat_slots.contains(&slot_idx) {
                funcs.push(f.to_string());
            }
            slot_idx += 1;
        }
        if !funcs.is_empty() {
            result.push((dll.dll.to_string(), funcs));
        }
    }
    result
}
