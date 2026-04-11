pub mod isa;

pub mod frontend {
    pub mod ast {
        pub use adeb_core::ast::*;
        pub use adeb_core::types::{RegSize, Type};
    }
}

pub mod backend {
    pub mod cpu {
        pub mod iat_registry {
            use std::collections::HashMap;

            // ── Multi-DLL IAT Registry v5 ─────────────────────────
            // 13 DLLs, 200+ slots: msvcrt, kernel32, user32, gdi32,
            // opengl32, ole32, dxgi, d3d9, d3d11, d3d12,
            // d3dcompiler_47, ws2_32

            pub struct DllImport {
                pub dll: &'static str,
                pub functions: &'static [&'static str],
            }

            pub static DLL_IMPORTS: &[DllImport] = &[
                // ── Phase 1: C Runtime (msvcrt.dll) — 94 functions ──
                DllImport { dll: "msvcrt.dll", functions: &[
                    // stdio — core
                    "printf", "fprintf", "sprintf", "_snprintf", "scanf", "sscanf",
                    "puts", "putchar", "getchar", "fgets", "fputs",
                    "fopen", "fclose", "fread", "fwrite", "fseek", "ftell", "rewind",
                    "feof", "ferror", "fflush", "perror",
                    // stdio — extras
                    "vprintf", "vfprintf", "vsprintf",
                    "setbuf", "setvbuf", "ungetc", "freopen",
                    "remove", "rename", "tmpfile", "tmpnam",
                    "fgetc", "fputc", "clearerr",
                    // stdlib — core
                    "malloc", "calloc", "realloc", "free",
                    "atoi", "atof", "atol", "strtol", "strtoul", "strtod",
                    "abs", "rand", "srand", "qsort", "bsearch",
                    "exit", "getenv", "system",
                    // stdlib — extras
                    "abort", "_exit", "atexit",
                    "strtoll", "strtoull", "strtof",
                    "labs", "llabs",
                    // string — core
                    "memset", "memcpy", "memmove", "memcmp",
                    "strlen", "strcpy", "strncpy", "strcat", "strncat",
                    "strcmp", "strncmp", "strchr", "strrchr", "strstr", "strtok",
                    // string — extras
                    "memchr", "strerror", "strpbrk", "strspn", "strcspn",
                    "strcoll", "strxfrm",
                    // math (msvcrt.dll exports)
                    "sin", "cos", "tan", "asin", "acos", "atan", "atan2",
                    "exp", "log", "log10", "pow", "sqrt",
                    "ceil", "floor", "fabs", "fmod",
                    "sinh", "cosh", "tanh",
                    "ldexp", "frexp", "modf",
                    // time — core + extras
                    "time", "clock", "difftime", "strftime",
                    "mktime", "localtime", "gmtime", "asctime", "ctime",
                    // signal
                    "signal", "raise",
                    // locale
                    "setlocale", "localeconv",
                    // errno
                    "_errno",
                ] },
                // ── Phase 2: Win32 Core — kernel32.dll (30 functions) ──
                DllImport { dll: "kernel32.dll", functions: &[
                    // Process & module
                    "GetModuleHandleA", "GetModuleHandleW",
                    "LoadLibraryA", "LoadLibraryW", "FreeLibrary",
                    "GetProcAddress", "ExitProcess", "GetLastError",
                    "GetCurrentProcess", "GetCurrentProcessId",
                    "GetCurrentThread", "GetCurrentThreadId",
                    // Memory
                    "VirtualAlloc", "VirtualFree", "VirtualProtect",
                    "HeapCreate", "HeapDestroy", "HeapAlloc", "HeapFree",
                    // File I/O
                    "CreateFileA", "ReadFile", "WriteFile", "CloseHandle",
                    "GetFileSize", "SetFilePointer",
                    // Sync & timing
                    "Sleep", "GetTickCount", "QueryPerformanceCounter", "QueryPerformanceFrequency",
                    // Console
                    "GetStdHandle", "WriteConsoleA",
                    // Thread
                    "CreateThread", "WaitForSingleObject",
                    // Environment
                    "GetEnvironmentVariableA", "SetEnvironmentVariableA",
                    "GetCommandLineA", "GetSystemInfo",
                    // Debug
                    "OutputDebugStringA", "IsDebuggerPresent",
                ] },
                // ── Phase 2: Win32 UI — user32.dll (40 functions) ──
                DllImport { dll: "user32.dll", functions: &[
                    // Window class & creation
                    "RegisterClassA", "RegisterClassExA", "UnregisterClassA",
                    "CreateWindowExA", "DestroyWindow",
                    "ShowWindow", "UpdateWindow", "InvalidateRect",
                    "MoveWindow", "SetWindowPos", "AdjustWindowRect",
                    "SetWindowTextA", "GetWindowTextA",
                    "GetClientRect", "GetWindowRect",
                    // Message loop
                    "PeekMessageA", "GetMessageA", "TranslateMessage", "DispatchMessageA",
                    "PostQuitMessage", "PostMessageA", "SendMessageA",
                    "DefWindowProcA",
                    // DC & painting
                    "GetDC", "ReleaseDC", "BeginPaint", "EndPaint",
                    // Dialog & message box
                    "MessageBoxA", "DialogBoxParamA",
                    // Input
                    "GetKeyState", "GetAsyncKeyState",
                    "GetCursorPos", "SetCursorPos", "ShowCursor",
                    "SetCapture", "ReleaseCapture",
                    // Timer
                    "SetTimer", "KillTimer",
                    // Resources
                    "LoadCursorA", "LoadIconA",
                    // System
                    "GetSystemMetrics", "GetDesktopWindow",
                ] },
                // ── Phase 2: Win32 GDI — gdi32.dll (30 functions) ──
                DllImport { dll: "gdi32.dll", functions: &[
                    // Pixel format (OpenGL)
                    "SwapBuffers", "ChoosePixelFormat", "SetPixelFormat",
                    "DescribePixelFormat",
                    // Pixel & shape drawing
                    "SetPixel", "GetPixel",
                    "Rectangle", "Ellipse", "Polygon",
                    "MoveToEx", "LineTo",
                    // Brush & pen
                    "CreateSolidBrush", "CreatePen",
                    "SelectObject", "DeleteObject", "GetStockObject",
                    // Bitmap & DC
                    "CreateCompatibleDC", "CreateCompatibleBitmap",
                    "BitBlt", "StretchBlt",
                    "DeleteDC",
                    // Text
                    "TextOutA", "CreateFontA", "SetTextColor", "SetBkColor",
                    "SetBkMode", "GetTextExtentPoint32A",
                    // Color
                    "SetDCBrushColor", "SetDCPenColor",
                    // Region
                    "CreateRectRgn",
                ] },
                // ── Phase 2: OpenGL 1.1 — opengl32.dll (31 functions) ──
                DllImport { dll: "opengl32.dll", functions: &[
                    "wglCreateContext", "wglMakeCurrent", "wglDeleteContext",
                    "wglGetProcAddress",
                    "glClear", "glClearColor", "glEnable", "glDisable",
                    "glDepthFunc", "glShadeModel", "glViewport",
                    "glMatrixMode", "glLoadIdentity",
                    "glTranslatef", "glRotatef", "glScalef", "glFrustum",
                    "glBegin", "glEnd",
                    "glVertex3f", "glColor3f", "glColor4f",
                    "glNormal3f", "glLightfv", "glMaterialfv", "glMaterialf",
                    "glColorMaterial", "glFlush",
                    "glGetString", "glGetError",
                ] },
                // ── Phase 3: COM — ole32.dll (6 functions) ──
                DllImport { dll: "ole32.dll", functions: &[
                    "CoInitialize", "CoInitializeEx", "CoUninitialize",
                    "CoCreateInstance", "CoTaskMemAlloc", "CoTaskMemFree",
                ] },
                // ── Phase 3: DXGI — dxgi.dll (3 functions) ──
                DllImport { dll: "dxgi.dll", functions: &[
                    "CreateDXGIFactory", "CreateDXGIFactory1", "CreateDXGIFactory2",
                ] },
                // ── Phase 3: DirectX 9 — d3d9.dll (2 functions) ──
                DllImport { dll: "d3d9.dll", functions: &[
                    "Direct3DCreate9", "Direct3DCreate9Ex",
                ] },
                // ── Phase 3: DirectX 11 — d3d11.dll (2 functions) ──
                DllImport { dll: "d3d11.dll", functions: &[
                    "D3D11CreateDevice", "D3D11CreateDeviceAndSwapChain",
                ] },
                // ── Phase 3: DirectX 12 — d3d12.dll (4 functions) ──
                DllImport { dll: "d3d12.dll", functions: &[
                    "D3D12CreateDevice", "D3D12GetDebugInterface",
                    "D3D12SerializeRootSignature", "D3D12SerializeVersionedRootSignature",
                ] },
                // ── Phase 3: HLSL Compiler — d3dcompiler_47.dll (4 functions) ──
                DllImport { dll: "d3dcompiler_47.dll", functions: &[
                    "D3DCompile", "D3DCompile2",
                    "D3DCompileFromFile", "D3DReflect",
                ] },
                // ── Phase 4: Networking — ws2_32.dll (20 functions) ──
                DllImport { dll: "ws2_32.dll", functions: &[
                    "WSAStartup", "WSACleanup", "WSAGetLastError",
                    "socket", "closesocket", "bind", "listen", "accept",
                    "connect", "send", "recv", "sendto", "recvfrom",
                    "select", "shutdown",
                    "htons", "htonl", "ntohs", "ntohl",
                    "inet_addr",
                ] },
            ];

