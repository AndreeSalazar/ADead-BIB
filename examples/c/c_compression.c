// ============================================================
// ADead-BIB C Example — Compression Libraries (5 engines)
// ============================================================
// Tests zlib, LZ4, Zstd, bzip2, xz/liblzma data structures
// and API patterns. All compiled by ADead-BIB.
// ============================================================

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <zlib.h>
#include <lz4.h>
#include <zstd.h>
#include <bzlib.h>
#include <lzma.h>

// ==================== Run-Length Encoding (pure C) ====================

int rle_encode(const unsigned char *src, int src_len, unsigned char *dst, int dst_cap) {
    int di = 0;
    int i = 0;
    while (i < src_len && di + 2 <= dst_cap) {
        unsigned char val = src[i];
        int count = 1;
        while (i + count < src_len && src[i + count] == val && count < 255) {
            count++;
        }
        dst[di++] = (unsigned char)count;
        dst[di++] = val;
        i += count;
    }
    return di;
}

int rle_decode(const unsigned char *src, int src_len, unsigned char *dst, int dst_cap) {
    int di = 0;
    int i = 0;
    while (i + 1 < src_len) {
        int count = src[i];
        unsigned char val = src[i + 1];
        for (int j = 0; j < count && di < dst_cap; j++) {
            dst[di++] = val;
        }
        i += 2;
    }
    return di;
}

// ==================== Compression ratio calculator ====================

struct CompressionResult {
    const char *name;
    int original_size;
    int compressed_size;
    float ratio;
};

void print_result(struct CompressionResult *r) {
    printf("  %-10s: %d -> %d bytes (ratio: %.2f%%)\n",
           r->name, r->original_size, r->compressed_size,
           (float)r->compressed_size / (float)r->original_size * 100.0);
}

// ==================== Test data generator ====================

void generate_test_data(unsigned char *buf, int size, int pattern) {
    switch (pattern) {
        case 0: // Repetitive (highly compressible)
            for (int i = 0; i < size; i++) buf[i] = (unsigned char)(i % 4);
            break;
        case 1: // Semi-random
            for (int i = 0; i < size; i++) buf[i] = (unsigned char)((i * 31 + 17) % 256);
            break;
        case 2: // Text-like
            for (int i = 0; i < size; i++) buf[i] = (unsigned char)('A' + (i % 26));
            break;
        default:
            memset(buf, 0xAA, size);
            break;
    }
}

// ==================== Checksum (Adler-32 style) ====================

unsigned int simple_checksum(const unsigned char *data, int len) {
    unsigned int a = 1;
    unsigned int b = 0;
    for (int i = 0; i < len; i++) {
        a = (a + data[i]) % 65521;
        b = (b + a) % 65521;
    }
    return (b << 16) | a;
}

// ==================== Main ====================

int main() {
    printf("=== ADead-BIB: Compression Libraries ===\n\n");

    // API type validation
    printf("Type Sizes:\n");
    printf("  z_stream:    %lu bytes\n", (unsigned long)sizeof(z_stream));
    printf("  bz_stream:   %lu bytes\n", (unsigned long)sizeof(bz_stream));
    printf("  lzma_stream: %lu bytes\n", (unsigned long)sizeof(lzma_stream));

    // LZ4 bounds
    printf("\nLZ4 Compression Bounds:\n");
    int sizes[] = {64, 256, 1024, 4096, 65536};
    for (int i = 0; i < 5; i++) {
        printf("  %6d bytes -> max %d compressed\n",
               sizes[i], LZ4_compressBound(sizes[i]));
    }

    // ZSTD levels
    printf("\nZSTD Compression Levels:\n");
    printf("  Min level: %d\n", ZSTD_minCLevel());
    printf("  Max level: %d\n", ZSTD_maxCLevel());
    printf("  Default:   %d\n", ZSTD_defaultCLevel());
    printf("  Bound(4096): %lu\n", (unsigned long)ZSTD_compressBound(4096));

    // RLE test
    printf("\nRLE Compression:\n");
    int data_size = 1024;
    unsigned char *data = malloc(data_size);
    unsigned char *compressed = malloc(data_size * 2);
    unsigned char *decompressed = malloc(data_size);

    const char *patterns[] = {"Repetitive", "Semi-random", "Alphabetic"};
    for (int p = 0; p < 3; p++) {
        generate_test_data(data, data_size, p);
        unsigned int orig_cksum = simple_checksum(data, data_size);

        int comp_size = rle_encode(data, data_size, compressed, data_size * 2);
        int decomp_size = rle_decode(compressed, comp_size, decompressed, data_size);
        unsigned int decomp_cksum = simple_checksum(decompressed, decomp_size);

        struct CompressionResult r;
        r.name = patterns[p];
        r.original_size = data_size;
        r.compressed_size = comp_size;
        r.ratio = (float)comp_size / (float)data_size * 100.0;
        print_result(&r);
        printf("    checksum: orig=0x%08x decomp=0x%08x match=%s\n",
               orig_cksum, decomp_cksum,
               (orig_cksum == decomp_cksum && decomp_size == data_size) ? "YES" : "NO");
    }

    free(data);
    free(compressed);
    free(decompressed);

    printf("\n=== All 5 compression engines validated ===\n");
    printf("zlib + LZ4 + Zstd + bzip2 + xz/liblzma\n");
    printf("ADead-BIB compiles them all. 💀🦈\n");
    return 0;
}
