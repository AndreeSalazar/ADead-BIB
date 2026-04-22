// ============================================================
// IAT Registry — Multi-DLL Import Address Table
// ============================================================
//
// Extracted from lib.rs monolith. Single responsibility:
//   - Define DLL import tables (18 DLLs, 340+ functions)
//   - Build .idata section bytes for PE generation
//   - Slot lookup for ISA compiler
//
// ============================================================

use std::collections::HashMap;
use std::collections::HashSet;

// ── Multi-DLL IAT Registry v5 ─────────────────────────
// 18 DLLs, 300+ slots: msvcrt, kernel32, user32, gdi32,
// opengl32, ole32, oleaut32, dxgi, d3d9, d3d11, d3d12,
// d3dcompiler_47, advapi32, shell32, winmm, comdlg32, ws2_32

pub struct DllImport {
    pub dll: &'static str,
    pub functions: &'static [&'static str],
}

pub static DLL_IMPORTS: &[DllImport] = &[
    // ── Phase 1: C Runtime (msvcrt.dll) — 334 functions ──
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
        "hypot", "log2", "exp2", "cbrt",
        "round", "trunc", "fmin", "fmax",
        "copysign", "nextafter", "_isnan", "_finite",
        "fma", "remainder", "nearbyint", "rint",
        // wchar (msvcrt.dll exports)
        "wprintf", "fwprintf", "swprintf",
        "wcscpy", "wcsncpy", "wcscat", "wcsncat",
        "wcscmp", "wcsncmp", "wcslen",
        "wcschr", "wcsrchr", "wcsstr", "wcstok",
        "wcstol", "wcstoul", "wcstod",
        "mbstowcs", "wcstombs", "mbtowc", "wctomb",
        "iswalpha", "iswdigit", "iswalnum", "iswspace",
        "towupper", "towlower",
        // time — core + extras
        "time", "clock", "difftime", "strftime",
        "mktime", "localtime", "gmtime", "asctime", "ctime",
        // signal
        "signal", "raise",
        // locale
        "setlocale", "localeconv",
        // errno
        "_errno",
        // stdio — missing
        "fscanf", "fgetpos", "fsetpos", "vsnprintf",
        // stdlib — missing
        "div", "ldiv", "lldiv",
        "_aligned_malloc", "_aligned_free",
        "_beginthread", "_endthread", "_beginthreadex", "_endthreadex",
        "_stricmp", "_strnicmp",
        "_snprintf_s", "sprintf_s", "strcpy_s", "strncpy_s", "strcat_s",
        "_open", "_close", "_read", "_write", "_lseek",
        "_stat", "_fstat",
        "_mkdir", "_rmdir", "_chdir", "_getcwd",
        "_findfirst", "_findnext", "_findclose",
        "_access",
        "_itoa", "_ltoa", "_ui64toa",
        "_fullpath", "_makepath", "_splitpath",
        // security
        "__security_init_cookie", "__security_check_cookie",
        // math — missing
        "lround", "llround", "lroundf", "llroundf",
        "scalbn", "scalbnf", "scalbln", "scalblnf",
        "ilogb", "ilogbf", "logb", "logbf",
        "nan", "nanf",
        "fdim", "fdimf",
        // wchar — missing
        "wscanf", "fwscanf", "swscanf",
        "wmemcpy", "wmemmove", "wmemset", "wmemcmp", "wmemchr",
        "_mbrtowc", "_wcrtomb", "_mbrlen",
        "fgetwc", "fputwc", "fgetws", "fputws", "getwc", "putwc",
        "wcsftime",
        "iswprint", "iswpunct", "iswcntrl", "iswxdigit", "iswgraph",
        "iswupper", "iswlower",
        // MSVC CRT extras
        "_wfopen", "_wfreopen",
        "_setmode", "_fileno",
        "_isatty",
        "_putenv", "_wgetenv",
        "_strdup", "_wcsdup",
        "_strlwr", "_strupr",
        "_gcvt", "_ecvt", "_fcvt",
        "_swab",
        "_sleep",
        // setjmp/longjmp
        "setjmp", "longjmp", "_setjmp", "_longjmp",
        // math — float variants still missing
        "sinf", "cosf", "tanf", "asinf", "acosf", "atanf", "atan2f",
        "expf", "logf", "log10f", "log2f", "powf", "sqrtf",
        "ceilf", "floorf", "fabsf", "fmodf",
        "sinhf", "coshf", "tanhf",
        "hypotf", "cbrtf", "exp2f",
        "copysignf", "nextafterf",
        "fmaf", "remainderf", "nearbyintf", "rintf",
        "erff", "erfcf", "tgammaf", "lgammaf",
        // math — special functions
        "erf", "erfc", "tgamma", "lgamma",
        // stdlib — still missing
        "strtold", "atoll",
        "_ultoa", "_i64toa",
        "_wtoi", "_wtol", "_wtof",
        "_wcsicmp", "_wcsnicmp",
        // string — still missing
        "_strnset", "_strrev", "_strset",
        // locale — still missing
        "_setmbcp", "_getmbcp",
        "_ismbblead",
        // process
        "_spawnl", "_spawnlp", "_spawnle",
        "_getpid",
        // UCRT wide I/O
        "_vsnwprintf", "vswprintf",
        "_vscwprintf",
        // UCRT extras
        "_dup", "_dup2", "_pipe",
        "_umask", "_chmod",
        "_unlink",
        "_tempnam",
        "_hypot",
    ] },
    // ── Phase 2: Win32 Core — kernel32.dll ──
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
        // Threading — extended
        "ExitThread", "ResumeThread", "SuspendThread",
        "WaitForMultipleObjects",
        "CreateMutexA", "ReleaseMutex",
        "CreateEventA", "SetEvent", "ResetEvent",
        "CreateSemaphoreA", "ReleaseSemaphore",
        // Critical sections
        "InitializeCriticalSection", "EnterCriticalSection",
        "LeaveCriticalSection", "DeleteCriticalSection",
        "TryEnterCriticalSection",
        // Atomics / Interlocked
        "InterlockedIncrement", "InterlockedDecrement",
        "InterlockedExchange", "InterlockedCompareExchange",
        "InterlockedExchangeAdd",
        // Process
        "CreateProcessA", "TerminateProcess", "GetExitCodeProcess",
        // Memory-mapped files
        "CreateFileMappingA", "MapViewOfFile", "UnmapViewOfFile",
        // Pipes
        "CreatePipe",
        // File ops — extended
        "SetEndOfFile", "FlushFileBuffers",
        "GetFileSizeEx",
        "GetFileAttributesA", "SetFileAttributesA",
        "FindFirstFileA", "FindNextFileA", "FindClose",
        "CreateDirectoryA", "RemoveDirectoryA",
        "DeleteFileA", "MoveFileA", "CopyFileA",
        "GetTempPathA", "GetTempFileNameA",
        "GetFullPathNameA",
        // Time — extended
        "GetSystemTime", "GetLocalTime",
        "SystemTimeToFileTime", "FileTimeToSystemTime",
        "GetTickCount64",
        // System info
        "GetNativeSystemInfo",
        "GetComputerNameA",
        // Error handling
        "SetLastError", "FormatMessageA",
        // Modules — extended
        "GetModuleFileNameA",
        // TLS
        "TlsAlloc", "TlsFree", "TlsGetValue", "TlsSetValue",
        // Console — extended
        "ReadConsoleA", "SetConsoleTextAttribute",
        "GetConsoleScreenBufferInfo",
        // Misc
        "GetVersionExA",
        "MultiByteToWideChar", "WideCharToMultiByte",
        "SetUnhandledExceptionFilter",
    ] },
    // ── Phase 2: Win32 UI — user32.dll ──
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
    // ── Phase 2: Win32 GDI — gdi32.dll ──
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
    // ── Phase 2: OpenGL 1.1 — opengl32.dll ──
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
    // ── Phase 3: COM — ole32.dll ──
    DllImport { dll: "ole32.dll", functions: &[
        "CoInitialize", "CoInitializeEx", "CoUninitialize",
        "CoCreateInstance", "CoGetClassObject",
        "CoTaskMemAlloc", "CoTaskMemRealloc", "CoTaskMemFree",
        "StringFromCLSID", "CLSIDFromString",
        "StringFromGUID2", "IIDFromString", "StringFromIID",
        "CoRegisterClassObject", "CoRevokeClassObject",
        "PropVariantClear",
        "OleInitialize", "OleUninitialize",
        "CoMarshalInterThreadInterfaceInStream",
        "CoGetInterfaceAndReleaseStream",
    ] },
    // ── Phase 3: COM Automation — oleaut32.dll ──
    DllImport { dll: "oleaut32.dll", functions: &[
        "SysAllocString", "SysAllocStringLen",
        "SysFreeString", "SysStringLen", "SysStringByteLen",
        "SysReAllocString", "SysReAllocStringLen",
        "VariantInit", "VariantClear", "VariantCopy",
        "VariantChangeType", "VariantChangeTypeEx",
        "SafeArrayCreate", "SafeArrayCreateVector",
        "SafeArrayDestroy", "SafeArrayAccessData", "SafeArrayUnaccessData",
        "SafeArrayGetLBound", "SafeArrayGetUBound",
        "RegisterTypeLib",
    ] },
    // ── Phase 3: DXGI — dxgi.dll ──
    DllImport { dll: "dxgi.dll", functions: &[
        "CreateDXGIFactory", "CreateDXGIFactory1", "CreateDXGIFactory2",
        "DXGIGetDebugInterface1",
    ] },
    // ── Phase 3: DirectX 9 — d3d9.dll ──
    DllImport { dll: "d3d9.dll", functions: &[
        "Direct3DCreate9", "Direct3DCreate9Ex",
        "D3DPERF_BeginEvent", "D3DPERF_EndEvent",
        "D3DPERF_SetMarker", "D3DPERF_SetRegion",
        "D3DPERF_QueryRepeatFrame", "D3DPERF_SetOptions",
        "D3DPERF_GetStatus",
    ] },
    // ── Phase 3: DirectX 11 — d3d11.dll ──
    DllImport { dll: "d3d11.dll", functions: &[
        "D3D11CreateDevice", "D3D11CreateDeviceAndSwapChain",
        "D3D11On12CreateDevice",
    ] },
    // ── Phase 3: DirectX 12 — d3d12.dll ──
    DllImport { dll: "d3d12.dll", functions: &[
        "D3D12CreateDevice", "D3D12GetDebugInterface",
        "D3D12SerializeRootSignature", "D3D12SerializeVersionedRootSignature",
        "D3D12CreateRootSignatureDeserializer",
        "D3D12CreateVersionedRootSignatureDeserializer",
        "D3D12EnableExperimentalFeatures", "D3D12GetInterface",
    ] },
    // ── Phase 3: HLSL Compiler — d3dcompiler_47.dll ──
    DllImport { dll: "d3dcompiler_47.dll", functions: &[
        "D3DCompile", "D3DCompile2",
        "D3DCompileFromFile", "D3DReflect",
        "D3DCreateBlob", "D3DDisassemble",
        "D3DGetBlobPart", "D3DStripShader",
        "D3DReadFileToBlob", "D3DWriteBlobToFile",
        "D3DPreprocess", "D3DGetDebugInfo",
        "D3DGetInputSignatureBlob", "D3DGetOutputSignatureBlob",
        "D3DGetInputAndOutputSignatureBlob",
    ] },
    // ── Phase 4: Security & Registry — advapi32.dll ──
    DllImport { dll: "advapi32.dll", functions: &[
        "RegOpenKeyExA", "RegCloseKey",
        "RegQueryValueExA", "RegSetValueExA",
        "RegCreateKeyExA", "RegDeleteKeyA", "RegDeleteValueA",
        "RegEnumKeyExA", "RegEnumValueA",
        "OpenProcessToken", "GetTokenInformation",
        "LookupPrivilegeValueA", "AdjustTokenPrivileges",
        "GetUserNameA",
        "CryptAcquireContextA", "CryptGenRandom", "CryptReleaseContext",
        "InitializeSecurityDescriptor", "SetSecurityDescriptorDacl",
        "GetSecurityDescriptorDacl",
    ] },
    // ── Phase 4: Shell — shell32.dll ──
    DllImport { dll: "shell32.dll", functions: &[
        "ShellExecuteA", "ShellExecuteExA",
        "SHGetFolderPathA", "SHGetKnownFolderPath",
        "SHBrowseForFolderA", "SHGetPathFromIDListA",
        "SHFileOperationA",
        "DragAcceptFiles", "DragQueryFileA", "DragFinish",
    ] },
    // ── Phase 4: Multimedia — winmm.dll ──
    DllImport { dll: "winmm.dll", functions: &[
        "PlaySoundA",
        "waveOutOpen", "waveOutClose", "waveOutWrite",
        "waveOutPrepareHeader", "waveOutUnprepareHeader",
        "waveOutSetVolume", "waveOutGetVolume",
        "timeGetTime", "timeBeginPeriod", "timeEndPeriod",
        "joyGetPosEx",
    ] },
    // ── Phase 4: Common Dialogs — comdlg32.dll ──
    DllImport { dll: "comdlg32.dll", functions: &[
        "GetOpenFileNameA", "GetSaveFileNameA",
        "ChooseColorA", "ChooseFontA",
        "PrintDlgA", "CommDlgExtendedError",
    ] },
    // ── Phase 4: Networking — ws2_32.dll ──
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