            // Legacy compat: flat list of all functions across all DLLs
            pub const IAT_DLL: &str = "msvcrt.dll";
            pub const IAT_ENTRIES: [&str; 4] = ["printf", "scanf", "malloc", "free"];

            fn all_functions() -> Vec<&'static str> {
                let mut v = Vec::new();
                for dll in DLL_IMPORTS {
                    for f in dll.functions { v.push(*f); }
                }
                v
            }

            fn total_function_count() -> usize {
                DLL_IMPORTS.iter().map(|d| d.functions.len()).sum()
            }

            #[derive(Debug, Clone)]
            pub struct IdataBuildResult {
                pub bytes: Vec<u8>,
                pub slot_to_iat_rva: Vec<u32>,
                pub import_dir_rva: u32,
                pub import_dir_size: u32,
                pub iat_rva: u32,
                pub iat_size: u32,
                pub program_strings_offset: u32,
            }

            pub fn slot_for_function(name: &str) -> Option<usize> {
                let mut idx = 0;
                for dll in DLL_IMPORTS {
                    for f in dll.functions {
                        if *f == name { return Some(idx); }
                        idx += 1;
                    }
                }
                None
            }

            fn align_up(value: usize, align: usize) -> usize {
                if align == 0 { return value; }
                (value + (align - 1)) & !(align - 1)
            }

