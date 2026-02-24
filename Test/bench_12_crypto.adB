// ============================================================
// ADead-BIB Benchmark Suite — 12: Crypto & Hashing
// ============================================================
// Implements hash functions and simple ciphers using pure
// integer arithmetic (no bitwise operators available).
// Combines C-style procedural with Rust-inspired patterns.
// ============================================================

int mod_int(int a, int b) {
    return a - (a / b) * b
}

int abs_val(int x) {
    if x < 0 {
        return 0 - x
    }
    return x
}

// Simulate a AND b (for small non-negative values)
// Works by extracting bits and combining
int and_int(int a, int b) {
    int result = 0
    int place = 1
    int aa = a
    int bb = b
    int k = 0
    while k < 30 {
        int bit_a = mod_int(aa, 2)
        int bit_b = mod_int(bb, 2)
        // bit_a AND bit_b = bit_a * bit_b
        result = result + bit_a * bit_b * place
        aa = aa / 2
        bb = bb / 2
        place = place * 2
        k = k + 1
    }
    return result
}

// Simulate a XOR b using: a XOR b = a + b - 2*(a AND b)
int xor_int(int a, int b) {
    return a + b - 2 * and_int(a, b)
}

// Simulate rotate-right by n bits on a 30-bit value
int rotate_right(int val, int n) {
    int v = mod_int(abs_val(val), 1073741824)
    int shift = mod_int(n, 30)
    int i = 0
    int right_part = v
    while i < shift {
        right_part = right_part / 2
        i = i + 1
    }
    int left_shift = 30 - shift
    int left_part = v
    int j = 0
    while j < shift {
        left_part = mod_int(left_part, 2)
        j = j + 1
    }
    // Simplified: just divide and remix
    int low_bits = mod_int(v, 1)
    i = 0
    int divisor = 1
    while i < shift {
        divisor = divisor * 2
        i = i + 1
    }
    int high = v / divisor
    int low = mod_int(v, divisor)
    int multiplier = 1
    i = 0
    while i < left_shift {
        multiplier = multiplier * 2
        if multiplier > 1073741824 {
            multiplier = 1073741824
        }
        i = i + 1
    }
    int result = high + low * multiplier
    return mod_int(abs_val(result), 1073741824)
}

// ============================================================
// 1. FNV-1a hash simulation (1M integers)
// ============================================================
void bench_fnv1a() {
    printf("[1] FNV-1a hash simulation (1M)...\n")
    int hash = 2166136261
    hash = mod_int(hash, 1073741824)
    int i = 0
    while i < 1000000 {
        // FNV-1a: hash = (hash XOR byte) * FNV_prime
        // Simulated with multiplicative mixing
        hash = hash * 16777619 + i
        hash = mod_int(abs_val(hash), 1073741824)
        hash = hash * 16777619 + 1
        hash = mod_int(abs_val(hash), 1073741824)
        i = i + 1
    }
    printf("  fnv1a hash = ")
    printf(hash)
    printf("\n\n")
}

// ============================================================
// 2. DJB2 hash simulation (1M integers)
// ============================================================
void bench_djb2() {
    printf("[2] DJB2 hash simulation (1M)...\n")
    int hash = 5381
    int i = 0
    while i < 1000000 {
        // djb2: hash = hash * 33 + c
        hash = hash * 33 + mod_int(i, 256)
        hash = mod_int(abs_val(hash), 1073741824)
        i = i + 1
    }
    printf("  djb2 hash = ")
    printf(hash)
    printf("\n\n")
}

// ============================================================
// 3. CRC32 simulation — polynomial division emulation (100K)
// ============================================================
void bench_crc32() {
    printf("[3] CRC32 simulation (100K)...\n")
    int crc = 1073741823
    int poly = 79764919
    int i = 0
    while i < 100000 {
        int data = mod_int(i * 7 + 13, 256)
        crc = crc + data
        crc = mod_int(abs_val(crc), 1073741824)
        // Process 8 "bits"
        int bit = 0
        while bit < 8 {
            int low_bit = mod_int(crc, 2)
            crc = crc / 2
            if low_bit == 1 {
                // Simulate XOR with polynomial via add+mod
                crc = crc + poly
                crc = mod_int(abs_val(crc), 1073741824)
            }
            bit = bit + 1
        }
        i = i + 1
    }
    printf("  crc32 = ")
    printf(crc)
    printf("\n\n")
}

