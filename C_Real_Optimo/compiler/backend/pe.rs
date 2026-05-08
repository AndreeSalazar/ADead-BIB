//! PE Executable Writer for Windows
//! DOS header, PE header, sections .text .data .rdata .idata
//! Generado automáticamente
#![allow(dead_code)]

// ============== PE CONSTANTS ==============

const DOS_SIGNATURE: u16 = 0x5A4D;      // "MZ"
const PE_SIGNATURE: u32 = 0x00004550;   // "PE\0\0"
const IMAGE_FILE_MACHINE_AMD64: u16 = 0x8664;
const IMAGE_FILE_EXECUTABLE_IMAGE: u16 = 0x0002;
const IMAGE_FILE_LARGE_ADDRESS_AWARE: u16 = 0x0020;

const IMAGE_SUBSYSTEM_CONSOLE: u16 = 3;
const IMAGE_SUBSYSTEM_WINDOWS_GUI: u16 = 2;

const IMAGE_SCN_CNT_CODE: u32 = 0x00000020;
const IMAGE_SCN_CNT_INITIALIZED_DATA: u32 = 0x00000040;
const IMAGE_SCN_MEM_EXECUTE: u32 = 0x20000000;
const IMAGE_SCN_MEM_READ: u32 = 0x40000000;
const IMAGE_SCN_MEM_WRITE: u32 = 0x80000000;

const SECTION_ALIGNMENT: u32 = 0x1000;
const FILE_ALIGNMENT: u32 = 0x200;
const IMAGE_BASE: u64 = 0x140000000;

// ============== PE BUILDER ==============

pub struct PeBuilder {
    pub code: Vec<u8>,
    pub data: Vec<u8>,
    pub rdata: Vec<u8>,
    pub imports: Vec<ImportEntry>,
    pub subsystem: u16,
    pub entry_rva: u32,
}

#[derive(Clone)]
pub struct ImportEntry {
    pub dll: String,
    pub functions: Vec<String>,
}

