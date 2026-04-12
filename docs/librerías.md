# ADead-BIB + ASM-BIB — LISTA TOTAL DE LIBRERÍAS PARA REEMPLAZAR MSVC

> **Objetivo:** Catálogo COMPLETO de todo lo necesario para que ADead-BIB sea alternativa total a MSVC, GCC y LLVM.  
> **Fecha:** 2026-04-12  
> **Estado actual:** ADead-BIB v10.0 + ASM-BIB v2.0  
> **Arquitectura:** 100% Rust — Sin LLVM, Sin GCC, Sin MSVC  

---

## RESUMEN EJECUTIVO

| Categoría | MSVC tiene | ADead-BIB tiene | FALTA |
|-----------|-----------|-----------------|-------|
| C Standard Library (libc) | ~200 funciones | ~138 IAT + stdlib headers | ~60 funciones (codegen pendiente) |
| C++ Standard Library (STL) | ~2000+ clases/funciones | ~35 HPP templates | ~1965+ |
| Win32 API DLLs | 50+ DLLs | 5 DLLs (kernel32, user32, gdi32, opengl32, msvcrt) | 45+ DLLs |
| COM/OLE | Completo | ❌ Nada | TODO |
| DirectX (9-12) | SDK completo | ❌ Headers parciales, codegen vacío | TODO |
| Vulkan | SDK completo | Declaraciones | Runtime/loader |
| Kernel headers (ntddk) | Completo | fastos_kernel.rs parcial | ~95% |
| Linker features | LINK.exe completo | PE Writer básico | DLL, LIB, PDB, LTCG |
| Debugger integration | PDB completo | ❌ Nada | TODO |
| Resource compiler | rc.exe | ❌ Nada | TODO |
| MIDL/IDL compiler | midl.exe | ❌ Nada | TODO |

---

## ══════════════════════════════════════════════════
## PARTE 1: C STANDARD LIBRARY (libc) COMPLETA
## ══════════════════════════════════════════════════

### 1.1 `<stdio.h>` — I/O Estándar

| Función | MSVC | ADead-BIB | Estado |
|---------|------|-----------|--------|
| `printf` | ✅ | ✅ IAT msvcrt | ✅ Funciona |
| `fprintf` | ✅ | ✅ IAT msvcrt | ✅ IAT listo, codegen pendiente |
| `sprintf` | ✅ | ✅ IAT msvcrt | ✅ Funciona |
| `snprintf` | ✅ | ✅ IAT (_snprintf) | ✅ Funciona |
| `scanf` | ✅ | ✅ IAT msvcrt | ✅ Funciona |
| `fscanf` | ✅ | 🟡 Falta IAT | 🔴 Falta IAT |
| `sscanf` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `fopen` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `fclose` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `fread` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `fwrite` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `fgets` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `fputs` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `fgetc` / `getc` | ✅ | ✅ IAT (fgetc) | ✅ IAT listo |
| `fputc` / `putc` | ✅ | ✅ IAT (fputc) | ✅ IAT listo |
| `puts` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `getchar` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `putchar` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `fseek` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `ftell` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `rewind` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `fflush` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `feof` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `ferror` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `clearerr` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `remove` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `rename` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `tmpfile` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `tmpnam` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `perror` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `setbuf` / `setvbuf` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `vprintf` / `vfprintf` / `vsprintf` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `ungetc` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `freopen` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `fgetpos` / `fsetpos` | ✅ | 🟡 Falta IAT | 🔴 Falta IAT |

### 1.2 `<stdlib.h>` — Utilidades Generales

| Función | MSVC | ADead-BIB | Estado |
|---------|------|-----------|--------|
| `malloc` | ✅ | ✅ IAT | ✅ Funciona |
| `calloc` | ✅ | ✅ IAT | ✅ IAT listo |
| `realloc` | ✅ | ✅ IAT | ✅ IAT listo |
| `free` | ✅ | ✅ IAT | ✅ Funciona |
| `atoi` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `atol` / `atoll` | ✅ | ✅ IAT (atol) | ✅ IAT listo |
| `atof` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `strtol` / `strtoul` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `strtoll` / `strtoull` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `strtod` / `strtof` / `strtold` | ✅ | ✅ IAT (strtod, strtof) | ✅ IAT listo |
| `abs` / `labs` / `llabs` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `div` / `ldiv` / `lldiv` | ✅ | 🟡 Falta IAT | 🔴 Falta IAT |
| `rand` / `srand` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `qsort` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `bsearch` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `exit` / `_exit` | ✅ | ✅ IAT + ExitProcess | ✅ Funciona |
| `atexit` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `abort` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `getenv` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `system` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `mbstowcs` / `wcstombs` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `mbtowc` / `wctomb` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `aligned_alloc` (C11) | ✅ | ❌ | 🔴 Falta (no en msvcrt) |

### 1.3 `<string.h>` — Cadenas y Memoria

| Función | MSVC | ADead-BIB | ASM-BIB Bridge | Estado |
|---------|------|-----------|----------------|--------|
| `strlen` | ✅ | ✅ IAT | ✅ asm_strlen | ✅ Funciona |
| `strcpy` | ✅ | ✅ IAT | ✅ asm_strcpy | ✅ Funciona |
| `strncpy` | ✅ | ✅ IAT | ❌ | ✅ Funciona |
| `strcat` | ✅ | ✅ IAT | ✅ asm_strcat | ✅ Funciona |
| `strncat` | ✅ | ✅ IAT | ❌ | ✅ Funciona |
| `strcmp` | ✅ | ✅ IAT | ✅ asm_strcmp | ✅ Funciona |
| `strncmp` | ✅ | ✅ IAT | ❌ | ✅ Funciona |
| `strchr` | ✅ | ✅ IAT | ✅ asm_strchr | ✅ IAT listo |
| `strrchr` | ✅ | ✅ IAT msvcrt | ❌ | ✅ IAT listo |
| `strstr` | ✅ | ✅ IAT | ❌ | ✅ Funciona |
| `strtok` | ✅ | ✅ IAT | ❌ | ✅ Funciona |
| `memcpy` | ✅ | ✅ IAT | ✅ asm_memcpy | ✅ IAT listo |
| `memmove` | ✅ | ✅ IAT msvcrt | ❌ | ✅ IAT listo |
| `memset` | ✅ | ✅ IAT | ✅ asm_memset | ✅ IAT listo |
| `memcmp` | ✅ | ✅ IAT | ✅ asm_memcmp | ✅ IAT listo |
| `memchr` | ✅ | ✅ IAT msvcrt | ❌ | ✅ IAT listo |
| `strerror` | ✅ | ✅ IAT msvcrt | ❌ | ✅ IAT listo |
| `strpbrk` | ✅ | ✅ IAT msvcrt | ❌ | ✅ IAT listo |
| `strspn` / `strcspn` | ✅ | ✅ IAT msvcrt | ❌ | ✅ IAT listo |
| `strcoll` | ✅ | ✅ IAT msvcrt | ❌ | ✅ IAT listo |
| `strxfrm` | ✅ | ✅ IAT msvcrt | ❌ | ✅ IAT listo |

### 1.4 `<math.h>` — Matemáticas (REQUIERE SSE/x87 CODEGEN)

| Función | MSVC | ADead-BIB | Estado |
|---------|------|-----------|--------|
| `sin` / `cos` / `tan` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `asin` / `acos` / `atan` / `atan2` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `sinh` / `cosh` / `tanh` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `exp` / `exp2` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `log` / `log2` / `log10` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `pow` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `sqrt` / `cbrt` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `fabs` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `ceil` / `floor` / `round` / `trunc` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `fmod` / `remainder` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `frexp` / `ldexp` / `modf` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `hypot` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `copysign` / `nextafter` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `_isnan` / `_finite` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `fma` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `erf` / `erfc` / `tgamma` / `lgamma` | ✅ | ❌ | 🔴 Falta (no en msvcrt) |
| `fmin` / `fmax` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `nearbyint` / `rint` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `lround` / `llround` | ✅ | ❌ | 🔴 Falta IAT |

### 1.5 `<time.h>` — Tiempo

| Función | MSVC | ADead-BIB | Estado |
|---------|------|-----------|--------|
| `time` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `clock` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `difftime` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `mktime` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `localtime` / `gmtime` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `asctime` / `ctime` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `strftime` | ✅ | ✅ IAT msvcrt | ✅ IAT listo |
| `timespec_get` (C11) | ✅ | ❌ | 🔴 Falta (no en msvcrt) |

### 1.6 `<ctype.h>` — Clasificación de Caracteres ✅ COMPLETO

| Función | Estado |
|---------|--------|
| `isalpha`, `isdigit`, `isalnum`, `isspace`, `isupper`, `islower`, `isprint`, `ispunct`, `iscntrl`, `isxdigit`, `isgraph`, `toupper`, `tolower` | ✅ IMPLEMENTADO |

### 1.7 `<signal.h>` — Señales

| Función | Estado |
|---------|--------|
| `signal` | ✅ IAT msvcrt + fastos_signal.rs |
| `raise` | ✅ IAT msvcrt + fastos_signal.rs |
| Constantes: SIGABRT, SIGFPE, SIGILL, SIGINT, SIGSEGV, SIGTERM | ✅ fastos_signal.rs |

### 1.8 `<setjmp.h>` — Saltos No-Locales

| Función | Estado |
|---------|--------|
| `setjmp` / `longjmp` / `jmp_buf` | 🔴 Falta |

### 1.9 `<stdarg.h>` — Argumentos Variádicos

| Macro | Estado |
|-------|--------|
| `va_list`, `va_start`, `va_arg`, `va_end`, `va_copy` | 🔴 Falta codegen |

### 1.10 `<locale.h>` — Localización

| Función | Estado |
|---------|--------|
| `setlocale` | ✅ IAT msvcrt + fastos_signal.rs |
| `localeconv` | ✅ IAT msvcrt |
| `struct lconv` | ✅ fastos_stdlib.rs |

### 1.11 `<wchar.h>` — Caracteres Anchos (Unicode)

