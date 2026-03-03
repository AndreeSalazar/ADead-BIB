# ADead-BIB Standard Library v2.0

Complete C standard library with POSIX extensions, networking, and Vulkan support.
Optimized for ADead-BIB's PDP-11/VAX/x86/68000 heritage IR.

## Philosophy

```
C arquitecturas originales → ADead-BIB IR:
├── PDP-11 → IR_LOAD/IR_STORE (auto-increment/decrement)
├── VAX    → IR_FRAME (formal stack frames)
├── x86    → IR_SEGMENT (flat memory model)
└── 68000  → IR_REGISTER (typed registers)
```

## Structure

```
stdlib/
├── c/                    # C Standard Library (C99/C11/POSIX)
│   ├── stdio.h           # Standard I/O
│   ├── stdlib.h          # General utilities
│   ├── string.h          # String operations
│   ├── math.h            # Mathematical functions
│   ├── time.h            # Date and time
│   ├── errno.h           # Error numbers
│   ├── signal.h          # Signal handling
│   ├── pthread.h         # POSIX threads
│   ├── unistd.h          # POSIX standard
│   ├── fcntl.h           # File control
│   ├── dirent.h          # Directory entries
│   ├── dlfcn.h           # Dynamic linking
│   ├── poll.h            # I/O multiplexing
│   ├── termios.h         # Terminal I/O
│   ├── complex.h         # Complex numbers
│   ├── fenv.h            # Floating-point environment
│   ├── wchar.h           # Wide characters
│   ├── locale.h          # Localization
│   ├── setjmp.h          # Non-local jumps
│   ├── inttypes.h        # Integer format conversion
│   ├── netdb.h           # Network database
│   ├── sys/
│   │   ├── types.h       # System data types
│   │   ├── stat.h        # File status
│   │   ├── socket.h      # BSD sockets
│   │   ├── mman.h        # Memory management
│   │   ├── wait.h        # Process wait
│   │   ├── ioctl.h       # I/O control
│   │   ├── select.h      # I/O multiplexing
│   │   └── epoll.h       # Linux epoll
│   ├── netinet/
│   │   └── in.h          # Internet address family
│   ├── arpa/
│   │   └── inet.h        # Address manipulation
│   └── vulkan/
│       └── vulkan.h      # Vulkan 1.3 API
├── cpp/                  # C++ Standard Library
├── fasm/                 # FASM macros
├── graphics/             # Graphics libraries
└── os/                   # OS-specific
```

## Uso

```c
// C example
#include <stdio.h>
#include <stdlib.h>

int main() {
    printf("Hello from ADead-BIB!\n");
    return 0;
}
```

```cpp
// C++ example
#include <iostream>

int main() {
    std::cout << "Hello from ADead-BIB C++!" << std::endl;
    return 0;
}
```

## Compilación

```bash
# C
adB cc myfile.c -o myfile.exe

# C++
adB cxx myfile.cpp -o myfile.exe

# OS/Kernel (flat binary)
adB cc kernel.c -o kernel.bin --flat
```

## FASM Integration

ADead-BIB puede trabajar junto con FASM para optimizaciones de bajo nivel.
Ver `fasm/README.md` para instrucciones de configuración.
