//! OpenGL 4.0 Functions — Tessellation, subroutines, indirect draw, double uniforms
//! Based on Khronos canonical specifications
use super::types::*;

// Min sample shading
pub type PFNGLMINSAMPLESHADINGPROC = Option<unsafe extern "system" fn(value: GLfloat)>;

// Blend func indexed
pub type PFNGLBLENDEQUATIONIPROC = Option<unsafe extern "system" fn(buf: GLuint, mode: GLenum)>;
pub type PFNGLBLENDEQUATIONSEPARATEIPROC = Option<unsafe extern "system" fn(buf: GLuint, modeRGB: GLenum, modeAlpha: GLenum)>;
pub type PFNGLBLENDFUNCIPROC = Option<unsafe extern "system" fn(buf: GLuint, src: GLenum, dst: GLenum)>;
pub type PFNGLBLENDFUNCSEPARATEIPROC = Option<unsafe extern "system" fn(buf: GLuint, srcRGB: GLenum, dstRGB: GLenum, srcAlpha: GLenum, dstAlpha: GLenum)>;

// Draw indirect
pub type PFNGLDRAWARRAYSINDIRECTPROC = Option<unsafe extern "system" fn(mode: GLenum, indirect: *const GLvoid)>;
pub type PFNGLDRAWELEMENTSINDIRECTPROC = Option<unsafe extern "system" fn(mode: GLenum, type_: GLenum, indirect: *const GLvoid)>;

// Double uniforms
pub type PFNGLUNIFORM1DPROC = Option<unsafe extern "system" fn(location: GLint, x: GLdouble)>;
pub type PFNGLUNIFORM2DPROC = Option<unsafe extern "system" fn(location: GLint, x: GLdouble, y: GLdouble)>;
pub type PFNGLUNIFORM3DPROC = Option<unsafe extern "system" fn(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble)>;
pub type PFNGLUNIFORM4DPROC = Option<unsafe extern "system" fn(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble)>;
pub type PFNGLUNIFORM1DVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLdouble)>;
pub type PFNGLUNIFORM2DVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLdouble)>;
pub type PFNGLUNIFORM3DVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLdouble)>;
pub type PFNGLUNIFORM4DVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLdouble)>;
pub type PFNGLUNIFORMMATRIX2DVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble)>;
pub type PFNGLUNIFORMMATRIX3DVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble)>;
pub type PFNGLUNIFORMMATRIX4DVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble)>;
pub type PFNGLUNIFORMMATRIX2X3DVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble)>;
pub type PFNGLUNIFORMMATRIX2X4DVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble)>;
pub type PFNGLUNIFORMMATRIX3X2DVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble)>;
pub type PFNGLUNIFORMMATRIX3X4DVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble)>;
pub type PFNGLUNIFORMMATRIX4X2DVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble)>;
pub type PFNGLUNIFORMMATRIX4X3DVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble)>;
pub type PFNGLGETUNIFORMDVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLdouble)>;

// Subroutines
pub type PFNGLGETSUBROUTINEUNIFORMLOCATIONPROC = Option<unsafe extern "system" fn(program: GLuint, shadertype: GLenum, name: *const GLchar) -> GLint>;
pub type PFNGLGETSUBROUTINEINDEXPROC = Option<unsafe extern "system" fn(program: GLuint, shadertype: GLenum, name: *const GLchar) -> GLuint>;
pub type PFNGLGETACTIVESUBROUTINEUNIFORMIVPROC = Option<unsafe extern "system" fn(program: GLuint, shadertype: GLenum, index: GLuint, pname: GLenum, values: *mut GLint)>;
pub type PFNGLGETACTIVESUBROUTINEUNIFORMNAMEPROC = Option<unsafe extern "system" fn(program: GLuint, shadertype: GLenum, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar)>;
pub type PFNGLGETACTIVESUBROUTINENAMEPROC = Option<unsafe extern "system" fn(program: GLuint, shadertype: GLenum, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar)>;
pub type PFNGLUNIFORMSUBROUTINESUIVPROC = Option<unsafe extern "system" fn(shadertype: GLenum, count: GLsizei, indices: *const GLuint)>;
pub type PFNGLGETUNIFORMSUBROUTINEUIVPROC = Option<unsafe extern "system" fn(shadertype: GLenum, location: GLint, params: *mut GLuint)>;
pub type PFNGLGETPROGRAMSTAGEIVPROC = Option<unsafe extern "system" fn(program: GLuint, shadertype: GLenum, pname: GLenum, values: *mut GLint)>;

// Tessellation patch
pub type PFNGLPATCHPARAMETERIPROC = Option<unsafe extern "system" fn(pname: GLenum, value: GLint)>;
pub type PFNGLPATCHPARAMETERFVPROC = Option<unsafe extern "system" fn(pname: GLenum, values: *const GLfloat)>;

// Transform feedback objects
pub type PFNGLBINDTRANSFORMFEEDBACKPROC = Option<unsafe extern "system" fn(target: GLenum, id: GLuint)>;
pub type PFNGLDELETETRANSFORMFEEDBACKSPROC = Option<unsafe extern "system" fn(n: GLsizei, ids: *const GLuint)>;
pub type PFNGLGENTRANSFORMFEEDBACKSPROC = Option<unsafe extern "system" fn(n: GLsizei, ids: *mut GLuint)>;
pub type PFNGLISTRANSFORMFEEDBACKPROC = Option<unsafe extern "system" fn(id: GLuint) -> GLboolean>;
pub type PFNGLPAUSETRANSFORMFEEDBACKPROC = Option<unsafe extern "system" fn()>;
pub type PFNGLRESUMETRANSFORMFEEDBACKPROC = Option<unsafe extern "system" fn()>;