| Función | Estado |
|---------|--------|
| `wprintf` / `fwprintf` / `swprintf` | ✅ IAT msvcrt + fastos_wchar.rs |
| `wscanf` / `fwscanf` / `swscanf` | 🔴 Falta IAT |
| `wcscpy` / `wcsncpy` / `wcscat` / `wcsncat` | ✅ IAT msvcrt + fastos_wchar.rs |
| `wcscmp` / `wcsncmp` / `wcslen` | ✅ IAT msvcrt + fastos_wchar.rs |
| `wcschr` / `wcsrchr` / `wcsstr` / `wcstok` | ✅ IAT msvcrt + fastos_wchar.rs |
| `wmemcpy` / `wmemmove` / `wmemset` / `wmemcmp` | 🔴 Falta IAT |
| `mbrtowc` / `wcrtomb` / `mbrlen` | 🔴 Falta IAT |
| `fgetwc` / `fputwc` / `fgetws` / `fputws` | 🔴 Falta IAT |
| `wcstol` / `wcstoul` / `wcstod` | ✅ IAT msvcrt + fastos_wchar.rs |
| `wcsftime` | 🔴 Falta IAT |

### 1.12 `<wctype.h>` — Clasificación Caracteres Anchos

| Función | Estado |
|---------|--------|
| `iswalpha` / `iswdigit` / `iswalnum` / `iswspace` | ✅ IAT msvcrt + fastos_wchar.rs |
| `towupper` / `towlower` | ✅ IAT msvcrt + fastos_wchar.rs |

### 1.13 C11 Headers Adicionales

| Header | Funciones Clave | Estado |
|--------|----------------|--------|
| `<stdalign.h>` | `alignas`, `alignof` | 🔴 Falta |
| `<stdnoreturn.h>` | `_Noreturn` | 🔴 Falta |
| `<stdbool.h>` | `bool`, `true`, `false` | 🔴 Falta |
| `<stdatomic.h>` | `atomic_int`, `atomic_load`, `atomic_store`, `atomic_fetch_add` | 🔴 Falta |
| `<threads.h>` | `thrd_create`, `mtx_lock`, `cnd_wait`, `tss_create` | 🔴 Falta |
| `<uchar.h>` | `char16_t`, `char32_t`, `mbrtoc16`, `c16rtomb` | 🔴 Falta |
| `<complex.h>` | `_Complex`, `cabs`, `carg`, `cexp`, `cpow` | 🔴 Falta |
| `<tgmath.h>` | Type-generic math macros | 🔴 Falta |
| `<fenv.h>` | `fegetround`, `fesetround`, `feclearexcept` | 🔴 Falta |
| `<inttypes.h>` | `PRId64`, `PRIu32`, `imaxabs`, `strtoimax` | 🔴 Falta |

### 1.14 MSVC-Specific C Runtime Extensions

| Función | Propósito | Estado |
|---------|-----------|--------|
| `_aligned_malloc` / `_aligned_free` | Memoria alineada | 🔴 Falta |
| `_beginthread` / `_endthread` | Threads CRT | 🔴 Falta |
| `_beginthreadex` / `_endthreadex` | Threads CRT avanzado | 🔴 Falta |
| `_stricmp` / `_strnicmp` | Comparación case-insensitive | 🔴 Falta |
| `_snprintf_s` / `sprintf_s` / `strcpy_s` | Funciones seguras (_s) | 🔴 Falta |
| `_open` / `_close` / `_read` / `_write` | Low-level I/O | 🔴 Falta |
| `_stat` / `_fstat` | Estado de archivos | 🔴 Falta |
| `_mkdir` / `_rmdir` / `_chdir` / `_getcwd` | Directorios | 🔴 Falta |
| `_findfirst` / `_findnext` / `_findclose` | Búsqueda de archivos | 🔴 Falta |
| `_access` | Verificar acceso a archivos | 🔴 Falta |
| `_itoa` / `_ltoa` / `_ui64toa` | Conversión int→string | 🔴 Falta |
| `_fullpath` / `_makepath` / `_splitpath` | Rutas de archivos | 🔴 Falta |
| `__security_init_cookie` / `__security_check_cookie` | Stack cookie (GS) | 🔴 Falta |

---

## ══════════════════════════════════════════════════
## PARTE 2: WIN32 API — DLLs COMPLETAS
## ══════════════════════════════════════════════════

### 2.1 `kernel32.dll` — Kernel Base (PARCIAL → NECESITA EXPANSIÓN)

**YA TIENE:** GetModuleHandleA, LoadLibraryA, GetProcAddress, Sleep, ExitProcess, GetLastError, HeapAlloc, HeapFree, HeapCreate, VirtualAlloc, VirtualFree, CreateFileA, ReadFile, WriteFile, CloseHandle, GetStdHandle, WriteConsoleA

**FALTA:**

| Función | Propósito | Prioridad |
|---------|-----------|-----------|
| `CreateThread` / `ExitThread` / `ResumeThread` / `SuspendThread` | Threading | P0 |
| `WaitForSingleObject` / `WaitForMultipleObjects` | Sincronización | P0 |
| `CreateMutexA` / `ReleaseMutex` | Mutex | P0 |
| `CreateEventA` / `SetEvent` / `ResetEvent` | Eventos | P1 |
| `CreateSemaphoreA` / `ReleaseSemaphore` | Semáforos | P1 |
| `InitializeCriticalSection` / `EnterCriticalSection` / `LeaveCriticalSection` / `DeleteCriticalSection` | Critical sections | P0 |
| `InterlockedIncrement` / `InterlockedDecrement` / `InterlockedExchange` / `InterlockedCompareExchange` | Atomics | P0 |
| `CreateProcessA` / `TerminateProcess` / `GetExitCodeProcess` | Procesos | P1 |
| `GetCurrentProcess` / `GetCurrentProcessId` / `GetCurrentThread` / `GetCurrentThreadId` | Info procesos | P1 |
| `CreateFileMappingA` / `MapViewOfFile` / `UnmapViewOfFile` | Memory-mapped files | P1 |
| `CreatePipe` / `PeekNamedPipe` | Pipes | P1 |
| `CreateNamedPipeA` / `ConnectNamedPipe` | Named pipes | P2 |
| `SetFilePointer` / `SetEndOfFile` / `FlushFileBuffers` | File ops avanzadas | P1 |
| `GetFileSize` / `GetFileSizeEx` | Tamaño archivos | P1 |
| `GetFileAttributesA` / `SetFileAttributesA` | Atributos | P1 |
| `FindFirstFileA` / `FindNextFileA` / `FindClose` | Búsqueda archivos | P1 |
| `CreateDirectoryA` / `RemoveDirectoryA` | Directorios | P1 |
| `DeleteFileA` / `MoveFileA` / `CopyFileA` | Operaciones archivos | P1 |
| `GetTempPathA` / `GetTempFileNameA` | Temp files | P2 |
| `GetSystemTime` / `GetLocalTime` / `SystemTimeToFileTime` | Tiempo sistema | P1 |
| `QueryPerformanceCounter` / `QueryPerformanceFrequency` | High-res timer | P0 |
| `GetTickCount` / `GetTickCount64` | Timer simple | P1 |
| `GetSystemInfo` / `GetNativeSystemInfo` | Info sistema | P1 |
| `GlobalAlloc` / `GlobalFree` / `GlobalLock` / `GlobalUnlock` | Memoria global | P2 |
| `LocalAlloc` / `LocalFree` | Memoria local | P2 |
| `GetEnvironmentVariableA` / `SetEnvironmentVariableA` | Env vars | P1 |
| `GetComputerNameA` | Info PC | P2 |
| `OutputDebugStringA` | Debug output | P1 |
| `IsDebuggerPresent` | Detección debugger | P2 |
| `SetLastError` / `FormatMessageA` | Error handling | P1 |
| `GetModuleFileNameA` / `FreeLibrary` | Módulos | P1 |
| `TlsAlloc` / `TlsFree` / `TlsGetValue` / `TlsSetValue` | Thread Local Storage | P1 |
| `SetConsoleTitle` / `SetConsoleTextAttribute` / `GetConsoleScreenBufferInfo` | Consola | P2 |
| `DeviceIoControl` | Control dispositivos | P2 |
| `SetUnhandledExceptionFilter` / `AddVectoredExceptionHandler` | SEH | P1 |
| `RtlCaptureContext` / `RtlLookupFunctionEntry` / `RtlVirtualUnwind` | Stack unwinding | P1 |

### 2.2 `user32.dll` — Interfaz de Usuario (PARCIAL → NECESITA EXPANSIÓN)

**YA TIENE:** RegisterClassA, CreateWindowExA, ShowWindow, PeekMessageA, TranslateMessage, DispatchMessageA, PostQuitMessage, DefWindowProcA, DestroyWindow, GetDC, ReleaseDC, MessageBoxA

**FALTA:**

