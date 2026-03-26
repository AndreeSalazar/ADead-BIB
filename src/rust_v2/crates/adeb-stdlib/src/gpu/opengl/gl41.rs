//! OpenGL 4.1 Functions — Separate shader objects, viewport arrays, program binary
//! Based on Khronos canonical specifications
use super::types::*;

// Release shader compiler
pub type PFNGLRELEASESHADERCOMPILERPROC = Option<unsafe extern "system" fn()>;
pub type PFNGLSHADERBINARYPROC = Option<unsafe extern "system" fn(count: GLsizei, shaders: *const GLuint, binaryFormat: GLenum, binary: *const GLvoid, length: GLsizei)>;
pub type PFNGLGETSHADERPRECISIONFORMATPROC = Option<unsafe extern "system" fn(shadertype: GLenum, precisiontype: GLenum, range: *mut GLint, precision: *mut GLint)>;
pub type PFNGLDEPTHRANGEFPROC = Option<unsafe extern "system" fn(n: GLfloat, f: GLfloat)>;
pub type PFNGLCLEARDEPTHFPROC = Option<unsafe extern "system" fn(d: GLfloat)>;

// Program binary
pub type PFNGLGETPROGRAMBINARYPROC = Option<unsafe extern "system" fn(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, binaryFormat: *mut GLenum, binary: *mut GLvoid)>;
pub type PFNGLPROGRAMBINARYPROC = Option<unsafe extern "system" fn(program: GLuint, binaryFormat: GLenum, binary: *const GLvoid, length: GLsizei)>;
pub type PFNGLPROGRAMPARAMETERIPROC = Option<unsafe extern "system" fn(program: GLuint, pname: GLenum, value: GLint)>;

// Separate shader objects
pub type PFNGLUSEPROGRAMSTAGESPROC = Option<unsafe extern "system" fn(pipeline: GLuint, stages: GLbitfield, program: GLuint)>;
pub type PFNGLACTIVESHADERPROGRAMPROC = Option<unsafe extern "system" fn(pipeline: GLuint, program: GLuint)>;
pub type PFNGLCREATESHADERPROGRAMVPROC = Option<unsafe extern "system" fn(type_: GLenum, count: GLsizei, strings: *const *const GLchar) -> GLuint>;
pub type PFNGLBINDPROGRAMPIPELINEPROC = Option<unsafe extern "system" fn(pipeline: GLuint)>;
pub type PFNGLDELETEPROGRAMPIPELINESPROC = Option<unsafe extern "system" fn(n: GLsizei, pipelines: *const GLuint)>;
pub type PFNGLGENPROGRAMPIPELINESPROC = Option<unsafe extern "system" fn(n: GLsizei, pipelines: *mut GLuint)>;
pub type PFNGLISPROGRAMPIPELINEPROC = Option<unsafe extern "system" fn(pipeline: GLuint) -> GLboolean>;
pub type PFNGLGETPROGRAMPIPELINEIVPROC = Option<unsafe extern "system" fn(pipeline: GLuint, pname: GLenum, params: *mut GLint)>;
pub type PFNGLVALIDATEPROGRAMPIPELINEPROC = Option<unsafe extern "system" fn(pipeline: GLuint)>;
pub type PFNGLGETPROGRAMPIPELINEINFOLOGPROC = Option<unsafe extern "system" fn(pipeline: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar)>;

