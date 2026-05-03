//! ELF Executable Writer for Linux
//! ELF64 header, program headers, sections
//! Generado automáticamente
#![allow(dead_code)]

// ============== ELF CONSTANTS ==============

const ELF_MAGIC: [u8; 4] = [0x7F, b'E', b'L', b'F'];
const ELFCLASS64: u8 = 2;
const ELFDATA2LSB: u8 = 1;  // Little endian
const EV_CURRENT: u8 = 1;
const ELFOSABI_NONE: u8 = 0;

const ET_EXEC: u16 = 2;     // Executable
const ET_DYN: u16 = 3;      // Shared object (PIE)
const EM_X86_64: u16 = 62;

const PT_NULL: u32 = 0;
const PT_LOAD: u32 = 1;
const PT_DYNAMIC: u32 = 2;
const PT_INTERP: u32 = 3;
const PT_PHDR: u32 = 6;

const PF_X: u32 = 1;  // Execute
const PF_W: u32 = 2;  // Write
const PF_R: u32 = 4;  // Read

const SHT_NULL: u32 = 0;
const SHT_PROGBITS: u32 = 1;
const SHT_STRTAB: u32 = 3;

const SHF_WRITE: u64 = 1;
const SHF_ALLOC: u64 = 2;
const SHF_EXECINSTR: u64 = 4;

const PAGE_SIZE: u64 = 0x1000;
const BASE_ADDR: u64 = 0x400000;

// ============== ELF BUILDER ==============

pub struct ElfBuilder {
    pub code: Vec<u8>,
    pub data: Vec<u8>,
    pub rodata: Vec<u8>,
    pub entry_offset: u64,
    pub is_pie: bool,
}

impl ElfBuilder {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            data: Vec::new(),
            rodata: Vec::new(),
            entry_offset: 0,
            is_pie: false,
        }
    }
    
    pub fn pie(mut self) -> Self {
        self.is_pie = true;
        self
    }
    
    pub fn build(&self) -> Vec<u8> {
        let mut elf = Vec::new();
        
        // ELF Header (64 bytes)
        self.write_elf_header(&mut elf);
        
        // Program Headers
        let phdr_offset = elf.len() as u64;
        self.write_program_headers(&mut elf);
        
        // Align to page
        align_to(&mut elf, PAGE_SIZE as usize);
        
        // .text section
        let text_offset = elf.len();
        elf.extend_from_slice(&self.code);
        
        // .data section (if any)
        if !self.data.is_empty() {
            align_to(&mut elf, PAGE_SIZE as usize);
            elf.extend_from_slice(&self.data);
        }
        
        elf
    }
    
    fn write_elf_header(&self, elf: &mut Vec<u8>) {
        // e_ident (16 bytes)
        elf.extend_from_slice(&ELF_MAGIC);
        elf.push(ELFCLASS64);
        elf.push(ELFDATA2LSB);
        elf.push(EV_CURRENT);
        elf.push(ELFOSABI_NONE);
        elf.extend_from_slice(&[0u8; 8]); // padding
        
        // e_type
        let etype = if self.is_pie { ET_DYN } else { ET_EXEC };
        elf.extend_from_slice(&etype.to_le_bytes());
        
        // e_machine
        elf.extend_from_slice(&EM_X86_64.to_le_bytes());
        
        // e_version
        elf.extend_from_slice(&1u32.to_le_bytes());
        
        // e_entry
        let entry = BASE_ADDR + PAGE_SIZE + self.entry_offset;
        elf.extend_from_slice(&entry.to_le_bytes());
        
        // e_phoff (program header offset)
        elf.extend_from_slice(&64u64.to_le_bytes());
        
        // e_shoff (section header offset - 0 for minimal)
        elf.extend_from_slice(&0u64.to_le_bytes());
        
        // e_flags
        elf.extend_from_slice(&0u32.to_le_bytes());
        
        // e_ehsize
        elf.extend_from_slice(&64u16.to_le_bytes());
        
        // e_phentsize
        elf.extend_from_slice(&56u16.to_le_bytes());
        
        // e_phnum
        let phnum = self.count_segments();
        elf.extend_from_slice(&phnum.to_le_bytes());
        
        // e_shentsize
        elf.extend_from_slice(&64u16.to_le_bytes());
        
        // e_shnum
        elf.extend_from_slice(&0u16.to_le_bytes());
        
        // e_shstrndx
        elf.extend_from_slice(&0u16.to_le_bytes());
    }
    
    fn write_program_headers(&self, elf: &mut Vec<u8>) {
        // PT_LOAD for .text
        self.write_phdr(elf, PT_LOAD, PF_R | PF_X,
            PAGE_SIZE, BASE_ADDR + PAGE_SIZE, 
            self.code.len() as u64, self.code.len() as u64,
            PAGE_SIZE);
        
        // PT_LOAD for .data (if any)
        if !self.data.is_empty() {
            let data_offset = align_up(PAGE_SIZE + self.code.len() as u64, PAGE_SIZE);
            let data_vaddr = BASE_ADDR + data_offset;
            self.write_phdr(elf, PT_LOAD, PF_R | PF_W,
                data_offset, data_vaddr,
                self.data.len() as u64, self.data.len() as u64,
                PAGE_SIZE);
        }
    }
    
    fn write_phdr(&self, elf: &mut Vec<u8>, p_type: u32, p_flags: u32,
                  p_offset: u64, p_vaddr: u64, p_filesz: u64, p_memsz: u64, p_align: u64) {
        elf.extend_from_slice(&p_type.to_le_bytes());
        elf.extend_from_slice(&p_flags.to_le_bytes());
        elf.extend_from_slice(&p_offset.to_le_bytes());
        elf.extend_from_slice(&p_vaddr.to_le_bytes());
        elf.extend_from_slice(&p_vaddr.to_le_bytes()); // p_paddr = p_vaddr
        elf.extend_from_slice(&p_filesz.to_le_bytes());
        elf.extend_from_slice(&p_memsz.to_le_bytes());
        elf.extend_from_slice(&p_align.to_le_bytes());
    }
    
    fn count_segments(&self) -> u16 {
        let mut count = 1; // .text
        if !self.data.is_empty() { count += 1; }
        count
    }
}

impl Default for ElfBuilder {
    fn default() -> Self {
        Self::new()
    }
}

fn align_to(data: &mut Vec<u8>, alignment: usize) {
    while data.len() % alignment != 0 {
        data.push(0);
    }
}

fn align_up(size: u64, alignment: u64) -> u64 {
    (size + alignment - 1) & !(alignment - 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_elf_builder() {
        let mut builder = ElfBuilder::new();
        builder.code = vec![
            0xB8, 0x3C, 0x00, 0x00, 0x00,  // mov eax, 60 (exit syscall)
            0x31, 0xFF,                     // xor edi, edi
            0x0F, 0x05,                     // syscall
        ];
        let elf = builder.build();
        
        assert_eq!(&elf[0..4], &ELF_MAGIC);
    }
}
