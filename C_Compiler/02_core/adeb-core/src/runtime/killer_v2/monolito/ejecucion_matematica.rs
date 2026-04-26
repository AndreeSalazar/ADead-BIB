// ============================================================================
// ejecucion_matematica.rs — IEEE 754 Strict Determinista
// ============================================================================
//
// FILOSOFÍA: Resultado matemático idéntico en toda plataforma x86-64.
// NaN = error explícito, NUNCA silencioso.
// Division by zero = error, NUNCA infinity.
// Overflow = error, NUNCA wrap silencioso.
//
// Modos:
//   - Strict: Todo error es fatal (default)
//   - Saturating: Overflow → MAX/MIN, underflow → 0
//   - Wrapping: C-standard wrapping para unsigned (para compat)
//
// Autor: Eddi Andreé Salazar Matos
// ============================================================================

/// Modo de ejecución matemática
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MathMode {
    /// IEEE 754 strict — cualquier error es fatal
    Strict,
    /// Saturating — overflow clamps a MAX/MIN
    Saturating,
    /// Wrapping — C-standard wrapping (para compatibilidad)
    Wrapping,
}

impl Default for MathMode {
    fn default() -> Self {
        MathMode::Strict
    }
}

/// Errores matemáticos — siempre explícitos
#[derive(Debug, Clone, PartialEq)]
pub enum MathError {
    /// División por cero
    DivisionByZero { operation: String },
    /// Overflow en operación
    Overflow { operation: String, lhs: String, rhs: String },
    /// Underflow en operación
    Underflow { operation: String },
    /// NaN resultado
    NotANumber { operation: String },
    /// Infinity resultado
    Infinity { operation: String },
    /// Negative overflow (underflow de signo)
    NegativeOverflow { operation: String },
}

impl std::fmt::Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MathError::DivisionByZero { operation } => write!(f, "Division by zero in {}", operation),
            MathError::Overflow { operation, lhs, rhs } => {
                write!(f, "Overflow in {} ({} op {})", operation, lhs, rhs)
            }
            MathError::Underflow { operation } => write!(f, "Underflow in {}", operation),
            MathError::NotANumber { operation } => write!(f, "NaN result in {}", operation),
            MathError::Infinity { operation } => write!(f, "Infinity result in {}", operation),
            MathError::NegativeOverflow { operation } => write!(f, "Negative overflow in {}", operation),
        }
    }
}

impl std::error::Error for MathError {}

/// Tipo de resultado matemático
pub type MathResult<T> = Result<T, MathError>;

/// Ejecutor matemático determinista
#[derive(Debug, Clone)]
pub struct StrictMath {
    mode: MathMode,
    /// Contador de operaciones (para profiling)
    op_count: u64,
}

impl StrictMath {
    pub fn new(mode: MathMode) -> Self {
        Self { mode, op_count: 0 }
    }

    /// Modo strict por defecto
    pub fn strict() -> Self {
        Self::new(MathMode::Strict)
    }

    /// Número de operaciones realizadas
    pub fn operation_count(&self) -> u64 {
        self.op_count
    }

    // ─── Integer Operations ─────────────────────────────────────────────

    /// Suma de enteros con overflow checking
    pub fn add_i64(&mut self, a: i64, b: i64) -> MathResult<i64> {
        self.op_count += 1;
        match self.mode {
            MathMode::Strict => {
                a.checked_add(b).ok_or_else(|| MathError::Overflow {
                    operation: "add_i64".into(),
                    lhs: a.to_string(),
                    rhs: b.to_string(),
                })
            }
            MathMode::Saturating => Ok(a.saturating_add(b)),
            MathMode::Wrapping => Ok(a.wrapping_add(b)),
        }
    }

    /// Resta de enteros con overflow checking
    pub fn sub_i64(&mut self, a: i64, b: i64) -> MathResult<i64> {
        self.op_count += 1;
        match self.mode {
            MathMode::Strict => {
                a.checked_sub(b).ok_or_else(|| MathError::Overflow {
                    operation: "sub_i64".into(),
                    lhs: a.to_string(),
                    rhs: b.to_string(),
                })
            }
            MathMode::Saturating => Ok(a.saturating_sub(b)),
            MathMode::Wrapping => Ok(a.wrapping_sub(b)),
        }
    }

