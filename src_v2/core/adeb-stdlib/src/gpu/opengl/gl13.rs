//! OpenGL 1.3 Functions - Multitexture, compressed textures
use super::types::*;

pub type PFNGLACTIVETEXTUREPROC = Option<unsafe extern "system" fn(texture: GLenum)>;
pub type PFNGLSAMPLECOVERAGEPROC = Option<unsafe extern "system" fn(value: GLfloat, invert: GLboolean)>;
pub type PFNGLCOMPRESSEDTEXIMAGE3DPROC = Option<unsafe extern "system" fn(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *const GLvoid)>;
pub type PFNGLCOMPRESSEDTEXIMAGE2DPROC = Option<unsafe extern "system" fn(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *const GLvoid)>;
pub type PFNGLCOMPRESSEDTEXIMAGE1DPROC = Option<unsafe extern "system" fn(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, border: GLint, imageSize: GLsizei, data: *const GLvoid)>;
pub type PFNGLCOMPRESSEDTEXSUBIMAGE3DPROC = Option<unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const GLvoid)>;
pub type PFNGLCOMPRESSEDTEXSUBIMAGE2DPROC = Option<unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const GLvoid)>;
pub type PFNGLCOMPRESSEDTEXSUBIMAGE1DPROC = Option<unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *const GLvoid)>;
pub type PFNGLGETCOMPRESSEDTEXIMAGEPROC = Option<unsafe extern "system" fn(target: GLenum, level: GLint, img: *mut GLvoid)>;

#[derive(Default)]
pub struct GL13 {
    pub glActiveTexture: PFNGLACTIVETEXTUREPROC,
    pub glSampleCoverage: PFNGLSAMPLECOVERAGEPROC,
    pub glCompressedTexImage3D: PFNGLCOMPRESSEDTEXIMAGE3DPROC,
    pub glCompressedTexImage2D: PFNGLCOMPRESSEDTEXIMAGE2DPROC,
    pub glCompressedTexImage1D: PFNGLCOMPRESSEDTEXIMAGE1DPROC,
    pub glCompressedTexSubImage3D: PFNGLCOMPRESSEDTEXSUBIMAGE3DPROC,
    pub glCompressedTexSubImage2D: PFNGLCOMPRESSEDTEXSUBIMAGE2DPROC,
    pub glCompressedTexSubImage1D: PFNGLCOMPRESSEDTEXSUBIMAGE1DPROC,
    pub glGetCompressedTexImage: PFNGLGETCOMPRESSEDTEXIMAGEPROC,
}
