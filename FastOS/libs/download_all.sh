#!/bin/bash
# FastOS Libraries — Download Script
# Descarga todas las librerías C/C++ necesarias para FastOS
# Compilables con ADead-BIB

set -e

LIBS_DIR="$(dirname "$0")"
cd "$LIBS_DIR"

echo "╔══════════════════════════════════════════════════════════════════╗"
echo "║         FastOS Libraries — ADead-BIB Download Script            ║"
echo "╚══════════════════════════════════════════════════════════════════╝"

# ============================================================
# FASE 1 — CORE (Base absoluta del OS)
# ============================================================
echo ""
echo "[FASE 1] Descargando Core Libraries..."
mkdir -p core

# musl libc — Reemplazo de glibc
echo "  → musl libc 1.2.5"
wget -q -P core/musl https://musl.libc.org/releases/musl-1.2.5.tar.gz

# ============================================================
# FASE 2 — COMPRESSION
# ============================================================
echo ""
echo "[FASE 2] Descargando Compression Libraries..."
mkdir -p compression

# zlib — Base de compresión
echo "  → zlib 1.3.1"
wget -q -P compression/zlib https://zlib.net/zlib-1.3.1.tar.gz

# lz4 — Ultra rápido
echo "  → lz4 1.9.4"
wget -q -P compression/lz4 https://github.com/lz4/lz4/archive/refs/tags/v1.9.4.tar.gz

# zstd — Moderno eficiente
echo "  → zstd 1.5.5"
wget -q -P compression/zstd https://github.com/facebook/zstd/releases/download/v1.5.5/zstd-1.5.5.tar.gz

# bzip2 — Clásico
echo "  → bzip2 1.0.8"
wget -q -P compression/bzip2 https://sourceware.org/pub/bzip2/bzip2-1.0.8.tar.gz

# xz/liblzma — Máxima compresión
echo "  → xz 5.4.5"
wget -q -P compression/xz https://tukaani.org/xz/xz-5.4.5.tar.gz

# ============================================================
# FASE 3 — IMAGES
# ============================================================
echo ""
echo "[FASE 3] Descargando Image Libraries..."
mkdir -p images

# libpng — PNG
echo "  → libpng 1.6.40"
wget -q -P images/libpng https://download.sourceforge.net/libpng/libpng-1.6.40.tar.gz

# libjpeg-turbo — JPEG optimizado
echo "  → libjpeg-turbo 3.0.1"
wget -q -P images/libjpeg https://github.com/libjpeg-turbo/libjpeg-turbo/releases/download/3.0.1/libjpeg-turbo-3.0.1.tar.gz

# libwebp — WebP moderno
echo "  → libwebp 1.3.2"
wget -q -P images/libwebp https://storage.googleapis.com/downloads.webmproject.org/releases/webp/libwebp-1.3.2.tar.gz

# stb — Header-only image loaders
echo "  → stb (header-only)"
mkdir -p images/stb
wget -q -O images/stb/stb_image.h https://raw.githubusercontent.com/nothings/stb/master/stb_image.h
wget -q -O images/stb/stb_image_write.h https://raw.githubusercontent.com/nothings/stb/master/stb_image_write.h

# ============================================================
# FASE 4 — AUDIO
# ============================================================
echo ""
echo "[FASE 4] Descargando Audio Libraries..."
mkdir -p audio

# libogg — Container OGG
echo "  → libogg 1.3.5"
wget -q -P audio/libogg https://downloads.xiph.org/releases/ogg/libogg-1.3.5.tar.gz

# libvorbis — OGG Vorbis codec
echo "  → libvorbis 1.3.7"
wget -q -P audio/libvorbis https://downloads.xiph.org/releases/vorbis/libvorbis-1.3.7.tar.gz

# libopus — Opus moderno
echo "  → libopus 1.4"
wget -q -P audio/libopus https://downloads.xiph.org/releases/opus/opus-1.4.tar.gz

# libflac — FLAC lossless
echo "  → libflac 1.4.3"
wget -q -P audio/libflac https://downloads.xiph.org/releases/flac/flac-1.4.3.tar.xz

# miniaudio — Header-only audio
echo "  → miniaudio (header-only)"
mkdir -p audio/miniaudio
wget -q -O audio/miniaudio/miniaudio.h https://raw.githubusercontent.com/mackron/miniaudio/master/miniaudio.h

# ============================================================
# FASE 5 — NETWORK
# ============================================================
echo ""
echo "[FASE 5] Descargando Network Libraries..."
mkdir -p network

# libcurl — HTTP/HTTPS
echo "  → libcurl 8.5.0"
wget -q -P network/libcurl https://curl.se/download/curl-8.5.0.tar.gz