| Función | Propósito | Prioridad |
|---------|-----------|-----------|
| `RegisterClassExA` / `UnregisterClassA` | Window class avanzada | P1 |
| `GetMessageA` / `PostMessageA` / `SendMessageA` | Mensajes | P0 |
| `GetWindowLongA` / `SetWindowLongA` / `GetWindowLongPtrA` / `SetWindowLongPtrA` | Window props | P0 |
| `SetWindowTextA` / `GetWindowTextA` | Texto ventana | P1 |
| `GetClientRect` / `GetWindowRect` | Tamaño ventana | P0 |
| `AdjustWindowRect` / `AdjustWindowRectEx` | Tamaño frame | P0 |
| `MoveWindow` / `SetWindowPos` | Posición ventana | P1 |
| `InvalidateRect` / `UpdateWindow` / `RedrawWindow` | Repaint | P0 |
| `BeginPaint` / `EndPaint` | Pintura | P0 |
| `SetTimer` / `KillTimer` | Timers | P1 |
| `GetKeyState` / `GetAsyncKeyState` / `GetKeyboardState` | Input teclado | P0 |
| `GetCursorPos` / `SetCursorPos` / `ScreenToClient` / `ClientToScreen` | Input mouse | P0 |
| `SetCapture` / `ReleaseCapture` | Mouse capture | P1 |
| `LoadCursorA` / `LoadIconA` / `SetCursor` | Cursores/iconos | P1 |
| `LoadImageA` | Cargar imágenes | P1 |
| `GetSystemMetrics` | Métricas sistema | P1 |
| `EnableWindow` / `IsWindowVisible` / `IsWindowEnabled` | Estado ventana | P1 |
| `SetFocus` / `GetFocus` / `SetForegroundWindow` | Foco | P1 |
| `DialogBoxParamA` / `EndDialog` / `CreateDialogParamA` | Diálogos modales | P2 |
| `GetDlgItem` / `SetDlgItemTextA` / `GetDlgItemTextA` | Controles diálogo | P2 |
| `DrawTextA` / `FillRect` / `FrameRect` | Dibujo | P1 |
| `MapVirtualKeyA` / `VkKeyScanA` | Teclas virtuales | P1 |
| `TrackMouseEvent` | Mouse tracking | P1 |
| `SetWindowsHookExA` / `UnhookWindowsHookEx` / `CallNextHookEx` | Hooks | P2 |
| `EnumWindows` / `FindWindowA` | Buscar ventanas | P2 |
| `MonitorFromWindow` / `GetMonitorInfoA` | Multi-monitor | P1 |
| `ChangeDisplaySettingsA` / `EnumDisplaySettingsA` | Display settings | P1 |
| `RegisterRawInputDevices` / `GetRawInputData` | Raw input | P1 |

### 2.3 `gdi32.dll` — Graphics Device Interface (PARCIAL → NECESITA EXPANSIÓN)

**YA TIENE:** SwapBuffers, ChoosePixelFormat, SetPixelFormat, SetPixel, CreateSolidBrush, DeleteObject, SelectObject, Rectangle

**FALTA:**

| Función | Propósito | Prioridad |
|---------|-----------|-----------|
| `CreateDCA` / `CreateCompatibleDC` / `DeleteDC` | Device contexts | P0 |
| `CreateCompatibleBitmap` / `CreateDIBSection` | Bitmaps | P0 |
| `BitBlt` / `StretchBlt` / `AlphaBlend` / `TransparentBlt` | Blitting | P0 |
| `CreateFontA` / `CreateFontIndirectA` | Fuentes | P1 |
| `TextOutA` / `ExtTextOutA` | Texto | P1 |
| `GetTextExtentPoint32A` / `GetTextMetricsA` | Métricas texto | P1 |
| `SetBkMode` / `SetBkColor` / `SetTextColor` | Colores texto | P1 |
| `CreatePen` / `MoveToEx` / `LineTo` / `Polyline` / `PolyBezier` | Líneas/curvas | P1 |
| `Ellipse` / `Polygon` / `Pie` / `Arc` | Formas | P1 |
| `SaveDC` / `RestoreDC` | Estado DC | P1 |
| `GetPixel` / `SetROP2` | Pixel ops | P1 |
| `GetDeviceCaps` / `GetObject` / `GetStockObject` | Info objetos GDI | P1 |
| `CreateRectRgn` / `CombineRgn` / `SelectClipRgn` | Regiones/Clipping | P2 |
| `BeginPath` / `EndPath` / `StrokePath` / `FillPath` | Paths GDI | P2 |
| `GradientFill` (msimg32.dll) | Gradientes | P2 |

### 2.4 `advapi32.dll` — Seguridad y Registro (NO EXISTE)

| Función | Propósito | Prioridad |
|---------|-----------|-----------|
| `RegOpenKeyExA` / `RegCloseKey` | Abrir/cerrar registro | P1 |
| `RegQueryValueExA` / `RegSetValueExA` | Leer/escribir registro | P1 |
| `RegCreateKeyExA` / `RegDeleteKeyA` / `RegDeleteValueA` | Crear/eliminar | P1 |
| `RegEnumKeyExA` / `RegEnumValueA` | Enumerar | P2 |
| `OpenProcessToken` / `GetTokenInformation` | Tokens seguridad | P2 |
| `LookupPrivilegeValueA` / `AdjustTokenPrivileges` | Privilegios | P2 |
| `OpenSCManagerA` / `OpenServiceA` / `StartServiceA` | Servicios | P2 |
| `CryptAcquireContextA` / `CryptGenRandom` | Cripto legacy | P2 |
| `GetUserNameA` | Info usuario | P2 |
| `InitializeSecurityDescriptor` / `SetSecurityDescriptorDacl` | ACLs | P3 |

### 2.5 `ws2_32.dll` — Winsock 2 / Networking (NO EXISTE)

| Función | Propósito | Prioridad |
|---------|-----------|-----------|
| `WSAStartup` / `WSACleanup` | Inicialización | P0 |
| `socket` / `closesocket` | Crear/cerrar socket | P0 |
| `bind` / `listen` / `accept` / `connect` | Servidor/cliente | P0 |
| `send` / `recv` | Enviar/recibir TCP | P0 |
| `sendto` / `recvfrom` | UDP | P0 |
| `select` | Multiplexing | P0 |
| `ioctlsocket` / `setsockopt` / `getsockopt` | Control socket | P1 |
| `getaddrinfo` / `freeaddrinfo` | DNS resolution | P0 |
| `gethostbyname` / `gethostbyaddr` | DNS legacy | P1 |
| `inet_addr` / `inet_ntoa` / `inet_pton` / `inet_ntop` | Conversión IP | P0 |
| `htons` / `htonl` / `ntohs` / `ntohl` | Byte order | P0 |
| `WSAGetLastError` | Error handling | P0 |
| `WSAAsyncSelect` / `WSAEventSelect` | Async I/O | P1 |
| `WSASend` / `WSARecv` | Overlapped I/O | P1 |
| `WSAPoll` | Poll sockets | P1 |
| `shutdown` / `getpeername` / `getsockname` | Info socket | P1 |

### 2.6 `ole32.dll` + `oleaut32.dll` — COM/OLE (NO EXISTE — CRÍTICO PARA DX)

| Función | Propósito | Prioridad |
|---------|-----------|-----------|
| `CoInitialize` / `CoInitializeEx` | Inicializar COM | P0 |
| `CoUninitialize` | Cerrar COM | P0 |
| `CoCreateInstance` | Crear objeto COM | P0 |
| `CoTaskMemAlloc` / `CoTaskMemFree` | Memoria COM | P0 |
| `CoGetClassObject` | Class factory | P1 |
| `StringFromCLSID` / `CLSIDFromString` | Conversión GUID | P1 |
| `StringFromGUID2` / `IIDFromString` | IID conversion | P1 |
| `SysAllocString` / `SysFreeString` | BSTR strings | P1 |
| `VariantInit` / `VariantClear` / `VariantChangeType` | VARIANT | P1 |
| `SafeArrayCreate` / `SafeArrayDestroy` / `SafeArrayAccessData` | SAFEARRAY | P2 |
| `OleInitialize` / `OleUninitialize` | OLE init | P2 |

### 2.7 `shell32.dll` — Shell (NO EXISTE)

| Función | Propósito | Prioridad |
|---------|-----------|-----------|
| `ShellExecuteA` / `ShellExecuteExA` | Ejecutar/abrir archivos | P1 |
| `SHGetFolderPathA` / `SHGetKnownFolderPath` | Rutas especiales | P1 |
| `SHBrowseForFolderA` / `SHGetPathFromIDListA` | Diálogo carpetas | P2 |
| `SHFileOperationA` | Copiar/mover/borrar | P2 |
| `DragAcceptFiles` / `DragQueryFileA` / `DragFinish` | Drag & drop | P2 |
| `Shell_NotifyIconA` | Tray icon | P2 |
| `SHGetFileInfoA` | Info archivos | P2 |

### 2.8 `winmm.dll` — Multimedia (NO EXISTE)

| Función | Propósito | Prioridad |
|---------|-----------|-----------|
| `PlaySoundA` | Reproducir sonido | P1 |
| `waveOutOpen` / `waveOutClose` | Audio output | P1 |
| `waveOutWrite` / `waveOutPrepareHeader` / `waveOutUnprepareHeader` | Audio buffers | P1 |
| `waveOutSetVolume` / `waveOutGetVolume` | Volumen | P2 |
| `waveInOpen` / `waveInClose` / `waveInStart` / `waveInStop` | Audio input | P2 |
| `midiOutOpen` / `midiOutClose` / `midiOutShortMsg` | MIDI | P3 |
| `timeGetTime` / `timeBeginPeriod` / `timeEndPeriod` | Timer multimedia | P1 |
| `joyGetPosEx` / `joyGetNumDevs` | Joystick legacy | P3 |

### 2.9 `comdlg32.dll` — Diálogos Comunes (NO EXISTE)

| Función | Propósito | Prioridad |
|---------|-----------|-----------|
| `GetOpenFileNameA` / `GetSaveFileNameA` | Abrir/guardar | P1 |
| `ChooseColorA` | Color | P2 |
| `ChooseFontA` | Fuente | P2 |
| `PrintDlgA` | Imprimir | P3 |
| `FindTextA` / `ReplaceTextA` | Buscar/reemplazar | P3 |

### 2.10 `comctl32.dll` — Controles Comunes (NO EXISTE)

| Función | Propósito | Prioridad |
|---------|-----------|-----------|
| `InitCommonControls` / `InitCommonControlsEx` | Inicializar | P1 |
| `CreateStatusWindowA` | Status bar | P2 |
| `CreateToolbarEx` | Toolbar | P2 |
| `ImageList_Create` / `ImageList_Add` / `ImageList_Destroy` | Image lists | P2 |
| `TaskDialogIndirect` | Task dialog | P2 |
| Controles: ListView, TreeView, TabControl, ProgressBar, Rebar, ToolTip | UI avanzada | P2 |

### 2.11 `bcrypt.dll` — Crypto Next-Gen (NO EXISTE)

