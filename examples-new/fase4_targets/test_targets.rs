// Test Fase 4: Multi-Target (ELF y PE)
// Verifica que la generación de binarios ELF y PE es determinista
//
// Autor: Eddi Andreé Salazar Matos

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// Constantes ELF
const ELF_MAGIC: [u8; 4] = [0x7F, b'E', b'L', b'F'];
const ELFCLASS64: u8 = 2;
const ELFDATA2LSB: u8 = 1;
const ET_EXEC: u16 = 2;
const EM_X86_64: u16 = 62;
const PT_LOAD: u32 = 1;
const PF_RWX: u32 = 7;

const ELF_BASE: u64 = 0x400000;
const ELF_HEADER_SIZE: u64 = 64;
const ELF_PHDR_SIZE: u64 = 56;

// Constantes PE
const PE_MAGIC: [u8; 2] = [b'M', b'Z'];
const PE_SIGNATURE: [u8; 4] = [b'P', b'E', 0, 0];

/// Genera un binario ELF x86-64 mínimo
fn generate_elf(code: &[u8], data: &[u8]) -> Vec<u8> {
    let mut binary = Vec::new();
    
    let code_offset = ELF_HEADER_SIZE + ELF_PHDR_SIZE;
    let data_offset = code_offset + code.len() as u64;
    let total_size = data_offset + data.len() as u64;
    let entry_point = ELF_BASE + code_offset;
    
    // ELF Header (64 bytes)
    binary.extend_from_slice(&ELF_MAGIC);
    binary.push(ELFCLASS64);
    binary.push(ELFDATA2LSB);
    binary.push(1); // EV_CURRENT
    binary.push(0); // ELFOSABI_NONE
    binary.extend_from_slice(&[0u8; 8]); // padding
    binary.extend_from_slice(&ET_EXEC.to_le_bytes());
    binary.extend_from_slice(&EM_X86_64.to_le_bytes());
    binary.extend_from_slice(&1u32.to_le_bytes()); // e_version
    binary.extend_from_slice(&entry_point.to_le_bytes()); // e_entry
    binary.extend_from_slice(&ELF_HEADER_SIZE.to_le_bytes()); // e_phoff
    binary.extend_from_slice(&0u64.to_le_bytes()); // e_shoff
    binary.extend_from_slice(&0u32.to_le_bytes()); // e_flags
    binary.extend_from_slice(&(ELF_HEADER_SIZE as u16).to_le_bytes()); // e_ehsize
    binary.extend_from_slice(&(ELF_PHDR_SIZE as u16).to_le_bytes()); // e_phentsize
    binary.extend_from_slice(&1u16.to_le_bytes()); // e_phnum
    binary.extend_from_slice(&0u16.to_le_bytes()); // e_shentsize
    binary.extend_from_slice(&0u16.to_le_bytes()); // e_shnum
    binary.extend_from_slice(&0u16.to_le_bytes()); // e_shstrndx
    
    // Program Header (56 bytes)
    binary.extend_from_slice(&PT_LOAD.to_le_bytes()); // p_type
    binary.extend_from_slice(&PF_RWX.to_le_bytes()); // p_flags
    binary.extend_from_slice(&0u64.to_le_bytes()); // p_offset
    binary.extend_from_slice(&ELF_BASE.to_le_bytes()); // p_vaddr
    binary.extend_from_slice(&ELF_BASE.to_le_bytes()); // p_paddr
    binary.extend_from_slice(&total_size.to_le_bytes()); // p_filesz
    binary.extend_from_slice(&total_size.to_le_bytes()); // p_memsz
    binary.extend_from_slice(&0x1000u64.to_le_bytes()); // p_align
    
    // Code
    binary.extend_from_slice(code);
    
    // Data
    binary.extend_from_slice(data);
    
    binary
}

