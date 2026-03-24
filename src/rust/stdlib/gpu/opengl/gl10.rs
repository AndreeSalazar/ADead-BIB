//! OpenGL 1.0 Functions
//! Based on Khronos canonical specifications

use super::types::*;

// Function pointer types for GL 1.0
pub type PFNGLCULLFACEPROC = Option<unsafe extern "system" fn(mode: GLenum)>;
pub type PFNGLFABORDFACEPROC = Option<unsafe extern "system" fn(mode: GLenum)>;
pub type PFNGLHINTPROC = Option<unsafe extern "system" fn(target: GLenum, mode: GLenum)>;
pub type PFNGLLINEWIDTHPROC = Option<unsafe extern "system" fn(width: GLfloat)>;
pub type PFNGLPOINTSIZEPROC = Option<unsafe extern "system" fn(size: GLfloat)>;
pub type PFNGLPOLYGONMODEPROC = Option<unsafe extern "system" fn(face: GLenum, mode: GLenum)>;
pub type PFNGLSCISSORPROC = Option<unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei)>;
pub type PFNGLTEXPARAMETERFPROC = Option<unsafe extern "system" fn(target: GLenum, pname: GLenum, param: GLfloat)>;
pub type PFNGLTEXPARAMETERFVPROC = Option<unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLfloat)>;
pub type PFNGLTEXPARAMETERIPROC = Option<unsafe extern "system" fn(target: GLenum, pname: GLenum, param: GLint)>;
pub type PFNGLTEXPARAMETERIVPROC = Option<unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLint)>;
pub type PFNGLTEXIMAGE1DPROC = Option<unsafe extern "system" fn(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const GLvoid)>;
pub type PFNGLTEXIMAGE2DPROC = Option<unsafe extern "system" fn(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const GLvoid)>;
pub type PFNGLDRAWBUFFERPROC = Option<unsafe extern "system" fn(buf: GLenum)>;
pub type PFNGLCLEARPROC = Option<unsafe extern "system" fn(mask: GLbitfield)>;
pub type PFNGLCLEARCOLORPROC = Option<unsafe extern "system" fn(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat)>;
pub type PFNGLCLEARSTENCILPROC = Option<unsafe extern "system" fn(s: GLint)>;
pub type PFNGLCLEARDEPTHPROC = Option<unsafe extern "system" fn(depth: GLdouble)>;
pub type PFNGLSTENCILMASKPROC = Option<unsafe extern "system" fn(mask: GLuint)>;
pub type PFNGLCOLORMASKPROC = Option<unsafe extern "system" fn(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean)>;
pub type PFNGLDEPTHMASKPROC = Option<unsafe extern "system" fn(flag: GLboolean)>;
pub type PFNGLDISABLEPROC = Option<unsafe extern "system" fn(cap: GLenum)>;
pub type PFNGLENABLEPROC = Option<unsafe extern "system" fn(cap: GLenum)>;
pub type PFNGLFINISHPROC = Option<unsafe extern "system" fn()>;
pub type PFNGLFLUSHPROC = Option<unsafe extern "system" fn()>;
pub type PFNGLBLENDFUNCPROC = Option<unsafe extern "system" fn(sfactor: GLenum, dfactor: GLenum)>;
pub type PFNGLLOGICOPPROC = Option<unsafe extern "system" fn(opcode: GLenum)>;
pub type PFNGLSTENCILFUNCPROC = Option<unsafe extern "system" fn(func: GLenum, ref_: GLint, mask: GLuint)>;
pub type PFNGLSTENCILOPPROC = Option<unsafe extern "system" fn(fail: GLenum, zfail: GLenum, zpass: GLenum)>;
pub type PFNGLDEPTHFUNCPROC = Option<unsafe extern "system" fn(func: GLenum)>;
pub type PFNGLPIXELSTOREFPROC = Option<unsafe extern "system" fn(pname: GLenum, param: GLfloat)>;
pub type PFNGLPIXELSTOREIPROC = Option<unsafe extern "system" fn(pname: GLenum, param: GLint)>;
pub type PFNGLREADBUFFERPROC = Option<unsafe extern "system" fn(src: GLenum)>;
pub type PFNGLREADPIXELSPROC = Option<unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *mut GLvoid)>;
pub type PFNGLGETBABORLEANVPROC = Option<unsafe extern "system" fn(pname: GLenum, data: *mut GLboolean)>;
pub type PFNGLGETDOUBLEVPROC = Option<unsafe extern "system" fn(pname: GLenum, data: *mut GLdouble)>;
pub type PFNGLGETERRORPROC = Option<unsafe extern "system" fn() -> GLenum>;
pub type PFNGLGETFLOATVPROC = Option<unsafe extern "system" fn(pname: GLenum, data: *mut GLfloat)>;
pub type PFNGLGETINTEGERVPROC = Option<unsafe extern "system" fn(pname: GLenum, data: *mut GLint)>;
pub type PFNGLGETSTRINGPROC = Option<unsafe extern "system" fn(name: GLenum) -> *const GLubyte>;
pub type PFNGLGETTEXIMAGEPROC = Option<unsafe extern "system" fn(target: GLenum, level: GLint, format: GLenum, type_: GLenum, pixels: *mut GLvoid)>;
pub type PFNGLGETTEXPARAMETERFVPROC = Option<unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLfloat)>;
pub type PFNGLGETTEXPARAMETERIVPROC = Option<unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint)>;
pub type PFNGLGETTEXLEVELPARAMETERFVPROC = Option<unsafe extern "system" fn(target: GLenum, level: GLint, pname: GLenum, params: *mut GLfloat)>;
pub type PFNGLGETTEXLEVELPARAMETERIVPROC = Option<unsafe extern "system" fn(target: GLenum, level: GLint, pname: GLenum, params: *mut GLint)>;
pub type PFNGLISENABLEDPROC = Option<unsafe extern "system" fn(cap: GLenum) -> GLboolean>;
pub type PFNGLDEPTHRANGEPROC = Option<unsafe extern "system" fn(n: GLdouble, f: GLdouble)>;
pub type PFNGLVIEWPORTPROC = Option<unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei)>;

