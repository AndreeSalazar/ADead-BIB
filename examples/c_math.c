// ============================================================
// ADead-BIB C Example — Matemáticas y Cómputo Numérico
// ============================================================
// Fixed-point math, vector operations, matrix multiply,
// numeric algorithms — para FastOS sin libm overhead.
// ============================================================

#include <stdio.h>
#include <math.h>

// ==================== Fixed-Point (16.16) ====================

typedef int fixed_t;

fixed_t float_to_fixed(float f) {
    return (fixed_t)(f * 65536.0);
}

float fixed_to_float(fixed_t f) {
    return (float)f / 65536.0;
}

fixed_t fixed_mul(fixed_t a, fixed_t b) {
    long long result = (long long)a * (long long)b;
    return (fixed_t)(result >> 16);
}

fixed_t fixed_div(fixed_t a, fixed_t b) {
    long long result = ((long long)a << 16) / b;
    return (fixed_t)result;
}

// ==================== Vector Math ====================

struct Vec4 {
    float x;
    float y;
    float z;
    float w;
};

float vec4_dot(struct Vec4 a, struct Vec4 b) {
    return a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w;
}

float vec4_length_sq(struct Vec4 v) {
    return v.x * v.x + v.y * v.y + v.z * v.z + v.w * v.w;
}

float vec4_length(struct Vec4 v) {
    return sqrt(vec4_length_sq(v));
}

struct Vec4 vec4_normalize(struct Vec4 v) {
    float len = vec4_length(v);
    struct Vec4 result;
    if (len > 0.0001) {
        float inv = 1.0 / len;
        result.x = v.x * inv;
        result.y = v.y * inv;
        result.z = v.z * inv;
        result.w = v.w * inv;
    } else {
        result.x = 0.0;
        result.y = 0.0;
        result.z = 0.0;
        result.w = 0.0;
    }
    return result;
}

struct Vec4 vec4_cross3(struct Vec4 a, struct Vec4 b) {
    struct Vec4 result;
    result.x = a.y * b.z - a.z * b.y;
    result.y = a.z * b.x - a.x * b.z;
    result.z = a.x * b.y - a.y * b.x;
    result.w = 0.0;
    return result;
}

struct Vec4 vec4_lerp(struct Vec4 a, struct Vec4 b, float t) {
    float inv = 1.0 - t;
    struct Vec4 result;
    result.x = a.x * inv + b.x * t;
    result.y = a.y * inv + b.y * t;
    result.z = a.z * inv + b.z * t;
    result.w = a.w * inv + b.w * t;
    return result;
}

// ==================== Matrix 4x4 ====================

struct Mat4 {
    float m[16];
};

struct Mat4 mat4_identity(void) {
    struct Mat4 result;
    for (int i = 0; i < 16; i++) result.m[i] = 0.0;
    result.m[0] = 1.0;
    result.m[5] = 1.0;
    result.m[10] = 1.0;
    result.m[15] = 1.0;
    return result;
}

struct Mat4 mat4_multiply(struct Mat4 a, struct Mat4 b) {
    struct Mat4 result;
    for (int row = 0; row < 4; row++) {
        for (int col = 0; col < 4; col++) {
            float sum = 0.0;
            for (int k = 0; k < 4; k++) {
                sum += a.m[row * 4 + k] * b.m[k * 4 + col];
            }
            result.m[row * 4 + col] = sum;
        }
    }
    return result;
}

struct Vec4 mat4_transform(struct Mat4 m, struct Vec4 v) {
    struct Vec4 result;
    result.x = m.m[0] * v.x + m.m[1] * v.y + m.m[2] * v.z + m.m[3] * v.w;
    result.y = m.m[4] * v.x + m.m[5] * v.y + m.m[6] * v.z + m.m[7] * v.w;
    result.z = m.m[8] * v.x + m.m[9] * v.y + m.m[10] * v.z + m.m[11] * v.w;
    result.w = m.m[12] * v.x + m.m[13] * v.y + m.m[14] * v.z + m.m[15] * v.w;
    return result;
}

struct Mat4 mat4_scale(float sx, float sy, float sz) {
    struct Mat4 m = mat4_identity();
    m.m[0] = sx;
    m.m[5] = sy;
    m.m[10] = sz;
    return m;
}

// ==================== Numeric Algorithms ====================

float newton_sqrt(float x) {
    if (x <= 0.0) return 0.0;
    float guess = x / 2.0;
    for (int i = 0; i < 20; i++) {
        guess = (guess + x / guess) / 2.0;
    }
    return guess;
}