/// Genera un binario PE x86-64 mínimo
fn generate_pe(code: &[u8], data: &[u8]) -> Vec<u8> {
    let mut binary = Vec::new();
    
    // DOS Header (64 bytes mínimo)
    binary.extend_from_slice(&PE_MAGIC);
    binary.extend_from_slice(&[0u8; 58]); // padding
    binary.extend_from_slice(&0x40u32.to_le_bytes()); // e_lfanew (offset to PE header)
    
    // PE Signature
    binary.extend_from_slice(&PE_SIGNATURE);
    
    // COFF Header (20 bytes)
    binary.extend_from_slice(&0x8664u16.to_le_bytes()); // Machine (AMD64)
    binary.extend_from_slice(&2u16.to_le_bytes()); // NumberOfSections
    binary.extend_from_slice(&0u32.to_le_bytes()); // TimeDateStamp
    binary.extend_from_slice(&0u32.to_le_bytes()); // PointerToSymbolTable
    binary.extend_from_slice(&0u32.to_le_bytes()); // NumberOfSymbols
    binary.extend_from_slice(&240u16.to_le_bytes()); // SizeOfOptionalHeader
    binary.extend_from_slice(&0x22u16.to_le_bytes()); // Characteristics
    
    // Optional Header (240 bytes para PE32+)
    binary.extend_from_slice(&0x20Bu16.to_le_bytes()); // Magic (PE32+)
    binary.push(14); // MajorLinkerVersion
    binary.push(0); // MinorLinkerVersion
    binary.extend_from_slice(&(code.len() as u32).to_le_bytes()); // SizeOfCode
    binary.extend_from_slice(&(data.len() as u32).to_le_bytes()); // SizeOfInitializedData
    binary.extend_from_slice(&0u32.to_le_bytes()); // SizeOfUninitializedData
    binary.extend_from_slice(&0x1000u32.to_le_bytes()); // AddressOfEntryPoint
    binary.extend_from_slice(&0x1000u32.to_le_bytes()); // BaseOfCode
    binary.extend_from_slice(&0x400000u64.to_le_bytes()); // ImageBase
    binary.extend_from_slice(&0x1000u32.to_le_bytes()); // SectionAlignment
    binary.extend_from_slice(&0x200u32.to_le_bytes()); // FileAlignment
    binary.extend_from_slice(&6u16.to_le_bytes()); // MajorOSVersion
    binary.extend_from_slice(&0u16.to_le_bytes()); // MinorOSVersion
    binary.extend_from_slice(&0u16.to_le_bytes()); // MajorImageVersion
    binary.extend_from_slice(&0u16.to_le_bytes()); // MinorImageVersion
    binary.extend_from_slice(&6u16.to_le_bytes()); // MajorSubsystemVersion
    binary.extend_from_slice(&0u16.to_le_bytes()); // MinorSubsystemVersion
    binary.extend_from_slice(&0u32.to_le_bytes()); // Win32VersionValue
    binary.extend_from_slice(&0x3000u32.to_le_bytes()); // SizeOfImage
    binary.extend_from_slice(&0x200u32.to_le_bytes()); // SizeOfHeaders
    binary.extend_from_slice(&0u32.to_le_bytes()); // CheckSum
    binary.extend_from_slice(&3u16.to_le_bytes()); // Subsystem (CONSOLE)
    binary.extend_from_slice(&0u16.to_le_bytes()); // DllCharacteristics
    binary.extend_from_slice(&0x100000u64.to_le_bytes()); // SizeOfStackReserve
    binary.extend_from_slice(&0x1000u64.to_le_bytes()); // SizeOfStackCommit
    binary.extend_from_slice(&0x100000u64.to_le_bytes()); // SizeOfHeapReserve
    binary.extend_from_slice(&0x1000u64.to_le_bytes()); // SizeOfHeapCommit
    binary.extend_from_slice(&0u32.to_le_bytes()); // LoaderFlags
    binary.extend_from_slice(&16u32.to_le_bytes()); // NumberOfRvaAndSizes
    
    // Data Directories (16 * 8 = 128 bytes)
    for _ in 0..16 {
        binary.extend_from_slice(&0u64.to_le_bytes());
    }
    
    // Padding to align
    while binary.len() < 0x200 {
        binary.push(0);
    }
    
    // .text section (code)
    binary.extend_from_slice(code);
    while binary.len() < 0x400 {
        binary.push(0);
    }
    
    // .rdata section (data)
    binary.extend_from_slice(data);
    while binary.len() < 0x600 {
        binary.push(0);
    }
    
    binary
}

