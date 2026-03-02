# musl libc — FastOS Core Library

**Reemplazo de glibc para FastOS. C puro, sin overhead.**

## Descarga Oficial

```bash
# Sitio oficial
https://musl.libc.org/

# Git repository
git clone git://git.musl-libc.org/musl

# Versión actual: 1.2.5
wget https://musl.libc.org/releases/musl-1.2.5.tar.gz
```

## Compilación con ADead-BIB

```bash
# Extraer
tar xzf musl-1.2.5.tar.gz
cd musl-1.2.5

# Configurar para ADead-BIB
./configure --prefix=/opt/fastos \
            --target=x86_64-fastos \
            CC="adB cc" \
            CFLAGS="-O2 -fno-stack-protector"

# Compilar
make -j$(nproc)
make install
```

## Componentes Principales

| Archivo | Función | Prioridad |
|---------|---------|-----------|
| `src/string/` | strlen, strcpy, memcpy | 🔴 CRÍTICO |
| `src/stdlib/` | malloc, free, atoi | 🔴 CRÍTICO |
| `src/stdio/` | printf, fopen, fread | 🔴 CRÍTICO |
| `src/math/` | sin, cos, sqrt | 🔴 CRÍTICO |
| `src/thread/` | pthread_create | 🟠 ALTA |
| `src/network/` | socket, connect | 🟠 ALTA |

## Tamaño

| Versión | glibc | musl | ADead-BIB |
|---------|-------|------|-----------|
| Completo | ~2MB | ~550KB | ~50KB |
| Mínimo | ~800KB | ~200KB | ~20KB |

## Licencia

MIT License — Libre para FastOS
