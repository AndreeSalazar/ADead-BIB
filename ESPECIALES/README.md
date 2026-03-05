# ADead-BIB — Carpeta ESPECIALES
## Headers Globales para Compilación Nativa

Esta carpeta contiene **headers de sistema globales** diseñados específicamente para ADead-BIB.
Permiten compilar código C/C++ con acceso a Win32, FastOS y la biblioteca estándar sin depender
de LLVM, GCC ni Clang.

---

## 📁 Headers disponibles

| Header | Descripción |
|--------|-------------|
| `windows.h` | API Win32/Win64 completa: tipos, macros, kernel32, user32, gdi32, advapi32, winsock2 |
| `fastOs.h` | Interfaz nativa FastOS: syscalls, formato `.po`, threads, sockets, gestión de memoria |
| `stdint.h` | Tipos enteros exactos: `int8_t` → `uint64_t`, límites, literales |
| `stdio.h` | I/O estándar: printf/scanf, FILE, fopen/fclose, putchar/getchar |
| `stdlib.h` | Biblioteca estándar: malloc/free, exit/abort, atoi, rand, qsort, bsearch |
| `string.h` | Operaciones de cadena: strlen, strcpy, memcpy, strcmp, strstr, popcount |
| `math.h` | Matemáticas: sin/cos/tan, sqrt/pow/log/exp, constantes π/e/τ |

---

## 🚀 Pipeline .exe → .po (FastOS)

ADead-BIB soporta compilación **nativa para FastOS** generando archivos `.po` (Portable Object).
El formato `.po` es el equivalente FastOS de los `.exe` de Windows o los ELF de Linux.

### Compilar para Windows (.exe)
```
adeadc cxx mi_programa.cpp -o mi_programa.exe
```

### Compilar para FastOS (.po)
```
adeadc cxx mi_programa.cpp --target=fastos -o mi_programa.po
```

### Estructura del .po header
```c
#include "ESPECIALES/fastOs.h"

int fo_main(int argc, char** argv) {
    fo_puts("Hola desde FastOS!\n");
    fo_exit(0);
    return 0;
}
```

---

## 🪟 Win32 API — Tipos clave

```c
#include "ESPECIALES/windows.h"

// Tipos
DWORD   dwValue = 0;
HANDLE  hFile = INVALID_HANDLE_VALUE;
BOOL    bResult = FALSE;
LPSTR   pszText = NULL;

// Funciones kernel32
hFile = CreateFileA("datos.txt", GENERIC_READ, 0, NULL, OPEN_EXISTING, 0, NULL);
WriteConsoleA(GetStdHandle(STD_OUTPUT_HANDLE), "Hola\n", 5, NULL, NULL);

// GUI
MessageBoxA(NULL, "ADead-BIB", "Compilado con ADead-BIB", MB_OK | MB_ICONINFORMATION);
```

---

## 📡 WinSock 2 — Red

```c
#include "ESPECIALES/windows.h"

WSADATA wsaData;
WSAStartup(MAKEWORD(2, 2), &wsaData);

SOCKET sock = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP);
struct sockaddr_in addr;
addr.sin_family = AF_INET;
addr.sin_port = htons(8080);
addr.sin_addr.s_addr = inet_addr("127.0.0.1");

connect(sock, (sockaddr*)&addr, sizeof(addr));
send(sock, "GET / HTTP/1.0\r\n\r\n", 18, 0);
closesocket(sock);
WSACleanup();
```

---

## 🔢 FastOS Syscalls

```c
#include "ESPECIALES/fastOs.h"

// Alto nivel
fo_write(FD_STDOUT, "Hola\n", 5);
void* mem = fo_alloc(1024);
fo_free(mem);

// Bajo nivel
__fastos_syscall3(SYS_WRITE, FD_STDOUT, (u64)"Hola\n", 5);
```

---

## ⚡ Integración con ADead-BIB

Para usar estos headers desde tu código C/C++:

```cpp
// C++
#include "../../ESPECIALES/windows.h"
#include "../../ESPECIALES/fastOs.h"

int main() {
    MessageBoxA(NULL, "ADead-BIB compila Win32!", "Test", MB_OK);
    return 0;
}
```

```c
// C  
#include "ESPECIALES/math.h"
#include "ESPECIALES/string.h"

int main() {
    double r = sqrt(16.0);  // 4.0
    char buf[64];
    strcpy(buf, "Compilado con ADead-BIB");
    return 0;
}
```

---

## 📦 Versiones compatibles

| Sistema | Versión mínima |
|---------|---------------|
| Windows | 10 (SDK 10.0.19041+) |
| Windows Server | 2016+ |
| FastOS | 1.0+ |
| ADead-BIB | 0.1.0+ |

> **Nota**: Estos headers son la implementación nativa de ADead-BIB.
> No requieren MSVC, MinGW, LLVM, ni ningún otro toolchain externo.
> ADead-BIB genera el código nativo directamente — **Sin GCC. Sin LLVM. Sin Clang. Solo ADead-BIB. 💀🦈**