// Program uniform (separate shader)
pub type PFNGLPROGRAMUNIFORM1IPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLint)>;
pub type PFNGLPROGRAMUNIFORM1IVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint)>;
pub type PFNGLPROGRAMUNIFORM1FPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLfloat)>;
pub type PFNGLPROGRAMUNIFORM1FVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat)>;
pub type PFNGLPROGRAMUNIFORM1DPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLdouble)>;
pub type PFNGLPROGRAMUNIFORM1DVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble)>;
pub type PFNGLPROGRAMUNIFORM1UIPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLuint)>;
pub type PFNGLPROGRAMUNIFORM1UIVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint)>;
pub type PFNGLPROGRAMUNIFORM2IPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLint, v1: GLint)>;
pub type PFNGLPROGRAMUNIFORM2IVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint)>;
pub type PFNGLPROGRAMUNIFORM2FPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat)>;
pub type PFNGLPROGRAMUNIFORM2FVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat)>;
pub type PFNGLPROGRAMUNIFORM2DPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble)>;
pub type PFNGLPROGRAMUNIFORM2DVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble)>;
pub type PFNGLPROGRAMUNIFORM2UIPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLuint, v1: GLuint)>;
pub type PFNGLPROGRAMUNIFORM2UIVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint)>;
pub type PFNGLPROGRAMUNIFORM3IPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint)>;
pub type PFNGLPROGRAMUNIFORM3IVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint)>;
pub type PFNGLPROGRAMUNIFORM3FPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat)>;
pub type PFNGLPROGRAMUNIFORM3FVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat)>;
pub type PFNGLPROGRAMUNIFORM3DPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble)>;
pub type PFNGLPROGRAMUNIFORM3DVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble)>;
pub type PFNGLPROGRAMUNIFORM3UIPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint)>;
pub type PFNGLPROGRAMUNIFORM3UIVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint)>;
pub type PFNGLPROGRAMUNIFORM4IPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint)>;
pub type PFNGLPROGRAMUNIFORM4IVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint)>;
pub type PFNGLPROGRAMUNIFORM4FPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat)>;
pub type PFNGLPROGRAMUNIFORM4FVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat)>;
pub type PFNGLPROGRAMUNIFORM4DPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble, v3: GLdouble)>;
pub type PFNGLPROGRAMUNIFORM4DVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble)>;
pub type PFNGLPROGRAMUNIFORM4UIPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint)>;
pub type PFNGLPROGRAMUNIFORM4UIVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint)>;
pub type PFNGLPROGRAMUNIFORMMATRIX2FVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat)>;
pub type PFNGLPROGRAMUNIFORMMATRIX3FVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat)>;
pub type PFNGLPROGRAMUNIFORMMATRIX4FVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat)>;
pub type PFNGLPROGRAMUNIFORMMATRIX2X3FVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat)>;
pub type PFNGLPROGRAMUNIFORMMATRIX3X2FVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat)>;
pub type PFNGLPROGRAMUNIFORMMATRIX2X4FVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat)>;
pub type PFNGLPROGRAMUNIFORMMATRIX4X2FVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat)>;
pub type PFNGLPROGRAMUNIFORMMATRIX3X4FVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat)>;
pub type PFNGLPROGRAMUNIFORMMATRIX4X3FVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat)>;
pub type PFNGLPROGRAMUNIFORMMATRIX2DVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble)>;
pub type PFNGLPROGRAMUNIFORMMATRIX3DVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble)>;
pub type PFNGLPROGRAMUNIFORMMATRIX4DVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble)>;
pub type PFNGLPROGRAMUNIFORMMATRIX2X3DVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble)>;
pub type PFNGLPROGRAMUNIFORMMATRIX3X2DVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble)>;
pub type PFNGLPROGRAMUNIFORMMATRIX2X4DVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble)>;
pub type PFNGLPROGRAMUNIFORMMATRIX4X2DVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble)>;
pub type PFNGLPROGRAMUNIFORMMATRIX3X4DVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble)>;
pub type PFNGLPROGRAMUNIFORMMATRIX4X3DVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble)>;

// Viewport arrays
pub type PFNGLVIEWPORTARRAYVPROC = Option<unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLfloat)>;
pub type PFNGLVIEWPORTINDEXEDFPROC = Option<unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, w: GLfloat, h: GLfloat)>;
pub type PFNGLVIEWPORTINDEXEDFVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLfloat)>;
pub type PFNGLSCISSORARRAYVPROC = Option<unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLint)>;
pub type PFNGLSCISSORINDEXEDPROC = Option<unsafe extern "system" fn(index: GLuint, left: GLint, bottom: GLint, width: GLsizei, height: GLsizei)>;
pub type PFNGLSCISSORINDEXEDVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLint)>;
pub type PFNGLDEPTHRANGEARRAYVPROC = Option<unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLdouble)>;
pub type PFNGLDEPTHRANGEINDEXEDPROC = Option<unsafe extern "system" fn(index: GLuint, n: GLdouble, f: GLdouble)>;
pub type PFNGLGETFLOATI_VPROC = Option<unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLfloat)>;
pub type PFNGLGETDOUBLEI_VPROC = Option<unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLdouble)>;

// Vertex attrib L (double)
pub type PFNGLVERTEXATTRIBL1DPROC = Option<unsafe extern "system" fn(index: GLuint, x: GLdouble)>;
pub type PFNGLVERTEXATTRIBL2DPROC = Option<unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble)>;
pub type PFNGLVERTEXATTRIBL3DPROC = Option<unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble)>;
pub type PFNGLVERTEXATTRIBL4DPROC = Option<unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble)>;
pub type PFNGLVERTEXATTRIBL1DVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLdouble)>;
pub type PFNGLVERTEXATTRIBL2DVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLdouble)>;
pub type PFNGLVERTEXATTRIBL3DVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLdouble)>;
pub type PFNGLVERTEXATTRIBL4DVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLdouble)>;
pub type PFNGLVERTEXATTRIBLPOINTERPROC = Option<unsafe extern "system" fn(index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const GLvoid)>;
pub type PFNGLGETVERTEXATTRIBLDVPROC = Option<unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLdouble)>;

