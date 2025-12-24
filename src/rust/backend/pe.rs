// PE (Portable Executable) Generator
// Genera binarios Windows .exe funcionales con soporte para imports (printf)
// Versión validada según especificación PE

use std::fs::File;
use std::io::Write;

pub fn generate_pe(opcodes: &[u8], _data: &[u8], output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(output_path)?;
    
    // Constants
    let file_align = 0x200;
    // let section_align = 0x1000;
    
    // Calculate code size aligned to file alignment
    let code_raw_size = ((opcodes.len() + file_align - 1) / file_align * file_align) as u32;
    // Virtual size is actual size
    let code_virtual_size = opcodes.len() as u32;
    
    // .idata content generation
    // RVA Base for .idata: 0x2000 (assuming .text < 0x1000)
    let idata_rva: u32 = 0x2000;
    
    // Layout offsets within .idata:
    // 0x00: IDT (Import Directory Table) - 1 entry (msvcrt) + null (2 * 20 bytes = 40 bytes)
    // 0x28: ILT (Import Lookup Table) - 1 entry (printf) + null (2 * 8 bytes = 16 bytes, PE32+ uses 64-bit thunks)
    // 0x38: IAT (Import Address Table) - 1 entry (printf) + null (2 * 8 bytes = 16 bytes). RVA: 0x2038
    // 0x48: Strings area
    
    // Reserve enough space for IDT+ILT+IAT+strings ("msvcrt.dll", "printf", program strings)
    let mut idata = vec![0u8; file_align]; // initial capacity
    
    // IDT[0] for msvcrt.dll
    // OriginalFirstThunk (RVA of ILT) = 0x2028
    idata[0..4].copy_from_slice(&(idata_rva + 0x28).to_le_bytes());
    // Name (RVA of DLL name) = 0x2048
    idata[12..16].copy_from_slice(&(idata_rva + 0x48).to_le_bytes());
    // FirstThunk (RVA of IAT) = 0x2038
    idata[16..20].copy_from_slice(&(idata_rva + 0x38).to_le_bytes());
    
    // ILT[0] points to Hint/Name table. RVA = 0x2054
    // 0x48 + "msvcrt.dll\0".len() = 0x48 + 11 = 0x53 => next even boundary 0x54.
    // PE32+ requires 64-bit entries for ILT/IAT (IMAGE_THUNK_DATA64)
    let import_by_name_rva: u32 = idata_rva + 0x54;
    idata[0x28..0x30].copy_from_slice(&(import_by_name_rva as u64).to_le_bytes());
    
    // IAT[0] same as ILT[0] initially (loader overwrites with resolved address)
    idata[0x38..0x40].copy_from_slice(&(import_by_name_rva as u64).to_le_bytes());
    
    // Strings
    let dll_name = b"msvcrt.dll\0";
    idata[0x48..0x48+dll_name.len()].copy_from_slice(dll_name);
    
    // Hint/Name at 0x54
    // Hint (2 bytes) = 0
    let func_name = b"printf\0";
    idata[0x56..0x56+func_name.len()].copy_from_slice(func_name);
    
    // Append program strings to .idata after 0x60
    // Ensure we have enough capacity
    let program_strings_offset = 0x60usize;
    // Grow idata to fit appended data
    if program_strings_offset + _data.len() > idata.len() {
        let needed = program_strings_offset + _data.len();
        let aligned = ((needed + file_align - 1) / file_align) * file_align;
        idata.resize(aligned, 0);
    }
    idata[program_strings_offset..program_strings_offset + _data.len()]
        .copy_from_slice(_data);
    
    let idata_raw_size = idata.len() as u32;
    let idata_virtual_size = idata_raw_size;
    
    // Headers
    // DOS Header (64 bytes)
    let mut dos = vec![0u8; 64];
    dos[0] = 0x4D; // 'M'
    dos[1] = 0x5A; // 'Z'
    dos[0x3C] = 0x40; // e_lfanew
    file.write_all(&dos)?;
    
    // PE Signature
    file.write_all(b"PE\0\0")?;
    
    // COFF Header (20 bytes)
    let mut coff = vec![0u8; 20];
    coff[0] = 0x64; coff[1] = 0x86; // x64
    coff[2] = 0x02; coff[3] = 0x00; // NumberOfSections: 2
    coff[16] = 0xF0; coff[17] = 0x00; // SizeOfOptionalHeader
    coff[18] = 0x22; coff[19] = 0x00; // Characteristics
    file.write_all(&coff)?;
    
    // Optional Header (240 bytes)
    let mut opt = vec![0u8; 240];
    opt[0] = 0x0B; opt[1] = 0x02; // Magic PE32+
    opt[2] = 14; // Linker version
    opt[4..8].copy_from_slice(&code_raw_size.to_le_bytes()); // SizeOfCode
    opt[8..12].copy_from_slice(&idata_raw_size.to_le_bytes()); // SizeOfInitializedData
    opt[16..20].copy_from_slice(&0x1000u32.to_le_bytes()); // AddressOfEntryPoint
    opt[20..24].copy_from_slice(&0x1000u32.to_le_bytes()); // BaseOfCode
    opt[24..32].copy_from_slice(&0x0000000140000000u64.to_le_bytes()); // ImageBase
    opt[32..36].copy_from_slice(&0x1000u32.to_le_bytes()); // SectionAlignment
    opt[36..40].copy_from_slice(&0x200u32.to_le_bytes()); // FileAlignment
    opt[40] = 6; // MajorOSVersion
    opt[48] = 6; // MajorSubsystemVersion
    
    // SizeOfImage: headers (0x1000) + .text (0x1000) + .idata (0x1000) = 0x3000
    let size_of_image = 0x3000u32;
    opt[56..60].copy_from_slice(&size_of_image.to_le_bytes());
    opt[60..64].copy_from_slice(&0x400u32.to_le_bytes()); // SizeOfHeaders
    opt[68] = 0x03; // Subsystem CUI
    opt[72..80].copy_from_slice(&0x100000u64.to_le_bytes()); // StackReserve
    opt[80..88].copy_from_slice(&0x1000u64.to_le_bytes()); // StackCommit
    opt[88..96].copy_from_slice(&0x100000u64.to_le_bytes()); // HeapReserve
    opt[96..104].copy_from_slice(&0x1000u64.to_le_bytes()); // HeapCommit
    opt[108..112].copy_from_slice(&16u32.to_le_bytes()); // NumberOfRvaAndSizes
    
    // Data Directory [1] Import Table
    // RVA: 0x2000, Size: 40 (IDT size)
    opt[120..124].copy_from_slice(&0x2000u32.to_le_bytes());
    opt[124..128].copy_from_slice(&40u32.to_le_bytes());
    
    file.write_all(&opt)?;
    
    // Section Headers
    // .text
    let mut sec_text = vec![0u8; 40];
    sec_text[0..5].copy_from_slice(b".text");
    sec_text[8..12].copy_from_slice(&code_virtual_size.to_le_bytes()); // VirtualSize
    sec_text[12..16].copy_from_slice(&0x1000u32.to_le_bytes()); // VirtualAddress
    sec_text[16..20].copy_from_slice(&code_raw_size.to_le_bytes()); // SizeOfRawData
    sec_text[20..24].copy_from_slice(&0x400u32.to_le_bytes()); // PointerToRawData (aligned to 0x200)
    sec_text[36..40].copy_from_slice(&0x60000020u32.to_le_bytes()); // Characteristics
    file.write_all(&sec_text)?;
    
    // .idata
    let mut sec_idata = vec![0u8; 40];
    sec_idata[0..6].copy_from_slice(b".idata");
    sec_idata[8..12].copy_from_slice(&idata_virtual_size.to_le_bytes()); // VirtualSize
    sec_idata[12..16].copy_from_slice(&0x2000u32.to_le_bytes()); // VirtualAddress
    sec_idata[16..20].copy_from_slice(&idata_raw_size.to_le_bytes()); // SizeOfRawData
    // PointerToRawData = PointerToRawData(.text) + SizeOfRawData(.text)
    let idata_ptr = 0x400 + code_raw_size;
    sec_idata[20..24].copy_from_slice(&idata_ptr.to_le_bytes());
    sec_idata[36..40].copy_from_slice(&0xC0000040u32.to_le_bytes()); // Characteristics (READ | WRITE | INITIALIZED_DATA)
    file.write_all(&sec_idata)?;
    
    // Padding to 0x400
    let headers_size = 64 + 4 + 20 + 240 + 40 + 40;
    let padding = 0x400 - headers_size;
    file.write_all(&vec![0u8; padding])?;
    
    // Write .text
    file.write_all(opcodes)?;
    let text_padding = code_raw_size - opcodes.len() as u32;
    file.write_all(&vec![0u8; text_padding as usize])?;
    
    // Write .idata
    file.write_all(&idata)?;
    
    Ok(())
}