// ============================================================
// 4. SHA-like compression (100K blocks)
// ============================================================
void bench_sha_compress() {
    printf("[4] SHA-like compression (100K blocks)...\n")
    int h0 = 1779033703
    int h1 = 1073741789
    int h2 = 982451653
    int h3 = 567890123
    h0 = mod_int(h0, 1073741824)
    h1 = mod_int(h1, 1073741824)
    h2 = mod_int(h2, 1073741824)
    h3 = mod_int(h3, 1073741824)

    int i = 0
    while i < 100000 {
        int w = mod_int(i * 2654435761, 1073741824)

        // Simulate rotation: shift right by 7 + shift right by 18
        int s0 = h0 / 128 + h0 / 262144
        s0 = mod_int(abs_val(s0), 1073741824)

        int s1 = h2 / 64 + h2 / 131072
        s1 = mod_int(abs_val(s1), 1073741824)

        // Majority: approx (h0 + h1 + h2) / 2
        int maj = (h0 + h1 + h2) / 2
        maj = mod_int(abs_val(maj), 1073741824)

        // Choose: approx (h2 * h1 + (1073741823 - h2) * h3)
        // Simplified to avoid overflow
        int ch = mod_int(h2, 1024) * mod_int(h1, 1024)
        ch = ch + mod_int(1073741823 - h2, 1024) * mod_int(h3, 1024)
        ch = mod_int(abs_val(ch), 1073741824)

        int t1 = h3 + s1 + ch + w + 428
        t1 = mod_int(abs_val(t1), 1073741824)

        int t2 = s0 + maj
        t2 = mod_int(abs_val(t2), 1073741824)

        h3 = h2
        h2 = h1
        h1 = h0 + t1
        h1 = mod_int(abs_val(h1), 1073741824)
        h0 = t1 + t2
        h0 = mod_int(abs_val(h0), 1073741824)

        i = i + 1
    }

    int digest = h0 + h1 + h2 + h3
    digest = mod_int(abs_val(digest), 1073741824)
    printf("  sha digest = ")
    printf(digest)
    printf("\n\n")
}

// ============================================================
// 5. Caesar cipher — encrypt/decrypt 1M values, verify roundtrip
// ============================================================
int caesar_encrypt(int value, int key, int modulus) {
    int enc = value + key
    return mod_int(abs_val(enc), modulus)
}

int caesar_decrypt(int cipher, int key, int modulus) {
    int dec = cipher - key + modulus
    return mod_int(abs_val(dec), modulus)
}

void bench_caesar() {
    printf("[5] Caesar cipher roundtrip (1M)...\n")
    int key = 37
    int modulus = 1000003
    int matches = 0
    int last_enc = 0
    int i = 0
    while i < 1000000 {
        int plain = mod_int(i * 7 + 13, modulus)
        int enc = caesar_encrypt(plain, key, modulus)
        int dec = caesar_decrypt(enc, key, modulus)
        if dec == plain {
            matches = matches + 1
        }
        last_enc = enc
        i = i + 1
    }
    printf("  roundtrip matches = ")
    printf(matches)
    printf(" / 1000000\n")
    printf("  last encrypted = ")
    printf(last_enc)
    printf("\n\n")
}

// ============================================================
// 6. XOR cipher simulation (100K)
// ============================================================
void bench_xor_cipher() {
    printf("[6] XOR cipher simulation (100K)...\n")
    int key = 12345
    int checksum = 0
    int i = 0
    while i < 100000 {
        int plain = mod_int(i * 31 + 7, 65536)
        int k = mod_int(key + i, 65536)
        // XOR encrypt: a XOR b = a + b - 2*(a AND b)
        int a_and_b = and_int(plain, k)
        int cipher = plain + k - 2 * a_and_b
        // XOR decrypt (same operation)
        int d_and_c = and_int(cipher, k)
        int decrypted = cipher + k - 2 * d_and_c
        if decrypted == plain {
            checksum = checksum + 1
        }
        i = i + 1
    }
    printf("  xor roundtrip matches = ")
    printf(checksum)
    printf(" / 100000\n\n")
}

// ============================================================
// 7. LFSR — Linear Feedback Shift Register (1M numbers)
// ============================================================
void bench_lfsr() {
    printf("[7] LFSR pseudo-random generator (1M)...\n")
    int state = 1
    int sum = 0
    int i = 0
    while i < 1000000 {
        // Extract feedback bit from positions: bit0 XOR bit2 XOR bit3 XOR bit5
        int bit0 = mod_int(state, 2)
        int bit2 = mod_int(state / 4, 2)
        int bit3 = mod_int(state / 8, 2)
        int bit5 = mod_int(state / 32, 2)
        // XOR chain via arithmetic: a XOR b = a + b - 2*a*b (for single bits)
        int fb = bit0 + bit2 - 2 * bit0 * bit2
        fb = fb + bit3 - 2 * fb * bit3
        fb = fb + bit5 - 2 * fb * bit5

        // Shift right by 1 and insert feedback bit at bit 15
        state = state / 2 + fb * 32768

        sum = sum + state
        if sum > 1000000000 {
            sum = mod_int(sum, 1000000)
        }
        i = i + 1
    }
    printf("  lfsr final state = ")
    printf(state)
    printf("\n")
    printf("  lfsr sum = ")
    printf(sum)
    printf("\n\n")
}

