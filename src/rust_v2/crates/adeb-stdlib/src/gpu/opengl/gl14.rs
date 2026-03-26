//! OpenGL 1.4 Functions - Blend equations, depth textures
use super::types::*;

pub type PFNGLBLENDFUNCSEPARATEPROC = Option<unsafe extern "system" fn(sfactorRGB: GLenum, dfactorRGB: GLenum, sfactorAlpha: GLenum, dfactorAlpha: GLenum)>;
pub type PFNGLMULTIDRAWARRAYSPROC = Option<unsafe extern "system" fn(mode: GLenum, first: *const GLint, count: *const GLsizei, drawcount: GLsizei)>;
pub type PFNGLMULTIDRAWELEMENTSPROC = Option<unsafe extern "system" fn(mode: GLenum, count: *const GLsizei, type_: GLenum, indices: *const *const GLvoid, drawcount: GLsizei)>;
pub type PFNGLPOINTPARAMETERFPROC = Option<unsafe extern "system" fn(pname: GLenum, param: GLfloat)>;
pub type PFNGLPOINTPARAMETERFVPROC = Option<unsafe extern "system" fn(pname: GLenum, params: *const GLfloat)>;
pub type PFNGLPOINTPARAMETERIPROC = Option<unsafe extern "system" fn(pname: GLenum, param: GLint)>;
pub type PFNGLPOINTPARAMETERIVPROC = Option<unsafe extern "system" fn(pname: GLenum, params: *const GLint)>;
pub type PFNGLBLENDCOLORPROC = Option<unsafe extern "system" fn(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat)>;
pub type PFNGLBLENDEQUATIONPROC = Option<unsafe extern "system" fn(mode: GLenum)>;

#[derive(Default)]
pub struct GL14 {
    pub glBlendFuncSeparate: PFNGLBLENDFUNCSEPARATEPROC,
    pub glMultiDrawArrays: PFNGLMULTIDRAWARRAYSPROC,
    pub glMultiDrawElements: PFNGLMULTIDRAWELEMENTSPROC,
    pub glPointParameterf: PFNGLPOINTPARAMETERFPROC,
    pub glPointParameterfv: PFNGLPOINTPARAMETERFVPROC,
    pub glPointParameteri: PFNGLPOINTPARAMETERIPROC,
    pub glPointParameteriv: PFNGLPOINTPARAMETERIVPROC,
    pub glBlendColor: PFNGLBLENDCOLORPROC,
    pub glBlendEquation: PFNGLBLENDEQUATIONPROC,
}