    /// Multiplicación de enteros con overflow checking
    pub fn mul_i64(&mut self, a: i64, b: i64) -> MathResult<i64> {
        self.op_count += 1;
        match self.mode {
            MathMode::Strict => {
                a.checked_mul(b).ok_or_else(|| MathError::Overflow {
                    operation: "mul_i64".into(),
                    lhs: a.to_string(),
                    rhs: b.to_string(),
                })
            }
            MathMode::Saturating => Ok(a.saturating_mul(b)),
            MathMode::Wrapping => Ok(a.wrapping_mul(b)),
        }
    }

    /// División de enteros — NUNCA divide por cero
    pub fn div_i64(&mut self, a: i64, b: i64) -> MathResult<i64> {
        self.op_count += 1;
        if b == 0 {
            return Err(MathError::DivisionByZero {
                operation: format!("div_i64({}, 0)", a),
            });
        }
        // Check for MIN / -1 overflow
        if a == i64::MIN && b == -1 {
            return match self.mode {
                MathMode::Strict => Err(MathError::Overflow {
                    operation: "div_i64".into(),
                    lhs: a.to_string(),
                    rhs: b.to_string(),
                }),
                MathMode::Saturating => Ok(i64::MAX),
                MathMode::Wrapping => Ok(a.wrapping_div(b)),
            };
        }
        Ok(a / b)
    }

    /// Módulo de enteros — NUNCA divide por cero
    pub fn rem_i64(&mut self, a: i64, b: i64) -> MathResult<i64> {
        self.op_count += 1;
        if b == 0 {
            return Err(MathError::DivisionByZero {
                operation: format!("rem_i64({}, 0)", a),
            });
        }
        if a == i64::MIN && b == -1 {
            return Ok(0); // Mathematically correct
        }
        Ok(a % b)
    }

    /// Negación con overflow checking (MIN → error en strict)
    pub fn neg_i64(&mut self, a: i64) -> MathResult<i64> {
        self.op_count += 1;
        match self.mode {
            MathMode::Strict => {
                a.checked_neg().ok_or_else(|| MathError::NegativeOverflow {
                    operation: format!("neg_i64({})", a),
                })
            }
            MathMode::Saturating => Ok(if a == i64::MIN { i64::MAX } else { -a }),
            MathMode::Wrapping => Ok(a.wrapping_neg()),
        }
    }

    // ─── Unsigned Integer Operations ────────────────────────────────────

    /// Suma unsigned
    pub fn add_u64(&mut self, a: u64, b: u64) -> MathResult<u64> {
        self.op_count += 1;
        match self.mode {
            MathMode::Strict => {
                a.checked_add(b).ok_or_else(|| MathError::Overflow {
                    operation: "add_u64".into(),
                    lhs: a.to_string(),
                    rhs: b.to_string(),
                })
            }
            MathMode::Saturating => Ok(a.saturating_add(b)),
            MathMode::Wrapping => Ok(a.wrapping_add(b)),
        }
    }

    /// Multiplicación unsigned
    pub fn mul_u64(&mut self, a: u64, b: u64) -> MathResult<u64> {
        self.op_count += 1;
        match self.mode {
            MathMode::Strict => {
                a.checked_mul(b).ok_or_else(|| MathError::Overflow {
                    operation: "mul_u64".into(),
                    lhs: a.to_string(),
                    rhs: b.to_string(),
                })
            }
            MathMode::Saturating => Ok(a.saturating_mul(b)),
            MathMode::Wrapping => Ok(a.wrapping_mul(b)),
        }
    }

    // ─── Floating Point Operations ──────────────────────────────────────

    /// Suma float — verifica NaN e Infinity
    pub fn add_f64(&mut self, a: f64, b: f64) -> MathResult<f64> {
        self.op_count += 1;
        self.validate_float_inputs(a, b, "add_f64")?;
        let result = a + b;
        self.validate_float_result(result, "add_f64")
    }

    /// Resta float
    pub fn sub_f64(&mut self, a: f64, b: f64) -> MathResult<f64> {
        self.op_count += 1;
        self.validate_float_inputs(a, b, "sub_f64")?;
        let result = a - b;
        self.validate_float_result(result, "sub_f64")
    }

