//! OpenGL 1.2 Functions - 3D textures, packed pixels
use super::types::*;

pub type PFNGLDRAWRANGEELEMENTSPROC = Option<unsafe extern "system" fn(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *const GLvoid)>;
pub type PFNGLTEXIMAGE3DPROC = Option<unsafe extern "system" fn(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const GLvoid)>;
pub type PFNGLTEXSUBIMAGE3DPROC = Option<unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *const GLvoid)>;
pub type PFNGLCOPYTEXSUBIMAGE3DPROC = Option<unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei)>;

#[derive(Default)]
pub struct GL12 {
    pub glDrawRangeElements: PFNGLDRAWRANGEELEMENTSPROC,
    pub glTexImage3D: PFNGLTEXIMAGE3DPROC,
    pub glTexSubImage3D: PFNGLTEXSUBIMAGE3DPROC,
    pub glCopyTexSubImage3D: PFNGLCOPYTEXSUBIMAGE3DPROC,
}