// Query indexed
pub type PFNGLBEGINQUERYINDEXEDPROC = Option<unsafe extern "system" fn(target: GLenum, index: GLuint, id: GLuint)>;
pub type PFNGLENDQUERYINDEXEDPROC = Option<unsafe extern "system" fn(target: GLenum, index: GLuint)>;
pub type PFNGLGETQUERYINDEXEDIVPROC = Option<unsafe extern "system" fn(target: GLenum, index: GLuint, pname: GLenum, params: *mut GLint)>;

/// OpenGL 4.0 function table
#[derive(Default)]
pub struct GL40 {
    pub glMinSampleShading: PFNGLMINSAMPLESHADINGPROC,
    pub glBlendEquationi: PFNGLBLENDEQUATIONIPROC,
    pub glBlendEquationSeparatei: PFNGLBLENDEQUATIONSEPARATEIPROC,
    pub glBlendFunci: PFNGLBLENDFUNCIPROC,
    pub glBlendFuncSeparatei: PFNGLBLENDFUNCSEPARATEIPROC,
    pub glDrawArraysIndirect: PFNGLDRAWARRAYSINDIRECTPROC,
    pub glDrawElementsIndirect: PFNGLDRAWELEMENTSINDIRECTPROC,
    pub glUniform1d: PFNGLUNIFORM1DPROC,
    pub glUniform2d: PFNGLUNIFORM2DPROC,
    pub glUniform3d: PFNGLUNIFORM3DPROC,
    pub glUniform4d: PFNGLUNIFORM4DPROC,
    pub glUniform1dv: PFNGLUNIFORM1DVPROC,
    pub glUniform2dv: PFNGLUNIFORM2DVPROC,
    pub glUniform3dv: PFNGLUNIFORM3DVPROC,
    pub glUniform4dv: PFNGLUNIFORM4DVPROC,
    pub glUniformMatrix2dv: PFNGLUNIFORMMATRIX2DVPROC,
    pub glUniformMatrix3dv: PFNGLUNIFORMMATRIX3DVPROC,
    pub glUniformMatrix4dv: PFNGLUNIFORMMATRIX4DVPROC,
    pub glUniformMatrix2x3dv: PFNGLUNIFORMMATRIX2X3DVPROC,
    pub glUniformMatrix2x4dv: PFNGLUNIFORMMATRIX2X4DVPROC,
    pub glUniformMatrix3x2dv: PFNGLUNIFORMMATRIX3X2DVPROC,
    pub glUniformMatrix3x4dv: PFNGLUNIFORMMATRIX3X4DVPROC,
    pub glUniformMatrix4x2dv: PFNGLUNIFORMMATRIX4X2DVPROC,
    pub glUniformMatrix4x3dv: PFNGLUNIFORMMATRIX4X3DVPROC,
    pub glGetUniformdv: PFNGLGETUNIFORMDVPROC,
    pub glGetSubroutineUniformLocation: PFNGLGETSUBROUTINEUNIFORMLOCATIONPROC,
    pub glGetSubroutineIndex: PFNGLGETSUBROUTINEINDEXPROC,
    pub glGetActiveSubroutineUniformiv: PFNGLGETACTIVESUBROUTINEUNIFORMIVPROC,
    pub glGetActiveSubroutineUniformName: PFNGLGETACTIVESUBROUTINEUNIFORMNAMEPROC,
    pub glGetActiveSubroutineName: PFNGLGETACTIVESUBROUTINENAMEPROC,
    pub glUniformSubroutinesuiv: PFNGLUNIFORMSUBROUTINESUIVPROC,
    pub glGetUniformSubroutineuiv: PFNGLGETUNIFORMSUBROUTINEUIVPROC,
    pub glGetProgramStageiv: PFNGLGETPROGRAMSTAGEIVPROC,
    pub glPatchParameteri: PFNGLPATCHPARAMETERIPROC,
    pub glPatchParameterfv: PFNGLPATCHPARAMETERFVPROC,
    pub glBindTransformFeedback: PFNGLBINDTRANSFORMFEEDBACKPROC,
    pub glDeleteTransformFeedbacks: PFNGLDELETETRANSFORMFEEDBACKSPROC,
    pub glGenTransformFeedbacks: PFNGLGENTRANSFORMFEEDBACKSPROC,
    pub glIsTransformFeedback: PFNGLISTRANSFORMFEEDBACKPROC,
    pub glPauseTransformFeedback: PFNGLPAUSETRANSFORMFEEDBACKPROC,
    pub glResumeTransformFeedback: PFNGLRESUMETRANSFORMFEEDBACKPROC,
    pub glBeginQueryIndexed: PFNGLBEGINQUERYINDEXEDPROC,
    pub glEndQueryIndexed: PFNGLENDQUERYINDEXEDPROC,
    pub glGetQueryIndexediv: PFNGLGETQUERYINDEXEDIVPROC,
}