    /// Multiplicación float
    pub fn mul_f64(&mut self, a: f64, b: f64) -> MathResult<f64> {
        self.op_count += 1;
        self.validate_float_inputs(a, b, "mul_f64")?;
        let result = a * b;
        self.validate_float_result(result, "mul_f64")
    }

    /// División float — NUNCA produce NaN/Infinity silenciosamente
    pub fn div_f64(&mut self, a: f64, b: f64) -> MathResult<f64> {
        self.op_count += 1;
        self.validate_float_inputs(a, b, "div_f64")?;
        if b == 0.0 {
            return Err(MathError::DivisionByZero {
                operation: format!("div_f64({}, 0.0)", a),
            });
        }
        let result = a / b;
        self.validate_float_result(result, "div_f64")
    }

    /// Raíz cuadrada — NUNCA produce NaN silenciosamente
    pub fn sqrt_f64(&mut self, a: f64) -> MathResult<f64> {
        self.op_count += 1;
        if a.is_nan() {
            return Err(MathError::NotANumber { operation: "sqrt_f64".into() });
        }
        if a < 0.0 {
            return Err(MathError::NotANumber {
                operation: format!("sqrt_f64({})", a),
            });
        }
        let result = a.sqrt();
        self.validate_float_result(result, "sqrt_f64")
    }

    // ─── Internal Validators ────────────────────────────────────────────

    fn validate_float_inputs(&self, a: f64, b: f64, op: &str) -> MathResult<()> {
        if self.mode == MathMode::Strict {
            if a.is_nan() || b.is_nan() {
                return Err(MathError::NotANumber { operation: op.into() });
            }
            if a.is_infinite() || b.is_infinite() {
                return Err(MathError::Infinity { operation: op.into() });
            }
        }
        Ok(())
    }

    fn validate_float_result(&self, result: f64, op: &str) -> MathResult<f64> {
        if self.mode == MathMode::Strict {
            if result.is_nan() {
                return Err(MathError::NotANumber { operation: op.into() });
            }
            if result.is_infinite() {
                return Err(MathError::Infinity { operation: op.into() });
            }
        }
        Ok(result)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strict_overflow() {
        let mut math = StrictMath::strict();
        let result = math.add_i64(i64::MAX, 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_saturating_overflow() {
        let mut math = StrictMath::new(MathMode::Saturating);
        let result = math.add_i64(i64::MAX, 1).unwrap();
        assert_eq!(result, i64::MAX);
    }

    #[test]
    fn test_wrapping_overflow() {
        let mut math = StrictMath::new(MathMode::Wrapping);
        let result = math.add_i64(i64::MAX, 1).unwrap();
        assert_eq!(result, i64::MIN);
    }

    #[test]
    fn test_division_by_zero() {
        let mut math = StrictMath::strict();
        let result = math.div_i64(42, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_min_div_neg1() {
        let mut math = StrictMath::strict();
        let result = math.div_i64(i64::MIN, -1);
        assert!(result.is_err()); // Overflow in strict mode
    }

    #[test]
    fn test_float_nan_rejected() {
        let mut math = StrictMath::strict();
        let result = math.add_f64(f64::NAN, 1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_float_inf_rejected() {
        let mut math = StrictMath::strict();
        let result = math.add_f64(f64::INFINITY, 1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_float_div_by_zero() {
        let mut math = StrictMath::strict();
        let result = math.div_f64(1.0, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_sqrt_negative() {
        let mut math = StrictMath::strict();
        let result = math.sqrt_f64(-1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_normal_arithmetic() {
        let mut math = StrictMath::strict();
        assert_eq!(math.add_i64(40, 2).unwrap(), 42);
        assert_eq!(math.mul_i64(6, 7).unwrap(), 42);
        assert_eq!(math.div_i64(84, 2).unwrap(), 42);
        assert_eq!(math.sub_i64(50, 8).unwrap(), 42);
        assert_eq!(math.operation_count(), 4);
    }

    #[test]
    fn test_float_normal() {
        let mut math = StrictMath::strict();
        let result = math.add_f64(3.14, 2.86).unwrap();
        assert!((result - 6.0).abs() < 1e-10);
    }
}
