//! OpenGL Constants - All Versions (1.0 - 4.6)
//! 
//! This module re-exports all OpenGL constants from the canonical Khronos specifications.
//! Constants are organized by version for clarity and to maintain compatibility tracking.
//!
//! # Organization
//! - `constants_gl1x` - OpenGL 1.0 through 1.5
//! - `constants_gl2x` - OpenGL 2.0 through 2.1
//! - `constants_gl3x` - OpenGL 3.0 through 3.3
//! - `constants_gl4x` - OpenGL 4.0 through 4.6 (final version)
//!
//! # Usage
//! ```rust
//! use opengl::constants::*;
//! 
//! // All constants are available directly
//! let primitive = GL_TRIANGLES;
//! let buffer_type = GL_ARRAY_BUFFER;
//! let shader = GL_VERTEX_SHADER;
//! ```

// Re-export all constants from version-specific modules
pub use super::constants_gl1x::*;
pub use super::constants_gl2x::*;
pub use super::constants_gl3x::*;
pub use super::constants_gl4x::*;

// ============================================================================
// Version Detection Helpers
// ============================================================================

use super::types::GLenum;

/// Check if a constant is available in a specific OpenGL version
#[inline]
pub const fn is_gl10_constant(c: GLenum) -> bool {
    // GL 1.0 constants are in ranges: 0x0000-0x0FFF, 0x1000-0x1FFF, etc.
    c < 0x8000
}

/// Check if constant requires GL 2.0+
#[inline]
pub const fn requires_gl20(c: GLenum) -> bool {
    matches!(c, 
        0x8B30..=0x8B8D | // Shaders
        0x8800..=0x8834   // Draw buffers, stencil separate
    )
}

/// Check if constant requires GL 3.0+
#[inline]
pub const fn requires_gl30(c: GLenum) -> bool {
    matches!(c,
        0x8CA6..=0x8D57 | // Framebuffer objects
        0x8C18..=0x8C1D | // Texture arrays
        0x8E13..=0x8E16   // Conditional render
    )
}

/// Check if constant requires GL 4.0+
#[inline]
pub const fn requires_gl40(c: GLenum) -> bool {
    matches!(c,
        0x8E72..=0x8E8A | // Tessellation
        0x8DE5..=0x8E4B | // Subroutines
        0x8E22..=0x8E25   // Transform feedback objects
    )
}

/// Check if constant requires GL 4.3+ (compute shaders)
#[inline]
pub const fn requires_gl43(c: GLenum) -> bool {
    matches!(c,
        0x91B9..=0x91BF | // Compute shaders
        0x90D2..=0x90DF | // Shader storage
        0x8242..=0x826D   // Debug output
    )
}

/// Check if constant requires GL 4.6 (SPIR-V)
#[inline]
pub const fn requires_gl46(c: GLenum) -> bool {
    matches!(c,
        0x9551..=0x9554 | // SPIR-V
        0x82EE..=0x82F7   // Pipeline statistics
    )
}

// ============================================================================
// Constant Categories
// ============================================================================

/// Categories of OpenGL constants for optimization purposes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstantCategory {
    /// Boolean values (GL_TRUE, GL_FALSE)
    Boolean,
    /// Data types (GL_FLOAT, GL_INT, etc.)
    DataType,
    /// Primitive types (GL_TRIANGLES, GL_LINES, etc.)
    Primitive,
    /// Buffer targets (GL_ARRAY_BUFFER, GL_ELEMENT_ARRAY_BUFFER, etc.)
    BufferTarget,
    /// Texture targets (GL_TEXTURE_2D, GL_TEXTURE_CUBE_MAP, etc.)
    TextureTarget,
    /// Shader types (GL_VERTEX_SHADER, GL_FRAGMENT_SHADER, etc.)
    ShaderType,
    /// Framebuffer attachments
    FramebufferAttachment,
    /// Blend factors
    BlendFactor,
    /// Comparison functions
    CompareFunc,
    /// Error codes
    Error,
    /// Capability flags (for glEnable/glDisable)
    Capability,
    /// Hints
    Hint,
    /// Other/Unknown
    Other,
}

/// Categorize a constant for optimization purposes
#[inline]
pub const fn categorize_constant(c: GLenum) -> ConstantCategory {
    match c {
        0 | 1 => ConstantCategory::Boolean,
        0x1400..=0x140B => ConstantCategory::DataType,
        0x0000..=0x000E => ConstantCategory::Primitive,
        0x8892..=0x90EF => ConstantCategory::BufferTarget,
        0x0DE0..=0x9105 if matches!(c, 0x0DE0 | 0x0DE1 | 0x806F | 0x8513 | 0x84F5 | 0x8C18 | 0x8C1A | 0x9009 | 0x9100 | 0x9102) => ConstantCategory::TextureTarget,
        0x8B30..=0x91B9 if matches!(c, 0x8B30 | 0x8B31 | 0x8DD9 | 0x8E87 | 0x8E88 | 0x91B9) => ConstantCategory::ShaderType,
        0x8CE0..=0x8D20 => ConstantCategory::FramebufferAttachment,
        0x0300..=0x0308 | 0x8001..=0x8004 => ConstantCategory::BlendFactor,
        0x0200..=0x0207 => ConstantCategory::CompareFunc,
        0x0500..=0x0507 => ConstantCategory::Error,
        _ => ConstantCategory::Other,
    }
}

// ============================================================================
// Compile-time Validation
// ============================================================================

/// Validate that a constant value is a valid OpenGL enum
/// This is useful for compile-time checks in const contexts
#[inline]
pub const fn is_valid_gl_enum(c: GLenum) -> bool {
    // OpenGL enums are typically in specific ranges
    // This is a basic validation - not exhaustive
    c <= 0xFFFF || (c >= 0x80000000 && c <= 0xFFFFFFFF)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_constants() {
        assert_eq!(GL_FALSE, 0);
        assert_eq!(GL_TRUE, 1);
        assert_eq!(GL_TRIANGLES, 0x0004);
        assert_eq!(GL_ARRAY_BUFFER, 0x8892);
    }

    #[test]
    fn test_shader_constants() {
        assert_eq!(GL_VERTEX_SHADER, 0x8B31);
        assert_eq!(GL_FRAGMENT_SHADER, 0x8B30);
        assert_eq!(GL_GEOMETRY_SHADER, 0x8DD9);
        assert_eq!(GL_COMPUTE_SHADER, 0x91B9);
    }

    #[test]
    fn test_gl46_constants() {
        assert_eq!(GL_SHADER_BINARY_FORMAT_SPIR_V, 0x9551);
        assert_eq!(GL_SPIR_V_BINARY, 0x9552);
    }

    #[test]
    fn test_version_detection() {
        assert!(is_gl10_constant(GL_TRIANGLES));
        assert!(requires_gl20(GL_VERTEX_SHADER));
        assert!(requires_gl30(GL_FRAMEBUFFER));
        assert!(requires_gl43(GL_COMPUTE_SHADER));
        assert!(requires_gl46(GL_SPIR_V_BINARY));
    }

    #[test]
    fn test_categorization() {
        assert_eq!(categorize_constant(GL_TRUE), ConstantCategory::Boolean);
        assert_eq!(categorize_constant(GL_FLOAT), ConstantCategory::DataType);
        assert_eq!(categorize_constant(GL_TRIANGLES), ConstantCategory::Primitive);
    }
}