            fn push_u32_to(buf: &mut [u8], off: usize, v: u32) {
                buf[off..off+4].copy_from_slice(&v.to_le_bytes());
            }

            fn push_u64_to(buf: &mut [u8], off: usize, v: u64) {
                buf[off..off+8].copy_from_slice(&v.to_le_bytes());
            }

            /// Legacy: build idata importing ALL DLLs (used by IsaCompiler::new for layout calculation)
            pub fn build_idata(idata_rva: u32, _extra_imports: &[&str]) -> IdataBuildResult {
                build_idata_filtered(idata_rva, &std::collections::HashSet::new())
            }

            /// Build idata section, only importing DLLs that have at least one used slot.
            /// If used_slots is empty, ALL DLLs are imported (legacy behavior).
            pub fn build_idata_filtered(idata_rva: u32, used_slots: &std::collections::HashSet<usize>) -> IdataBuildResult {
                let num_dlls = DLL_IMPORTS.len();
                let total_funcs = total_function_count();

                // Determine which DLLs are actually needed
                let mut dll_is_used = vec![used_slots.is_empty(); num_dlls];
                if !used_slots.is_empty() {
                    let mut slot_idx = 0usize;
                    for (di, dll) in DLL_IMPORTS.iter().enumerate() {
                        for _ in dll.functions {
                            if used_slots.contains(&slot_idx) {
                                dll_is_used[di] = true;
                            }
                            slot_idx += 1;
                        }
                    }
                }

                // IMPORTANT: Always use full num_dlls for directory size so that
                // OFT/IAT offsets remain identical to the layout computed at compile time.
                // Unused DLLs simply get zeroed descriptors (= null terminator for the loader).
                let import_desc_offset = 0usize;
                let import_desc_size = (num_dlls + 1) * 20;
                let mut cursor = import_desc_size;
                cursor = align_up(cursor, 8);

                // 2. Per-DLL OFT (Original First Thunk) arrays
                let mut dll_oft_offsets = Vec::with_capacity(num_dlls);
                for dll in DLL_IMPORTS {
                    dll_oft_offsets.push(cursor);
                    cursor += (dll.functions.len() + 1) * 8; // +1 for null terminator
                    cursor = align_up(cursor, 8);
                }

                // 3. IAT (First Thunk) — single contiguous array for all functions
                let iat_offset = cursor;
                let mut dll_iat_offsets = Vec::with_capacity(num_dlls);
                for dll in DLL_IMPORTS {
                    dll_iat_offsets.push(cursor);
                    cursor += (dll.functions.len() + 1) * 8;
                    cursor = align_up(cursor, 8);
                }

                // 4. Hint/Name entries for each function
                cursor = align_up(cursor, 2);
                let mut hint_name_offsets: Vec<u32> = Vec::with_capacity(total_funcs);
                for dll in DLL_IMPORTS {
                    for f in dll.functions {
                        hint_name_offsets.push(cursor as u32);
                        cursor += 2; // hint (u16)
                        cursor += f.as_bytes().len() + 1; // name + null
                        cursor = align_up(cursor, 2);
                    }
                }

                // 5. DLL name strings
                let mut dll_name_offsets = Vec::with_capacity(num_dlls);
                for dll in DLL_IMPORTS {
                    dll_name_offsets.push(cursor);
                    cursor += dll.dll.as_bytes().len() + 1;
                    cursor = align_up(cursor, 2);
                }

                cursor = align_up(cursor, 8);
                let program_strings_offset = cursor as u32;

                // Build the byte buffer
                let mut bytes = vec![0u8; program_strings_offset as usize];

                let import_dir_rva = idata_rva;
                let import_dir_size = import_desc_size as u32;
                let iat_rva = idata_rva + iat_offset as u32;

                // Build slot_to_iat_rva: flat index across all DLLs
                let mut slot_to_iat_rva = Vec::with_capacity(total_funcs);
                for (di, dll) in DLL_IMPORTS.iter().enumerate() {
                    for fi in 0..dll.functions.len() {
                        slot_to_iat_rva.push(idata_rva + dll_iat_offsets[di] as u32 + (fi as u32 * 8));
                    }
                }

                // Write import descriptors — compact used DLLs to the front
                // (zeroed entries in the middle would be treated as null terminator by the PE loader)
                let mut global_func_idx = 0usize;
                let mut desc_idx = 0usize;
                for (di, dll) in DLL_IMPORTS.iter().enumerate() {
                    if dll_is_used[di] {
                        let desc_off = import_desc_offset + desc_idx * 20;
                        let oft_rva = idata_rva + dll_oft_offsets[di] as u32;
                        let dll_name_rva = idata_rva + dll_name_offsets[di] as u32;
                        let first_thunk_rva = idata_rva + dll_iat_offsets[di] as u32;

                        push_u32_to(&mut bytes, desc_off + 0, oft_rva);       // OriginalFirstThunk
                        push_u32_to(&mut bytes, desc_off + 4, 0);             // TimeDateStamp
                        push_u32_to(&mut bytes, desc_off + 8, 0);             // ForwarderChain
                        push_u32_to(&mut bytes, desc_off + 12, dll_name_rva); // Name
                        push_u32_to(&mut bytes, desc_off + 16, first_thunk_rva); // FirstThunk
                        desc_idx += 1;
                    }

                    // Always write OFT + IAT entries for ALL DLLs (keeps slot RVAs stable)
                    for fi in 0..dll.functions.len() {
                        let hn_rva = idata_rva + hint_name_offsets[global_func_idx];
                        let entry = hn_rva as u64;
                        let oft_entry_off = dll_oft_offsets[di] + fi * 8;
                        let iat_entry_off = dll_iat_offsets[di] + fi * 8;
                        push_u64_to(&mut bytes, oft_entry_off, entry);
                        push_u64_to(&mut bytes, iat_entry_off, entry);
                        global_func_idx += 1;
                    }
                    // Null terminators
                    let oft_null = dll_oft_offsets[di] + dll.functions.len() * 8;
                    let iat_null = dll_iat_offsets[di] + dll.functions.len() * 8;
                    push_u64_to(&mut bytes, oft_null, 0);
                    push_u64_to(&mut bytes, iat_null, 0);
                }
                // Null-terminated import descriptor
                // Already zeroed from vec![0u8; ...]

                // Write Hint/Name entries
                global_func_idx = 0;
                for dll in DLL_IMPORTS {
                    for f in dll.functions {
                        let off = hint_name_offsets[global_func_idx] as usize;
                        // hint = 0
                        bytes[off] = 0; bytes[off+1] = 0;
                        let name_bytes = f.as_bytes();
                        bytes[off+2..off+2+name_bytes.len()].copy_from_slice(name_bytes);
                        bytes[off+2+name_bytes.len()] = 0;
                        global_func_idx += 1;
                    }
                }

                // Write DLL name strings
                for (di, dll) in DLL_IMPORTS.iter().enumerate() {
                    let off = dll_name_offsets[di];
                    let dll_bytes = dll.dll.as_bytes();
                    bytes[off..off+dll_bytes.len()].copy_from_slice(dll_bytes);
                    bytes[off+dll_bytes.len()] = 0;
                }

                // Total IAT size: sum of all DLL IAT arrays
                let iat_total_size: u32 = DLL_IMPORTS.iter().enumerate().map(|(di, dll)| {
                    ((dll.functions.len() + 1) * 8) as u32
                }).sum();

                // Report actual import directory size based on used DLLs
                let actual_import_dir_size = ((desc_idx + 1) * 20) as u32;

                IdataBuildResult {
                    bytes,
                    slot_to_iat_rva,
                    import_dir_rva,
                    import_dir_size: actual_import_dir_size,
                    iat_rva,
                    iat_size: iat_total_size,
                    program_strings_offset,
                }
            }

