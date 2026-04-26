// ============================================================================
// error_como_valor.rs — Error Handling como Valores, Cero Excepciones
// ============================================================================
//
// FILOSOFÍA: Result<T, KillerError> para todo. Cero excepciones. Cero panics.
// Unwind = abort. Error propagation con ? operator nativo.
//
// Cada error es un VALOR en el type system:
//   - Siempre visible en la firma de la función
//   - Nunca se propaga silenciosamente
//   - El flujo de error es controlado por el programador
//   - No hay "exception tables" en el runtime
//
// Categorías de error:
//   - Memory: allocation failures, out of bounds, null deref
//   - Math: overflow, div by zero, NaN
//   - IO: file not found, permission denied
//   - ABI: calling convention violation
//   - Lifetime: use after free, double free
//   - Internal: compiler/runtime bugs
//
// Autor: Eddi Andreé Salazar Matos
// ============================================================================

/// Result alias para el runtime Killer
pub type KillerResult<T> = Result<T, KillerError>;

/// Error principal del Runtime Killer
///
/// Diseñado para ser:
///   1. Exhaustivo — cubre toda categoría de fallo
///   2. Informativo — incluye contexto suficiente para debug
///   3. Cheap — no alloca en heap para errores comunes
///   4. Propagable — funciona con ? operator de Rust
#[derive(Debug, Clone)]
pub struct KillerError {
    /// Categoría del error
    pub kind: ErrorKind,
    /// Mensaje legible para humanos
    pub message: String,
    /// Ubicación en el código fuente (archivo:línea:columna)
    pub location: Option<ErrorLocation>,
    /// Cadena de errores causales (para error chaining)
    pub cause: Option<Box<KillerError>>,
    /// Código numérico para matching rápido
    pub code: u32,
}

impl KillerError {
    /// Crear un error simple
    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            code: kind.code(),
            kind,
            message: message.into(),
            location: None,
            cause: None,
        }
    }

    /// Crear un error con ubicación
    pub fn with_location(mut self, file: &str, line: u32, col: u32) -> Self {
        self.location = Some(ErrorLocation {
            file: file.to_string(),
            line,
            column: col,
        });
        self
    }

    /// Crear un error con causa
    pub fn with_cause(mut self, cause: KillerError) -> Self {
        self.cause = Some(Box::new(cause));
        self
    }

    /// ¿Es un error de memoria?
    pub fn is_memory_error(&self) -> bool {
        matches!(self.kind, ErrorKind::OutOfMemory | ErrorKind::NullDereference |
                 ErrorKind::OutOfBounds | ErrorKind::UseAfterFree | ErrorKind::DoubleFree)
    }

    /// ¿Es un error matemático?
    pub fn is_math_error(&self) -> bool {
        matches!(self.kind, ErrorKind::DivisionByZero | ErrorKind::IntegerOverflow |
                 ErrorKind::FloatNaN | ErrorKind::FloatInfinity)
    }

    /// ¿Es un error fatal (no recuperable)?
    pub fn is_fatal(&self) -> bool {
        matches!(self.kind, ErrorKind::StackOverflow | ErrorKind::NullDereference |
                 ErrorKind::UseAfterFree | ErrorKind::DoubleFree | ErrorKind::InternalError)
    }
}

impl std::fmt::Display for KillerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[E{:04}] {}: {}", self.code, self.kind, self.message)?;
        if let Some(loc) = &self.location {
            write!(f, " (at {}:{}:{})", loc.file, loc.line, loc.column)?;
        }
        if let Some(cause) = &self.cause {
            write!(f, "\n  caused by: {}", cause)?;
        }
        Ok(())
    }
}

impl std::error::Error for KillerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.cause.as_ref().map(|c| c.as_ref() as &dyn std::error::Error)
    }
}