| Función | Propósito | Prioridad |
|---------|-----------|-----------|
| `BCryptOpenAlgorithmProvider` / `BCryptCloseAlgorithmProvider` | Algoritmo | P1 |
| `BCryptGenRandom` | Random crypto | P1 |
| `BCryptCreateHash` / `BCryptHashData` / `BCryptFinishHash` / `BCryptDestroyHash` | Hashing | P1 |
| `BCryptEncrypt` / `BCryptDecrypt` | AES | P1 |
| `BCryptGenerateKeyPair` / `BCryptFinalizeKeyPair` | Claves | P2 |
| `BCryptSignHash` / `BCryptVerifySignature` | Firmas | P2 |

### 2.12 `winhttp.dll` — HTTP (NO EXISTE)

| Función | Propósito | Prioridad |
|---------|-----------|-----------|
| `WinHttpOpen` / `WinHttpConnect` / `WinHttpOpenRequest` | HTTP client | P1 |
| `WinHttpSendRequest` / `WinHttpReceiveResponse` | Enviar/recibir | P1 |
| `WinHttpReadData` / `WinHttpQueryHeaders` | Leer datos | P1 |
| `WinHttpCloseHandle` | Cerrar handle | P1 |
| `WinHttpSetCredentials` | Autenticación | P2 |

### 2.13 DLLs Adicionales Necesarias

| DLL | Funciones Clave | Propósito | Prioridad |
|-----|----------------|-----------|-----------|
| `ntdll.dll` | NtQuerySystemInformation, NtCreateFile, RtlInitUnicodeString | NT nativo | P2 |
| `dbghelp.dll` | SymInitialize, SymFromAddr, StackWalk64, MiniDumpWriteDump | Debug/crash dump | P1 |
| `psapi.dll` | EnumProcesses, GetModuleInformation, GetProcessMemoryInfo | Info procesos | P2 |
| `iphlpapi.dll` | GetAdaptersInfo, GetNetworkParams | Info red | P2 |
| `version.dll` | GetFileVersionInfoA, VerQueryValueA | Versión archivos | P2 |
| `shlwapi.dll` | PathCombineA, PathFindExtensionA, PathIsDirectoryA | Path helpers | P2 |
| `setupapi.dll` | SetupDiGetClassDevs, SetupDiEnumDeviceInterfaces | Device enum | P3 |
| `hid.dll` | HidD_GetHidGuid, HidP_GetCaps | HID input | P3 |
| `xinput1_4.dll` | XInputGetState, XInputSetState, XInputGetCapabilities | Gamepad | P1 |
| `dwmapi.dll` | DwmExtendFrameIntoClientArea, DwmFlush | DWM composición | P2 |
| `uxtheme.dll` | SetWindowTheme, IsThemeActive, OpenThemeData | Visual styles | P2 |
| `rpcrt4.dll` | UuidCreate, UuidToStringA | UUID/RPC | P2 |
| `crypt32.dll` | CertOpenStore, CertFindCertificateInStore | Certificados | P2 |
| `mswsock.dll` | TransmitFile, AcceptEx, ConnectEx | Winsock ext | P2 |
| `secur32.dll` | AcquireCredentialsHandleA, InitializeSecurityContextA | SSPI auth | P3 |
| `powrprof.dll` | CallNtPowerInformation, SetSuspendState | Power mgmt | P3 |
| `userenv.dll` | GetUserProfileDirectoryA | Perfiles | P3 |

---

## ══════════════════════════════════════════════════
## PARTE 3: DIRECTX — COMPLETO DX9 → DX12
## ══════════════════════════════════════════════════

### 3.1 DirectX 9 (`d3d9.dll`)

#### IAT Necesario:
- `Direct3DCreate9` — P0
- `Direct3DCreate9Ex` — P1

#### Interfaces COM (vtable):

| Interface | Métodos Clave | Prioridad |
|-----------|--------------|-----------|
| `IDirect3D9` | CreateDevice, GetAdapterCount, GetAdapterDisplayMode, CheckDeviceType, GetDeviceCaps | P0 |
| `IDirect3DDevice9` | Clear, BeginScene, EndScene, Present, SetRenderState, SetTransform, DrawPrimitive, DrawIndexedPrimitive, CreateVertexBuffer, CreateIndexBuffer, CreateTexture, SetTexture, SetStreamSource, SetFVF, SetVertexShader, SetPixelShader, CreateVertexDeclaration, SetSamplerState, GetBackBuffer, SetRenderTarget, Reset, TestCooperativeLevel | P0 |
| `IDirect3DVertexBuffer9` | Lock, Unlock | P0 |
| `IDirect3DIndexBuffer9` | Lock, Unlock | P0 |
| `IDirect3DTexture9` | GetSurfaceLevel, LockRect, UnlockRect, GetLevelDesc, GenerateMipSubLevels | P0 |
| `IDirect3DSurface9` | LockRect, UnlockRect, GetDesc, GetDC, ReleaseDC | P1 |
| `IDirect3DSwapChain9` | Present, GetBackBuffer | P1 |
| `IDirect3DVertexShader9` | (IUnknown Release) | P0 |
| `IDirect3DPixelShader9` | (IUnknown Release) | P0 |
| `IDirect3DVertexDeclaration9` | GetDeclaration | P1 |
| `IDirect3DQuery9` | Issue, GetData | P2 |
| `IDirect3DStateBlock9` | Capture, Apply | P2 |
| `IDirect3DCubeTexture9` | GetCubeMapSurface, LockRect | P2 |

#### Estructuras/Tipos D3D9:

| Tipo | Estado |
|------|--------|
| `D3DPRESENT_PARAMETERS` (BackBufferWidth/Height, SwapEffect, Windowed, etc.) | 🔴 Falta |
| `D3DVIEWPORT9`, `D3DLIGHT9`, `D3DMATERIAL9`, `D3DCAPS9`, `D3DMATRIX` | 🔴 Falta |
| `D3DVERTEXELEMENT9`, `D3DLOCKED_RECT` | 🔴 Falta |
| Enums: `D3DFORMAT`, `D3DPOOL`, `D3DUSAGE`, `D3DPRIMITIVETYPE`, `D3DTRANSFORMSTATETYPE`, `D3DRENDERSTATETYPE`, `D3DSAMPLERSTATETYPE`, `D3DDEVTYPE`, `D3DSWAPEFFECT`, `D3DCULL`, `D3DFILLMODE` | 🔴 Falta |

#### D3DX9 Utility Library:

| Función | Propósito | Prioridad |
|---------|-----------|-----------|
| `D3DXCreateTextureFromFileA` | Textura de archivo | P1 |
| `D3DXCreateSprite` / `ID3DXSprite::Draw` | 2D sprites | P1 |
| `D3DXCreateFont` / `ID3DXFont::DrawTextA` | Texto | P1 |
| `D3DXCreateMeshFVF` / `D3DXLoadMeshFromXA` | Meshes | P2 |
| `D3DXCreateEffect` | Effects framework | P2 |
| `D3DXMatrixIdentity/Translation/RotationY/PerspectiveFovLH` | Matrices | P1 |
| `D3DXVec3Normalize/Cross/Dot` | Vectores | P1 |
| `D3DXCompileShaderFromFileA` | Compilar shaders | P1 |

### 3.2 DirectX 11 (`d3d11.dll` + `dxgi.dll`)

#### IAT Necesario:
- `d3d11.dll` → `D3D11CreateDevice`, `D3D11CreateDeviceAndSwapChain` — P0
- `dxgi.dll` → `CreateDXGIFactory`, `CreateDXGIFactory1`, `CreateDXGIFactory2` — P0
- `d3dcompiler_47.dll` → `D3DCompile`, `D3DCompileFromFile`, `D3DReflect`, `D3DCreateBlob` — P0

#### Interfaces COM (vtable) — DXGI:

| Interface | Métodos Clave | Prioridad |
|-----------|--------------|-----------|
| `IDXGIFactory` / `IDXGIFactory1` / `IDXGIFactory2` | CreateSwapChain, EnumAdapters, MakeWindowAssociation, CreateSwapChainForHwnd | P0 |
| `IDXGIAdapter` / `IDXGIAdapter1` | EnumOutputs, GetDesc, CheckInterfaceSupport | P0 |
| `IDXGIOutput` | GetDisplayModeList, FindClosestMatchingMode | P1 |
| `IDXGISwapChain` / `IDXGISwapChain1` | Present, GetBuffer, ResizeBuffers, SetFullscreenState | P0 |
| `IDXGIDevice` | GetAdapter | P1 |
| `IDXGISurface` | Map, Unmap, GetDesc | P2 |

#### Interfaces COM (vtable) — D3D11:

| Interface | Métodos Clave | Prioridad |
|-----------|--------------|-----------|
| `ID3D11Device` | CreateBuffer, CreateTexture2D, CreateShaderResourceView, CreateRenderTargetView, CreateDepthStencilView, CreateVertexShader, CreatePixelShader, CreateGeometryShader, CreateHullShader, CreateDomainShader, CreateComputeShader, CreateInputLayout, CreateBlendState, CreateDepthStencilState, CreateRasterizerState, CreateSamplerState, CreateQuery, CreateUnorderedAccessView | P0 |
| `ID3D11DeviceContext` | IASetInputLayout, IASetVertexBuffers, IASetIndexBuffer, IASetPrimitiveTopology, VSSetShader, PSSetShader, GSSetShader, HSSetShader, DSSetShader, CSSetShader, VSSetConstantBuffers, PSSetConstantBuffers, PSSetShaderResources, PSSetSamplers, RSSetViewports, RSSetScissorRects, OMSetRenderTargets, OMSetBlendState, OMSetDepthStencilState, Draw, DrawIndexed, DrawInstanced, DrawIndexedInstanced, Dispatch, ClearRenderTargetView, ClearDepthStencilView, Map, Unmap, UpdateSubresource, CopyResource, Flush | P0 |
| `ID3D11Buffer` / `ID3D11Texture2D` | GetDesc | P0 |
| `ID3D11RenderTargetView` / `ID3D11DepthStencilView` / `ID3D11ShaderResourceView` | GetDesc | P0 |
| `ID3D11SamplerState` / `ID3D11BlendState` / `ID3D11DepthStencilState` / `ID3D11RasterizerState` | GetDesc | P1 |
| `ID3D11InputLayout` / `ID3D11VertexShader` / `ID3D11PixelShader` / `ID3D11GeometryShader` | (Release) | P0 |
| `ID3D11HullShader` / `ID3D11DomainShader` / `ID3D11ComputeShader` | (Release) | P1 |
| `ID3D11UnorderedAccessView` | GetDesc | P1 |
| `ID3D11Query` | GetData | P2 |

