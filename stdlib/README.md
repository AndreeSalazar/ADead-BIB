# ADead-BIB Standard Library

Librerías C y C++ nativas para ADead-BIB compiler.
Compatible con desarrollo de OS, aplicaciones nativas, y sistemas embebidos.

## Estructura

```
stdlib/
├── c/                    # C Standard Library Headers
│   ├── stdio.h          # Input/Output
│   ├── stdlib.h         # General utilities
│   ├── string.h         # String handling
│   ├── stdint.h         # Integer types
│   ├── stddef.h         # Common definitions
│   └── stdbool.h        # Boolean type
├── cpp/                  # C++ Standard Library Headers
│   ├── cstdio           # C stdio wrapper
│   ├── cstdlib          # C stdlib wrapper
│   ├── cstring          # C string wrapper
│   └── iostream         # Basic I/O streams
├── os/                   # OS Development Headers
│   ├── kernel.h         # Kernel primitives
│   ├── interrupts.h     # Interrupt handling
│   └── memory.h         # Memory management
├── graphics/             # Graphics Headers
│   ├── vulkan.h         # Vulkan basics
│   └── png.h            # PNG support
└── fasm/                 # FASM Integration
    ├── README.md        # FASM setup guide
    └── macros.inc       # FASM macros for ADead-BIB
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