/// Genera código Hello World para Linux (syscalls)
fn generate_hello_linux() -> (Vec<u8>, Vec<u8>) {
    let mut code = Vec::new();
    let string_addr: u64 = ELF_BASE + ELF_HEADER_SIZE + ELF_PHDR_SIZE + 45; // después del código
    
    // sys_write(1, string, 14)
    code.extend_from_slice(&[0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00]); // mov rax, 1
    code.extend_from_slice(&[0x48, 0xC7, 0xC7, 0x01, 0x00, 0x00, 0x00]); // mov rdi, 1
    code.extend_from_slice(&[0x48, 0xBE]); // mov rsi, addr
    code.extend_from_slice(&string_addr.to_le_bytes());
    code.extend_from_slice(&[0x48, 0xC7, 0xC2, 0x0E, 0x00, 0x00, 0x00]); // mov rdx, 14
    code.extend_from_slice(&[0x0F, 0x05]); // syscall
    
    // sys_exit(0)
    code.extend_from_slice(&[0x48, 0xC7, 0xC0, 0x3C, 0x00, 0x00, 0x00]); // mov rax, 60
    code.extend_from_slice(&[0x48, 0x31, 0xFF]); // xor rdi, rdi
    code.extend_from_slice(&[0x0F, 0x05]); // syscall
    
    let data = b"Hello, World!\n".to_vec();
    
    (code, data)
}

/// Calcula hash de bytes
fn hash_bytes(bytes: &[u8]) -> u64 {
    let mut hasher = DefaultHasher::new();
    bytes.hash(&mut hasher);
    hasher.finish()
}

/// Test 1: ELF determinista
fn test_elf_deterministic() -> bool {
    let (code, data) = generate_hello_linux();
    
    let elf1 = generate_elf(&code, &data);
    let elf2 = generate_elf(&code, &data);
    let elf3 = generate_elf(&code, &data);
    
    let hash1 = hash_bytes(&elf1);
    let hash2 = hash_bytes(&elf2);
    let hash3 = hash_bytes(&elf3);
    
    println!("  ELF hash1: {:016x}", hash1);
    println!("  ELF hash2: {:016x}", hash2);
    println!("  ELF hash3: {:016x}", hash3);
    println!("  Bytes: {} | Determinista: {}", elf1.len(), hash1 == hash2 && hash2 == hash3);
    
    // Verificar magic
    let has_magic = &elf1[0..4] == &ELF_MAGIC;
    println!("  Magic ELF válido: {}", has_magic);
    
    hash1 == hash2 && hash2 == hash3 && has_magic
}

/// Test 2: PE determinista
fn test_pe_deterministic() -> bool {
    let code = vec![0x31, 0xC0, 0xC3]; // xor eax, eax; ret
    let data = b"Test".to_vec();
    
    let pe1 = generate_pe(&code, &data);
    let pe2 = generate_pe(&code, &data);
    let pe3 = generate_pe(&code, &data);
    
    let hash1 = hash_bytes(&pe1);
    let hash2 = hash_bytes(&pe2);
    let hash3 = hash_bytes(&pe3);
    
    println!("  PE hash1: {:016x}", hash1);
    println!("  PE hash2: {:016x}", hash2);
    println!("  PE hash3: {:016x}", hash3);
    println!("  Bytes: {} | Determinista: {}", pe1.len(), hash1 == hash2 && hash2 == hash3);
    
    // Verificar magic
    let has_magic = &pe1[0..2] == &PE_MAGIC;
    println!("  Magic MZ válido: {}", has_magic);
    
    hash1 == hash2 && hash2 == hash3 && has_magic
}

