// ============================================================
// ADead-BIB C99 Standard Library
// ============================================================
// Implementaciones propias — Sin libc externa
// Cada módulo implementa un header C99 estándar
// usando syscalls directos o instrucciones x87/SSE2
// ============================================================

pub mod fastos_stdio;
pub mod fastos_stdlib;
pub mod fastos_string;
pub mod fastos_math;
pub mod fastos_time;
pub mod fastos_assert;
pub mod fastos_errno;
pub mod fastos_limits;
pub mod fastos_types;