# mbedtls — TLS embebido (más simple que OpenSSL)
echo "  → mbedtls 3.5.1"
wget -q -P network/mbedtls https://github.com/Mbed-TLS/mbedtls/archive/refs/tags/v3.5.1.tar.gz

# c-ares — DNS async
echo "  → c-ares 1.24.0"
wget -q -P network/c-ares https://c-ares.org/download/c-ares-1.24.0.tar.gz

# ============================================================
# FASE 6 — FONTS
# ============================================================
echo ""
echo "[FASE 6] Descargando Font Libraries..."
mkdir -p fonts

# freetype2 — Render TTF/OTF
echo "  → freetype 2.13.2"
wget -q -P fonts/freetype https://download.savannah.gnu.org/releases/freetype/freetype-2.13.2.tar.gz

# harfbuzz — Text shaping
echo "  → harfbuzz 8.3.0"
wget -q -P fonts/harfbuzz https://github.com/harfbuzz/harfbuzz/releases/download/8.3.0/harfbuzz-8.3.0.tar.xz

# ============================================================
# FASE 7 — DATABASE
# ============================================================
echo ""
echo "[FASE 7] Descargando Database Libraries..."
mkdir -p database

# sqlite3 — SQL embebido
echo "  → sqlite 3.44.2"
wget -q -P database/sqlite https://www.sqlite.org/2023/sqlite-autoconf-3440200.tar.gz

# ============================================================
# FASE 8 — PARSERS
# ============================================================
echo ""
echo "[FASE 8] Descargando Parser Libraries..."
mkdir -p parsers

# expat — XML parser
echo "  → expat 2.5.0"
wget -q -P parsers/expat https://github.com/libexpat/libexpat/releases/download/R_2_5_0/expat-2.5.0.tar.gz

# cJSON — JSON parser
echo "  → cJSON 1.7.16"
wget -q -P parsers/cjson https://github.com/DaveGamble/cJSON/archive/refs/tags/v1.7.16.tar.gz

# jsmn — JSON tokenizer mínimo (header-only)
echo "  → jsmn (header-only)"
mkdir -p parsers/jsmn
wget -q -O parsers/jsmn/jsmn.h https://raw.githubusercontent.com/zserge/jsmn/master/jsmn.h

# ============================================================
# FASE 9 — GRAPHICS/GPU
# ============================================================
echo ""
echo "[FASE 9] Descargando Graphics Libraries..."
mkdir -p display

# Vulkan Headers
echo "  → Vulkan Headers 1.3.270"
wget -q -P display/vulkan https://github.com/KhronosGroup/Vulkan-Headers/archive/refs/tags/v1.3.270.tar.gz

# GLFW — Window/input
echo "  → GLFW 3.3.9"
wget -q -P display/glfw https://github.com/glfw/glfw/releases/download/3.3.9/glfw-3.3.9.zip

# ============================================================
# FASE 10 — MATH
# ============================================================
echo ""
echo "[FASE 10] Descargando Math Libraries..."
mkdir -p math

# cglm — GLM en C puro (header-only)
echo "  → cglm 0.9.1"
wget -q -P math/cglm https://github.com/recp/cglm/archive/refs/tags/v0.9.1.tar.gz

# ============================================================
# FASE 11 — PHYSICS
# ============================================================
echo ""
echo "[FASE 11] Descargando Physics Libraries..."
mkdir -p physics

# Box2D — 2D física
echo "  → Box2D 2.4.1"
wget -q -P physics/box2d https://github.com/erincatto/box2d/archive/refs/tags/v2.4.1.tar.gz

# ============================================================
# FASE 12 — AI/ML
# ============================================================
echo ""
echo "[FASE 12] Descargando AI Libraries..."
mkdir -p ai

# ggml — ML tensors (base de llama.cpp)
echo "  → ggml (latest)"
git clone --depth 1 https://github.com/ggerganov/ggml.git ai/ggml 2>/dev/null || echo "    (ya existe)"

# ============================================================
# RESUMEN
# ============================================================
echo ""
echo "╔══════════════════════════════════════════════════════════════════╗"
echo "║                    DESCARGA COMPLETADA                           ║"
echo "╚══════════════════════════════════════════════════════════════════╝"
echo ""
echo "Librerías descargadas:"
find . -name "*.tar.gz" -o -name "*.tar.xz" -o -name "*.zip" | wc -l
echo ""
echo "Para compilar con ADead-BIB:"
echo "  cd <libreria>"
echo "  tar xzf *.tar.gz"
echo "  ./configure CC='adB cc'"
echo "  make"
echo ""
echo "🦈 FastOS + ADead-BIB — Sin GCC, sin LLVM, sin overhead 💀"