            pub fn build_iat_name_to_rva_map(idata_rva: u32) -> HashMap<String, u32> {
                let result = build_idata(idata_rva, &[]);
                let mut map = HashMap::new();
                let mut idx = 0;
                for dll in DLL_IMPORTS {
                    for f in dll.functions {
                        map.insert(f.to_string(), result.slot_to_iat_rva[idx]);
                        idx += 1;
                    }
                }
                map
            }
        }
    }
}

pub mod flat_binary {
    pub struct FlatBinaryGenerator {
        org: u64,
        fixed_size: Option<usize>,
    }

    impl FlatBinaryGenerator {
        pub fn new(org: u64) -> Self {
            Self {
                org,
                fixed_size: None,
            }
        }

        pub fn set_fixed_size(&mut self, size: usize) {
            self.fixed_size = Some(size);
        }

        pub fn generate(&self, code: &[u8], data: &[u8]) -> Vec<u8> {
            let _ = self.org;
            let mut out = Vec::with_capacity(code.len() + data.len());
            out.extend_from_slice(code);
            out.extend_from_slice(data);
            if let Some(sz) = self.fixed_size {
                if out.len() < sz {
                    out.resize(sz, 0);
                } else if out.len() > sz {
                    out.truncate(sz);
                }
            }
            out
        }
    }
}

