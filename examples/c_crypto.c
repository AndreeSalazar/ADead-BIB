// ============================================================
// ADead-BIB C Example — Cryptography & Security
// ============================================================
// Hash functions, encryption patterns, TLS types, SSH types
// compiled by ADead-BIB for FastOS security layer.
// ============================================================

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <openssl/ssl.h>
#include <libssh2.h>
#include <curl/curl.h>

// ==================== SHA-256 (Pure C implementation) ====================

void sha256_simple_hash(const unsigned char *data, int len, unsigned char *out) {
    // Simplified hash for demonstration (NOT cryptographic!)
    unsigned int h0 = 0x6a09e667;
    unsigned int h1 = 0xbb67ae85;
    unsigned int h2 = 0x3c6ef372;
    unsigned int h3 = 0xa54ff53a;
    unsigned int h4 = 0x510e527f;
    unsigned int h5 = 0x9b05688c;
    unsigned int h6 = 0x1f83d9ab;
    unsigned int h7 = 0x5be0cd19;

    for (int i = 0; i < len; i++) {
        unsigned int byte = data[i];
        h0 = (h0 ^ byte) * 0x01000193;
        h1 = (h1 ^ byte) * 0x01000193;
        h2 = (h2 ^ (byte << 8)) * 0x01000193;
        h3 = (h3 ^ (byte << 16)) * 0x01000193;
        h4 ^= h0 + h1;
        h5 ^= h2 + h3;
        h6 = (h6 + h4) ^ (h6 >> 11);
        h7 = (h7 + h5) ^ (h7 >> 13);
    }

    unsigned int *out32 = (unsigned int *)out;
    out32[0] = h0; out32[1] = h1;
    out32[2] = h2; out32[3] = h3;
    out32[4] = h4; out32[5] = h5;
    out32[6] = h6; out32[7] = h7;
}

void print_hash(const unsigned char *hash, int len) {
    for (int i = 0; i < len; i++) {
        printf("%02x", hash[i]);
    }
}

// ==================== XOR Cipher ====================

void xor_encrypt(unsigned char *data, int len, const unsigned char *key, int key_len) {
    for (int i = 0; i < len; i++) {
        data[i] ^= key[i % key_len];
    }
}

// ==================== Base64 Encoder ====================

static const char b64_table[] = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

int base64_encode(const unsigned char *src, int src_len, char *dst, int dst_cap) {
    int di = 0;
    int i;
    for (i = 0; i + 2 < src_len && di + 4 <= dst_cap; i += 3) {
        dst[di++] = b64_table[(src[i] >> 2) & 0x3F];
        dst[di++] = b64_table[((src[i] & 0x03) << 4) | ((src[i+1] >> 4) & 0x0F)];
        dst[di++] = b64_table[((src[i+1] & 0x0F) << 2) | ((src[i+2] >> 6) & 0x03)];
        dst[di++] = b64_table[src[i+2] & 0x3F];
    }
    if (i < src_len && di + 4 <= dst_cap) {
        dst[di++] = b64_table[(src[i] >> 2) & 0x3F];
        if (i + 1 < src_len) {
            dst[di++] = b64_table[((src[i] & 0x03) << 4) | ((src[i+1] >> 4) & 0x0F)];
            dst[di++] = b64_table[((src[i+1] & 0x0F) << 2)];
        } else {
            dst[di++] = b64_table[((src[i] & 0x03) << 4)];
            dst[di++] = '=';
        }
        dst[di++] = '=';
    }
    if (di < dst_cap) dst[di] = '\0';
    return di;
}

// ==================== HMAC-like construction ====================