#### Estructuras D3D11:

| Tipo | Estado |
|------|--------|
| `D3D11_BUFFER_DESC`, `D3D11_TEXTURE2D_DESC`, `D3D11_SUBRESOURCE_DATA`, `D3D11_MAPPED_SUBRESOURCE` | 🔴 |
| `D3D11_SHADER_RESOURCE_VIEW_DESC`, `D3D11_RENDER_TARGET_VIEW_DESC`, `D3D11_DEPTH_STENCIL_VIEW_DESC` | 🔴 |
| `D3D11_SAMPLER_DESC`, `D3D11_BLEND_DESC`, `D3D11_DEPTH_STENCIL_DESC`, `D3D11_RASTERIZER_DESC` | 🔴 |
| `D3D11_INPUT_ELEMENT_DESC`, `D3D11_VIEWPORT`, `D3D11_RECT`, `D3D11_BOX` | 🔴 |
| `DXGI_SWAP_CHAIN_DESC`, `DXGI_MODE_DESC`, `DXGI_SAMPLE_DESC`, `DXGI_ADAPTER_DESC` | 🔴 |
| Enums: `DXGI_FORMAT`, `D3D11_USAGE`, `D3D11_BIND_FLAG`, `D3D11_CPU_ACCESS_FLAG`, `D3D11_MAP`, `D3D_PRIMITIVE_TOPOLOGY`, `D3D11_FILTER`, `D3D11_TEXTURE_ADDRESS_MODE`, `D3D11_COMPARISON_FUNC`, `D3D11_BLEND`, `D3D11_BLEND_OP`, `D3D11_FILL_MODE`, `D3D11_CULL_MODE` | 🔴 |

### 3.3 DirectX 12 (`d3d12.dll` + `dxgi.dll`)

#### IAT Necesario:
- `d3d12.dll` → `D3D12CreateDevice`, `D3D12GetDebugInterface`, `D3D12SerializeRootSignature`, `D3D12SerializeVersionedRootSignature` — P0
- `dxgi.dll` → `CreateDXGIFactory2` — P0

#### Interfaces COM (vtable):

| Interface | Métodos Clave | Prioridad |
|-----------|--------------|-----------|
| `ID3D12Device` | CreateCommandQueue, CreateCommandAllocator, CreateGraphicsPipelineState, CreateComputePipelineState, CreateCommandList, CreateDescriptorHeap, CreateRootSignature, CreateConstantBufferView, CreateShaderResourceView, CreateUnorderedAccessView, CreateRenderTargetView, CreateDepthStencilView, CreateSampler, CreateCommittedResource, CreatePlacedResource, CreateFence, GetDescriptorHandleIncrementSize | P0 |
| `ID3D12CommandQueue` | ExecuteCommandLists, Signal, Wait, GetTimestampFrequency | P0 |
| `ID3D12CommandAllocator` | Reset | P0 |
| `ID3D12GraphicsCommandList` | Close, Reset, ClearRenderTargetView, ClearDepthStencilView, OMSetRenderTargets, RSSetViewports, RSSetScissorRects, IASetPrimitiveTopology, IASetVertexBuffers, IASetIndexBuffer, DrawInstanced, DrawIndexedInstanced, Dispatch, SetGraphicsRootSignature, SetComputeRootSignature, SetPipelineState, SetDescriptorHeaps, SetGraphicsRootDescriptorTable, SetGraphicsRoot32BitConstants, ResourceBarrier, CopyResource, CopyTextureRegion, CopyBufferRegion, ResolveSubresource, ExecuteBundle | P0 |
| `ID3D12DescriptorHeap` | GetCPUDescriptorHandleForHeapStart, GetGPUDescriptorHandleForHeapStart, GetDesc | P0 |
| `ID3D12RootSignature` / `ID3D12PipelineState` | (Release / GetCachedBlob) | P0 |
| `ID3D12Fence` | GetCompletedValue, SetEventOnCompletion, Signal | P0 |
| `ID3D12Resource` | Map, Unmap, GetGPUVirtualAddress, GetDesc | P0 |
| `ID3D12Heap` | GetDesc | P1 |
| `ID3D12GraphicsCommandList1-6` | BeginRenderPass, EndRenderPass, DispatchMesh, DispatchRays | P2 |
| **DXR** `ID3D12Device5` | CreateStateObject, GetRaytracingAccelerationStructurePrebuildInfo | P3 |
| **DXR** `ID3D12GraphicsCommandList4` | BuildRaytracingAccelerationStructure, DispatchRays | P3 |
| **Mesh** `ID3D12GraphicsCommandList6` | DispatchMesh | P3 |

#### Estructuras D3D12:

| Tipo | Estado |
|------|--------|
| `D3D12_COMMAND_QUEUE_DESC`, `D3D12_DESCRIPTOR_HEAP_DESC` | 🔴 |
| `D3D12_GRAPHICS_PIPELINE_STATE_DESC`, `D3D12_COMPUTE_PIPELINE_STATE_DESC` | 🔴 |
| `D3D12_ROOT_SIGNATURE_DESC`, `D3D12_ROOT_PARAMETER`, `D3D12_DESCRIPTOR_RANGE`, `D3D12_STATIC_SAMPLER_DESC` | 🔴 |
| `D3D12_RESOURCE_DESC`, `D3D12_HEAP_PROPERTIES`, `D3D12_CLEAR_VALUE` | 🔴 |
| `D3D12_VERTEX_BUFFER_VIEW`, `D3D12_INDEX_BUFFER_VIEW`, `D3D12_INPUT_ELEMENT_DESC` | 🔴 |
| `D3D12_SHADER_BYTECODE`, `D3D12_BLEND_DESC`, `D3D12_RASTERIZER_DESC`, `D3D12_DEPTH_STENCIL_DESC` | 🔴 |
| `D3D12_CPU_DESCRIPTOR_HANDLE`, `D3D12_GPU_DESCRIPTOR_HANDLE` | 🔴 |
| `D3D12_RESOURCE_BARRIER` (Transition, Aliasing, UAV) | 🔴 |
| `D3D12_VIEWPORT`, `D3D12_RECT`, `D3D12_BOX`, `D3D12_TEXTURE_COPY_LOCATION` | 🔴 |
| Enums: `D3D12_COMMAND_LIST_TYPE`, `D3D12_DESCRIPTOR_HEAP_TYPE/FLAGS`, `D3D12_HEAP_TYPE`, `D3D12_RESOURCE_STATES`, `D3D12_RESOURCE_DIMENSION/FLAGS`, `D3D12_ROOT_SIGNATURE_FLAGS`, `D3D12_ROOT_PARAMETER_TYPE`, `D3D12_SHADER_VISIBILITY`, `D3D12_DESCRIPTOR_RANGE_TYPE`, `D3D12_FILTER`, `D3D12_TEXTURE_ADDRESS_MODE`, `D3D12_PRIMITIVE_TOPOLOGY_TYPE`, `D3D12_FENCE_FLAGS`, `D3D12_RESOURCE_BARRIER_TYPE/FLAGS` | 🔴 |

### 3.4 Direct2D (`d2d1.dll`) — Gráficos 2D Acelerados

| Interface | Métodos Clave | Prioridad |
|-----------|--------------|-----------|
| `ID2D1Factory` | CreateHwndRenderTarget, CreateDCRenderTarget, CreatePathGeometry | P1 |
| `ID2D1HwndRenderTarget` | BeginDraw, EndDraw, Clear, DrawRectangle, FillRectangle, DrawEllipse, FillEllipse, DrawLine, DrawText, DrawBitmap, CreateSolidColorBrush, CreateLinearGradientBrush | P1 |
| `ID2D1SolidColorBrush` | SetColor, GetColor | P1 |
| `ID2D1PathGeometry` | Open, Close (via GeometrySink) | P2 |
| `ID2D1Bitmap` | GetSize, CopyFromRenderTarget | P1 |
| `ID2D1DeviceContext` (D2D1.1) | CreateBitmapFromDxgiSurface, SetTarget | P2 |

### 3.5 DirectWrite (`dwrite.dll`) — Texto

| Interface | Métodos Clave | Prioridad |
|-----------|--------------|-----------|
| `IDWriteFactory` | CreateTextFormat, CreateTextLayout, GetSystemFontCollection | P1 |
| `IDWriteTextFormat` | SetTextAlignment, SetParagraphAlignment, SetWordWrapping | P1 |
| `IDWriteTextLayout` | GetMetrics, SetFontSize, SetFontWeight | P2 |
| `DWriteCreateFactory` | Función de entrada | P1 |

### 3.6 XInput — Gamepad

| Función | Prioridad |
|---------|-----------|
| `XInputGetState` | P0 |
| `XInputSetState` | P0 |
| `XInputGetCapabilities` | P1 |
| `XInputEnable` | P2 |
| `XInputGetBatteryInformation` | P3 |

### 3.7 XAudio2 / DirectSound — Audio

| Interface/Función | Prioridad |
|-------------------|-----------|
| `XAudio2Create` (xaudio2_9.dll) | P1 |
| `IXAudio2::CreateSourceVoice`, `CreateMasteringVoice` | P1 |
| `IXAudio2SourceVoice::Start/Stop/SubmitSourceBuffer/SetVolume` | P1 |
| `DirectSoundCreate8` (dsound.dll) | P2 |
| `IDirectSound8::CreateSoundBuffer`, `SetCooperativeLevel` | P2 |
| `IDirectSoundBuffer8::Lock/Unlock/Play/Stop/SetVolume` | P2 |
| WASAPI: `IMMDeviceEnumerator`, `IAudioClient`, `IAudioRenderClient` | P2 |