pub mod elf {
    pub fn generate_elf(
        _code: &[u8],
        _data: &[u8],
        _output_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Err("ELF output not implemented in this workspace snapshot".into())
    }
}

pub mod po {
    pub struct PoOutput;

    impl PoOutput {
        pub fn new() -> Self {
            Self
        }

        pub fn generate(
            &self,
            code: &[u8],
            data: &[u8],
            output_path: &str,
        ) -> Result<usize, Box<dyn std::error::Error>> {
            let header_size = 32u32;
            let code_offset = header_size;
            let code_size = code.len() as u32;
            let data_offset = code_offset + code_size;
            let data_size = data.len() as u32;

            let mut header = [0u8; 32];
            header[0..4].copy_from_slice(&0x506F4F53u32.to_le_bytes());
            header[4] = 0x80;
            header[5] = 64u8;
            header[6..8].copy_from_slice(&0u16.to_le_bytes());
            header[8..12].copy_from_slice(&code_offset.to_le_bytes());
            header[12..16].copy_from_slice(&code_size.to_le_bytes());
            header[16..20].copy_from_slice(&data_offset.to_le_bytes());
            header[20..24].copy_from_slice(&data_size.to_le_bytes());
            header[24..28].copy_from_slice(&0u32.to_le_bytes());
            header[28..32].copy_from_slice(&0u32.to_le_bytes());

            let mut bin = Vec::new();
            bin.extend_from_slice(&header);
            bin.extend_from_slice(code);
            bin.extend_from_slice(data);
            std::fs::write(output_path, &bin)?;
            Ok(bin.len())
        }
    }