float fast_inv_sqrt(float x) {
    float half = 0.5 * x;
    int i = *(int *)&x;
    i = 0x5F3759DF - (i >> 1);
    x = *(float *)&i;
    x = x * (1.5 - half * x * x);
    x = x * (1.5 - half * x * x);
    return x;
}

float lerp(float a, float b, float t) {
    return a + (b - a) * t;
}

float smoothstep(float edge0, float edge1, float x) {
    float t = (x - edge0) / (edge1 - edge0);
    if (t < 0.0) t = 0.0;
    if (t > 1.0) t = 1.0;
    return t * t * (3.0 - 2.0 * t);
}

float remap(float value, float in_min, float in_max, float out_min, float out_max) {
    return out_min + (value - in_min) * (out_max - out_min) / (in_max - in_min);
}

int int_pow(int base, int exp) {
    int result = 1;
    while (exp > 0) {
        if (exp & 1) result *= base;
        base *= base;
        exp >>= 1;
    }
    return result;
}

// ==================== Main ====================

int main() {
    printf("=== ADead-BIB: Math & Numeric ===\n\n");

    // Fixed-point
    printf("Fixed-Point (16.16):\n");
    fixed_t fa = float_to_fixed(3.14);
    fixed_t fb = float_to_fixed(2.71);
    printf("  3.14 -> fixed: %d -> back: %.4f\n", fa, fixed_to_float(fa));
    printf("  3.14 * 2.71 = %.4f\n", fixed_to_float(fixed_mul(fa, fb)));
    printf("  3.14 / 2.71 = %.4f\n", fixed_to_float(fixed_div(fa, fb)));

    // Vector4
    printf("\nVec4:\n");
    struct Vec4 v1;
    v1.x = 1.0; v1.y = 2.0; v1.z = 3.0; v1.w = 0.0;
    struct Vec4 v2;
    v2.x = 4.0; v2.y = 5.0; v2.z = 6.0; v2.w = 0.0;
    printf("  v1 = (%.1f, %.1f, %.1f, %.1f)\n", v1.x, v1.y, v1.z, v1.w);
    printf("  v2 = (%.1f, %.1f, %.1f, %.1f)\n", v2.x, v2.y, v2.z, v2.w);
    printf("  dot = %.1f\n", vec4_dot(v1, v2));
    printf("  |v1| = %.4f\n", vec4_length(v1));

    struct Vec4 norm = vec4_normalize(v1);
    printf("  norm = (%.4f, %.4f, %.4f)\n", norm.x, norm.y, norm.z);

    struct Vec4 cross = vec4_cross3(v1, v2);
    printf("  cross = (%.1f, %.1f, %.1f)\n", cross.x, cross.y, cross.z);

    struct Vec4 mid = vec4_lerp(v1, v2, 0.5);
    printf("  lerp(0.5) = (%.1f, %.1f, %.1f)\n", mid.x, mid.y, mid.z);

    // Matrix 4x4
    printf("\nMat4:\n");
    struct Mat4 scale = mat4_scale(2.0, 3.0, 4.0);
    struct Vec4 point;
    point.x = 1.0; point.y = 1.0; point.z = 1.0; point.w = 1.0;
    struct Vec4 transformed = mat4_transform(scale, point);
    printf("  scale(2,3,4) * (1,1,1,1) = (%.1f, %.1f, %.1f, %.1f)\n",
           transformed.x, transformed.y, transformed.z, transformed.w);

    // Numeric
    printf("\nNumeric:\n");
    printf("  newton_sqrt(2)  = %.6f\n", newton_sqrt(2.0));
    printf("  stdlib sqrt(2)  = %.6f\n", sqrt(2.0));
    printf("  inv_sqrt(4.0)   = %.6f\n", fast_inv_sqrt(4.0));
    printf("  smoothstep(0.5) = %.4f\n", smoothstep(0.0, 1.0, 0.5));
    printf("  remap(50, 0..100, 0..1) = %.2f\n", remap(50.0, 0.0, 100.0, 0.0, 1.0));
    printf("  2^10 = %d\n", int_pow(2, 10));
    printf("  3^7  = %d\n", int_pow(3, 7));

    // Trigonometry
    printf("\nTrigonometry:\n");
    printf("  sin(0)      = %.6f\n", sin(0.0));
    printf("  sin(pi/2)   = %.6f\n", sin(3.14159265 / 2.0));
    printf("  cos(0)      = %.6f\n", cos(0.0));
    printf("  cos(pi)     = %.6f\n", cos(3.14159265));
    printf("  atan2(1, 1) = %.6f\n", atan2(1.0, 1.0));

    printf("\n=== Complete ===\n");
    return 0;
}
