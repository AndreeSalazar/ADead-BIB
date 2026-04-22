// ============================================================
// fastos_fenv.rs — <fenv.h> implementation
// ============================================================
// Floating-point environment control (C99 §7.6)
// Rounding modes, exception flags, FP state save/restore
// ============================================================

pub const FENV_FUNCTIONS: &[&str] = &[
    "feclearexcept", "fegetexceptflag", "feraiseexcept", "fesetexceptflag",
    "fetestexcept",
    "fegetround", "fesetround",
    "fegetenv", "feholdexcept", "fesetenv", "feupdateenv",
];

pub const FENV_MACROS: &[(&str, &str)] = &[
    ("FE_DIVBYZERO", "0x04"),
    ("FE_INEXACT", "0x20"),
    ("FE_INVALID", "0x01"),
    ("FE_OVERFLOW", "0x08"),
    ("FE_UNDERFLOW", "0x10"),
    ("FE_ALL_EXCEPT", "0x3F"),
    ("FE_TONEAREST", "0x0000"),
    ("FE_DOWNWARD", "0x0400"),
    ("FE_TOWARDZERO", "0x0C00"),
    ("FE_UPWARD", "0x0800"),
    ("FE_DFL_ENV", "((const fenv_t*)(-1))"),
];

pub const FENV_TYPES: &[&str] = &["fenv_t", "fexcept_t"];

pub fn is_fenv_symbol(name: &str) -> bool {
    FENV_FUNCTIONS.contains(&name)
        || FENV_MACROS.iter().any(|(n, _)| *n == name)
        || FENV_TYPES.contains(&name)
}