### 3.8 DirectInput (legacy)

| Interface/Función | Prioridad |
|-------------------|-----------|
| `DirectInput8Create` (dinput8.dll) | P2 |
| `IDirectInput8::CreateDevice`, `EnumDevices` | P2 |
| `IDirectInputDevice8::SetDataFormat/SetCooperativeLevel/Acquire/GetDeviceState/Poll` | P2 |

---

## ══════════════════════════════════════════════════
## PARTE 4: OPENGL COMPLETO (1.0 → 4.6)
## ══════════════════════════════════════════════════

### Estado Actual:
- ✅ OpenGL 1.0-4.6 declaraciones en `adeb-stdlib/src/gpu/opengl/` (gl10.rs → gl46.rs)
- ✅ Constantes GL completas, GLSL shader bridge
- ✅ Test 10 pasa (OpenGL 1.1 context + rendering)

### FALTA para OpenGL Completo:

| Componente | Descripción | Prioridad |
|------------|-------------|-----------|
| `wglGetProcAddress` runtime extension loading | Cargar GL 2.0+ en runtime | P0 |
| GL Extension Loader (GLAD/GLEW equivalent) | Auto-carga funciones GL | P0 |
| WGL extensions: `wglSwapIntervalEXT`, `wglCreateContextAttribsARB`, `wglChoosePixelFormatARB` | Context moderno, VSync | P0 |
| ARB_debug_output / KHR_debug | Debug callbacks | P1 |
| ARB_framebuffer_object | FBOs | P1 |
| ARB_vertex_array_object | VAOs | P1 |
| ARB_shader_storage_buffer_object | SSBOs | P2 |
| ARB_compute_shader | Compute shaders GL | P2 |
| ARB_bindless_texture | Texturas sin binding | P3 |
| NV_mesh_shader / EXT_mesh_shader | Mesh shaders | P3 |

---

## ══════════════════════════════════════════════════
## PARTE 5: VULKAN COMPLETO
## ══════════════════════════════════════════════════

### Estado Actual:
- ✅ Vulkan declarations en `adeb-stdlib/src/gpu/`
- ✅ `ash` bindings + `gpu-allocator` en Cargo.toml
- ❌ Test 16 (Vulkan) requiere vtable fix

### FALTA:

| Componente | Descripción | Prioridad |
|------------|-------------|-----------|
| `vulkan-1.dll` IAT: `vkGetInstanceProcAddr`, `vkCreateInstance` | Entry point | P0 |
| Instance/Device functions loader | Via `vkGet*ProcAddr` | P0 |
| VK_KHR_surface / VK_KHR_win32_surface | Superficie Windows | P0 |
| VK_KHR_swapchain | Swap chain | P0 |
| VK_EXT_debug_utils | Validación | P0 |
| VK_KHR_dynamic_rendering (1.3) | Render sin render pass | P1 |
| VK_KHR_synchronization2 (1.3) | Sync moderno | P1 |
| VK_KHR_ray_tracing_pipeline | Ray tracing | P2 |
| VK_KHR_acceleration_structure | BLAS/TLAS | P2 |
| VK_EXT_mesh_shader | Mesh shaders | P3 |
| Validation layer integration | `VK_LAYER_KHRONOS_validation` | P1 |
| SPIR-V compiler equivalent | Compilar shaders | P1 |
| VMA equivalent | Memoria GPU | P1 |

---

## ══════════════════════════════════════════════════
## PARTE 6: KERNEL DEVELOPMENT (Ring 0)
## ══════════════════════════════════════════════════

### Estado Actual:
- ✅ `fastos_kernel.rs` parcial
- ✅ ASM-BIB `stdlib_ring3.pasm` (Ring 1-3)
- ❌ Sin soporte Ring 0

### 6.1 NT Kernel Headers (ntddk.h/wdm.h equivalentes)

| Componente | Funciones Clave | Prioridad |
|------------|----------------|-----------|
| **Driver Entry** | DriverEntry, DriverUnload, IRP dispatch | P0 |
| **IoManager** | IoCreateDevice, IoDeleteDevice, IoCreateSymbolicLink, IoCompleteRequest, IoGetCurrentIrpStackLocation | P0 |
| **Memory** | ExAllocatePoolWithTag, ExFreePoolWithTag, ExAllocatePool2, MmAllocateNonCachedMemory, MmMapIoSpace, MmMapLockedPagesSpecifyCache | P0 |
| **Process/Thread** | PsCreateSystemThread, PsTerminateSystemThread, PsGetCurrentProcess, KeGetCurrentIrql | P0 |
| **Sync** | KeInitializeSpinLock, KeAcquireSpinLock, KeReleaseSpinLock, KeInitializeMutex, KeWaitForSingleObject, KeSetEvent, KeInitializeEvent | P0 |
| **Registry** | ZwOpenKey, ZwQueryValueKey, ZwSetValueKey, ZwCreateKey, ZwClose | P1 |
| **File I/O** | ZwCreateFile, ZwReadFile, ZwWriteFile, ZwClose, ZwQueryInformationFile | P1 |
| **String** | RtlInitUnicodeString, RtlCompareUnicodeString, RtlCopyUnicodeString, RtlStringCbPrintfW | P0 |
| **Timer/DPC** | KeInitializeTimer, KeSetTimer, KeInitializeDpc, KeInsertQueueDpc | P1 |
| **Interrupts** | IoConnectInterrupt, IoDisconnectInterrupt, KeSynchronizeExecution | P1 |
| **PnP** | IoRegisterDeviceInterface, IoSetDeviceInterfaceState | P1 |
| **Power** | PoStartNextPowerIrp, PoCallDriver, PoRequestPowerIrp | P2 |
| **Debug** | DbgPrint, KdPrint, DbgBreakPoint | P0 |

### 6.2 Tipos del Kernel

| Tipo | Prioridad |
|------|-----------|
| `DRIVER_OBJECT`, `DEVICE_OBJECT`, `IRP`, `IO_STACK_LOCATION` | P0 |
| `UNICODE_STRING`, `NTSTATUS`, `KIRQL`, `LARGE_INTEGER` | P0 |
| `KSPIN_LOCK`, `KMUTEX`, `KEVENT`, `KTIMER`, `KDPC` | P0 |
| `MDL`, `POOL_TYPE`/`POOL_FLAGS` | P1 |
| `FILE_OBJECT`, `DEVICE_EXTENSION` | P1 |
| `LIST_ENTRY`, `SLIST_HEADER` | P0 |
| `IO_STATUS_BLOCK`, `OBJECT_ATTRIBUTES`, `KINTERRUPT` | P0 |

### 6.3 PE Driver Format

| Requisito | Prioridad |
|-----------|-----------|
| PE `Subsystem = NATIVE (1)` | P0 |
| `.INIT` section (descartable) + `.PAGE` section (pageable) | P0 |
| `_DRIVER_ENTRY@8` entry point | P0 |
| Sin CRT (no msvcrt.dll) | P0 |
| Import de `ntoskrnl.exe` + `hal.dll` | P0 |
| `.sys` output format | P0 |
| Code signing awareness | P2 |
| WDF support (kmdf/umdf) | P3 |

### 6.4 ASM-BIB Ring 0 Module (`stdlib_ring0.pasm` — NUEVO)

| Función ASM | Propósito | Prioridad |
|-------------|-----------|-----------|
| `asm_cli` / `asm_sti` | Disable/enable interrupts | P0 |
| `asm_hlt` | Halt CPU | P0 |
| `asm_in` / `asm_out` (8/16/32-bit) | Port I/O | P0 |
| `asm_rdmsr` / `asm_wrmsr` | Model-Specific Registers | P0 |
| `asm_invlpg` | Invalidar TLB entry | P0 |
| `asm_lidt` / `asm_sidt` | IDT load/store | P0 |
| `asm_lgdt` / `asm_sgdt` | GDT load/store | P0 |
| `asm_ltr` | Load Task Register | P1 |
| `asm_cpuid` | CPU identification | P0 |
| `asm_rdtsc` / `asm_rdtscp` | Timestamp counter | P0 |
| `asm_xgetbv` / `asm_xsetbv` | Extended control reg | P1 |
| `asm_swapgs` | Swap GS base (user↔kernel) | P0 |
| `asm_wbinvd` | Write-back + invalidate cache | P1 |
| `asm_mfence` / `asm_sfence` / `asm_lfence` | Memory fences | P0 |
| `asm_pause` | Spin-loop hint | P0 |
| `asm_cr0/cr3/cr4_read/write` | Control registers | P0 |
| `asm_iret` / `asm_sysret` / `asm_syscall` | Return from interrupt/syscall | P0 |

---

## ══════════════════════════════════════════════════
## PARTE 7: TOOLCHAIN — HERRAMIENTAS FALTANTES
## ══════════════════════════════════════════════════

### 7.1 Linker Features (MSVC LINK.exe equivalente)

| Feature | MSVC | ADead-BIB | Estado |
|---------|------|-----------|--------|
| EXE output | ✅ | ✅ | ✅ Funciona |
| DLL output (.dll + export table) | ✅ | ❌ | 🔴 Falta |
| .lib static library creation | ✅ | ❌ | 🔴 Falta |
| .lib import library generation | ✅ | ❌ | 🔴 Falta |
| DEF file processing | ✅ | ❌ | 🔴 Falta |
| Multiple .obj linking | ✅ | ⚠️ Solo bridge | 🟡 Parcial |
| Section merging | ✅ | ⚠️ Básico | 🟡 |
| Base relocations | ✅ | ✅ ASM-BIB | ✅ |
| ASLR (/DYNAMICBASE) | ✅ | ❌ | 🔴 Falta |
| DEP/NX (/NXCOMPAT) | ✅ | ❌ | 🔴 Falta |
| PDB generation | ✅ | ❌ | 🔴 Falta |
| LTCG/LTO | ✅ | ❌ | 🔴 Falta |
| Incremental linking | ✅ | ❌ | 🔴 Falta |
| Manifest embedding | ✅ | ❌ | 🔴 Falta |
| /SUBSYSTEM:NATIVE | ✅ | ❌ | 🔴 Falta (para drivers) |
| /ENTRY custom entry | ✅ | ❌ | 🔴 Falta |
| /STACK /HEAP size | ✅ | ❌ | 🔴 Falta |
| /OPT:REF /OPT:ICF | ✅ | ❌ | 🔴 Falta |
| /MAP file | ✅ | ❌ | 🔴 Falta |
| TLS (.tls section) | ✅ | ❌ | 🔴 Falta |
| Exception tables (.pdata/.xdata) | ✅ | ❌ | 🔴 Falta |
| Delay-load DLL | ✅ | ❌ | 🔴 Falta |

