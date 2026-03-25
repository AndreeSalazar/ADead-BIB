//! OpenGL 2.0 Functions — Shaders, GLSL, vertex attribs, draw buffers
//! Based on Khronos canonical specifications
use super::types::*;

// Blend equation
pub type PFNGLBLENDEQUATIONSEPARATEPROC = Option<unsafe extern "system" fn(modeRGB: GLenum, modeAlpha: GLenum)>;

// Draw buffers
pub type PFNGLDRAWBUFFERSPROC = Option<unsafe extern "system" fn(n: GLsizei, bufs: *const GLenum)>;

// Stencil separate
pub type PFNGLSTENCILOPSEPARATEPROC = Option<unsafe extern "system" fn(face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum)>;
pub type PFNGLSTENCILFUNCSEPARATEPROC = Option<unsafe extern "system" fn(face: GLenum, func: GLenum, ref_: GLint, mask: GLuint)>;
pub type PFNGLSTENCILMASKSEPARATEPROC = Option<unsafe extern "system" fn(face: GLenum, mask: GLuint)>;

// Shader objects
pub type PFNGLATTACHSHADERPROC = Option<unsafe extern "system" fn(program: GLuint, shader: GLuint)>;
pub type PFNGLBINDATTRIBLOCATIONPROC = Option<unsafe extern "system" fn(program: GLuint, index: GLuint, name: *const GLchar)>;
pub type PFNGLCOMPILESHADERPROC = Option<unsafe extern "system" fn(shader: GLuint)>;
pub type PFNGLCREATEPROGRAMPROC = Option<unsafe extern "system" fn() -> GLuint>;
pub type PFNGLCREATESHADERPROC = Option<unsafe extern "system" fn(type_: GLenum) -> GLuint>;
pub type PFNGLDELETEPROGRAMPROC = Option<unsafe extern "system" fn(program: GLuint)>;
pub type PFNGLDELETESHADERPROC = Option<unsafe extern "system" fn(shader: GLuint)>;
pub type PFNGLDETACHSHADERPROC = Option<unsafe extern "system" fn(program: GLuint, shader: GLuint)>;
pub type PFNGLDISABLEVERTEXATTRIBARRAYPROC = Option<unsafe extern "system" fn(index: GLuint)>;
pub type PFNGLENABLEVERTEXATTRIBARRAYPROC = Option<unsafe extern "system" fn(index: GLuint)>;
pub type PFNGLGETACTIVEATTRIBPROC = Option<unsafe extern "system" fn(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar)>;
pub type PFNGLGETACTIVEUNIFORMPROC = Option<unsafe extern "system" fn(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar)>;
pub type PFNGLGETATTACHEDSHADERSPROC = Option<unsafe extern "system" fn(program: GLuint, maxCount: GLsizei, count: *mut GLsizei, shaders: *mut GLuint)>;
pub type PFNGLGETATTRIBLOCATIONPROC = Option<unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint>;
pub type PFNGLGETPROGRAMIVPROC = Option<unsafe extern "system" fn(program: GLuint, pname: GLenum, params: *mut GLint)>;
pub type PFNGLGETPROGRAMINFOLOGPROC = Option<unsafe extern "system" fn(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar)>;
pub type PFNGLGETSHADERIVPROC = Option<unsafe extern "system" fn(shader: GLuint, pname: GLenum, params: *mut GLint)>;
pub type PFNGLGETSHADERINFOLOGPROC = Option<unsafe extern "system" fn(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar)>;
pub type PFNGLGETSHADERSOURCEPROC = Option<unsafe extern "system" fn(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar)>;
pub type PFNGLGETUNIFORMLOCATIONPROC = Option<unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint>;
pub type PFNGLGETUNIFORMFVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLfloat)>;
pub type PFNGLGETUNIFORMIVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLint)>;
pub type PFNGLGETVERTEXATTRIBDVPROC = Option<unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLdouble)>;
pub type PFNGLGETVERTEXATTRIBFVPROC = Option<unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLfloat)>;
pub type PFNGLGETVERTEXATTRIBIVPROC = Option<unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLint)>;
pub type PFNGLGETVERTEXATTRIBPOINTERVPROC = Option<unsafe extern "system" fn(index: GLuint, pname: GLenum, pointer: *mut *mut GLvoid)>;
pub type PFNGLISPROGRAMPROC = Option<unsafe extern "system" fn(program: GLuint) -> GLboolean>;
pub type PFNGLISSHADERPROC = Option<unsafe extern "system" fn(shader: GLuint) -> GLboolean>;
pub type PFNGLLINKPROGRAMPROC = Option<unsafe extern "system" fn(program: GLuint)>;
pub type PFNGLSHADERSOURCEPROC = Option<unsafe extern "system" fn(shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint)>;
pub type PFNGLUSEPROGRAMPROC = Option<unsafe extern "system" fn(program: GLuint)>;
pub type PFNGLUNIFORM1FPROC = Option<unsafe extern "system" fn(location: GLint, v0: GLfloat)>;
pub type PFNGLUNIFORM2FPROC = Option<unsafe extern "system" fn(location: GLint, v0: GLfloat, v1: GLfloat)>;
pub type PFNGLUNIFORM3FPROC = Option<unsafe extern "system" fn(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat)>;
pub type PFNGLUNIFORM4FPROC = Option<unsafe extern "system" fn(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat)>;
pub type PFNGLUNIFORM1IPROC = Option<unsafe extern "system" fn(location: GLint, v0: GLint)>;
pub type PFNGLUNIFORM2IPROC = Option<unsafe extern "system" fn(location: GLint, v0: GLint, v1: GLint)>;
pub type PFNGLUNIFORM3IPROC = Option<unsafe extern "system" fn(location: GLint, v0: GLint, v1: GLint, v2: GLint)>;
pub type PFNGLUNIFORM4IPROC = Option<unsafe extern "system" fn(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint)>;
pub type PFNGLUNIFORM1FVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat)>;
pub type PFNGLUNIFORM2FVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat)>;
pub type PFNGLUNIFORM3FVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat)>;
pub type PFNGLUNIFORM4FVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat)>;
pub type PFNGLUNIFORM1IVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint)>;
pub type PFNGLUNIFORM2IVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint)>;
pub type PFNGLUNIFORM3IVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint)>;
pub type PFNGLUNIFORM4IVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint)>;
pub type PFNGLUNIFORMMATRIX2FVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat)>;
pub type PFNGLUNIFORMMATRIX3FVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat)>;
pub type PFNGLUNIFORMMATRIX4FVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat)>;
pub type PFNGLVALIDATEPROGRAMPROC = Option<unsafe extern "system" fn(program: GLuint)>;
pub type PFNGLVERTEXATTRIB1DPROC = Option<unsafe extern "system" fn(index: GLuint, x: GLdouble)>;
pub type PFNGLVERTEXATTRIB1DVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLdouble)>;
pub type PFNGLVERTEXATTRIB1FPROC = Option<unsafe extern "system" fn(index: GLuint, x: GLfloat)>;
pub type PFNGLVERTEXATTRIB1FVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLfloat)>;
pub type PFNGLVERTEXATTRIB1SPROC = Option<unsafe extern "system" fn(index: GLuint, x: GLshort)>;
pub type PFNGLVERTEXATTRIB1SVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLshort)>;
pub type PFNGLVERTEXATTRIB2DPROC = Option<unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble)>;
pub type PFNGLVERTEXATTRIB2DVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLdouble)>;
pub type PFNGLVERTEXATTRIB2FPROC = Option<unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat)>;
pub type PFNGLVERTEXATTRIB2FVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLfloat)>;
pub type PFNGLVERTEXATTRIB2SPROC = Option<unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort)>;
pub type PFNGLVERTEXATTRIB2SVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLshort)>;
pub type PFNGLVERTEXATTRIB3DPROC = Option<unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble)>;
pub type PFNGLVERTEXATTRIB3DVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLdouble)>;
pub type PFNGLVERTEXATTRIB3FPROC = Option<unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat)>;
pub type PFNGLVERTEXATTRIB3FVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLfloat)>;
pub type PFNGLVERTEXATTRIB3SPROC = Option<unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort, z: GLshort)>;
pub type PFNGLVERTEXATTRIB3SVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLshort)>;
pub type PFNGLVERTEXATTRIB4NBVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLbyte)>;
pub type PFNGLVERTEXATTRIB4NIVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLint)>;
pub type PFNGLVERTEXATTRIB4NSVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLshort)>;
pub type PFNGLVERTEXATTRIB4NUBPROC = Option<unsafe extern "system" fn(index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte)>;
pub type PFNGLVERTEXATTRIB4NUBVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLubyte)>;
pub type PFNGLVERTEXATTRIB4NUIVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLuint)>;
pub type PFNGLVERTEXATTRIB4NUSVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLushort)>;
pub type PFNGLVERTEXATTRIB4BVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLbyte)>;
pub type PFNGLVERTEXATTRIB4DPROC = Option<unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble)>;
pub type PFNGLVERTEXATTRIB4DVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLdouble)>;
pub type PFNGLVERTEXATTRIB4FPROC = Option<unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat)>;
pub type PFNGLVERTEXATTRIB4FVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLfloat)>;
pub type PFNGLVERTEXATTRIB4IVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLint)>;
pub type PFNGLVERTEXATTRIB4SPROC = Option<unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort)>;
pub type PFNGLVERTEXATTRIB4SVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLshort)>;
pub type PFNGLVERTEXATTRIB4UBVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLubyte)>;
pub type PFNGLVERTEXATTRIB4UIVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLuint)>;
pub type PFNGLVERTEXATTRIB4USVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLushort)>;
pub type PFNGLVERTEXATTRIBPOINTERPROC = Option<unsafe extern "system" fn(index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const GLvoid)>;