pub fn all_functions() -> Vec<&'static str> {
    let mut v = Vec::new();
    for dll in DLL_IMPORTS {
        for f in dll.functions { v.push(*f); }
    }
    v
}

pub fn total_function_count() -> usize {
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
    let mut all_slots = HashSet::new();
    for i in 0..total_function_count() {
        all_slots.insert(i);
    }
    build_idata_filtered(idata_rva, &all_slots)
}

/// Build idata section, only importing DLLs that have at least one used slot.
/// If used_slots is empty, NO DLLs are imported (produces a minimal valid idata).
pub fn build_idata_filtered(idata_rva: u32, used_slots: &HashSet<usize>) -> IdataBuildResult {
    let num_dlls = DLL_IMPORTS.len();
    let total_funcs = total_function_count();

    // Determine which DLLs are actually needed
    let mut dll_is_used = vec![false; num_dlls];
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
    let _import_dir_size = import_desc_size as u32;
    let iat_rva = idata_rva + iat_offset as u32;

    // Build slot_to_iat_rva: flat index across all DLLs.
    // For used slots, compute compact position within the DLL's IAT block.
    // For unused slots, keep the original position (it won't matter since they're not referenced).
    let mut slot_to_iat_rva = Vec::with_capacity(total_funcs);
    {
        let mut gfi = 0usize;
        for (di, dll) in DLL_IMPORTS.iter().enumerate() {
            // Count how many used slots precede each slot in this DLL
            let mut compact_idx = 0u32;
            for fi in 0..dll.functions.len() {
                let global_slot = gfi + fi;
                if used_slots.contains(&global_slot) {
                    slot_to_iat_rva.push(idata_rva + dll_iat_offsets[di] as u32 + (compact_idx * 8));
                    compact_idx += 1;
                } else {
                    // Unused slot — keep original position for backward compat
                    slot_to_iat_rva.push(idata_rva + dll_iat_offsets[di] as u32 + (fi as u32 * 8));
                }
            }
            gfi += dll.functions.len();
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

        // Only write OFT + IAT entries for INDIVIDUALLY USED functions within used DLLs.
        // Writing ALL functions of a used DLL causes 0xC0000139 if any function name
        // doesn't exist in the system's DLL (e.g. extended msvcrt functions).
        if dll_is_used[di] {
            // Collect used function indices within this DLL for compact OFT/IAT
            let mut used_fi: Vec<usize> = Vec::new();
            for fi in 0..dll.functions.len() {
                let global_slot = global_func_idx + fi;
                if used_slots.contains(&global_slot) {
                    used_fi.push(fi);
                }
            }
            // Write compact OFT/IAT: only used functions, contiguous from start
            for (compact_idx, &fi) in used_fi.iter().enumerate() {
                let hn_rva = idata_rva + hint_name_offsets[global_func_idx + fi];
                let entry = hn_rva as u64;
                let oft_entry_off = dll_oft_offsets[di] + compact_idx * 8;
                let iat_entry_off = dll_iat_offsets[di] + compact_idx * 8;
                push_u64_to(&mut bytes, oft_entry_off, entry);
                push_u64_to(&mut bytes, iat_entry_off, entry);
            }
            // Null terminator right after the last used entry
            let term_idx = used_fi.len();
            let oft_null = dll_oft_offsets[di] + term_idx * 8;
            let iat_null = dll_iat_offsets[di] + term_idx * 8;
            push_u64_to(&mut bytes, oft_null, 0);
            push_u64_to(&mut bytes, iat_null, 0);
        }
        global_func_idx += dll.functions.len();
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
    let iat_total_size: u32 = DLL_IMPORTS.iter().map(|dll| {
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