### 7.2 Resource Compiler (rc.exe equivalente)

| Feature | Estado |
|---------|--------|
| .rc file parsing | 🔴 Falta |
| ICON, CURSOR, BITMAP resource | 🔴 Falta |
| DIALOG, MENU, ACCELERATOR resource | 🔴 Falta |
| STRING TABLE, VERSIONINFO, MANIFEST | 🔴 Falta |
| Custom resource types | 🔴 Falta |
| .res → .obj embedding | 🔴 Falta |

### 7.3 MIDL Compiler (midl.exe equivalente)

| Feature | Estado |
|---------|--------|
| .idl file parsing | 🔴 Falta |
| COM interface generation | 🔴 Falta |
| Type library (.tlb) generation | 🔴 Falta |
| Proxy/stub generation | 🔴 Falta |

### 7.4 Debugger / Debug Info

| Feature | Estado |
|---------|--------|
| PDB generation | 🔴 Falta |
| DWARF debug info (GCC compat) | 🔴 Falta |
| Source-level debugging | 🔴 Falta |
| Line number tables | 🔴 Falta |
| Variable location info | 🔴 Falta |
| Stack frame info (.pdata/.xdata) | 🔴 Falta |
| Type info for debugger | 🔴 Falta |

### 7.5 Build System

| Feature | Estado |
|---------|--------|
| CMake toolchain support | 🔴 Falta |
| Precompiled headers | 🔴 Falta |
| Parallel compilation | 🔴 Falta |
| Dependency tracking | 🔴 Falta |
| Response files (@file) | 🔴 Falta |

### 7.6 Static Analyzer / Sanitizers

| Feature | Estado |
|---------|--------|
| UB Detector (21+ categorías) | ✅ Existe |
| Address Sanitizer (ASan) | 🔴 Falta |
| Thread Sanitizer (TSan) | 🔴 Falta |
| Static analysis | ✅ adeb-bg (parcial) |
| Code coverage | 🔴 Falta |
| Profile-Guided Optimization (PGO) | 🔴 Falta |

---

## ══════════════════════════════════════════════════
## PARTE 8: C++ STL COMPLETA
## ══════════════════════════════════════════════════

### Estado Actual: ~35 HPP templates generados

### 8.1 Containers

| Header | Estado |
|--------|--------|
| `<vector>`, `<list>`, `<deque>`, `<map>`, `<set>`, `<array>`, `<stack>`, `<queue>`, `<span>` | ✅ HPP template |
| `<unordered_map>`, `<unordered_set>` | ❌ Falta |
| `<forward_list>` | ❌ Falta |
| `<flat_map>`, `<flat_set>` (C++23) | ❌ Falta |
| `<mdspan>` (C++23) | ❌ Falta |

### 8.2 Strings & I/O

| Header | Estado |
|--------|--------|
| `<string>`, `<string_view>`, `<iostream>`, `<regex>` | ✅ HPP template |
| `<iomanip>`, `<sstream>`, `<fstream>` | ❌ Falta |
| `<format>` (C++20), `<print>` (C++23) | ❌ Falta |
| `<charconv>` (C++17) | ❌ Falta |
| `<locale>` | ❌ Falta |

### 8.3 Algorithms & Numerics

| Header | Estado |
|--------|--------|
| `<algorithm>`, `<numeric>`, `<random>` | ✅ HPP template |
| `<ranges>` (C++20), `<execution>` (C++17) | ❌ Falta |
| `<complex>`, `<valarray>` | ❌ Falta |
| `<bit>` (C++20), `<numbers>` (C++20) | ❌ Falta |

### 8.4 Memory

| Header | Estado |
|--------|--------|
| `<memory>` (unique_ptr, shared_ptr, weak_ptr) | ✅ HPP template |
| `<memory_resource>` (C++17), `<new>` | ❌ Falta |

### 8.5 Concurrency

| Header | Estado |
|--------|--------|
| `<thread>`, `<mutex>`, `<condition_variable>`, `<future>`, `<atomic>` | ✅ HPP template |
| `<semaphore>` (C++20), `<latch>`, `<barrier>` (C++20) | ❌ Falta |
| `<stop_token>`, `<jthread>` (C++20) | ❌ Falta |

### 8.6 Utilities

| Header | Estado |
|--------|--------|
| `<tuple>`, `<optional>`, `<variant>`, `<any>`, `<functional>`, `<utility>` | ✅ HPP template |
| `<type_traits>`, `<concepts>` (C++20) | ❌ Falta |
| `<coroutine>` (C++20) | ❌ Falta |
| `<expected>` (C++23), `<source_location>` (C++20) | ❌ Falta |
| `<typeinfo>`, `<typeindex>`, `<bitset>` | ❌ Falta |

### 8.7 C++ Language Runtime

| Feature | Estado |
|---------|--------|
| RTTI (typeid, dynamic_cast) | 🔴 Falta |
| Exceptions (try/catch/throw) | 🔴 Falta |
| Virtual function tables (vtable gen) | 🔴 Falta |
| Constructor/Destructor calls | 🔴 Falta |
| Name mangling (MSVC scheme) | ✅ `cpp_name_mangler.rs` existe |
| Operator overloading codegen | 🔴 Falta |
| Template instantiation | 🔴 Falta |
| Lambda codegen | 🔴 Falta |
| Move semantics codegen | 🔴 Falta |
| constexpr evaluation | 🔴 Falta |
| Structured bindings (C++17) | 🔴 Falta |
| Modules (C++20), Concepts (C++20), Coroutines (C++20) | 🔴 Falta |

---

## ══════════════════════════════════════════════════
## PARTE 9: ASM-BIB MÓDULOS NUEVOS NECESARIOS
## ══════════════════════════════════════════════════

### 9.1 Módulos stdlib

| Módulo | Funciones | Estado |
|--------|-----------|--------|
| **Ring 3 Base** (`stdlib_ring3.pasm`) | 21 func (string, math, bit, utility) | ✅ Existe |
| **Ring 0 Kernel** (`stdlib_ring0.pasm`) | cli/sti, hlt, in/out, rdmsr/wrmsr, cpuid, rdtsc, cr read/write, invlpg, lgdt, lidt, swapgs, iret, syscall, mfence/sfence/lfence, pause | 🔴 Falta |
| **SIMD SSE** (`stdlib_sse.pasm`) | memcpy_sse, memset_sse, strlen_sse, strcmp_sse (16-byte) | 🔴 Falta |
| **SIMD AVX** (`stdlib_avx.pasm`) | memcpy_avx, memset_avx, strlen_avx (32-byte) | 🔴 Falta |
| **SIMD AVX-512** (`stdlib_avx512.pasm`) | memcpy_avx512, memset_avx512 (64-byte) | 🔴 Falta |
| **Crypto** (`stdlib_crypto.pasm`) | aes_encrypt_block (AES-NI), sha256_block (SHA ext), crc32c (SSE4.2) | 🔴 Falta |
| **Threading** (`stdlib_thread.pasm`) | spinlock_acquire/release, atomic_cas/add/xchg, barrier | 🔴 Falta |
| **Float/Double** (`stdlib_float.pasm`) | float_add/mul/div/sqrt, float_to_int, int_to_float, double variants | 🔴 Falta |
| **System** (`stdlib_system.pasm`) | cpuid_check, get_tsc, get_cpu_freq, cache_flush, prefetch_data | 🔴 Falta |
| **Context Switch** (`stdlib_context.pasm`) | save_context, restore_context, switch_stack | 🔴 Falta |
| **Interrupt** (`stdlib_interrupt.pasm`) | isr_common_stub, save/restore_all_regs, idt_entry_setup | 🔴 Falta |
| **Paging** (`stdlib_paging.pasm`) | setup_pml4, map_page, unmap_page, flush_tlb, get/set_cr3 | 🔴 Falta |

### 9.2 Instrucciones x86-64 Faltantes en ASM-BIB Encoder