/// OpenGL 2.0 function table
#[derive(Default)]
pub struct GL20 {
    pub glBlendEquationSeparate: PFNGLBLENDEQUATIONSEPARATEPROC,
    pub glDrawBuffers: PFNGLDRAWBUFFERSPROC,
    pub glStencilOpSeparate: PFNGLSTENCILOPSEPARATEPROC,
    pub glStencilFuncSeparate: PFNGLSTENCILFUNCSEPARATEPROC,
    pub glStencilMaskSeparate: PFNGLSTENCILMASKSEPARATEPROC,
    pub glAttachShader: PFNGLATTACHSHADERPROC,
    pub glBindAttribLocation: PFNGLBINDATTRIBLOCATIONPROC,
    pub glCompileShader: PFNGLCOMPILESHADERPROC,
    pub glCreateProgram: PFNGLCREATEPROGRAMPROC,
    pub glCreateShader: PFNGLCREATESHADERPROC,
    pub glDeleteProgram: PFNGLDELETEPROGRAMPROC,
    pub glDeleteShader: PFNGLDELETESHADERPROC,
    pub glDetachShader: PFNGLDETACHSHADERPROC,
    pub glDisableVertexAttribArray: PFNGLDISABLEVERTEXATTRIBARRAYPROC,
    pub glEnableVertexAttribArray: PFNGLENABLEVERTEXATTRIBARRAYPROC,
    pub glGetActiveAttrib: PFNGLGETACTIVEATTRIBPROC,
    pub glGetActiveUniform: PFNGLGETACTIVEUNIFORMPROC,
    pub glGetAttachedShaders: PFNGLGETATTACHEDSHADERSPROC,
    pub glGetAttribLocation: PFNGLGETATTRIBLOCATIONPROC,
    pub glGetProgramiv: PFNGLGETPROGRAMIVPROC,
    pub glGetProgramInfoLog: PFNGLGETPROGRAMINFOLOGPROC,
    pub glGetShaderiv: PFNGLGETSHADERIVPROC,
    pub glGetShaderInfoLog: PFNGLGETSHADERINFOLOGPROC,
    pub glGetShaderSource: PFNGLGETSHADERSOURCEPROC,
    pub glGetUniformLocation: PFNGLGETUNIFORMLOCATIONPROC,
    pub glGetUniformfv: PFNGLGETUNIFORMFVPROC,
    pub glGetUniformiv: PFNGLGETUNIFORMIVPROC,
    pub glGetVertexAttribdv: PFNGLGETVERTEXATTRIBDVPROC,
    pub glGetVertexAttribfv: PFNGLGETVERTEXATTRIBFVPROC,
    pub glGetVertexAttribiv: PFNGLGETVERTEXATTRIBIVPROC,
    pub glGetVertexAttribPointerv: PFNGLGETVERTEXATTRIBPOINTERVPROC,
    pub glIsProgram: PFNGLISPROGRAMPROC,
    pub glIsShader: PFNGLISSHADERPROC,
    pub glLinkProgram: PFNGLLINKPROGRAMPROC,
    pub glShaderSource: PFNGLSHADERSOURCEPROC,
    pub glUseProgram: PFNGLUSEPROGRAMPROC,
    pub glUniform1f: PFNGLUNIFORM1FPROC,
    pub glUniform2f: PFNGLUNIFORM2FPROC,
    pub glUniform3f: PFNGLUNIFORM3FPROC,
    pub glUniform4f: PFNGLUNIFORM4FPROC,
    pub glUniform1i: PFNGLUNIFORM1IPROC,
    pub glUniform2i: PFNGLUNIFORM2IPROC,
    pub glUniform3i: PFNGLUNIFORM3IPROC,
    pub glUniform4i: PFNGLUNIFORM4IPROC,
    pub glUniform1fv: PFNGLUNIFORM1FVPROC,
    pub glUniform2fv: PFNGLUNIFORM2FVPROC,
    pub glUniform3fv: PFNGLUNIFORM3FVPROC,
    pub glUniform4fv: PFNGLUNIFORM4FVPROC,
    pub glUniform1iv: PFNGLUNIFORM1IVPROC,
    pub glUniform2iv: PFNGLUNIFORM2IVPROC,
    pub glUniform3iv: PFNGLUNIFORM3IVPROC,
    pub glUniform4iv: PFNGLUNIFORM4IVPROC,
    pub glUniformMatrix2fv: PFNGLUNIFORMMATRIX2FVPROC,
    pub glUniformMatrix3fv: PFNGLUNIFORMMATRIX3FVPROC,
    pub glUniformMatrix4fv: PFNGLUNIFORMMATRIX4FVPROC,
    pub glValidateProgram: PFNGLVALIDATEPROGRAMPROC,
    pub glVertexAttrib1d: PFNGLVERTEXATTRIB1DPROC,
    pub glVertexAttrib1dv: PFNGLVERTEXATTRIB1DVPROC,
    pub glVertexAttrib1f: PFNGLVERTEXATTRIB1FPROC,
    pub glVertexAttrib1fv: PFNGLVERTEXATTRIB1FVPROC,
    pub glVertexAttrib1s: PFNGLVERTEXATTRIB1SPROC,
    pub glVertexAttrib1sv: PFNGLVERTEXATTRIB1SVPROC,
    pub glVertexAttrib2d: PFNGLVERTEXATTRIB2DPROC,
    pub glVertexAttrib2dv: PFNGLVERTEXATTRIB2DVPROC,
    pub glVertexAttrib2f: PFNGLVERTEXATTRIB2FPROC,
    pub glVertexAttrib2fv: PFNGLVERTEXATTRIB2FVPROC,
    pub glVertexAttrib2s: PFNGLVERTEXATTRIB2SPROC,
    pub glVertexAttrib2sv: PFNGLVERTEXATTRIB2SVPROC,
    pub glVertexAttrib3d: PFNGLVERTEXATTRIB3DPROC,
    pub glVertexAttrib3dv: PFNGLVERTEXATTRIB3DVPROC,
    pub glVertexAttrib3f: PFNGLVERTEXATTRIB3FPROC,
    pub glVertexAttrib3fv: PFNGLVERTEXATTRIB3FVPROC,
    pub glVertexAttrib3s: PFNGLVERTEXATTRIB3SPROC,
    pub glVertexAttrib3sv: PFNGLVERTEXATTRIB3SVPROC,
    pub glVertexAttrib4Nbv: PFNGLVERTEXATTRIB4NBVPROC,
    pub glVertexAttrib4Niv: PFNGLVERTEXATTRIB4NIVPROC,
    pub glVertexAttrib4Nsv: PFNGLVERTEXATTRIB4NSVPROC,
    pub glVertexAttrib4Nub: PFNGLVERTEXATTRIB4NUBPROC,
    pub glVertexAttrib4Nubv: PFNGLVERTEXATTRIB4NUBVPROC,
    pub glVertexAttrib4Nuiv: PFNGLVERTEXATTRIB4NUIVPROC,
    pub glVertexAttrib4Nusv: PFNGLVERTEXATTRIB4NUSVPROC,
    pub glVertexAttrib4bv: PFNGLVERTEXATTRIB4BVPROC,
    pub glVertexAttrib4d: PFNGLVERTEXATTRIB4DPROC,
    pub glVertexAttrib4dv: PFNGLVERTEXATTRIB4DVPROC,
    pub glVertexAttrib4f: PFNGLVERTEXATTRIB4FPROC,
    pub glVertexAttrib4fv: PFNGLVERTEXATTRIB4FVPROC,
    pub glVertexAttrib4iv: PFNGLVERTEXATTRIB4IVPROC,
    pub glVertexAttrib4s: PFNGLVERTEXATTRIB4SPROC,
    pub glVertexAttrib4sv: PFNGLVERTEXATTRIB4SVPROC,
    pub glVertexAttrib4ubv: PFNGLVERTEXATTRIB4UBVPROC,
    pub glVertexAttrib4uiv: PFNGLVERTEXATTRIB4UIVPROC,
    pub glVertexAttrib4usv: PFNGLVERTEXATTRIB4USVPROC,
    pub glVertexAttribPointer: PFNGLVERTEXATTRIBPOINTERPROC,
}
