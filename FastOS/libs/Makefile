# FastOS Libraries — Master Makefile
# Compila todas las librerías C/C++ con ADead-BIB
# Sin GCC, sin LLVM, sin overhead

# ADead-BIB Compiler
ADEAD = cargo run --manifest-path=../../Cargo.toml --
CC = $(ADEAD) cc
CXX = $(ADEAD) cxx

# Flags
CFLAGS = -O2 -fno-stack-protector -ffreestanding
LDFLAGS = 

# Output directory
BUILD = ../build/libs

# ============================================================
# TARGETS
# ============================================================

.PHONY: all clean core compression images audio network fonts database parsers display math physics ai

all: dirs core compression images audio network fonts database parsers display

dirs:
	@mkdir -p $(BUILD)

# ============================================================
# FASE 1 — CORE
# ============================================================
core: dirs
	@echo "[CORE] Building core libraries..."
	@echo "  Note: musl requires manual configuration"
	@echo "  Run: cd core/musl && ./configure CC='adB cc' && make"

# ============================================================
# FASE 2 — COMPRESSION
# ============================================================
compression: zlib lz4 zstd

zlib: dirs
	@echo "[ZLIB] Building zlib..."
	@if [ -d compression/zlib/zlib-* ]; then \
		cd compression/zlib/zlib-* && \
		$(CC) -c *.c $(CFLAGS) && \
		ar rcs $(BUILD)/libz.a *.o; \
	fi

lz4: dirs
	@echo "[LZ4] Building lz4..."
	@if [ -d compression/lz4/lz4-* ]; then \
		cd compression/lz4/lz4-*/lib && \
		$(CC) -c lz4.c lz4hc.c lz4frame.c $(CFLAGS) && \
		ar rcs $(BUILD)/liblz4.a *.o; \
	fi

zstd: dirs
	@echo "[ZSTD] Building zstd..."
	@if [ -d compression/zstd/zstd-* ]; then \
		cd compression/zstd/zstd-*/lib && \
		$(CC) -c common/*.c compress/*.c decompress/*.c $(CFLAGS) && \
		ar rcs $(BUILD)/libzstd.a *.o; \
	fi

# ============================================================
# FASE 3 — IMAGES
# ============================================================
images: libpng libjpeg stb

libpng: dirs zlib
	@echo "[PNG] Building libpng..."
	@if [ -d images/libpng/libpng-* ]; then \
		cd images/libpng/libpng-* && \
		$(CC) -c *.c $(CFLAGS) -I../../compression/zlib/zlib-* && \
		ar rcs $(BUILD)/libpng.a *.o; \
	fi

libjpeg: dirs
	@echo "[JPEG] Building libjpeg-turbo..."
	@if [ -d images/libjpeg/libjpeg-turbo-* ]; then \
		cd images/libjpeg/libjpeg-turbo-* && \
		$(CC) -c *.c $(CFLAGS) && \
		ar rcs $(BUILD)/libjpeg.a *.o; \
	fi

stb: dirs
	@echo "[STB] stb is header-only, no build needed"
	@cp images/stb/*.h $(BUILD)/ 2>/dev/null || true

# ============================================================
# FASE 4 — AUDIO
# ============================================================
audio: libogg libvorbis libopus miniaudio

libogg: dirs
	@echo "[OGG] Building libogg..."
	@if [ -d audio/libogg/libogg-* ]; then \
		cd audio/libogg/libogg-*/src && \
		$(CC) -c *.c $(CFLAGS) -I../include && \
		ar rcs $(BUILD)/libogg.a *.o; \
	fi

libvorbis: dirs libogg
	@echo "[VORBIS] Building libvorbis..."
	@if [ -d audio/libvorbis/libvorbis-* ]; then \
		cd audio/libvorbis/libvorbis-*/lib && \
		$(CC) -c *.c $(CFLAGS) -I../include -I../../libogg/libogg-*/include && \
		ar rcs $(BUILD)/libvorbis.a *.o; \
	fi

libopus: dirs
	@echo "[OPUS] Building libopus..."
	@if [ -d audio/libopus/opus-* ]; then \
		cd audio/libopus/opus-*/src && \
		$(CC) -c *.c $(CFLAGS) -I../include && \
		ar rcs $(BUILD)/libopus.a *.o; \
	fi

miniaudio: dirs
	@echo "[MINIAUDIO] miniaudio is header-only, no build needed"
	@cp audio/miniaudio/*.h $(BUILD)/ 2>/dev/null || true

# ============================================================
# FASE 5 — NETWORK
# ============================================================
network: mbedtls c-ares

mbedtls: dirs
	@echo "[MBEDTLS] Building mbedtls..."
	@if [ -d network/mbedtls/mbedtls-* ]; then \
		cd network/mbedtls/mbedtls-*/library && \
		$(CC) -c *.c $(CFLAGS) -I../include && \
		ar rcs $(BUILD)/libmbedtls.a *.o; \
	fi