    impl Default for PoOutput {
        fn default() -> Self {
            Self::new()
        }
    }
}

pub mod pe {
    use crate::backend::cpu::iat_registry;

    fn align_up_u32(v: u32, a: u32) -> u32 {
        if a == 0 {
            return v;
        }
        (v + (a - 1)) & !(a - 1)
    }

    fn align_up_usize(v: usize, a: usize) -> usize {
        if a == 0 {
            return v;
        }
        (v + (a - 1)) & !(a - 1)
    }

    fn push_u16(buf: &mut Vec<u8>, v: u16) {
        buf.extend_from_slice(&v.to_le_bytes());
    }

    fn push_u32(buf: &mut Vec<u8>, v: u32) {
        buf.extend_from_slice(&v.to_le_bytes());
    }

    fn push_u64(buf: &mut Vec<u8>, v: u64) {
        buf.extend_from_slice(&v.to_le_bytes());
    }

    /// The assumed idata_rva that the ISA compiler uses during code generation.
    /// The PE builder patches code bytes when the actual idata_rva differs.
    pub const ASSUMED_IDATA_RVA: u32 = 0x2000;

    pub fn generate_pe_with_offsets(
        code: &[u8],
        data: &[u8],
        output_path: &str,
        iat_call_offsets: &[usize],
        string_imm64_offsets: &[usize],
    ) -> Result<(), Box<dyn std::error::Error>> {
        generate_pe_filtered(code, data, output_path, iat_call_offsets, string_imm64_offsets, &std::collections::HashSet::new())
    }

