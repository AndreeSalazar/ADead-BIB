/*
 * ADead-BIB Graphics Library
 * png.h - PNG image support basics
 */

#ifndef _ADEAD_PNG_H
#define _ADEAD_PNG_H

#include "../c/stdint.h"
#include "../c/stddef.h"

/* PNG signature */
#define PNG_SIGNATURE "\x89PNG\r\n\x1a\n"
#define PNG_SIGNATURE_SIZE 8

/* PNG chunk types */
#define PNG_CHUNK_IHDR 0x49484452  /* Image header */
#define PNG_CHUNK_PLTE 0x504C5445  /* Palette */
#define PNG_CHUNK_IDAT 0x49444154  /* Image data */
#define PNG_CHUNK_IEND 0x49454E44  /* Image end */
#define PNG_CHUNK_tRNS 0x74524E53  /* Transparency */
#define PNG_CHUNK_gAMA 0x67414D41  /* Gamma */
#define PNG_CHUNK_cHRM 0x6348524D  /* Chromaticity */
#define PNG_CHUNK_sRGB 0x73524742  /* Standard RGB */
#define PNG_CHUNK_tEXt 0x74455874  /* Text */

/* PNG color types */
#define PNG_COLOR_TYPE_GRAYSCALE       0
#define PNG_COLOR_TYPE_RGB             2
#define PNG_COLOR_TYPE_PALETTE         3
#define PNG_COLOR_TYPE_GRAYSCALE_ALPHA 4
#define PNG_COLOR_TYPE_RGBA            6

/* PNG compression methods */
#define PNG_COMPRESSION_DEFLATE 0

/* PNG filter methods */
#define PNG_FILTER_NONE    0
#define PNG_FILTER_SUB     1
#define PNG_FILTER_UP      2
#define PNG_FILTER_AVERAGE 3
#define PNG_FILTER_PAETH   4

/* PNG interlace methods */
#define PNG_INTERLACE_NONE  0
#define PNG_INTERLACE_ADAM7 1

/* PNG chunk header */
typedef struct {
    uint32_t length;
    uint32_t type;
} png_chunk_header;

/* PNG IHDR chunk data */
typedef struct {
    uint32_t width;
    uint32_t height;
    uint8_t  bit_depth;
    uint8_t  color_type;
    uint8_t  compression;
    uint8_t  filter;
    uint8_t  interlace;
} __attribute__((packed)) png_ihdr;

/* PNG image structure */
typedef struct {
    uint32_t width;
    uint32_t height;
    uint8_t  bit_depth;
    uint8_t  color_type;
    uint8_t  channels;
    uint8_t* data;
    size_t   data_size;
    uint8_t* palette;
    size_t   palette_size;
} png_image;

/* PNG error codes */
typedef enum {
    PNG_OK = 0,
    PNG_ERROR_INVALID_SIGNATURE = -1,
    PNG_ERROR_INVALID_HEADER = -2,
    PNG_ERROR_UNSUPPORTED_FORMAT = -3,
    PNG_ERROR_DECOMPRESSION = -4,
    PNG_ERROR_CRC = -5,
    PNG_ERROR_OUT_OF_MEMORY = -6,
    PNG_ERROR_IO = -7,
} png_error;

/* Function declarations */
png_error png_read_header(const uint8_t* data, size_t size, png_ihdr* header);
png_error png_decode(const uint8_t* data, size_t size, png_image* image);
void png_free(png_image* image);

/* Utility functions */
uint32_t png_crc32(const uint8_t* data, size_t length);
uint32_t png_swap32(uint32_t value);

/* Inline byte swap for big-endian PNG format */
static inline uint32_t png_be32(uint32_t x) {
    return ((x >> 24) & 0xFF) |
           ((x >> 8) & 0xFF00) |
           ((x << 8) & 0xFF0000) |
           ((x << 24) & 0xFF000000);
}

static inline uint16_t png_be16(uint16_t x) {
    return ((x >> 8) & 0xFF) | ((x << 8) & 0xFF00);
}

#endif /* _ADEAD_PNG_H */
