# PE Translator — ADead Universal Binary Backend

> **ADead-BIB → PE / ELF / FsOS**
> One format in, any platform out.

## Architecture

```
            ADead Language
                  ↓
              Compiler
                  ↓
            ADead-BIB (.bib)
                  ↓
        ┌─────────┼─────────┐
        ↓         ↓         ↓
   PE Backend  ELF Backend  FsOS Backend
        ↓         ↓         ↓
    .exe/.dll   ELF64     FastOS native
        ↓         ↓         ↓
    Windows    Linux      FastOS
```

## ADead-BIB Format v1.0

Universal binary intermediate format — independent of OS, executable format, and ISA.

```
Magic: 0x42494241 ("ABIB")

┌──────────────────────────┐
│  BibHeader (64 bytes)    │  magic, version, arch, section_count, entry_point
├──────────────────────────┤
│  SectionHeader[] (48 ea) │  name, type, flags, offset, size, alignment
├──────────────────────────┤
│  .code                   │  raw x86-64 machine code
│  .data                   │  initialized read/write data
│  .rodata                 │  read-only constants/strings
│  .bss                    │  uninitialized data
│  .import                 │  serialized import table
│  .symtab                 │  symbol table
│  .reloc                  │  relocation entries
│  .meta                   │  metadata (compiler, source)
└──────────────────────────┘
```

### Section Types

| Type     | ID     | Description                        |
|----------|--------|------------------------------------|
| Code     | 0x0001 | Raw machine code                   |
| Data     | 0x0002 | Initialized read/write data        |
| RoData   | 0x0003 | Read-only constants                |
| Bss      | 0x0004 | Uninitialized data (zero-filled)   |
| Import   | 0x0010 | Import table (DLL/SO symbols)      |
| Export   | 0x0011 | Export table                       |
| Symbol   | 0x0020 | Symbol table                       |
| Reloc    | 0x0030 | Relocation entries                 |
| Meta     | 0x0040 | Metadata key-value pairs           |
| Debug    | 0x0050 | Debug information                  |

### Architectures

| Arch     | ID     |
|----------|--------|
| x86-64   | 0x0001 |
| ARM64    | 0x0002 |
| RISC-V64 | 0x0003 |
| Wasm32   | 0x0010 |

## Backends

### PE Backend (Windows)
- Full PE32+ (64-bit) with DOS header, COFF, Optional Header
- Multi-DLL import tables (kernel32, user32, d3d12, msvcrt, etc.)
- Base relocations (.reloc) for ASLR
- Console (CUI) and GUI subsystems
- DEP/NX compatible
- Compatible with Windows 7+ x64 loader

### ELF Backend (Linux)
- ELF64 static executable
- Single LOAD segment (code)
- System V ABI, x86-64

### FsOS Backend (FastOS)
- Native FastOS executable format
- Magic: "FsOS" (0x534F7346)
- Minimal header (64 bytes) + section table + data
- Designed for direct loading by FastOS kernel

## Usage

### CLI

```bash
# Translate BIB to PE
pe-translator program.bib -o program.exe -t pe -s console

# Translate BIB to ELF
pe-translator program.bib -o program -t elf

# Translate BIB to FastOS native
pe-translator program.bib -o app.fsos -t fastos

# GUI application (DirectX 12)
pe-translator game.bib -o game.exe -t pe -s gui

# Show BIB module info
pe-translator program.bib --info

# Generate and translate a demo
pe-translator --demo -o demo.exe
```

### Options

| Flag            | Description                              | Default          |
|-----------------|------------------------------------------|------------------|
| `-o <path>`     | Output file path                         | `output.exe`     |
| `-t <target>`   | Target: `pe`, `dll`, `elf`, `so`, `fastos` | `pe`           |
| `-s <sub>`      | Subsystem: `console`, `gui`, `native`    | `console`        |
| `--base <hex>`  | Image base address                       | `0x140000000`    |
| `--info`        | Show BIB module info                     | —                |
| `--demo`        | Generate demo BIB and translate          | —                |

### Rust API

```rust
use pe_translator::bib::builder::BibBuilder;
use pe_translator::bib::format::Arch;
use pe_translator::targets::{self, BackendConfig};

// Build a BIB module
let module = BibBuilder::new(Arch::X86_64)
    .code(&machine_code)
    .rodata(&string_data)
    .function("main", 0, code_len)
    .import("kernel32.dll", "ExitProcess", 0)
    .import("d3d12.dll", "D3D12CreateDevice", 0)
    .entry("main")
    .meta("compiler", "ADead-BIB v1.0")
    .build();

// Translate to PE
let config = BackendConfig::windows_console();
let backend = targets::select_backend(config.format);
backend.write(&module, &config)?;
```

## Project Structure

```
PE Translator/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs              # CLI entry point
    ├── lib.rs               # Library root
    ├── bib/
    │   ├── mod.rs           # BIB module
    │   ├── format.rs        # ADead-BIB format specification (structs, enums)
    │   ├── builder.rs       # High-level BIB module builder API
    │   ├── writer.rs        # Serialize BibModule → .bib file
    │   └── reader.rs        # Deserialize .bib file → BibModule
    ├── targets/
    │   ├── mod.rs           # BinaryBackend trait + target registry
    │   ├── pe.rs            # Windows PE32+ generator
    │   ├── elf.rs           # Linux ELF64 generator
    │   └── fastos.rs        # FastOS FsOS generator
    └── runtime/
        └── mod.rs           # Runtime init stubs (Windows/Linux/FastOS)
```

## Pipeline Integration

```
ADead Source (.adB)
       ↓
   ADead Compiler (src/rust/)
       ↓  AST → IR → Optimizer → x86-64 bytes
       ↓
   ADead-BIB (.bib)
       ↓
   PE Translator
       ↓
   ┌────────────────────────────┐
   │  Windows: .exe / .dll     │
   │  Linux:   ELF executable  │
   │  FastOS:  .fsos native    │
   └────────────────────────────┘
```

## Build

```bash
cd "PE Translator"
cargo build --release
```

## Demo Output

```
=== ADead PE Translator — Demo Mode ===
=== ADead-BIB Module ===
  Arch:       x86-64
  Sections:   2
  Symbols:    5
  Imports:    2 modules (4 symbols)

  BIB saved: demo.bib (640 bytes)
  Translating to PE (Windows) ...
  Output: demo.exe (2560 bytes)
  Demo complete!
```

## License

Part of ADead-BIB — Binary Is Binary.
