# Test_c — ADead-BIB C99 Test Suite

> Tests de verificación para la stdlib C de ADead-BIB  
> Cada test se compila con `adb cc` y verifica una funcionalidad específica

---

## Tests Disponibles

| Test | Header | Descripción | Estado |
|------|--------|-------------|--------|
| `01_ctype_basic.c` | `<ctype.h>` | isalpha, isdigit, isalnum, isspace, isupper, islower | ✅ |
| `02_ctype_extended.c` | `<ctype.h>` | isprint, isgraph, iscntrl, ispunct, isxdigit, isblank, toupper, tolower | ✅ |
| `03_ctype_loop_parser.c` | `<ctype.h>` | Uso real: clasificar string, to_upper, parse_hex, validar identifier | ✅ |
| `04_ctype_edge_cases.c` | `<ctype.h>` | Boundaries: NUL, EOF, 0x1F, 0x7F, toupper/tolower con no-letters | ✅ |

## Cómo Ejecutar

```bash
# Compilar un test individual
adb cc Test_c/01_ctype_basic.c -o test_ctype.exe

# Compilar y ejecutar
adb run Test_c/01_ctype_basic.c

# Step mode — ver pipeline completo
adb step Test_c/01_ctype_basic.c

# Todos los tests
for f in Test_c/*.c; do adb cc "$f" -o "bin/$(basename $f .c).exe"; done
```

## Convención de Nombres

```
XX_header_descripcion.c
│  │      │
│  │      └── qué se prueba
│  └── header principal
└── número secuencial
```

## Output Esperado

Cada test incluye al final un comentario con el output esperado exacto.
Si la salida difiere, el test falla.

---

*ADead-BIB Test Canon — C99 compliant — 💀🦈*
