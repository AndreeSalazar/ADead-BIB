//! OpenGL 1.1 Functions
//! Adds texture objects and vertex arrays

use super::types::*;

pub type PFNGLDRAWARRAYSPROC = Option<unsafe extern "system" fn(mode: GLenum, first: GLint, count: GLsizei)>;
pub type PFNGLDRAWELEMENTSPROC = Option<unsafe extern "system" fn(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const GLvoid)>;
pub type PFNGLGENTEXTURESPROC = Option<unsafe extern "system" fn(n: GLsizei, textures: *mut GLuint)>;
pub type PFNGLDELETETEXTURESPROC = Option<unsafe extern "system" fn(n: GLsizei, textures: *const GLuint)>;
pub type PFNGLBINDTEXTUREPROC = Option<unsafe extern "system" fn(target: GLenum, texture: GLuint)>;
pub type PFNGLISTEXTUREPROC = Option<unsafe extern "system" fn(texture: GLuint) -> GLboolean>;
pub type PFNGLTEXSUBIMAGE1DPROC = Option<unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *const GLvoid)>;
pub type PFNGLTEXSUBIMAGE2DPROC = Option<unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const GLvoid)>;
pub type PFNGLCOPYTEXIMAGE1DPROC = Option<unsafe extern "system" fn(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, border: GLint)>;
pub type PFNGLCOPYTEXIMAGE2DPROC = Option<unsafe extern "system" fn(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint)>;
pub type PFNGLCOPYTEXSUBIMAGE1DPROC = Option<unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei)>;
pub type PFNGLCOPYTEXSUBIMAGE2DPROC = Option<unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei)>;
pub type PFNGLPOLYGONOFFSETPROC = Option<unsafe extern "system" fn(factor: GLfloat, units: GLfloat)>;

#[derive(Default)]
pub struct GL11 {
    pub glDrawArrays: PFNGLDRAWARRAYSPROC,
    pub glDrawElements: PFNGLDRAWELEMENTSPROC,
    pub glGenTextures: PFNGLGENTEXTURESPROC,
    pub glDeleteTextures: PFNGLDELETETEXTURESPROC,
    pub glBindTexture: PFNGLBINDTEXTUREPROC,
    pub glIsTexture: PFNGLISTEXTUREPROC,
    pub glTexSubImage1D: PFNGLTEXSUBIMAGE1DPROC,
    pub glTexSubImage2D: PFNGLTEXSUBIMAGE2DPROC,
    pub glCopyTexImage1D: PFNGLCOPYTEXIMAGE1DPROC,
    pub glCopyTexImage2D: PFNGLCOPYTEXIMAGE2DPROC,
    pub glCopyTexSubImage1D: PFNGLCOPYTEXSUBIMAGE1DPROC,
    pub glCopyTexSubImage2D: PFNGLCOPYTEXSUBIMAGE2DPROC,
    pub glPolygonOffset: PFNGLPOLYGONOFFSETPROC,
}
