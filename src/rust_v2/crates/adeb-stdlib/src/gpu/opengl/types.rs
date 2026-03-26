//! OpenGL Type Definitions
//! 
//! Based on Khronos canonical specifications.
//! All types are platform-independent and match the OpenGL ABI.

#![allow(non_camel_case_types)]

// =============================================================================
// BASIC TYPES (OpenGL 1.0+)
// =============================================================================

/// 8-bit signed integer
pub type GLbyte = i8;

/// 8-bit unsigned integer  
pub type GLubyte = u8;

/// 16-bit signed integer
pub type GLshort = i16;

/// 16-bit unsigned integer
pub type GLushort = u16;

/// 32-bit signed integer
pub type GLint = i32;

/// 32-bit unsigned integer
pub type GLuint = u32;

/// 32-bit floating point
pub type GLfloat = f32;

/// 64-bit floating point (double precision)
pub type GLdouble = f64;

/// Boolean type
pub type GLboolean = u8;

/// Size type (signed)
pub type GLsizei = i32;

/// Enum type
pub type GLenum = u32;

/// Bit field type
pub type GLbitfield = u32;

/// Void type
pub type GLvoid = core::ffi::c_void;

/// Clamped float [0.0, 1.0]
pub type GLclampf = f32;

/// Clamped double [0.0, 1.0]
pub type GLclampd = f64;

// =============================================================================
// EXTENDED TYPES (OpenGL 1.5+)
// =============================================================================

/// Pointer-sized signed integer (for buffer offsets)
pub type GLintptr = isize;

/// Pointer-sized unsigned integer (for buffer sizes)
pub type GLsizeiptr = isize;

// =============================================================================
// 64-BIT TYPES (OpenGL 3.2+)
// =============================================================================

/// 64-bit signed integer
pub type GLint64 = i64;

/// 64-bit unsigned integer
pub type GLuint64 = u64;

// =============================================================================
// SYNC TYPES (OpenGL 3.2+)
// =============================================================================

/// Sync object handle
pub type GLsync = *mut core::ffi::c_void;

// =============================================================================
// DEBUG TYPES (OpenGL 4.3+)
// =============================================================================

/// Debug callback function pointer
pub type GLDEBUGPROC = Option<
    unsafe extern "system" fn(
        source: GLenum,
        gltype: GLenum,
        id: GLuint,
        severity: GLenum,
        length: GLsizei,
        message: *const GLchar,
        userParam: *mut GLvoid,
    ),
>;

/// Debug callback function pointer (ARB extension)
pub type GLDEBUGPROCARB = GLDEBUGPROC;

/// Debug callback function pointer (AMD extension)
pub type GLDEBUGPROCAMD = Option<
    unsafe extern "system" fn(
        id: GLuint,
        category: GLenum,
        severity: GLenum,
        length: GLsizei,
        message: *const GLchar,
        userParam: *mut GLvoid,
    ),
>;

// =============================================================================
// CHARACTER TYPES
// =============================================================================

/// Character type (for strings)
pub type GLchar = i8;

/// Character type (ARB extension)
pub type GLcharARB = i8;

// =============================================================================
// HANDLE TYPES (OpenGL 4.1+)
// =============================================================================

/// Half-precision float (16-bit)
pub type GLhalf = u16;

/// Half-precision float (ARB extension)
pub type GLhalfARB = u16;

/// Half-precision float (NV extension)
pub type GLhalfNV = u16;

// =============================================================================
// FIXED-POINT TYPES (OpenGL ES compatibility)
// =============================================================================

/// Fixed-point type (16.16 format)
pub type GLfixed = i32;

// =============================================================================
// HANDLE TYPES (OpenGL 4.5+)
// =============================================================================

/// Handle type for bindless textures (ARB_bindless_texture)
pub type GLuint64EXT = u64;

/// Handle type for bindless textures
pub type GLhandleARB = u32;

// =============================================================================
// VERTEX ATTRIB TYPES (OpenGL 4.1+)
// =============================================================================

/// 10-10-10-2 packed vertex format
pub type GLuint_10_10_10_2 = u32;

/// 2-10-10-10 packed vertex format (reversed)
pub type GLuint_2_10_10_10_rev = u32;

// =============================================================================
// BUFFER OBJECT TYPES
// =============================================================================

/// Vertex Array Object handle
pub type GLVertexArray = GLuint;

