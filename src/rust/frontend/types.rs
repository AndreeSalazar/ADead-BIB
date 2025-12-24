#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,            // 64-bit signed integer
    Float,          // 64-bit float (double)
    Bool,
    String,
    Void,
    
    // Tipos compuestos
    Array(Box<Type>),      // Array dinámico
    FixedArray(Box<Type>, usize), // Array estático
    
    // Tipos SIMD (potenciación)
    Vec4,           // 4 floats (128-bit SSE)
    Vec8,           // 8 floats (256-bit AVX)
    Vec16,          // 16 floats (512-bit AVX-512)
    
    // Objetos
    Class(String),
    
    // Desconocido (para inferencia)
    Unknown,
}

impl Type {
    pub fn size_in_bytes(&self) -> usize {
        match self {
            Type::Int | Type::Float => 8,
            Type::Bool => 1,
            Type::String => 8, // Puntero
            Type::Void => 0,
            Type::Array(_) => 16, // Ptr + Length
            Type::FixedArray(t, size) => t.size_in_bytes() * size,
            Type::Vec4 => 16,
            Type::Vec8 => 32,
            Type::Vec16 => 64,
            Type::Class(_) => 8, // Puntero a objeto
            Type::Unknown => 8, // Asumimos 64-bit
        }
    }
    
    pub fn is_simd(&self) -> bool {
        matches!(self, Type::Vec4 | Type::Vec8 | Type::Vec16)
    }
}