// ============================================================
// 8. Merkle-Damgård hash chain (100K blocks)
// ============================================================
int md_compress(int state, int block) {
    // Compression: mix state with block through multiple rounds
    int s = state
    int r = 0
    while r < 4 {
        s = s * 31 + block
        s = mod_int(abs_val(s), 1073741824)
        s = s + s / 128
        s = mod_int(abs_val(s), 1073741824)
        block = block + s / 256
        block = mod_int(abs_val(block), 1073741824)
        r = r + 1
    }
    return s
}

void bench_merkle_damgard() {
    printf("[8] Merkle-Damgard hash chain (100K blocks)...\n")
    int state = 0
    int i = 0
    while i < 100000 {
        int block = mod_int(i * 2654435761, 1073741824)
        state = md_compress(state, block)
        i = i + 1
    }
    printf("  merkle-damgard hash = ")
    printf(state)
    printf("\n\n")
}

// ============================================================
// 9. Key derivation — PBKDF2-like stretching (10K iterations)
// ============================================================
int kdf_hash(int key, int salt) {
    int h = key + salt
    h = h * 16777619 + 1
    h = mod_int(abs_val(h), 1073741824)
    h = h * 31 + salt
    h = mod_int(abs_val(h), 1073741824)
    return h
}

void bench_key_derivation() {
    printf("[9] Key derivation PBKDF2-like (10K iterations)...\n")
    int password = 123456789
    int salt = 987654321
    salt = mod_int(salt, 1073741824)
    int derived = password
    int i = 0
    while i < 10000 {
        derived = kdf_hash(derived, salt + i)
        // Mix rounds to simulate HMAC
        int inner = derived * 33 + salt
        inner = mod_int(abs_val(inner), 1073741824)
        int outer = inner * 65537 + derived
        outer = mod_int(abs_val(outer), 1073741824)
        derived = outer
        i = i + 1
    }
    printf("  derived key = ")
    printf(derived)
    printf("\n\n")
}

// ============================================================
// 10. Checksum validator — compute & verify 100K messages
// ============================================================
int compute_checksum(int msg, int salt) {
    int cs = msg * 2654435761
    cs = mod_int(abs_val(cs), 1073741824)
    cs = cs + salt * 31
    cs = mod_int(abs_val(cs), 1073741824)
    cs = cs * 16777619
    cs = mod_int(abs_val(cs), 1073741824)
    return cs
}

int verify_checksum(int msg, int salt, int expected) {
    int actual = compute_checksum(msg, salt)
    if actual == expected {
        return 1
    }
    return 0
}

void bench_checksum_validator() {
    printf("[10] Checksum validator (100K messages)...\n")
    int salt = 42
    int pass_count = 0
    int fail_count = 0
    int last_cs = 0
    int i = 0
    while i < 100000 {
        int msg = i * 7 + 13
        int cs = compute_checksum(msg, salt)
        int ok = verify_checksum(msg, salt, cs)
        if ok == 1 {
            pass_count = pass_count + 1
        }
        if ok == 0 {
            fail_count = fail_count + 1
        }
        last_cs = cs
        i = i + 1
    }
    printf("  checksum passes = ")
    printf(pass_count)
    printf(" / 100000\n")
    printf("  checksum fails  = ")
    printf(fail_count)
    printf("\n")
    printf("  last checksum   = ")
    printf(last_cs)
    printf("\n\n")
}

// ============================================================
// Main — run all crypto & hashing benchmarks
// ============================================================
int main() {
    printf("============================================================\n")
    printf("ADead-BIB Benchmark Suite — 12: Crypto & Hashing\n")
    printf("============================================================\n\n")

    bench_fnv1a()
    bench_djb2()
    bench_crc32()
    bench_sha_compress()
    bench_caesar()
    bench_xor_cipher()
    bench_lfsr()
    bench_merkle_damgard()
    bench_key_derivation()
    bench_checksum_validator()

    printf("============================================================\n")
    printf("All crypto & hashing benchmarks complete.\n")
    printf("============================================================\n")
    return 0
}