/// OpenGL 1.0 function table
#[derive(Default)]
pub struct GL10 {
    pub glCullFace: PFNGLCULLFACEPROC,
    pub glFrontFace: PFNGLFABORDFACEPROC,
    pub glHint: PFNGLHINTPROC,
    pub glLineWidth: PFNGLLINEWIDTHPROC,
    pub glPointSize: PFNGLPOINTSIZEPROC,
    pub glPolygonMode: PFNGLPOLYGONMODEPROC,
    pub glScissor: PFNGLSCISSORPROC,
    pub glTexParameterf: PFNGLTEXPARAMETERFPROC,
    pub glTexParameterfv: PFNGLTEXPARAMETERFVPROC,
    pub glTexParameteri: PFNGLTEXPARAMETERIPROC,
    pub glTexParameteriv: PFNGLTEXPARAMETERIVPROC,
    pub glTexImage1D: PFNGLTEXIMAGE1DPROC,
    pub glTexImage2D: PFNGLTEXIMAGE2DPROC,
    pub glDrawBuffer: PFNGLDRAWBUFFERPROC,
    pub glClear: PFNGLCLEARPROC,
    pub glClearColor: PFNGLCLEARCOLORPROC,
    pub glClearStencil: PFNGLCLEARSTENCILPROC,
    pub glClearDepth: PFNGLCLEARDEPTHPROC,
    pub glStencilMask: PFNGLSTENCILMASKPROC,
    pub glColorMask: PFNGLCOLORMASKPROC,
    pub glDepthMask: PFNGLDEPTHMASKPROC,
    pub glDisable: PFNGLDISABLEPROC,
    pub glEnable: PFNGLENABLEPROC,
    pub glFinish: PFNGLFINISHPROC,
    pub glFlush: PFNGLFLUSHPROC,
    pub glBlendFunc: PFNGLBLENDFUNCPROC,
    pub glLogicOp: PFNGLLOGICOPPROC,
    pub glStencilFunc: PFNGLSTENCILFUNCPROC,
    pub glStencilOp: PFNGLSTENCILOPPROC,
    pub glDepthFunc: PFNGLDEPTHFUNCPROC,
    pub glPixelStoref: PFNGLPIXELSTOREFPROC,
    pub glPixelStorei: PFNGLPIXELSTOREIPROC,
    pub glReadBuffer: PFNGLREADBUFFERPROC,
    pub glReadPixels: PFNGLREADPIXELSPROC,
    pub glGetBooleanv: PFNGLGETBABORLEANVPROC,
    pub glGetDoublev: PFNGLGETDOUBLEVPROC,
    pub glGetError: PFNGLGETERRORPROC,
    pub glGetFloatv: PFNGLGETFLOATVPROC,
    pub glGetIntegerv: PFNGLGETINTEGERVPROC,
    pub glGetString: PFNGLGETSTRINGPROC,
    pub glGetTexImage: PFNGLGETTEXIMAGEPROC,
    pub glGetTexParameterfv: PFNGLGETTEXPARAMETERFVPROC,
    pub glGetTexParameteriv: PFNGLGETTEXPARAMETERIVPROC,
    pub glGetTexLevelParameterfv: PFNGLGETTEXLEVELPARAMETERFVPROC,
    pub glGetTexLevelParameteriv: PFNGLGETTEXLEVELPARAMETERIVPROC,
    pub glIsEnabled: PFNGLISENABLEDPROC,
    pub glDepthRange: PFNGLDEPTHRANGEPROC,
    pub glViewport: PFNGLVIEWPORTPROC,
}