| Categoría | Instrucciones Clave | Estado |
|-----------|--------------------|--------|
| **SSE 128-bit** | MOVAPS/UPS, ADDPS/MULPS/SUBPS/DIVPS, SQRTPS, CMPPS, SHUFPS, PXOR/POR/PAND, PADD/PSUB, PMULLW/D, PSLL/PSRL/PSRA, PCMPEQ/PCMPGT, PSHUFB/PSHUFD, PUNPCKL/H, PEXTR/PINSR | 🟡 Parcial |
| **SSE2 double** | MOVAPD/UPD, ADDPD/MULPD/SUBPD/DIVPD, SQRTPD, MOVSD, ADDSD/MULSD/SUBSD/DIVSD, COMISD/UCOMISD | 🟡 Parcial |
| **SSE3/SSSE3** | HADDPS/HSUBPS, MOVDDUP, PALIGNR, PABS, PMULHRSW | 🟡 Parcial |
| **SSE4.1/4.2** | ROUNDPS/PD, BLENDPS/PD, DPPS, PMINSB/D, PMAXSB/D, PMOVSXBW/BD/BQ, PMOVZXBW/BD/BQ, PTEST, PCMPEQQ, CRC32, POPCNT | 🟡 Parcial |
| **AVX 256-bit** | V-prefixed SSE + VBROADCASTSS/SD, VPERMPD/PS, VINSERTF128, VEXTRACTF128, VZEROALL/VZEROUPPER | 🟡 VEX encoder existe |
| **AVX2 256-int** | VPADDB/W/D/Q, VPAND/POR/PXOR, VPMULLW/D, VPSHUFB, VPCMPEQ, VPGATHER, VPERMD/Q, VPBROADCAST | 🔴 Falta |
| **AVX-512** | VMOVDQA32/64 (512), VADDPS/PD (512), mask registers (k0-k7), VPCOMPRESSD, VPEXPANDD, VPTESTMD, VPTERNLOGD | 🔴 Falta |
| **BMI1/BMI2** | ANDN, BEXTR, BLSI, BLSMSK, BLSR, TZCNT, LZCNT, PDEP, PEXT, BZHI, MULX, RORX, SARX/SHRX/SHLX | 🔴 Falta |
| **FMA** | VFMADD132/213/231 PS/PD/SS/SD, VFMSUB, VFNMADD, VFNMSUB | 🔴 Falta |
| **AES-NI** | AESENC, AESENCLAST, AESDEC, AESDECLAST, AESIMC, AESKEYGENASSIST | 🔴 Falta |
| **SHA** | SHA1MSG1/2, SHA1NEXTE, SHA1RNDS4, SHA256MSG1/2, SHA256RNDS2 | 🔴 Falta |
| **x87 FPU** | FLD/FST/FSTP, FADD/FMUL/FSUB/FDIV, FCOM, FABS, FCHS, FSQRT, FSIN/FCOS/FPTAN/FPATAN, F2XM1, FYL2X, FSCALE, FRNDINT, FILD/FIST, FLDPI/FLDL2E/FLDLN2/FLDZ/FLD1, FXCH, FINIT, FCLEX, FWAIT | 🔴 Falta |
| **CLFLUSH/CLWB/CLFLUSHOPT** | Cache line flush | 🔴 Falta |
| **MOVBE** | Byte-swap load/store | 🔴 Falta |

---

## ══════════════════════════════════════════════════
## PARTE 10: CODEGEN FIXES REQUERIDOS (ISA COMPILER)
## ══════════════════════════════════════════════════

| # | Fix | Bloquea | Complejidad | Estado |
|---|-----|---------|-------------|--------|
| 1 | Struct stack allocation + field offset | Tests 06,07,15,18,21 + DX | Alta | 🔴 |
| 2 | Byte-level memory ops (memcpy/memset/memcmp/strchr) | Tests 02,14,19 | Media | 🔴 |
| 3 | `goto` / label forward-ref resolution | Tests 11-13 (DX) | Media | 🔴 |
| 4 | Vtable triple-pointer cast (`void***`) | Tests 11-13 (DX), 16 (Vulkan) | Alta | 🔴 |
| 5 | Function pointer typedef + indirect call (`call rax`) | Tests 11-13 (DX), 16, 22 | Alta | 🔴 |
| 6 | Unresolved label patches | Test 07 | Media | 🔴 |
| 7 | `%s` ternary expr in printf | Tests 03,05 (cosmético) | Baja | 🔴 |
| 8 | calloc zero-check loop | Test 04 | Media | 🔴 |
| 9 | realloc + strcmp | Test 04 | Media | 🔴 |
| 10 | Float/double arithmetic (SSE/x87 codegen) | Test 23, OpenGL, DX, math.h | Alta | 🔴 |
| 11 | Enum codegen completo | Test 20 | Baja | 🔴 |
| 12 | Deep recursion stack management | Test 17 | Media | 🔴 |
| 13 | Union type-punning | Test 18 | Media | 🔴 |
| 14 | Global/static variables | Múltiples | Media | 🔴 |
| 15 | Array initializer lists (`int a[] = {1,2,3}`) | Múltiples | Media | 🔴 |
| 16 | Compound literals (`(Type){...}`) | DX, COM | Media | 🔴 |
| 17 | Designated initializers (`.field = value`) | DX, COM | Media | 🔴 |
| 18 | Cast codegen (pointer/integer casts) | DX, COM | Media | 🔴 |
| 19 | Comma operator | Varios | Baja | 🔴 |
| 20 | sizeof codegen (tipos complejos) | Structs, arrays | Media | 🔴 |
| 21 | Conditional compilation (`#if defined(...)`) | Headers complejos | Media | 🔴 |
| 22 | Multi-dimensional arrays (`a[i][j][k]`) | Varios | Media | 🔴 |
| 23 | String literal concatenation (`"hello" " world"`) | Varios | Baja | 🔴 |
| 24 | Bitfield struct members | Drivers, protocolos | Alta | 🔴 |
| 25 | Volatile qualifier handling | Hardware I/O, kernel | Media | 🔴 |
| 26 | Inline assembly (`__asm {}` / `asm()`) | Performance, kernel | Alta | 🔴 |
| 27 | SEH (`__try/__except/__finally`) | Windows kernel, drivers | Alta | 🔴 |
| 28 | Thread-local storage (`__declspec(thread)`) | Multi-threading | Media | 🔴 |

---

## ══════════════════════════════════════════════════
## PARTE 11: ROADMAP DE PRIORIDADES
## ══════════════════════════════════════════════════

### FASE 1 — Fundamentos C (Semanas 1-4)
1. Fix struct codegen (#1) → desbloquea 5+ tests
2. Fix byte-level memory (#2) → desbloquea Test 02 completo
3. Fix goto/labels (#3) + vtable (#4) + fn pointers (#5) → desbloquea DX9-12
4. Float/double codegen (#10) → desbloquea math.h y OpenGL avanzado
5. Expandir msvcrt.dll IAT: fopen/fclose/fread/fwrite/fprintf/sscanf/atoi/atof/strtol/qsort/rand/getenv

### FASE 2 — Win32 Completo (Semanas 5-8)
1. kernel32.dll: threading (CreateThread/Wait/Mutex/CriticalSection/Interlocked)
2. user32.dll: GetMessage/BeginPaint/GetClientRect/GetKeyState/GetCursorPos
3. gdi32.dll: CreateCompatibleDC/BitBlt/CreateFont/TextOut
4. ole32.dll: CoInitializeEx/CoCreateInstance/CoTaskMemAlloc (prerequisito DX)
5. ws2_32.dll: WSAStartup/socket/bind/listen/accept/connect/send/recv

### FASE 3 — DirectX (Semanas 9-14)
1. d3d9.dll IAT + IDirect3D9/IDirect3DDevice9 vtable structs
2. d3d11.dll + dxgi.dll IAT + structs + enums completos
3. d3d12.dll IAT + ID3D12Device/CommandQueue/GraphicsCommandList
4. d3dcompiler_47.dll: D3DCompile para HLSL
5. d2d1.dll + dwrite.dll: Direct2D + DirectWrite
6. XInput, XAudio2

### FASE 4 — Linker & Toolchain (Semanas 15-18)
1. DLL output (.dll + export table)
2. .lib static library creation
3. Resource compiler (.rc parsing + .res embedding)
4. PDB / debug info generation
5. ASLR + DEP + manifest support
6. /SUBSYSTEM:NATIVE para drivers

### FASE 5 — Kernel & Ring 0 (Semanas 19-22)
1. ASM-BIB `stdlib_ring0.pasm` module
2. NT kernel types (DRIVER_OBJECT, IRP, etc.)
3. ntoskrnl.exe import support
4. .sys driver output format
5. Inline assembly support

### FASE 6 — C++ Runtime (Semanas 23-30)
1. Vtable generation + virtual dispatch
2. Constructor/destructor call chains
3. Exception handling (try/catch/throw)
4. Template instantiation
5. RTTI
6. Lambda + move semantics codegen

### FASE 7 — Optimización & Polish (Semanas 31+)
1. SIMD ASM-BIB modules (SSE/AVX/AVX-512)
2. Crypto ASM-BIB module (AES-NI, SHA)
3. ASan / TSan sanitizers
4. PGO / LTO
5. CMake toolchain file
6. Vulkan complete + SPIR-V compiler

---

## ══════════════════════════════════════════════════
## RESUMEN ESTADÍSTICO TOTAL
## ══════════════════════════════════════════════════

| Categoría | Items Totales | ✅ Existe | 🟡 Parcial | 🔴 Falta |
|-----------|--------------|-----------|------------|----------|
| C libc funciones | ~200 | 25 | 8 | ~167 |
| C++ STL headers | ~70 | 35 | 0 | ~35 |
| C++ runtime features | 15 | 1 | 0 | 14 |
| Win32 DLLs (IAT) | 25+ | 5 | 0 | 20+ |
| Win32 funciones totales | ~600 | ~50 | 0 | ~550 |
| DirectX interfaces | ~50 | 0 | 0 | ~50 |
| DirectX structs/enums | ~100 | 0 | 0 | ~100 |
| OpenGL funciones | 2000+ | 2000+ decl | 0 | loader |
| Vulkan funciones | 500+ | 500+ decl | 0 | loader |
| Kernel tipos/funciones | ~80 | 5 | 0 | ~75 |
| ASM-BIB módulos | 12 | 1 | 0 | 11 |
| ASM-BIB instrucciones | ~400 ISA | ~150 | ~50 | ~200 |
| Codegen fixes | 28 | 0 | 0 | 28 |
| Toolchain features | 25 | 3 | 2 | 20 |
| **TOTAL ITEMS** | **~4000+** | **~2800** | **~60** | **~1200** |

> **Nota:** Los ~2800 items "existentes" son mayormente declaraciones (OpenGL, Vulkan, C++ HPP templates).  
> Los items funcionales probados y verificados son ~50 (funciones C + Win32 IAT que pasan tests).

---

*Reporte generado para ADead-BIB v9.0 + ASM-BIB v2.0*  
*Objetivo: Reemplazar MSVC como toolchain completa para aplicaciones Windows*  
*Incluye: libc, STL, Win32, COM, DirectX 9-12, OpenGL, Vulkan, Kernel, Toolchain*