    pub fn generate_pe_filtered(
        code: &[u8],
        data: &[u8],
        output_path: &str,
        iat_call_offsets: &[usize],
        string_imm64_offsets: &[usize],
        used_iat_slots: &std::collections::HashSet<usize>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file_alignment: u32 = 0x200;
        let section_alignment: u32 = 0x1000;

        let image_base: u64 = 0x0000000140000000;
        let text_rva: u32 = 0x1000;

        // Dynamic idata_rva: place .idata after .text's virtual pages
        let text_virtual_pages = align_up_u32(code.len() as u32, section_alignment);
        let idata_rva: u32 = text_rva + text_virtual_pages.max(section_alignment);

        let idata_result = iat_registry::build_idata_filtered(idata_rva, used_iat_slots);
        let mut idata = idata_result.bytes;
        if idata.len() != idata_result.program_strings_offset as usize {
            idata.resize(idata_result.program_strings_offset as usize, 0);
        }
        idata.extend_from_slice(data);

        // Patch code bytes when idata_rva differs from the assumed value
        let mut code = code.to_vec();
        let rva_delta = idata_rva as i64 - ASSUMED_IDATA_RVA as i64;
        if rva_delta != 0 {
            // Patch IAT call offsets: each is a RIP-relative disp32 (FF 15 [disp32])
            // The disp32 encodes (iat_rva - current_rip), so we adjust by the delta
            for &off in iat_call_offsets {
                if off + 4 <= code.len() {
                    let old_disp = i32::from_le_bytes([
                        code[off], code[off + 1], code[off + 2], code[off + 3],
                    ]);
                    let new_disp = old_disp + rva_delta as i32;
                    code[off..off + 4].copy_from_slice(&new_disp.to_le_bytes());
                }
            }

            // Patch string imm64 offsets: each is an absolute address (imagebase + idata_rva + string_offset)
            // Shift by the RVA delta
            for &off in string_imm64_offsets {
                if off + 8 <= code.len() {
                    let old_addr = u64::from_le_bytes([
                        code[off], code[off + 1], code[off + 2], code[off + 3],
                        code[off + 4], code[off + 5], code[off + 6], code[off + 7],
                    ]);
                    let new_addr = (old_addr as i64 + rva_delta) as u64;
                    code[off..off + 8].copy_from_slice(&new_addr.to_le_bytes());
                }
            }
        }

        let text_raw_size = align_up_u32(code.len() as u32, file_alignment);
        let idata_raw_size = align_up_u32(idata.len() as u32, file_alignment);

        let headers_size = 0x200u32;
        let text_raw_ptr = headers_size;
        let idata_raw_ptr = text_raw_ptr + text_raw_size;

        let text_virtual_size = code.len() as u32;
        let idata_virtual_size = idata.len() as u32;

        let size_of_image = align_up_u32(idata_rva + idata_virtual_size, section_alignment);
        let size_of_headers = headers_size;

        let e_lfanew: u32 = 0x80;
        let mut dos = vec![0u8; e_lfanew as usize];
        dos[0..2].copy_from_slice(b"MZ");
        dos[0x3C..0x40].copy_from_slice(&e_lfanew.to_le_bytes());

        let number_of_sections: u16 = 2;
        let size_of_optional_header: u16 = 0xF0;
        let characteristics: u16 = 0x0022;

        let mut headers = Vec::new();
        headers.extend_from_slice(&dos);
        headers.extend_from_slice(b"PE\0\0");

        push_u16(&mut headers, 0x8664);
        push_u16(&mut headers, number_of_sections);
        push_u32(&mut headers, 0);
        push_u32(&mut headers, 0);
        push_u32(&mut headers, 0);
        push_u16(&mut headers, size_of_optional_header);
        push_u16(&mut headers, characteristics);

        let mut opt = Vec::new();
        push_u16(&mut opt, 0x20B);
        opt.push(0);
        opt.push(0);
        push_u32(&mut opt, text_raw_size);
        push_u32(&mut opt, idata_raw_size);
        push_u32(&mut opt, 0);
        push_u32(&mut opt, text_rva);
        push_u32(&mut opt, text_rva);
        push_u64(&mut opt, image_base);
        push_u32(&mut opt, section_alignment);
        push_u32(&mut opt, file_alignment);
        push_u16(&mut opt, 6);
        push_u16(&mut opt, 0);
        push_u16(&mut opt, 0);
        push_u16(&mut opt, 0);
        push_u16(&mut opt, 6);
        push_u16(&mut opt, 0);
        push_u32(&mut opt, 0);
        push_u32(&mut opt, size_of_image);
        push_u32(&mut opt, size_of_headers);
        push_u32(&mut opt, 0);
        push_u16(&mut opt, 3);
        push_u16(&mut opt, 0x8100);
        push_u64(&mut opt, 0x100000);
        push_u64(&mut opt, 0x1000);
        push_u64(&mut opt, 0x100000);
        push_u64(&mut opt, 0x1000);
        push_u32(&mut opt, 0);
        push_u32(&mut opt, 16);

        for dir_index in 0..16 {
            if dir_index == 1 {
                push_u32(&mut opt, idata_result.import_dir_rva);
                push_u32(&mut opt, idata_result.import_dir_size);
            } else if dir_index == 12 {
                push_u32(&mut opt, idata_result.iat_rva);
                push_u32(&mut opt, idata_result.iat_size);
            } else {
                push_u32(&mut opt, 0);
                push_u32(&mut opt, 0);
            }
        }

        if opt.len() != size_of_optional_header as usize {
            return Err(format!(
                "Optional header size mismatch: got {}, expected {}",
                opt.len(),
                size_of_optional_header
            )
            .into());
        }
        headers.extend_from_slice(&opt);

        let mut sh = Vec::new();

        let mut name = [0u8; 8];
        name[..5].copy_from_slice(b".text");
        sh.extend_from_slice(&name);
        push_u32(&mut sh, text_virtual_size);
        push_u32(&mut sh, text_rva);
        push_u32(&mut sh, text_raw_size);
        push_u32(&mut sh, text_raw_ptr);
        push_u32(&mut sh, 0);
        push_u32(&mut sh, 0);
        push_u16(&mut sh, 0);
        push_u16(&mut sh, 0);
        push_u32(&mut sh, 0x60000020);

        let mut name2 = [0u8; 8];
        name2[..6].copy_from_slice(b".idata");
        sh.extend_from_slice(&name2);
        push_u32(&mut sh, idata_virtual_size);
        push_u32(&mut sh, idata_rva);
        push_u32(&mut sh, idata_raw_size);
        push_u32(&mut sh, idata_raw_ptr);
        push_u32(&mut sh, 0);
        push_u32(&mut sh, 0);
        push_u16(&mut sh, 0);
        push_u16(&mut sh, 0);
        push_u32(&mut sh, 0xC0000040);

        headers.extend_from_slice(&sh);

        if headers.len() > headers_size as usize {
            return Err("PE headers exceed 0x200".into());
        }
        headers.resize(headers_size as usize, 0);

        let mut out = Vec::new();
        out.extend_from_slice(&headers);

        let mut text_raw = code;
        text_raw.resize(text_raw_size as usize, 0x90);
        out.resize(text_raw_ptr as usize, 0);
        out.extend_from_slice(&text_raw);

        let mut idata_raw = idata;
        idata_raw.resize(idata_raw_size as usize, 0);
        out.resize(idata_raw_ptr as usize, 0);
        out.extend_from_slice(&idata_raw);

        let final_len = align_up_usize(out.len(), file_alignment as usize);
        out.resize(final_len, 0);

        std::fs::write(output_path, &out)?;
        Ok(())
    }
}