impl PeBuilder {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            data: Vec::new(),
            rdata: Vec::new(),
            imports: Vec::new(),
            subsystem: IMAGE_SUBSYSTEM_CONSOLE,
            entry_rva: 0,
        }
    }
    
    pub fn console(mut self) -> Self {
        self.subsystem = IMAGE_SUBSYSTEM_CONSOLE;
        self
    }
    
    pub fn gui(mut self) -> Self {
        self.subsystem = IMAGE_SUBSYSTEM_WINDOWS_GUI;
        self
    }
    
    pub fn add_import(&mut self, dll: &str, functions: Vec<&str>) {
        self.imports.push(ImportEntry {
            dll: dll.to_string(),
            functions: functions.iter().map(|s| s.to_string()).collect(),
        });
    }
    
    pub fn build(&self) -> Vec<u8> {
        let mut pe = Vec::new();
        
        // DOS Header (64 bytes)
        self.write_dos_header(&mut pe);
        
        // DOS Stub (optional, skip to PE)
        
        // PE Signature
        pe.extend_from_slice(&PE_SIGNATURE.to_le_bytes());
        
        // COFF File Header (20 bytes)
        let num_sections = self.count_sections();
        self.write_coff_header(&mut pe, num_sections);
        
        // Optional Header (PE32+, 112 bytes + data directories)
        self.write_optional_header(&mut pe, num_sections);
        
        // Section Headers
        self.write_section_headers(&mut pe, num_sections);
        
        // Pad to file alignment
        align_to(&mut pe, FILE_ALIGNMENT as usize);
        
        // .text section
        if !self.code.is_empty() {
            pe.extend_from_slice(&self.code);
            align_to(&mut pe, FILE_ALIGNMENT as usize);
        }
        
        // .data section
        if !self.data.is_empty() {
            pe.extend_from_slice(&self.data);
            align_to(&mut pe, FILE_ALIGNMENT as usize);
        }
        
        // .rdata section
        if !self.rdata.is_empty() {
            pe.extend_from_slice(&self.rdata);
            align_to(&mut pe, FILE_ALIGNMENT as usize);
        }
        
        // .idata section (imports)
        if !self.imports.is_empty() {
            self.write_idata(&mut pe);
            align_to(&mut pe, FILE_ALIGNMENT as usize);
        }
        
        pe
    }
    
    fn write_dos_header(&self, pe: &mut Vec<u8>) {
        // DOS Header - minimal stub
        pe.extend_from_slice(&DOS_SIGNATURE.to_le_bytes()); // e_magic
        pe.extend_from_slice(&[0u8; 58]); // padding
        pe.extend_from_slice(&0x80u32.to_le_bytes()); // e_lfanew (offset to PE header)
        
        // Pad to 0x80
        while pe.len() < 0x80 {
            pe.push(0);
        }
    }
    
    fn write_coff_header(&self, pe: &mut Vec<u8>, num_sections: u16) {
        pe.extend_from_slice(&IMAGE_FILE_MACHINE_AMD64.to_le_bytes());
        pe.extend_from_slice(&num_sections.to_le_bytes());
        pe.extend_from_slice(&0u32.to_le_bytes()); // TimeDateStamp
        pe.extend_from_slice(&0u32.to_le_bytes()); // PointerToSymbolTable
        pe.extend_from_slice(&0u32.to_le_bytes()); // NumberOfSymbols
        pe.extend_from_slice(&240u16.to_le_bytes()); // SizeOfOptionalHeader (PE32+)
        let characteristics = IMAGE_FILE_EXECUTABLE_IMAGE | IMAGE_FILE_LARGE_ADDRESS_AWARE;
        pe.extend_from_slice(&characteristics.to_le_bytes());
    }
    
    fn write_optional_header(&self, pe: &mut Vec<u8>, num_sections: u16) {
        let headers_size = 0x80 + 4 + 20 + 240 + (num_sections as usize * 40);
        let headers_size_aligned = align_up(headers_size, FILE_ALIGNMENT as usize);
        
        let text_rva = SECTION_ALIGNMENT;
        let text_size = align_up(self.code.len(), SECTION_ALIGNMENT as usize);
        let data_size = align_up(self.data.len(), SECTION_ALIGNMENT as usize);
        let rdata_size = align_up(self.rdata.len(), SECTION_ALIGNMENT as usize);
        
        // B-06: SizeOfImage debe cubrir todas las secciones (incluida .data),
        // no sólo .text. Sin esto, el loader rechaza el binario o trunca .data.
        let image_size = headers_size_aligned + text_size + data_size + rdata_size;
        let image_size_aligned = align_up(image_size, SECTION_ALIGNMENT as usize);
        
        // PE32+ Magic
        pe.extend_from_slice(&0x20Bu16.to_le_bytes());
        pe.push(14); // MajorLinkerVersion
        pe.push(0);  // MinorLinkerVersion
        pe.extend_from_slice(&(self.code.len() as u32).to_le_bytes()); // SizeOfCode
        pe.extend_from_slice(&(self.data.len() as u32).to_le_bytes()); // SizeOfInitializedData
        pe.extend_from_slice(&0u32.to_le_bytes()); // SizeOfUninitializedData
        let entry_point = text_rva + self.entry_rva;
        pe.extend_from_slice(&entry_point.to_le_bytes()); // AddressOfEntryPoint
        pe.extend_from_slice(&text_rva.to_le_bytes()); // BaseOfCode
        pe.extend_from_slice(&IMAGE_BASE.to_le_bytes()); // ImageBase
        pe.extend_from_slice(&SECTION_ALIGNMENT.to_le_bytes());
        pe.extend_from_slice(&FILE_ALIGNMENT.to_le_bytes());
        pe.extend_from_slice(&6u16.to_le_bytes()); // MajorOperatingSystemVersion
        pe.extend_from_slice(&0u16.to_le_bytes()); // MinorOperatingSystemVersion
        pe.extend_from_slice(&0u16.to_le_bytes()); // MajorImageVersion
        pe.extend_from_slice(&0u16.to_le_bytes()); // MinorImageVersion
        pe.extend_from_slice(&6u16.to_le_bytes()); // MajorSubsystemVersion
        pe.extend_from_slice(&0u16.to_le_bytes()); // MinorSubsystemVersion
        pe.extend_from_slice(&0u32.to_le_bytes()); // Win32VersionValue
        pe.extend_from_slice(&(image_size_aligned as u32).to_le_bytes()); // SizeOfImage
        pe.extend_from_slice(&(headers_size_aligned as u32).to_le_bytes()); // SizeOfHeaders
        pe.extend_from_slice(&0u32.to_le_bytes()); // CheckSum
        pe.extend_from_slice(&self.subsystem.to_le_bytes());
        pe.extend_from_slice(&0u16.to_le_bytes()); // DllCharacteristics
        pe.extend_from_slice(&0x100000u64.to_le_bytes()); // SizeOfStackReserve
        pe.extend_from_slice(&0x1000u64.to_le_bytes()); // SizeOfStackCommit
        pe.extend_from_slice(&0x100000u64.to_le_bytes()); // SizeOfHeapReserve
        pe.extend_from_slice(&0x1000u64.to_le_bytes()); // SizeOfHeapCommit
        pe.extend_from_slice(&0u32.to_le_bytes()); // LoaderFlags
        pe.extend_from_slice(&16u32.to_le_bytes()); // NumberOfRvaAndSizes
        
        // Data Directories (16 * 8 = 128 bytes)
        for _ in 0..16 {
            pe.extend_from_slice(&0u32.to_le_bytes()); // VirtualAddress
            pe.extend_from_slice(&0u32.to_le_bytes()); // Size
        }
    }
    
    fn write_section_headers(&self, pe: &mut Vec<u8>, _num_sections: u16) {
        let mut file_offset = 0x200u32; // After headers
        let mut rva = SECTION_ALIGNMENT;
        
        if !self.code.is_empty() {
            self.write_section(pe, b".text\0\0\0", rva, self.code.len() as u32, file_offset,
                IMAGE_SCN_CNT_CODE | IMAGE_SCN_MEM_EXECUTE | IMAGE_SCN_MEM_READ);
            file_offset += align_up(self.code.len(), FILE_ALIGNMENT as usize) as u32;
            rva += align_up(self.code.len(), SECTION_ALIGNMENT as usize) as u32;
        }
        
        if !self.data.is_empty() {
            self.write_section(pe, b".data\0\0\0", rva, self.data.len() as u32, file_offset,
                IMAGE_SCN_CNT_INITIALIZED_DATA | IMAGE_SCN_MEM_READ | IMAGE_SCN_MEM_WRITE);
        }
    }
    
    fn write_section(&self, pe: &mut Vec<u8>, name: &[u8], rva: u32, size: u32, 
                     file_offset: u32, characteristics: u32) {
        pe.extend_from_slice(&name[..8]); // Name (8 bytes)
        pe.extend_from_slice(&size.to_le_bytes()); // VirtualSize
        pe.extend_from_slice(&rva.to_le_bytes()); // VirtualAddress
        pe.extend_from_slice(&(align_up(size as usize, FILE_ALIGNMENT as usize) as u32).to_le_bytes()); // SizeOfRawData
        pe.extend_from_slice(&file_offset.to_le_bytes()); // PointerToRawData
        pe.extend_from_slice(&0u32.to_le_bytes()); // PointerToRelocations
        pe.extend_from_slice(&0u32.to_le_bytes()); // PointerToLinenumbers
        pe.extend_from_slice(&0u16.to_le_bytes()); // NumberOfRelocations
        pe.extend_from_slice(&0u16.to_le_bytes()); // NumberOfLinenumbers
        pe.extend_from_slice(&characteristics.to_le_bytes());
    }
    
    fn write_idata(&self, pe: &mut Vec<u8>) {
        // Import Directory Table
        for _import in &self.imports {
            // Import Directory Entry (20 bytes)
            pe.extend_from_slice(&0u32.to_le_bytes()); // OriginalFirstThunk
            pe.extend_from_slice(&0u32.to_le_bytes()); // TimeDateStamp
            pe.extend_from_slice(&0u32.to_le_bytes()); // ForwarderChain
            pe.extend_from_slice(&0u32.to_le_bytes()); // Name RVA
            pe.extend_from_slice(&0u32.to_le_bytes()); // FirstThunk
        }
        // Null terminator
        pe.extend_from_slice(&[0u8; 20]);
    }
    
    fn count_sections(&self) -> u16 {
        let mut count = 0;
        if !self.code.is_empty() { count += 1; }
        if !self.data.is_empty() { count += 1; }
        if !self.rdata.is_empty() { count += 1; }
        if !self.imports.is_empty() { count += 1; }
        count.max(1)
    }
}

impl Default for PeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

fn align_to(data: &mut Vec<u8>, alignment: usize) {
    while data.len() % alignment != 0 {
        data.push(0);
    }
}

fn align_up(size: usize, alignment: usize) -> usize {
    (size + alignment - 1) & !(alignment - 1)
}

trait ToLeBytes<const N: usize> {
    fn to_le_bytes(self) -> [u8; N];
}

impl ToLeBytes<4> for usize {
    fn to_le_bytes(self) -> [u8; 4] {
        (self as u32).to_le_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pe_builder() {
        let mut builder = PeBuilder::new().console();
        builder.code = vec![0xC3]; // ret
        let pe = builder.build();
        
        assert_eq!(&pe[0..2], &[0x4D, 0x5A]); // MZ
    }
}
