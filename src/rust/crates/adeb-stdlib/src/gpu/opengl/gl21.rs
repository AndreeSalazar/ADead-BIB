//! OpenGL 2.1 Functions — Non-square matrix uniforms
//! Based on Khronos canonical specifications
use super::types::*;

pub type PFNGLUNIFORMMATRIX2X3FVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat)>;
pub type PFNGLUNIFORMMATRIX3X2FVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat)>;
pub type PFNGLUNIFORMMATRIX2X4FVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat)>;
pub type PFNGLUNIFORMMATRIX4X2FVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat)>;
pub type PFNGLUNIFORMMATRIX3X4FVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat)>;
pub type PFNGLUNIFORMMATRIX4X3FVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat)>;

/// OpenGL 2.1 function table
#[derive(Default)]
pub struct GL21 {
    pub glUniformMatrix2x3fv: PFNGLUNIFORMMATRIX2X3FVPROC,
    pub glUniformMatrix3x2fv: PFNGLUNIFORMMATRIX3X2FVPROC,
    pub glUniformMatrix2x4fv: PFNGLUNIFORMMATRIX2X4FVPROC,
    pub glUniformMatrix4x2fv: PFNGLUNIFORMMATRIX4X2FVPROC,
    pub glUniformMatrix3x4fv: PFNGLUNIFORMMATRIX3X4FVPROC,
    pub glUniformMatrix4x3fv: PFNGLUNIFORMMATRIX4X3FVPROC,
}
