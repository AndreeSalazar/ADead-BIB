// PE (Portable Executable) Generator
// Genera binarios Windows .exe funcionales
// Binario + HEX = ADead-BIB - Directo a CPU

use std::fs::File;
use std::io::Write;

// Layout sincronizado con codegen.rs:
// 0x0000 - Headers (0x200 file, 0x1000 virtual)
// 0x1000 - .text (código)
// 0x2000 - .rdata (imports + strings)
//
// Dentro de .rdata (RVA 0x2000):
// 0x2000: Import Directory (20 bytes)
// 0x2014: Import Directory NULL terminator (20 bytes)
// 0x2028: ILT - Import Lookup Table (16 bytes)
// 0x2038: IAT - Import Address Table (16 bytes) <- printf address aquí
// 0x2048: DLL name "msvcrt.dll\0" (11 bytes)
// 0x2054: Hint/Name entry (2 + 7 = 9 bytes)
// 0x2060: User strings start

pub fn generate_pe(opcodes: &[u8], data: &[u8], output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut pe = Vec::new();
    
    // ========== DOS Header (64 bytes) ==========
    let mut dos = vec![0u8; 64];
    dos[0..2].copy_from_slice(b"MZ");
    dos[0x3C..0x40].copy_from_slice(&0x40u32.to_le_bytes()); // e_lfanew -> PE header at 0x40
    pe.extend_from_slice(&dos);
    
    // ========== PE Signature (4 bytes) ==========
    pe.extend_from_slice(b"PE\0\0");
    
    // ========== COFF Header (20 bytes) ==========
    pe.extend_from_slice(&0x8664u16.to_le_bytes()); // Machine: AMD64
    pe.extend_from_slice(&0x0002u16.to_le_bytes()); // NumberOfSections: 2
    pe.extend_from_slice(&0u32.to_le_bytes());       // TimeDateStamp
    pe.extend_from_slice(&0u32.to_le_bytes());       // PointerToSymbolTable
    pe.extend_from_slice(&0u32.to_le_bytes());       // NumberOfSymbols
    pe.extend_from_slice(&0x00F0u16.to_le_bytes()); // SizeOfOptionalHeader (240)
    pe.extend_from_slice(&0x0022u16.to_le_bytes()); // Characteristics: EXECUTABLE_IMAGE | LARGE_ADDRESS_AWARE
    
    // ========== Optional Header PE32+ (240 bytes) ==========
    let opt_start = pe.len();
    pe.extend_from_slice(&0x020Bu16.to_le_bytes()); // Magic: PE32+
    pe.push(0x0E); // MajorLinkerVersion
    pe.push(0x00); // MinorLinkerVersion
    
    let code_size = 0x200u32; // Tamaño de .text alineado
    pe.extend_from_slice(&code_size.to_le_bytes()); // SizeOfCode
    pe.extend_from_slice(&0x200u32.to_le_bytes());  // SizeOfInitializedData
    pe.extend_from_slice(&0u32.to_le_bytes());       // SizeOfUninitializedData
    pe.extend_from_slice(&0x1000u32.to_le_bytes()); // AddressOfEntryPoint (RVA)
    pe.extend_from_slice(&0x1000u32.to_le_bytes()); // BaseOfCode
    
    pe.extend_from_slice(&0x400000u64.to_le_bytes()); // ImageBase
    pe.extend_from_slice(&0x1000u32.to_le_bytes());   // SectionAlignment
    pe.extend_from_slice(&0x200u32.to_le_bytes());    // FileAlignment
    
    pe.extend_from_slice(&6u16.to_le_bytes());  // MajorOperatingSystemVersion
    pe.extend_from_slice(&0u16.to_le_bytes());  // MinorOperatingSystemVersion
    pe.extend_from_slice(&0u16.to_le_bytes());  // MajorImageVersion
    pe.extend_from_slice(&0u16.to_le_bytes());  // MinorImageVersion
    pe.extend_from_slice(&6u16.to_le_bytes());  // MajorSubsystemVersion
    pe.extend_from_slice(&0u16.to_le_bytes());  // MinorSubsystemVersion
    pe.extend_from_slice(&0u32.to_le_bytes());  // Win32VersionValue
    
    pe.extend_from_slice(&0x3000u32.to_le_bytes()); // SizeOfImage (3 pages)
    pe.extend_from_slice(&0x200u32.to_le_bytes());  // SizeOfHeaders
    pe.extend_from_slice(&0u32.to_le_bytes());       // CheckSum
    pe.extend_from_slice(&3u16.to_le_bytes());       // Subsystem: CONSOLE
    pe.extend_from_slice(&0x0000u16.to_le_bytes()); // DllCharacteristics: ninguno (sin ASLR para direcciones absolutas)
    
    pe.extend_from_slice(&0x100000u64.to_le_bytes()); // SizeOfStackReserve
    pe.extend_from_slice(&0x1000u64.to_le_bytes());   // SizeOfStackCommit
    pe.extend_from_slice(&0x100000u64.to_le_bytes()); // SizeOfHeapReserve
    pe.extend_from_slice(&0x1000u64.to_le_bytes());   // SizeOfHeapCommit
    pe.extend_from_slice(&0u32.to_le_bytes());         // LoaderFlags
    pe.extend_from_slice(&16u32.to_le_bytes());        // NumberOfRvaAndSizes
    
    // Data Directories (16 entries × 8 bytes = 128 bytes)
    // 0: Export - none
    pe.extend_from_slice(&0u64.to_le_bytes());
    // 1: Import Directory - RVA 0x2000, Size 40
    pe.extend_from_slice(&0x2000u32.to_le_bytes());
    pe.extend_from_slice(&40u32.to_le_bytes());
    // 2-11: Empty
    for _ in 2..12 {
        pe.extend_from_slice(&0u64.to_le_bytes());
    }
    // 12: IAT - RVA 0x2038, Size 16
    pe.extend_from_slice(&0x2038u32.to_le_bytes());
    pe.extend_from_slice(&16u32.to_le_bytes());
    // 13-15: Empty
    for _ in 13..16 {
        pe.extend_from_slice(&0u64.to_le_bytes());
    }
    
    assert_eq!(pe.len() - opt_start, 240, "Optional header size mismatch");
    
    // ========== Section Headers (2 × 40 bytes) ==========
    // .text section
    pe.extend_from_slice(b".text\0\0\0");
    pe.extend_from_slice(&code_size.to_le_bytes()); // VirtualSize
    pe.extend_from_slice(&0x1000u32.to_le_bytes()); // VirtualAddress
    pe.extend_from_slice(&code_size.to_le_bytes()); // SizeOfRawData
    pe.extend_from_slice(&0x200u32.to_le_bytes());  // PointerToRawData
    pe.extend_from_slice(&0u32.to_le_bytes());       // PointerToRelocations
    pe.extend_from_slice(&0u32.to_le_bytes());       // PointerToLinenumbers
    pe.extend_from_slice(&0u16.to_le_bytes());       // NumberOfRelocations
    pe.extend_from_slice(&0u16.to_le_bytes());       // NumberOfLinenumbers
    pe.extend_from_slice(&0x60000020u32.to_le_bytes()); // Characteristics: CODE | EXECUTE | READ
    
    // .rdata section
    pe.extend_from_slice(b".rdata\0\0");
    pe.extend_from_slice(&0x1000u32.to_le_bytes()); // VirtualSize
    pe.extend_from_slice(&0x2000u32.to_le_bytes()); // VirtualAddress
    pe.extend_from_slice(&0x200u32.to_le_bytes());  // SizeOfRawData
    pe.extend_from_slice(&0x400u32.to_le_bytes());  // PointerToRawData
    pe.extend_from_slice(&0u32.to_le_bytes());
    pe.extend_from_slice(&0u32.to_le_bytes());
    pe.extend_from_slice(&0u16.to_le_bytes());
    pe.extend_from_slice(&0u16.to_le_bytes());
    pe.extend_from_slice(&0x40000040u32.to_le_bytes()); // Characteristics: INITIALIZED_DATA | READ
    
    // Pad headers to 0x200
    while pe.len() < 0x200 {
        pe.push(0);
    }
    
    // ========== .text Section (file offset 0x200, RVA 0x1000) ==========
    let text_start = pe.len();
    pe.extend_from_slice(opcodes);
    // Pad to 0x200 bytes
    while pe.len() < text_start + 0x200 {
        pe.push(0);
    }
    
    // ========== .rdata Section (file offset 0x400, RVA 0x2000) ==========
    let rdata_start = pe.len();
    
    // Import Directory Entry (20 bytes) at RVA 0x2000
    // OriginalFirstThunk (ILT RVA)
    pe.extend_from_slice(&0x2028u32.to_le_bytes()); // ILT at 0x2028
    pe.extend_from_slice(&0u32.to_le_bytes());       // TimeDateStamp
    pe.extend_from_slice(&0u32.to_le_bytes());       // ForwarderChain
    pe.extend_from_slice(&0x2048u32.to_le_bytes()); // Name RVA (msvcrt.dll)
    pe.extend_from_slice(&0x2038u32.to_le_bytes()); // FirstThunk (IAT RVA)
    
    // Null Import Directory Entry (20 bytes) - terminator
    pe.extend_from_slice(&[0u8; 20]);
    
    // ILT - Import Lookup Table at RVA 0x2028 (8 bytes entry + 8 bytes null)
    pe.extend_from_slice(&0x2054u64.to_le_bytes()); // Points to Hint/Name
    pe.extend_from_slice(&0u64.to_le_bytes());       // Null terminator
    
    // IAT - Import Address Table at RVA 0x2038 (8 bytes entry + 8 bytes null)
    // Windows loader will overwrite this with actual printf address
    pe.extend_from_slice(&0x2054u64.to_le_bytes()); // Points to Hint/Name (same as ILT initially)
    pe.extend_from_slice(&0u64.to_le_bytes());       // Null terminator
    
    // DLL Name at RVA 0x2048
    pe.extend_from_slice(b"msvcrt.dll\0");
    
    // Padding to align Hint/Name at 0x2054
    while (pe.len() - rdata_start) < 0x54 {
        pe.push(0);
    }
    
    // Hint/Name Table Entry at RVA 0x2054
    pe.extend_from_slice(&0u16.to_le_bytes()); // Hint (0)
    pe.extend_from_slice(b"printf\0");
    
    // Padding to align strings at 0x2060
    while (pe.len() - rdata_start) < 0x60 {
        pe.push(0);
    }
    
    // User strings at RVA 0x2060
    pe.extend_from_slice(data);
    
    // Pad .rdata to 0x200 bytes
    while pe.len() < rdata_start + 0x200 {
        pe.push(0);
    }
    
    // Write to file
    let mut file = File::create(output_path)?;
    file.write_all(&pe)?;
    
    println!("   PE size: {} bytes", pe.len());
    println!("   Code: {} bytes", opcodes.len());
    println!("   Data: {} bytes", data.len());
    
    Ok(())
}