/// Buffer Object handle
pub type GLBuffer = GLuint;

/// Texture Object handle
pub type GLTexture = GLuint;

/// Framebuffer Object handle
pub type GLFramebuffer = GLuint;

/// Renderbuffer Object handle
pub type GLRenderbuffer = GLuint;

/// Shader Object handle
pub type GLShader = GLuint;

/// Program Object handle
pub type GLProgram = GLuint;

/// Program Pipeline Object handle (OpenGL 4.1+)
pub type GLProgramPipeline = GLuint;

/// Sampler Object handle (OpenGL 3.3+)
pub type GLSampler = GLuint;

/// Query Object handle
pub type GLQuery = GLuint;

/// Transform Feedback Object handle (OpenGL 4.0+)
pub type GLTransformFeedback = GLuint;

// =============================================================================
// CONSTANTS
// =============================================================================

pub const GL_FALSE: GLboolean = 0;
pub const GL_TRUE: GLboolean = 1;

// =============================================================================
// HELPER TRAITS
// =============================================================================

/// Trait for types that can be used as OpenGL vertex attributes
pub trait GLVertexAttrib {
    const GL_TYPE: GLenum;
    const COMPONENTS: GLint;
    const NORMALIZED: GLboolean;
}

impl GLVertexAttrib for f32 {
    const GL_TYPE: GLenum = 0x1406; // GL_FLOAT
    const COMPONENTS: GLint = 1;
    const NORMALIZED: GLboolean = GL_FALSE;
}

impl GLVertexAttrib for [f32; 2] {
    const GL_TYPE: GLenum = 0x1406; // GL_FLOAT
    const COMPONENTS: GLint = 2;
    const NORMALIZED: GLboolean = GL_FALSE;
}

impl GLVertexAttrib for [f32; 3] {
    const GL_TYPE: GLenum = 0x1406; // GL_FLOAT
    const COMPONENTS: GLint = 3;
    const NORMALIZED: GLboolean = GL_FALSE;
}

impl GLVertexAttrib for [f32; 4] {
    const GL_TYPE: GLenum = 0x1406; // GL_FLOAT
    const COMPONENTS: GLint = 4;
    const NORMALIZED: GLboolean = GL_FALSE;
}

impl GLVertexAttrib for i32 {
    const GL_TYPE: GLenum = 0x1404; // GL_INT
    const COMPONENTS: GLint = 1;
    const NORMALIZED: GLboolean = GL_FALSE;
}

impl GLVertexAttrib for u32 {
    const GL_TYPE: GLenum = 0x1405; // GL_UNSIGNED_INT
    const COMPONENTS: GLint = 1;
    const NORMALIZED: GLboolean = GL_FALSE;
}

impl GLVertexAttrib for u8 {
    const GL_TYPE: GLenum = 0x1401; // GL_UNSIGNED_BYTE
    const COMPONENTS: GLint = 1;
    const NORMALIZED: GLboolean = GL_TRUE;
}

impl GLVertexAttrib for [u8; 4] {
    const GL_TYPE: GLenum = 0x1401; // GL_UNSIGNED_BYTE
    const COMPONENTS: GLint = 4;
    const NORMALIZED: GLboolean = GL_TRUE;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_sizes() {
        assert_eq!(core::mem::size_of::<GLbyte>(), 1);
        assert_eq!(core::mem::size_of::<GLubyte>(), 1);
        assert_eq!(core::mem::size_of::<GLshort>(), 2);
        assert_eq!(core::mem::size_of::<GLushort>(), 2);
        assert_eq!(core::mem::size_of::<GLint>(), 4);
        assert_eq!(core::mem::size_of::<GLuint>(), 4);
        assert_eq!(core::mem::size_of::<GLfloat>(), 4);
        assert_eq!(core::mem::size_of::<GLdouble>(), 8);
        assert_eq!(core::mem::size_of::<GLint64>(), 8);
        assert_eq!(core::mem::size_of::<GLuint64>(), 8);
    }

    #[test]
    fn test_vertex_attrib_trait() {
        assert_eq!(<f32 as GLVertexAttrib>::COMPONENTS, 1);
        assert_eq!(<[f32; 3] as GLVertexAttrib>::COMPONENTS, 3);
        assert_eq!(<[f32; 4] as GLVertexAttrib>::COMPONENTS, 4);
    }
}
