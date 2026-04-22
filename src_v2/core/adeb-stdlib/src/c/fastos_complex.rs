// ============================================================
// fastos_complex.rs — <complex.h> implementation (C99)
// ============================================================
// Complex number arithmetic (C99 §7.3)
// Requires double-precision SSE2 codegen for real+imag parts
// ============================================================

pub const COMPLEX_FUNCTIONS: &[&str] = &[
    // Arithmetic
    "cabs", "cabsf", "cabsl",
    "carg", "cargf", "cargl",
    "conj", "conjf", "conjl",
    "cproj", "cprojf", "cprojl",
    "creal", "crealf", "creall",
    "cimag", "cimagf", "cimagl",
    // Exponential
    "cexp", "cexpf", "cexpl",
    "clog", "clogf", "clogl",
    // Power
    "cpow", "cpowf", "cpowl",
    "csqrt", "csqrtf", "csqrtl",
    // Trigonometric
    "csin", "csinf", "csinl",
    "ccos", "ccosf", "ccosl",
    "ctan", "ctanf", "ctanl",
    "casin", "casinf", "casinl",
    "cacos", "cacosf", "cacosl",
    "catan", "catanf", "catanl",
    // Hyperbolic
    "csinh", "csinhf", "csinhl",
    "ccosh", "ccoshf", "ccoshl",
    "ctanh", "ctanhf", "ctanhl",
    "casinh", "casinhf", "casinhl",
    "cacosh", "cacoshf", "cacoshl",
    "catanh", "catanhf", "catanhl",
];

pub const COMPLEX_MACROS: &[(&str, &str)] = &[
    ("_Complex_I", "((float _Complex)__builtin_complex(0.0f, 1.0f))"),
    ("I", "_Complex_I"),
    ("complex", "_Complex"),
    ("imaginary", "_Imaginary"),
];

pub const COMPLEX_TYPES: &[&str] = &[
    "float _Complex", "double _Complex", "long double _Complex",
];

pub fn is_complex_symbol(name: &str) -> bool {
    COMPLEX_FUNCTIONS.contains(&name)
        || COMPLEX_MACROS.iter().any(|(n, _)| *n == name)
}