void hmac_simple(const unsigned char *key, int key_len,
                 const unsigned char *data, int data_len,
                 unsigned char *out) {
    unsigned char ipad[64];
    unsigned char opad[64];
    memset(ipad, 0x36, 64);
    memset(opad, 0x5C, 64);
    for (int i = 0; i < key_len && i < 64; i++) {
        ipad[i] ^= key[i];
        opad[i] ^= key[i];
    }
    // Inner hash: H(ipad || data)
    unsigned char inner[96];
    memcpy(inner, ipad, 64);
    int copy_len = data_len < 32 ? data_len : 32;
    memcpy(inner + 64, data, copy_len);
    unsigned char inner_hash[32];
    sha256_simple_hash(inner, 64 + copy_len, inner_hash);

    // Outer hash: H(opad || inner_hash)
    unsigned char outer[96];
    memcpy(outer, opad, 64);
    memcpy(outer + 64, inner_hash, 32);
    sha256_simple_hash(outer, 96, out);
}

// ==================== Main ====================

int main() {
    printf("=== ADead-BIB: Cryptography & Security ===\n\n");

    // Hashing
    printf("Hash Functions:\n");
    const char *messages[] = {"ADead-BIB", "FastOS", "Hello, World!", ""};
    for (int i = 0; i < 4; i++) {
        unsigned char hash[32];
        sha256_simple_hash((const unsigned char *)messages[i], strlen(messages[i]), hash);
        printf("  hash(\"%s\") = ", messages[i]);
        print_hash(hash, 16);
        printf("...\n");
    }

    // XOR cipher
    printf("\nXOR Cipher:\n");
    char plaintext[] = "ADead-BIB FastOS";
    int pt_len = strlen(plaintext);
    unsigned char key[] = {0xDE, 0xAD, 0xBE, 0xEF};
    printf("  Plain:     %s\n", plaintext);
    xor_encrypt((unsigned char *)plaintext, pt_len, key, 4);
    printf("  Encrypted: ");
    for (int i = 0; i < pt_len; i++) printf("%02x", (unsigned char)plaintext[i]);
    printf("\n");
    xor_encrypt((unsigned char *)plaintext, pt_len, key, 4);
    printf("  Decrypted: %s\n", plaintext);

    // Base64
    printf("\nBase64 Encoding:\n");
    const char *b64_inputs[] = {"ADead-BIB", "FastOS", "A", "AB", "ABC"};
    for (int i = 0; i < 5; i++) {
        char encoded[256];
        base64_encode((const unsigned char *)b64_inputs[i], strlen(b64_inputs[i]), encoded, 256);
        printf("  \"%s\" -> \"%s\"\n", b64_inputs[i], encoded);
    }

    // HMAC
    printf("\nHMAC:\n");
    unsigned char hmac_key[] = "secret-key";
    unsigned char hmac_out[32];
    hmac_simple(hmac_key, strlen((char *)hmac_key),
                (unsigned char *)"message", 7, hmac_out);
    printf("  HMAC(\"secret-key\", \"message\") = ");
    print_hash(hmac_out, 16);
    printf("...\n");

    // SSL/TLS types
    printf("\nOpenSSL Types:\n");
    printf("  SSL* size:     %lu\n", (unsigned long)sizeof(SSL *));
    printf("  SSL_CTX* size: %lu\n", (unsigned long)sizeof(SSL_CTX *));
    printf("  EVP_MD* size:  %lu\n", (unsigned long)sizeof(EVP_MD *));

    // SSH2 types
    printf("\nSSH2 Types:\n");
    printf("  LIBSSH2_SESSION* size:  %lu\n", (unsigned long)sizeof(LIBSSH2_SESSION *));
    printf("  LIBSSH2_CHANNEL* size:  %lu\n", (unsigned long)sizeof(LIBSSH2_CHANNEL *));

    // CURL types
    printf("\ncURL Types:\n");
    printf("  CURL* size:      %lu\n", (unsigned long)sizeof(CURL *));
    printf("  curl_slist size: %lu\n", (unsigned long)sizeof(struct curl_slist));

    printf("\n=== Complete: OpenSSL + SSH2 + cURL ===\n");
    printf("ADead-BIB compiles full security stack. 💀🦈\n");
    return 0;
}