/// Categorías de error
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    // --- Memory Errors (1xx) ---
    /// Allocation failed — no hay memoria disponible
    OutOfMemory,
    /// Acceso fuera de límites
    OutOfBounds,
    /// Dereferencia de puntero nulo
    NullDereference,
    /// Uso después de liberar
    UseAfterFree,
    /// Doble liberación
    DoubleFree,
    /// Stack overflow
    StackOverflow,

    // --- Math Errors (2xx) ---
    /// División por cero
    DivisionByZero,
    /// Overflow de entero
    IntegerOverflow,
    /// Resultado NaN en float
    FloatNaN,
    /// Resultado Infinity en float
    FloatInfinity,

    // --- IO Errors (3xx) ---
    /// Archivo no encontrado
    FileNotFound,
    /// Permiso denegado
    PermissionDenied,
    /// Error de lectura/escritura
    IoError,

    // --- ABI Errors (4xx) ---
    /// Violación de calling convention
    AbiViolation,
    /// Stack no alineado
    StackMisalignment,
    /// Argumento inválido
    InvalidArgument,

    // --- Lifetime Errors (5xx) ---
    /// Recurso movido ya usado
    UseAfterMove,
    /// Mover mientras hay borrow activo
    MoveWhileBorrowed,

    // --- System Errors (9xx) ---
    /// Error interno del compilador/runtime
    InternalError,
    /// Feature no implementada
    NotImplemented,
    /// Assertion fallida
    AssertionFailed,
}

impl ErrorKind {
    /// Código numérico para matching rápido
    pub fn code(&self) -> u32 {
        match self {
            // Memory: 100-199
            ErrorKind::OutOfMemory => 100,
            ErrorKind::OutOfBounds => 101,
            ErrorKind::NullDereference => 102,
            ErrorKind::UseAfterFree => 103,
            ErrorKind::DoubleFree => 104,
            ErrorKind::StackOverflow => 105,
            // Math: 200-299
            ErrorKind::DivisionByZero => 200,
            ErrorKind::IntegerOverflow => 201,
            ErrorKind::FloatNaN => 202,
            ErrorKind::FloatInfinity => 203,
            // IO: 300-399
            ErrorKind::FileNotFound => 300,
            ErrorKind::PermissionDenied => 301,
            ErrorKind::IoError => 302,
            // ABI: 400-499
            ErrorKind::AbiViolation => 400,
            ErrorKind::StackMisalignment => 401,
            ErrorKind::InvalidArgument => 402,
            // Lifetime: 500-599
            ErrorKind::UseAfterMove => 500,
            ErrorKind::MoveWhileBorrowed => 501,
            // System: 900-999
            ErrorKind::InternalError => 900,
            ErrorKind::NotImplemented => 901,
            ErrorKind::AssertionFailed => 902,
        }
    }

    /// ¿Es recuperable?
    pub fn is_recoverable(&self) -> bool {
        !matches!(self,
            ErrorKind::StackOverflow | ErrorKind::NullDereference |
            ErrorKind::UseAfterFree | ErrorKind::DoubleFree |
            ErrorKind::InternalError
        )
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            ErrorKind::OutOfMemory => "OutOfMemory",
            ErrorKind::OutOfBounds => "OutOfBounds",
            ErrorKind::NullDereference => "NullDereference",
            ErrorKind::UseAfterFree => "UseAfterFree",
            ErrorKind::DoubleFree => "DoubleFree",
            ErrorKind::StackOverflow => "StackOverflow",
            ErrorKind::DivisionByZero => "DivisionByZero",
            ErrorKind::IntegerOverflow => "IntegerOverflow",
            ErrorKind::FloatNaN => "FloatNaN",
            ErrorKind::FloatInfinity => "FloatInfinity",
            ErrorKind::FileNotFound => "FileNotFound",
            ErrorKind::PermissionDenied => "PermissionDenied",
            ErrorKind::IoError => "IoError",
            ErrorKind::AbiViolation => "AbiViolation",
            ErrorKind::StackMisalignment => "StackMisalignment",
            ErrorKind::InvalidArgument => "InvalidArgument",
            ErrorKind::UseAfterMove => "UseAfterMove",
            ErrorKind::MoveWhileBorrowed => "MoveWhileBorrowed",
            ErrorKind::InternalError => "InternalError",
            ErrorKind::NotImplemented => "NotImplemented",
            ErrorKind::AssertionFailed => "AssertionFailed",
        };
        write!(f, "{}", name)
    }
}