c-ares: dirs
	@echo "[C-ARES] Building c-ares..."
	@if [ -d network/c-ares/c-ares-* ]; then \
		cd network/c-ares/c-ares-*/src/lib && \
		$(CC) -c *.c $(CFLAGS) -I../../include && \
		ar rcs $(BUILD)/libcares.a *.o; \
	fi

# ============================================================
# FASE 6 — FONTS
# ============================================================
fonts: freetype

freetype: dirs
	@echo "[FREETYPE] Building freetype..."
	@if [ -d fonts/freetype/freetype-* ]; then \
		cd fonts/freetype/freetype-*/src && \
		$(CC) -c base/*.c $(CFLAGS) -I../include && \
		ar rcs $(BUILD)/libfreetype.a *.o; \
	fi

# ============================================================
# FASE 7 — DATABASE
# ============================================================
database: sqlite

sqlite: dirs
	@echo "[SQLITE] Building sqlite..."
	@if [ -d database/sqlite/sqlite-* ]; then \
		cd database/sqlite/sqlite-* && \
		$(CC) -c sqlite3.c $(CFLAGS) -DSQLITE_OMIT_LOAD_EXTENSION && \
		ar rcs $(BUILD)/libsqlite3.a sqlite3.o; \
	fi

# ============================================================
# FASE 8 — PARSERS
# ============================================================
parsers: expat cjson jsmn

expat: dirs
	@echo "[EXPAT] Building expat..."
	@if [ -d parsers/expat/expat-* ]; then \
		cd parsers/expat/expat-*/lib && \
		$(CC) -c *.c $(CFLAGS) && \
		ar rcs $(BUILD)/libexpat.a *.o; \
	fi

cjson: dirs
	@echo "[CJSON] Building cJSON..."
	@if [ -d parsers/cjson/cJSON-* ]; then \
		cd parsers/cjson/cJSON-* && \
		$(CC) -c cJSON.c $(CFLAGS) && \
		ar rcs $(BUILD)/libcjson.a cJSON.o; \
	fi

jsmn: dirs
	@echo "[JSMN] jsmn is header-only, no build needed"
	@cp parsers/jsmn/*.h $(BUILD)/ 2>/dev/null || true

# ============================================================
# FASE 9 — DISPLAY/GPU
# ============================================================
display: vulkan-headers glfw

vulkan-headers: dirs
	@echo "[VULKAN] Vulkan headers are header-only"
	@if [ -d display/vulkan/Vulkan-Headers-* ]; then \
		cp -r display/vulkan/Vulkan-Headers-*/include/vulkan $(BUILD)/; \
	fi

glfw: dirs
	@echo "[GLFW] Building GLFW..."
	@if [ -d display/glfw/glfw-* ]; then \
		cd display/glfw/glfw-*/src && \
		$(CC) -c *.c $(CFLAGS) -I../include && \
		ar rcs $(BUILD)/libglfw.a *.o; \
	fi

# ============================================================
# FASE 10 — MATH
# ============================================================
math: cglm

cglm: dirs
	@echo "[CGLM] cglm is header-only"
	@if [ -d math/cglm/cglm-* ]; then \
		cp -r math/cglm/cglm-*/include/cglm $(BUILD)/; \
	fi

# ============================================================
# FASE 11 — PHYSICS
# ============================================================
physics: box2d

box2d: dirs
	@echo "[BOX2D] Building Box2D..."
	@if [ -d physics/box2d/box2d-* ]; then \
		cd physics/box2d/box2d-*/src && \
		$(CXX) -c *.cpp $(CFLAGS) -I../include && \
		ar rcs $(BUILD)/libbox2d.a *.o; \
	fi

# ============================================================
# CLEAN
# ============================================================
clean:
	@echo "[CLEAN] Removing build artifacts..."
	@rm -rf $(BUILD)/*
	@find . -name "*.o" -delete
	@echo "[CLEAN] Done"

# ============================================================
# HELP
# ============================================================
help:
	@echo "FastOS Libraries — ADead-BIB Build System"
	@echo ""
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@echo "  all         - Build all libraries"
	@echo "  core        - Build core libraries (musl)"
	@echo "  compression - Build zlib, lz4, zstd"
	@echo "  images      - Build libpng, libjpeg, stb"
	@echo "  audio       - Build libogg, libvorbis, libopus"
	@echo "  network     - Build mbedtls, c-ares"
	@echo "  fonts       - Build freetype"
	@echo "  database    - Build sqlite"
	@echo "  parsers     - Build expat, cjson"
	@echo "  display     - Build vulkan headers, glfw"
	@echo "  math        - Build cglm"
	@echo "  physics     - Build box2d"
	@echo "  clean       - Remove build artifacts"
	@echo ""
	@echo "Individual libraries:"
	@echo "  zlib, lz4, zstd, libpng, libjpeg, stb,"
	@echo "  libogg, libvorbis, libopus, miniaudio,"
	@echo "  mbedtls, c-ares, freetype, sqlite,"
	@echo "  expat, cjson, jsmn, glfw, cglm, box2d"