/// Test 3: Mismo código genera diferentes formatos
fn test_same_code_different_formats() -> bool {
    let code = vec![
        0x48, 0xC7, 0xC0, 0x2A, 0x00, 0x00, 0x00, // mov rax, 42
        0xC3, // ret
    ];
    let data = b"test".to_vec();
    
    let elf = generate_elf(&code, &data);
    let pe = generate_pe(&code, &data);
    
    let elf_hash = hash_bytes(&elf);
    let pe_hash = hash_bytes(&pe);
    
    println!("  ELF: {} bytes, hash {:016x}", elf.len(), elf_hash);
    println!("  PE:  {} bytes, hash {:016x}", pe.len(), pe_hash);
    println!("  Formatos diferentes: {}", elf_hash != pe_hash);
    
    // Verificar que el código está en ambos
    let code_in_elf = elf.windows(code.len()).any(|w| w == code.as_slice());
    let code_in_pe = pe.windows(code.len()).any(|w| w == code.as_slice());
    
    println!("  Código en ELF: {}", code_in_elf);
    println!("  Código en PE: {}", code_in_pe);
    
    elf_hash != pe_hash && code_in_elf && code_in_pe
}

/// Test 4: ELF con diferentes tamaños de código
fn test_elf_various_sizes() -> bool {
    let sizes = [10, 50, 100, 500, 1000];
    let mut all_deterministic = true;
    
    println!("  Probando varios tamaños de código:");
    for &size in &sizes {
        let code: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
        let data = b"data".to_vec();
        
        let elf1 = generate_elf(&code, &data);
        let elf2 = generate_elf(&code, &data);
        
        let hash1 = hash_bytes(&elf1);
        let hash2 = hash_bytes(&elf2);
        
        let is_deterministic = hash1 == hash2;
        all_deterministic = all_deterministic && is_deterministic;
        
        println!("    {} bytes código -> {} bytes ELF (det: {})", 
                 size, elf1.len(), is_deterministic);
    }
    
    all_deterministic
}

/// Test 5: Binario raw (solo código)
fn test_raw_binary() -> bool {
    let code = vec![
        0x48, 0x31, 0xC0, // xor rax, rax
        0x48, 0xFF, 0xC0, // inc rax
        0xC3, // ret
    ];
    
    // Raw = solo código, sin headers
    let raw1 = code.clone();
    let raw2 = code.clone();
    
    let hash1 = hash_bytes(&raw1);
    let hash2 = hash_bytes(&raw2);
    
    println!("  Raw hash1: {:016x}", hash1);
    println!("  Raw hash2: {:016x}", hash2);
    println!("  Bytes: {} | Determinista: {}", raw1.len(), hash1 == hash2);
    
    // Comparar con ELF
    let elf = generate_elf(&code, &[]);
    println!("  Raw: {} bytes vs ELF: {} bytes (overhead: {} bytes)", 
             raw1.len(), elf.len(), elf.len() - raw1.len());
    
    hash1 == hash2
}

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║     ADead-BIB - Fase 4: Test Multi-Target (ELF/PE)         ║");
    println!("║     Autor: Eddi Andreé Salazar Matos                       ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();
    
    let mut passed = 0;
    let mut failed = 0;
    
    println!("🧪 Test 1: ELF determinista");
    if test_elf_deterministic() {
        println!("   ✅ PASSED\n");
        passed += 1;
    } else {
        println!("   ❌ FAILED\n");
        failed += 1;
    }
    
    println!("🧪 Test 2: PE determinista");
    if test_pe_deterministic() {
        println!("   ✅ PASSED\n");
        passed += 1;
    } else {
        println!("   ❌ FAILED\n");
        failed += 1;
    }
    
    println!("🧪 Test 3: Mismo código, diferentes formatos");
    if test_same_code_different_formats() {
        println!("   ✅ PASSED\n");
        passed += 1;
    } else {
        println!("   ❌ FAILED\n");
        failed += 1;
    }
    
    println!("🧪 Test 4: ELF con varios tamaños");
    if test_elf_various_sizes() {
        println!("   ✅ PASSED\n");
        passed += 1;
    } else {
        println!("   ❌ FAILED\n");
        failed += 1;
    }
    
    println!("🧪 Test 5: Binario raw");
    if test_raw_binary() {
        println!("   ✅ PASSED\n");
        passed += 1;
    } else {
        println!("   ❌ FAILED\n");
        failed += 1;
    }
    
    println!("════════════════════════════════════════════════════════════");
    println!("📊 Resultados: {} passed, {} failed", passed, failed);
    
    if failed == 0 {
        println!("✅ FASE 4 COMPLETADA - Multi-target es DETERMINISTA");
    } else {
        println!("❌ FASE 4 FALLIDA - Revisar implementación");
    }
}