/// OpenGL 4.1 function table
#[derive(Default)]
pub struct GL41 {
    pub glReleaseShaderCompiler: PFNGLRELEASESHADERCOMPILERPROC,
    pub glShaderBinary: PFNGLSHADERBINARYPROC,
    pub glGetShaderPrecisionFormat: PFNGLGETSHADERPRECISIONFORMATPROC,
    pub glDepthRangef: PFNGLDEPTHRANGEFPROC,
    pub glClearDepthf: PFNGLCLEARDEPTHFPROC,
    pub glGetProgramBinary: PFNGLGETPROGRAMBINARYPROC,
    pub glProgramBinary: PFNGLPROGRAMBINARYPROC,
    pub glProgramParameteri: PFNGLPROGRAMPARAMETERIPROC,
    pub glUseProgramStages: PFNGLUSEPROGRAMSTAGESPROC,
    pub glActiveShaderProgram: PFNGLACTIVESHADERPROGRAMPROC,
    pub glCreateShaderProgramv: PFNGLCREATESHADERPROGRAMVPROC,
    pub glBindProgramPipeline: PFNGLBINDPROGRAMPIPELINEPROC,
    pub glDeleteProgramPipelines: PFNGLDELETEPROGRAMPIPELINESPROC,
    pub glGenProgramPipelines: PFNGLGENPROGRAMPIPELINESPROC,
    pub glIsProgramPipeline: PFNGLISPROGRAMPIPELINEPROC,
    pub glGetProgramPipelineiv: PFNGLGETPROGRAMPIPELINEIVPROC,
    pub glValidateProgramPipeline: PFNGLVALIDATEPROGRAMPIPELINEPROC,
    pub glGetProgramPipelineInfoLog: PFNGLGETPROGRAMPIPELINEINFOLOGPROC,
    pub glProgramUniform1i: PFNGLPROGRAMUNIFORM1IPROC,
    pub glProgramUniform1iv: PFNGLPROGRAMUNIFORM1IVPROC,
    pub glProgramUniform1f: PFNGLPROGRAMUNIFORM1FPROC,
    pub glProgramUniform1fv: PFNGLPROGRAMUNIFORM1FVPROC,
    pub glProgramUniform1d: PFNGLPROGRAMUNIFORM1DPROC,
    pub glProgramUniform1dv: PFNGLPROGRAMUNIFORM1DVPROC,
    pub glProgramUniform1ui: PFNGLPROGRAMUNIFORM1UIPROC,
    pub glProgramUniform1uiv: PFNGLPROGRAMUNIFORM1UIVPROC,
    pub glProgramUniform2i: PFNGLPROGRAMUNIFORM2IPROC,
    pub glProgramUniform2iv: PFNGLPROGRAMUNIFORM2IVPROC,
    pub glProgramUniform2f: PFNGLPROGRAMUNIFORM2FPROC,
    pub glProgramUniform2fv: PFNGLPROGRAMUNIFORM2FVPROC,
    pub glProgramUniform2d: PFNGLPROGRAMUNIFORM2DPROC,
    pub glProgramUniform2dv: PFNGLPROGRAMUNIFORM2DVPROC,
    pub glProgramUniform2ui: PFNGLPROGRAMUNIFORM2UIPROC,
    pub glProgramUniform2uiv: PFNGLPROGRAMUNIFORM2UIVPROC,
    pub glProgramUniform3i: PFNGLPROGRAMUNIFORM3IPROC,
    pub glProgramUniform3iv: PFNGLPROGRAMUNIFORM3IVPROC,
    pub glProgramUniform3f: PFNGLPROGRAMUNIFORM3FPROC,
    pub glProgramUniform3fv: PFNGLPROGRAMUNIFORM3FVPROC,
    pub glProgramUniform3d: PFNGLPROGRAMUNIFORM3DPROC,
    pub glProgramUniform3dv: PFNGLPROGRAMUNIFORM3DVPROC,
    pub glProgramUniform3ui: PFNGLPROGRAMUNIFORM3UIPROC,
    pub glProgramUniform3uiv: PFNGLPROGRAMUNIFORM3UIVPROC,
    pub glProgramUniform4i: PFNGLPROGRAMUNIFORM4IPROC,
    pub glProgramUniform4iv: PFNGLPROGRAMUNIFORM4IVPROC,
    pub glProgramUniform4f: PFNGLPROGRAMUNIFORM4FPROC,
    pub glProgramUniform4fv: PFNGLPROGRAMUNIFORM4FVPROC,
    pub glProgramUniform4d: PFNGLPROGRAMUNIFORM4DPROC,
    pub glProgramUniform4dv: PFNGLPROGRAMUNIFORM4DVPROC,
    pub glProgramUniform4ui: PFNGLPROGRAMUNIFORM4UIPROC,
    pub glProgramUniform4uiv: PFNGLPROGRAMUNIFORM4UIVPROC,
    pub glProgramUniformMatrix2fv: PFNGLPROGRAMUNIFORMMATRIX2FVPROC,
    pub glProgramUniformMatrix3fv: PFNGLPROGRAMUNIFORMMATRIX3FVPROC,
    pub glProgramUniformMatrix4fv: PFNGLPROGRAMUNIFORMMATRIX4FVPROC,
    pub glProgramUniformMatrix2x3fv: PFNGLPROGRAMUNIFORMMATRIX2X3FVPROC,
    pub glProgramUniformMatrix3x2fv: PFNGLPROGRAMUNIFORMMATRIX3X2FVPROC,
    pub glProgramUniformMatrix2x4fv: PFNGLPROGRAMUNIFORMMATRIX2X4FVPROC,
    pub glProgramUniformMatrix4x2fv: PFNGLPROGRAMUNIFORMMATRIX4X2FVPROC,
    pub glProgramUniformMatrix3x4fv: PFNGLPROGRAMUNIFORMMATRIX3X4FVPROC,
    pub glProgramUniformMatrix4x3fv: PFNGLPROGRAMUNIFORMMATRIX4X3FVPROC,
    pub glProgramUniformMatrix2dv: PFNGLPROGRAMUNIFORMMATRIX2DVPROC,
    pub glProgramUniformMatrix3dv: PFNGLPROGRAMUNIFORMMATRIX3DVPROC,
    pub glProgramUniformMatrix4dv: PFNGLPROGRAMUNIFORMMATRIX4DVPROC,
    pub glProgramUniformMatrix2x3dv: PFNGLPROGRAMUNIFORMMATRIX2X3DVPROC,
    pub glProgramUniformMatrix3x2dv: PFNGLPROGRAMUNIFORMMATRIX3X2DVPROC,
    pub glProgramUniformMatrix2x4dv: PFNGLPROGRAMUNIFORMMATRIX2X4DVPROC,
    pub glProgramUniformMatrix4x2dv: PFNGLPROGRAMUNIFORMMATRIX4X2DVPROC,
    pub glProgramUniformMatrix3x4dv: PFNGLPROGRAMUNIFORMMATRIX3X4DVPROC,
    pub glProgramUniformMatrix4x3dv: PFNGLPROGRAMUNIFORMMATRIX4X3DVPROC,
    pub glViewportArrayv: PFNGLVIEWPORTARRAYVPROC,
    pub glViewportIndexedf: PFNGLVIEWPORTINDEXEDFPROC,
    pub glViewportIndexedfv: PFNGLVIEWPORTINDEXEDFVPROC,
    pub glScissorArrayv: PFNGLSCISSORARRAYVPROC,
    pub glScissorIndexed: PFNGLSCISSORINDEXEDPROC,
    pub glScissorIndexedv: PFNGLSCISSORINDEXEDVPROC,
    pub glDepthRangeArrayv: PFNGLDEPTHRANGEARRAYVPROC,
    pub glDepthRangeIndexed: PFNGLDEPTHRANGEINDEXEDPROC,
    pub glGetFloati_v: PFNGLGETFLOATI_VPROC,
    pub glGetDoublei_v: PFNGLGETDOUBLEI_VPROC,
    pub glVertexAttribL1d: PFNGLVERTEXATTRIBL1DPROC,
    pub glVertexAttribL2d: PFNGLVERTEXATTRIBL2DPROC,
    pub glVertexAttribL3d: PFNGLVERTEXATTRIBL3DPROC,
    pub glVertexAttribL4d: PFNGLVERTEXATTRIBL4DPROC,
    pub glVertexAttribL1dv: PFNGLVERTEXATTRIBL1DVPROC,
    pub glVertexAttribL2dv: PFNGLVERTEXATTRIBL2DVPROC,
    pub glVertexAttribL3dv: PFNGLVERTEXATTRIBL3DVPROC,
    pub glVertexAttribL4dv: PFNGLVERTEXATTRIBL4DVPROC,
    pub glVertexAttribLPointer: PFNGLVERTEXATTRIBLPOINTERPROC,
    pub glGetVertexAttribLdv: PFNGLGETVERTEXATTRIBLDVPROC,
}