/// Ubicación en el código fuente
#[derive(Debug, Clone)]
pub struct ErrorLocation {
    pub file: String,
    pub line: u32,
    pub column: u32,
}

// ─── Convenience constructors ───────────────────────────────────────────

/// Crear error de out of memory
pub fn oom(detail: impl Into<String>) -> KillerError {
    KillerError::new(ErrorKind::OutOfMemory, detail)
}

/// Crear error de division by zero
pub fn div_zero(detail: impl Into<String>) -> KillerError {
    KillerError::new(ErrorKind::DivisionByZero, detail)
}

/// Crear error de out of bounds
pub fn out_of_bounds(index: usize, len: usize) -> KillerError {
    KillerError::new(
        ErrorKind::OutOfBounds,
        format!("index {} out of bounds for length {}", index, len),
    )
}

/// Crear error de null dereference
pub fn null_deref(detail: impl Into<String>) -> KillerError {
    KillerError::new(ErrorKind::NullDereference, detail)
}

/// Crear error interno
pub fn internal(detail: impl Into<String>) -> KillerError {
    KillerError::new(ErrorKind::InternalError, detail)
}

/// Crear error de feature no implementada
pub fn not_implemented(feature: impl Into<String>) -> KillerError {
    KillerError::new(ErrorKind::NotImplemented, feature)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = KillerError::new(ErrorKind::DivisionByZero, "42 / 0");
        assert_eq!(err.code, 200);
        assert!(err.is_math_error());
        assert!(!err.is_fatal());
    }

    #[test]
    fn test_error_with_location() {
        let err = KillerError::new(ErrorKind::NullDereference, "ptr was null")
            .with_location("main.c", 42, 10);
        assert!(err.is_fatal());
        let s = format!("{}", err);
        assert!(s.contains("main.c:42:10"));
    }

    #[test]
    fn test_error_chaining() {
        let inner = KillerError::new(ErrorKind::OutOfMemory, "arena full");
        let outer = KillerError::new(ErrorKind::InternalError, "failed to compile")
            .with_cause(inner);

        assert!(outer.cause.is_some());
        let s = format!("{}", outer);
        assert!(s.contains("caused by"));
    }

    #[test]
    fn test_error_codes() {
        assert_eq!(ErrorKind::OutOfMemory.code(), 100);
        assert_eq!(ErrorKind::DivisionByZero.code(), 200);
        assert_eq!(ErrorKind::FileNotFound.code(), 300);
        assert_eq!(ErrorKind::AbiViolation.code(), 400);
        assert_eq!(ErrorKind::UseAfterMove.code(), 500);
        assert_eq!(ErrorKind::InternalError.code(), 900);
    }

    #[test]
    fn test_convenience_constructors() {
        let e1 = oom("arena full");
        assert_eq!(e1.kind, ErrorKind::OutOfMemory);

        let e2 = div_zero("x / 0");
        assert_eq!(e2.kind, ErrorKind::DivisionByZero);

        let e3 = out_of_bounds(10, 5);
        assert_eq!(e3.kind, ErrorKind::OutOfBounds);
        assert!(e3.message.contains("10"));
        assert!(e3.message.contains("5"));
    }

    #[test]
    fn test_result_propagation() {
        fn might_fail(x: i64) -> KillerResult<i64> {
            if x == 0 {
                Err(div_zero("cannot divide by zero"))
            } else {
                Ok(42 / x)
            }
        }

        fn caller() -> KillerResult<i64> {
            let val = might_fail(2)?; // Should succeed
            Ok(val + 1)
        }

        assert_eq!(caller().unwrap(), 22);
        assert!(might_fail(0).is_err());
    }

    #[test]
    fn test_recoverability() {
        assert!(ErrorKind::DivisionByZero.is_recoverable());
        assert!(ErrorKind::OutOfBounds.is_recoverable());
        assert!(!ErrorKind::NullDereference.is_recoverable());
        assert!(!ErrorKind::StackOverflow.is_recoverable());
        assert!(!ErrorKind::InternalError.is_recoverable());
    }
}
