# tests/c — ADead-BIB C Test Suite

> Suite de pruebas y fixtures del compilador C de ADead-BIB  
> Organizada para separar documentación, fixtures y futuras pruebas de integración

---

## Estructura

```text
tests/c/
├── README.md
└── fixtures/
    ├── 01_ctype_basic.c
    ├── 02_ctype_extended.c
    ├── 03_ctype_loop_parser.c
    └── 04_ctype_edge_cases.c
```

## Fixtures disponibles

| Fixture | Header | Descripción | Estado |
|---|---|---|---|
| `01_ctype_basic.c` | `<ctype.h>` | isalpha, isdigit, isalnum, isspace, isupper, islower | ✅ |
| `02_ctype_extended.c` | `<ctype.h>` | isprint, isgraph, iscntrl, ispunct, isxdigit, isblank, toupper, tolower | ✅ |
| `03_ctype_loop_parser.c` | `<ctype.h>` | Uso real: clasificar string, to_upper, parse_hex, validar identifier | ✅ |
| `04_ctype_edge_cases.c` | `<ctype.h>` | Boundaries: NUL, EOF, 0x1F, 0x7F, toupper/tolower con no-letters | ✅ |

## Cómo ejecutar

```bash
# Compilar un fixture individual
adb cc tests/c/fixtures/01_ctype_basic.c -o test_ctype.exe

# Compilar y ejecutar
adb run tests/c/fixtures/01_ctype_basic.c

# Step mode — ver pipeline completo
adb step tests/c/fixtures/01_ctype_basic.c
```

## Convención de nombres

```text
XX_header_descripcion.c
│  │      │
│  │      └── qué se prueba
│  └── header principal
└── número secuencial
```
