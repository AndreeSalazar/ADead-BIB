//! # ADead-BIB OpenGL Global — Consolidado Universal v4.7
//!
//! REEMPLAZA: gl10.rs → gl46.rs (16 archivos) + constants_gl1x-4x (5 archivos)
//! 
//! ## Arquitectura:
//! - **GL1x**: OpenGL 1.0-1.5 (Fixed Function Pipeline)
//! - **GL2x**: OpenGL 2.0-2.1 (GLSL 1.10, Shaders)
//! - **GL3x**: OpenGL 3.0-3.3 (Core Profile, Geometry Shaders)
//! - **GL4x**: OpenGL 4.0-4.6 (Tessellation, Compute, SPIR-V)
//! - **GL47**: ADead-BIB Universal Shader Bridge
//!
//! ## Filosofía: "OpenGL como Expeller Universal"
//! Cualquier shader format → OpenGL Global → GPU Drivers

use super::types::*;

// ============================================================================
// SECTION 1: OpenGL 1.x Functions (Fixed Function Pipeline)
// ============================================================================

pub mod gl1x {
    use super::*;
    
    // GL 1.0 Core
    pub type PFNGLCULLFACEPROC = Option<unsafe extern "system" fn(mode: GLenum)>;
    pub type PFNGLFRONTFACEPROC = Option<unsafe extern "system" fn(mode: GLenum)>;
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
    pub type PFNGLGETBOOLEANVPROC = Option<unsafe extern "system" fn(pname: GLenum, data: *mut GLboolean)>;
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
    pub type PFNGLDEPTHRANGEPROC = Option<unsafe extern "system" fn(near: GLdouble, far: GLdouble)>;
    pub type PFNGLVIEWPORTPROC = Option<unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei)>;
    
    // GL 1.1
    pub type PFNGLDRAWARRAYSPROC = Option<unsafe extern "system" fn(mode: GLenum, first: GLint, count: GLsizei)>;
    pub type PFNGLDRAWELEMENTSPROC = Option<unsafe extern "system" fn(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const GLvoid)>;
    pub type PFNGLGENTEXTURESPROC = Option<unsafe extern "system" fn(n: GLsizei, textures: *mut GLuint)>;
    pub type PFNGLDELETETEXTURESPROC = Option<unsafe extern "system" fn(n: GLsizei, textures: *const GLuint)>;
    pub type PFNGLBINDTEXTUREPROC = Option<unsafe extern "system" fn(target: GLenum, texture: GLuint)>;
    pub type PFNGLPRIORITIZETEXTURESPROC = Option<unsafe extern "system" fn(n: GLsizei, textures: *const GLuint, priorities: *const GLfloat)>;
    pub type PFNGLARETEXTURESRESIDENTPROC = Option<unsafe extern "system" fn(n: GLsizei, textures: *const GLuint, residences: *mut GLboolean) -> GLboolean>;
    pub type PFNGLISTEXTUREPROC = Option<unsafe extern "system" fn(texture: GLuint) -> GLboolean>;
    pub type PFNGLTEXSUBIMAGE1DPROC = Option<unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *const GLvoid)>;
    pub type PFNGLTEXSUBIMAGE2DPROC = Option<unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const GLvoid)>;
    pub type PFNGLCOPYTEXIMAGE1DPROC = Option<unsafe extern "system" fn(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, border: GLint)>;
    pub type PFNGLCOPYTEXIMAGE2DPROC = Option<unsafe extern "system" fn(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint)>;
    pub type PFNGLCOPYTEXSUBIMAGE1DPROC = Option<unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei)>;
    pub type PFNGLCOPYTEXSUBIMAGE2DPROC = Option<unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei)>;
    
    // GL 1.2
    pub type PFNGLBLENDCOLORPROC = Option<unsafe extern "system" fn(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat)>;
    pub type PFNGLBLENDEQUATIONPROC = Option<unsafe extern "system" fn(mode: GLenum)>;
    
    // GL 1.3
    pub type PFNGLACTIVETEXTUREPROC = Option<unsafe extern "system" fn(texture: GLenum)>;
    pub type PFNGLSAMPLECOVERAGEPROC = Option<unsafe extern "system" fn(value: GLfloat, invert: GLboolean)>;
    pub type PFNGLCOMPRESSEDTEXIMAGE3DPROC = Option<unsafe extern "system" fn(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *const GLvoid)>;
    pub type PFNGLCOMPRESSEDTEXIMAGE2DPROC = Option<unsafe extern "system" fn(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *const GLvoid)>;
    pub type PFNGLCOMPRESSEDTEXIMAGE1DPROC = Option<unsafe extern "system" fn(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, border: GLint, imageSize: GLsizei, data: *const GLvoid)>;
    pub type PFNGLCOMPRESSEDTEXSUBIMAGE3DPROC = Option<unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const GLvoid)>;
    pub type PFNGLCOMPRESSEDTEXSUBIMAGE2DPROC = Option<unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const GLvoid)>;
    pub type PFNGLCOMPRESSEDTEXSUBIMAGE1DPROC = Option<unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *const GLvoid)>;
    pub type PFNGLGETCOMPRESSEDTEXIMAGEPROC = Option<unsafe extern "system" fn(target: GLenum, level: GLint, img: *mut GLvoid)>;
    
    // GL 1.4
    pub type PFNGLBLENDFUNCSEPARATEPROC = Option<unsafe extern "system" fn(sfactorRGB: GLenum, dfactorRGB: GLenum, sfactorAlpha: GLenum, dfactorAlpha: GLenum)>;
    pub type PFNGLMULTIDRAWARRAYSPROC = Option<unsafe extern "system" fn(mode: GLenum, first: *const GLint, count: *const GLsizei, drawcount: GLsizei)>;
    pub type PFNGLMULTIDRAWELEMENTSPROC = Option<unsafe extern "system" fn(mode: GLenum, count: *const GLsizei, type_: GLenum, indices: *const *const GLvoid, drawcount: GLsizei)>;
    pub type PFNGLPOINTPARAMETERFPROC = Option<unsafe extern "system" fn(pname: GLenum, param: GLfloat)>;
    pub type PFNGLPOINTPARAMETERFVPROC = Option<unsafe extern "system" fn(pname: GLenum, params: *const GLfloat)>;
    pub type PFNGLPOINTPARAMETERIPROC = Option<unsafe extern "system" fn(pname: GLenum, param: GLint)>;
    pub type PFNGLPOINTPARAMETERIVPROC = Option<unsafe extern "system" fn(pname: GLenum, params: *const GLint)>;
    
    // GL 1.5 — Buffer Objects (VBO)
    pub type PFNGLBINDBUFFERPROC = Option<unsafe extern "system" fn(target: GLenum, buffer: GLuint)>;
    pub type PFNGLDELETEBUFFERSPROC = Option<unsafe extern "system" fn(n: GLsizei, buffers: *const GLuint)>;
    pub type PFNGLGENBUFFERSPROC = Option<unsafe extern "system" fn(n: GLsizei, buffers: *mut GLuint)>;
    pub type PFNGLISBUFFERPROC = Option<unsafe extern "system" fn(buffer: GLuint) -> GLboolean>;
    pub type PFNGLBUFFERDATAPROC = Option<unsafe extern "system" fn(target: GLenum, size: GLsizeiptr, data: *const GLvoid, usage: GLenum)>;
    pub type PFNGLBUFFERSUBDATAPROC = Option<unsafe extern "system" fn(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *const GLvoid)>;
    pub type PFNGLGETBUFFERSUBDATAPROC = Option<unsafe extern "system" fn(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *mut GLvoid)>;
    pub type PFNGLMAPBUFFERPROC = Option<unsafe extern "system" fn(target: GLenum, access: GLenum) -> *mut GLvoid>;
    pub type PFNGLUNMAPBUFFERPROC = Option<unsafe extern "system" fn(target: GLenum) -> GLboolean>;
    pub type PFNGLGETBUFFERPARAMETERIVPROC = Option<unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint)>;
    pub type PFNGLGETBUFFERPOINTERVPROC = Option<unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut *mut GLvoid)>;
}

// ============================================================================
// SECTION 2: OpenGL 2.x Functions (Programmable Shaders)
// ============================================================================

pub mod gl2x {
    use super::*;
    
    // GL 2.0 — GLSL Shaders
    pub type PFNGLBLENDEQUATIONSEPARATEPROC = Option<unsafe extern "system" fn(modeRGB: GLenum, modeAlpha: GLenum)>;
    pub type PFNGLDRAWBUFFERSPROC = Option<unsafe extern "system" fn(n: GLsizei, bufs: *const GLenum)>;
    pub type PFNGLSTENCILOPSEPARATEPROC = Option<unsafe extern "system" fn(face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum)>;
    pub type PFNGLSTENCILFUNCSEPARATEPROC = Option<unsafe extern "system" fn(face: GLenum, func: GLenum, ref_: GLint, mask: GLuint)>;
    pub type PFNGLSTENCILMASKSEPARATEPROC = Option<unsafe extern "system" fn(face: GLenum, mask: GLuint)>;
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
    
    // GL 2.1 — Uniform matrices non-square
    pub type PFNGLUNIFORMMATRIX2X3FVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat)>;
    pub type PFNGLUNIFORMMATRIX3X2FVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat)>;
    pub type PFNGLUNIFORMMATRIX2X4FVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat)>;
    pub type PFNGLUNIFORMMATRIX4X2FVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat)>;
    pub type PFNGLUNIFORMMATRIX3X4FVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat)>;
    pub type PFNGLUNIFORMMATRIX4X3FVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat)>;
}

// ============================================================================
// SECTION 3: OpenGL 3.x Functions (Core Profile, VAO, FBO, Geometry)
// ============================================================================

pub mod gl3x {
    use super::*;
    
    // GL 3.0 — Framebuffers, VAO, integer textures
    pub type PFNGLCOLORMASKIPROC = Option<unsafe extern "system" fn(index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean)>;
    pub type PFNGLGETBOOLEANI_VPROC = Option<unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLboolean)>;
    pub type PFNGLGETINTEGERI_VPROC = Option<unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLint)>;
    pub type PFNGLENABLEIPROC = Option<unsafe extern "system" fn(target: GLenum, index: GLuint)>;
    pub type PFNGLDISABLEIPROC = Option<unsafe extern "system" fn(target: GLenum, index: GLuint)>;
    pub type PFNGLISENABLEDIPROC = Option<unsafe extern "system" fn(target: GLenum, index: GLuint) -> GLboolean>;
    pub type PFNGLBEGINTRANSFORMFEEDBACKPROC = Option<unsafe extern "system" fn(primitiveMode: GLenum)>;
    pub type PFNGLENDTRANSFORMFEEDBACKPROC = Option<unsafe extern "system" fn()>;
    pub type PFNGLBINDBUFFERRANGEPROC = Option<unsafe extern "system" fn(target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr)>;
    pub type PFNGLBINDBUFFERBASEPROC = Option<unsafe extern "system" fn(target: GLenum, index: GLuint, buffer: GLuint)>;
    pub type PFNGLTRANSFORMFEEDBACKVARYINGSPROC = Option<unsafe extern "system" fn(program: GLuint, count: GLsizei, varyings: *const *const GLchar, bufferMode: GLenum)>;
    pub type PFNGLGETTRANSFORMFEEDBACKVARYINGPROC = Option<unsafe extern "system" fn(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, type_: *mut GLenum, name: *mut GLchar)>;
    pub type PFNGLCLAMPCOLORPROC = Option<unsafe extern "system" fn(target: GLenum, clamp: GLenum)>;
    pub type PFNGLBEGINCONDITIONALRENDERPROC = Option<unsafe extern "system" fn(id: GLuint, mode: GLenum)>;
    pub type PFNGLENDCONDITIONALRENDERPROC = Option<unsafe extern "system" fn()>;
    pub type PFNGLVERTEXATTRIBIPOINTERPROC = Option<unsafe extern "system" fn(index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const GLvoid)>;
    pub type PFNGLGETVERTEXATTRIBIIVPROC = Option<unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLint)>;
    pub type PFNGLGETVERTEXATTRIBIUIVPROC = Option<unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLuint)>;
    pub type PFNGLVERTEXATTRIBI1IPROC = Option<unsafe extern "system" fn(index: GLuint, x: GLint)>;
    pub type PFNGLVERTEXATTRIBI2IPROC = Option<unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint)>;
    pub type PFNGLVERTEXATTRIBI3IPROC = Option<unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint, z: GLint)>;
    pub type PFNGLVERTEXATTRIBI4IPROC = Option<unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint)>;
    pub type PFNGLVERTEXATTRIBI1UIPROC = Option<unsafe extern "system" fn(index: GLuint, x: GLuint)>;
    pub type PFNGLVERTEXATTRIBI2UIPROC = Option<unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint)>;
    pub type PFNGLVERTEXATTRIBI3UIPROC = Option<unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint, z: GLuint)>;
    pub type PFNGLVERTEXATTRIBI4UIPROC = Option<unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint)>;
    pub type PFNGLVERTEXATTRIBI1IVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLint)>;
    pub type PFNGLVERTEXATTRIBI2IVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLint)>;
    pub type PFNGLVERTEXATTRIBI3IVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLint)>;
    pub type PFNGLVERTEXATTRIBI4IVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLint)>;
    pub type PFNGLVERTEXATTRIBI1UIVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLuint)>;
    pub type PFNGLVERTEXATTRIBI2UIVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLuint)>;
    pub type PFNGLVERTEXATTRIBI3UIVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLuint)>;
    pub type PFNGLVERTEXATTRIBI4UIVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLuint)>;
    pub type PFNGLVERTEXATTRIBI4BVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLbyte)>;
    pub type PFNGLVERTEXATTRIBI4SVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLshort)>;
    pub type PFNGLVERTEXATTRIBI4UBVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLubyte)>;
    pub type PFNGLVERTEXATTRIBI4USVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLushort)>;
    pub type PFNGLGETUNIFORMUIVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLuint)>;
    pub type PFNGLBINDFRAGDATALOCATIONPROC = Option<unsafe extern "system" fn(program: GLuint, color: GLuint, name: *const GLchar)>;
    pub type PFNGLGETFRAGDATALOCATIONPROC = Option<unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint>;
    pub type PFNGLUNIFORM1UIPROC = Option<unsafe extern "system" fn(location: GLint, v0: GLuint)>;
    pub type PFNGLUNIFORM2UIPROC = Option<unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint)>;
    pub type PFNGLUNIFORM3UIPROC = Option<unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint)>;
    pub type PFNGLUNIFORM4UIPROC = Option<unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint)>;
    pub type PFNGLUNIFORM1UIVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint)>;
    pub type PFNGLUNIFORM2UIVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint)>;
    pub type PFNGLUNIFORM3UIVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint)>;
    pub type PFNGLUNIFORM4UIVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint)>;
    pub type PFNGLTEXPARAMETERIIVPROC = Option<unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLint)>;
    pub type PFNGLTEXPARAMETERIUIVPROC = Option<unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLuint)>;
    pub type PFNGLGETTEXPARAMETERIIVPROC = Option<unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint)>;
    pub type PFNGLGETTEXPARAMETERIUIVPROC = Option<unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLuint)>;
    pub type PFNGLCLEARBUFFERIVPROC = Option<unsafe extern "system" fn(buffer: GLenum, drawbuffer: GLint, value: *const GLint)>;
    pub type PFNGLCLEARBUFFERUIVPROC = Option<unsafe extern "system" fn(buffer: GLenum, drawbuffer: GLint, value: *const GLuint)>;
    pub type PFNGLCLEARBUFFERFVPROC = Option<unsafe extern "system" fn(buffer: GLenum, drawbuffer: GLint, value: *const GLfloat)>;
    pub type PFNGLCLEARBUFFERFIPROC = Option<unsafe extern "system" fn(buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint)>;
    pub type PFNGLGETSTRINGIPROC = Option<unsafe extern "system" fn(name: GLenum, index: GLuint) -> *const GLubyte>;
    pub type PFNGLISRENDERBUFFERPROC = Option<unsafe extern "system" fn(renderbuffer: GLuint) -> GLboolean>;
    pub type PFNGLBINDRENDERBUFFERPROC = Option<unsafe extern "system" fn(target: GLenum, renderbuffer: GLuint)>;
    pub type PFNGLDELETERENDERBUFFERSPROC = Option<unsafe extern "system" fn(n: GLsizei, renderbuffers: *const GLuint)>;
    pub type PFNGLGENRENDERBUFFERSPROC = Option<unsafe extern "system" fn(n: GLsizei, renderbuffers: *mut GLuint)>;
    pub type PFNGLRENDERBUFFERSTORAGEPROC = Option<unsafe extern "system" fn(target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei)>;
    pub type PFNGLGETRENDERBUFFERPARAMETERIVPROC = Option<unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint)>;
    pub type PFNGLISFRAMEBUFFERPROC = Option<unsafe extern "system" fn(framebuffer: GLuint) -> GLboolean>;
    pub type PFNGLBINDFRAMEBUFFERPROC = Option<unsafe extern "system" fn(target: GLenum, framebuffer: GLuint)>;
    pub type PFNGLDELETEFRAMEBUFFERSPROC = Option<unsafe extern "system" fn(n: GLsizei, framebuffers: *const GLuint)>;
    pub type PFNGLGENFRAMEBUFFERSPROC = Option<unsafe extern "system" fn(n: GLsizei, framebuffers: *mut GLuint)>;
    pub type PFNGLCHECKFRAMEBUFFERSTATUSPROC = Option<unsafe extern "system" fn(target: GLenum) -> GLenum>;
    pub type PFNGLFRAMEBUFFERTEXTURE1DPROC = Option<unsafe extern "system" fn(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint)>;
    pub type PFNGLFRAMEBUFFERTEXTURE2DPROC = Option<unsafe extern "system" fn(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint)>;
    pub type PFNGLFRAMEBUFFERTEXTURE3DPROC = Option<unsafe extern "system" fn(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint, zoffset: GLint)>;
    pub type PFNGLFRAMEBUFFERRENDERBUFFERPROC = Option<unsafe extern "system" fn(target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint)>;
    pub type PFNGLGETFRAMEBUFFERATTACHMENTPARAMETERIVPROC = Option<unsafe extern "system" fn(target: GLenum, attachment: GLenum, pname: GLenum, params: *mut GLint)>;
    pub type PFNGLGENERATEMIPMAPPROC = Option<unsafe extern "system" fn(target: GLenum)>;
    pub type PFNGLBLITFRAMEBUFFERPROC = Option<unsafe extern "system" fn(srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum)>;
    pub type PFNGLRENDERBUFFERSTORAGEMULTISAMPLEPROC = Option<unsafe extern "system" fn(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei)>;
    pub type PFNGLFRAMEBUFFERTEXTURELAYERPROC = Option<unsafe extern "system" fn(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint)>;
    pub type PFNGLMAPBUFFERRANGEPROC = Option<unsafe extern "system" fn(target: GLenum, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut GLvoid>;
    pub type PFNGLFLUSHMAPPEDBUFFERRANGEPROC = Option<unsafe extern "system" fn(target: GLenum, offset: GLintptr, length: GLsizeiptr)>;
    pub type PFNGLBINDVERTEXARRAYPROC = Option<unsafe extern "system" fn(array: GLuint)>;
    pub type PFNGLDELETEVERTEXARRAYSPROC = Option<unsafe extern "system" fn(n: GLsizei, arrays: *const GLuint)>;
    pub type PFNGLGENVERTEXARRAYSPROC = Option<unsafe extern "system" fn(n: GLsizei, arrays: *mut GLuint)>;
    pub type PFNGLISVERTEXARRAYPROC = Option<unsafe extern "system" fn(array: GLuint) -> GLboolean>;
    
    // GL 3.1 — Instancing, texture buffer, uniform buffer
    pub type PFNGLDRAWARRAYSINSTANCEDPROC = Option<unsafe extern "system" fn(mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei)>;
    pub type PFNGLDRAWELEMENTSINSTANCEDPROC = Option<unsafe extern "system" fn(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const GLvoid, instancecount: GLsizei)>;
    pub type PFNGLTEXBUFFERPROC = Option<unsafe extern "system" fn(target: GLenum, internalformat: GLenum, buffer: GLuint)>;
    pub type PFNGLPRIMITIVERESTARTINDEXPROC = Option<unsafe extern "system" fn(index: GLuint)>;
    pub type PFNGLCOPYBUFFERSUBDATAPROC = Option<unsafe extern "system" fn(readTarget: GLenum, writeTarget: GLenum, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr)>;
    pub type PFNGLGETUNIFORMINDICESPROC = Option<unsafe extern "system" fn(program: GLuint, uniformCount: GLsizei, uniformNames: *const *const GLchar, uniformIndices: *mut GLuint)>;
    pub type PFNGLGETACTIVEUNIFORMSIVPROC = Option<unsafe extern "system" fn(program: GLuint, uniformCount: GLsizei, uniformIndices: *const GLuint, pname: GLenum, params: *mut GLint)>;
    pub type PFNGLGETACTIVEUNIFORMNAMEPROC = Option<unsafe extern "system" fn(program: GLuint, uniformIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformName: *mut GLchar)>;
    pub type PFNGLGETUNIFORMBLOCKINDEXPROC = Option<unsafe extern "system" fn(program: GLuint, uniformBlockName: *const GLchar) -> GLuint>;
    pub type PFNGLGETACTIVEUNIFORMBLOCKIVPROC = Option<unsafe extern "system" fn(program: GLuint, uniformBlockIndex: GLuint, pname: GLenum, params: *mut GLint)>;
    pub type PFNGLGETACTIVEUNIFORMBLOCKNAMEPROC = Option<unsafe extern "system" fn(program: GLuint, uniformBlockIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformBlockName: *mut GLchar)>;
    pub type PFNGLUNIFORMBLOCKBINDINGPROC = Option<unsafe extern "system" fn(program: GLuint, uniformBlockIndex: GLuint, uniformBlockBinding: GLuint)>;
    
    // GL 3.2 — Core profile, geometry shaders, sync objects
    pub type PFNGLDRAWELEMENTSBASEVERTEXPROC = Option<unsafe extern "system" fn(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const GLvoid, basevertex: GLint)>;
    pub type PFNGLDRAWRANGEELEMENTSBASEVERTEXPROC = Option<unsafe extern "system" fn(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *const GLvoid, basevertex: GLint)>;
    pub type PFNGLDRAWELEMENTSINSTANCEDBASEVERTEXPROC = Option<unsafe extern "system" fn(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const GLvoid, instancecount: GLsizei, basevertex: GLint)>;
    pub type PFNGLMULTIDRAWELEMENTSBASEVERTEXPROC = Option<unsafe extern "system" fn(mode: GLenum, count: *const GLsizei, type_: GLenum, indices: *const *const GLvoid, drawcount: GLsizei, basevertex: *const GLint)>;
    pub type PFNGLPROVOKINGVERTEXPROC = Option<unsafe extern "system" fn(mode: GLenum)>;
    pub type PFNGLFENCESYNCPROC = Option<unsafe extern "system" fn(condition: GLenum, flags: GLbitfield) -> GLsync>;
    pub type PFNGLISSYNCPROC = Option<unsafe extern "system" fn(sync: GLsync) -> GLboolean>;
    pub type PFNGLDELETESYNCPROC = Option<unsafe extern "system" fn(sync: GLsync)>;
    pub type PFNGLCLIENTWAITSYNCPROC = Option<unsafe extern "system" fn(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> GLenum>;
    pub type PFNGLWAITSYNCPROC = Option<unsafe extern "system" fn(sync: GLsync, flags: GLbitfield, timeout: GLuint64)>;
    pub type PFNGLGETINTEGER64VPROC = Option<unsafe extern "system" fn(pname: GLenum, data: *mut GLint64)>;
    pub type PFNGLGETSYNCIVPROC = Option<unsafe extern "system" fn(sync: GLsync, pname: GLenum, bufSize: GLsizei, length: *mut GLsizei, values: *mut GLint)>;
    pub type PFNGLGETINTEGER64I_VPROC = Option<unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLint64)>;
    pub type PFNGLGETBUFFERPARAMETERI64VPROC = Option<unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint64)>;
    pub type PFNGLFRAMEBUFFERTEXTUREPROC = Option<unsafe extern "system" fn(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint)>;
    pub type PFNGLTEXIMAGE2DMULTISAMPLEPROC = Option<unsafe extern "system" fn(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean)>;
    pub type PFNGLTEXIMAGE3DMULTISAMPLEPROC = Option<unsafe extern "system" fn(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean)>;
    pub type PFNGLGETMULTISAMPLEFVPROC = Option<unsafe extern "system" fn(pname: GLenum, index: GLuint, val: *mut GLfloat)>;
    pub type PFNGLSAMPLEMASKIPROC = Option<unsafe extern "system" fn(maskNumber: GLuint, mask: GLbitfield)>;
    
    // GL 3.3 — Blend func advanced, samplers
    pub type PFNGLBINDSAMPLERPROC = Option<unsafe extern "system" fn(unit: GLuint, sampler: GLuint)>;
    pub type PFNGLSAMPLERPARAMETERIPROC = Option<unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: GLint)>;
    pub type PFNGLSAMPLERPARAMETERIVPROC = Option<unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: *const GLint)>;
    pub type PFNGLSAMPLERPARAMETERFPROC = Option<unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: GLfloat)>;
    pub type PFNGLSAMPLERPARAMETERFVPROC = Option<unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: *const GLfloat)>;
    pub type PFNGLSAMPLERPARAMETERIIVPROC = Option<unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: *const GLint)>;
    pub type PFNGLSAMPLERPARAMETERIUIVPROC = Option<unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: *const GLuint)>;
    pub type PFNGLGETSAMPLERPARAMETERIVPROC = Option<unsafe extern "system" fn(sampler: GLuint, pname: GLenum, params: *mut GLint)>;
    pub type PFNGLGETSAMPLERPARAMETERIIVPROC = Option<unsafe extern "system" fn(sampler: GLuint, pname: GLenum, params: *mut GLint)>;
    pub type PFNGLGETSAMPLERPARAMETERFVPROC = Option<unsafe extern "system" fn(sampler: GLuint, pname: GLenum, params: *mut GLfloat)>;
    pub type PFNGLGETSAMPLERPARAMETERIUIVPROC = Option<unsafe extern "system" fn(sampler: GLuint, pname: GLenum, params: *mut GLuint)>;
    pub type PFNGLVERTEXATTRIBDIVISORPROC = Option<unsafe extern "system" fn(index: GLuint, divisor: GLuint)>;
}

// ============================================================================
// SECTION 4: OpenGL 4.x Functions (Tessellation, Compute, SPIR-V, DSA)
// ============================================================================

pub mod gl4x {
    use super::*;
    
    // GL 4.0 — Tessellation shaders, draw indirect
    pub type PFNGLMINSAMPLESHADINGPROC = Option<unsafe extern "system" fn(value: GLfloat)>;
    pub type PFNGLBLENDEQUATIONIPROC = Option<unsafe extern "system" fn(buf: GLuint, mode: GLenum)>;
    pub type PFNGLBLENDEQUATIONSEPARATEIPROC = Option<unsafe extern "system" fn(buf: GLuint, modeRGB: GLenum, modeAlpha: GLenum)>;
    pub type PFNGLBLENDFUNCIPROC = Option<unsafe extern "system" fn(buf: GLuint, src: GLenum, dst: GLenum)>;
    pub type PFNGLBLENDFUNCSEPARATEIPROC = Option<unsafe extern "system" fn(buf: GLuint, srcRGB: GLenum, dstRGB: GLenum, srcAlpha: GLenum, dstAlpha: GLenum)>;
    pub type PFNGLDRAWARRAYSINDIRECTPROC = Option<unsafe extern "system" fn(mode: GLenum, indirect: *const GLvoid)>;
    pub type PFNGLDRAWELEMENTSINDIRECTPROC = Option<unsafe extern "system" fn(mode: GLenum, type_: GLenum, indirect: *const GLvoid)>;
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
    pub type PFNGLGETSUBROUTINEUNIFORMLOCATIONPROC = Option<unsafe extern "system" fn(program: GLuint, shadertype: GLenum, name: *const GLchar) -> GLint>;
    pub type PFNGLGETSUBROUTINEINDEXPROC = Option<unsafe extern "system" fn(program: GLuint, shadertype: GLenum, name: *const GLchar) -> GLuint>;
    pub type PFNGLGETACTIVESUBROUTINEUNIFORMIVPROC = Option<unsafe extern "system" fn(program: GLuint, shadertype: GLenum, index: GLuint, pname: GLenum, values: *mut GLint)>;
    pub type PFNGLGETACTIVESUBROUTINEUNIFORMNAMEPROC = Option<unsafe extern "system" fn(program: GLuint, shadertype: GLenum, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar)>;
    pub type PFNGLGETACTIVESUBROUTINENAMEPROC = Option<unsafe extern "system" fn(program: GLuint, shadertype: GLenum, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar)>;
    pub type PFNGLUNIFORMSUBROUTINESUIVPROC = Option<unsafe extern "system" fn(shadertype: GLenum, count: GLsizei, indices: *const GLuint)>;
    pub type PFNGLGETUNIFORMSUBROUTINEUIVPROC = Option<unsafe extern "system" fn(shadertype: GLenum, location: GLint, params: *mut GLuint)>;
    pub type PFNGLGETPROGRAMSTAGEIVPROC = Option<unsafe extern "system" fn(program: GLuint, shadertype: GLenum, pname: GLenum, values: *mut GLint)>;
    pub type PFNGLPATCHPARAMETERIPROC = Option<unsafe extern "system" fn(pname: GLenum, value: GLint)>;
    pub type PFNGLPATCHPARAMETERFVPROC = Option<unsafe extern "system" fn(pname: GLenum, values: *const GLfloat)>;
    pub type PFNGLBINDTRANSFORMFEEDBACKPROC = Option<unsafe extern "system" fn(target: GLenum, id: GLuint)>;
    pub type PFNGLDELETETRANSFORMFEEDBACKSPROC = Option<unsafe extern "system" fn(n: GLsizei, ids: *const GLuint)>;
    pub type PFNGLGENTRANSFORMFEEDBACKSPROC = Option<unsafe extern "system" fn(n: GLsizei, ids: *mut GLuint)>;
    pub type PFNGLISTRANSFORMFEEDBACKPROC = Option<unsafe extern "system" fn(id: GLuint) -> GLboolean>;
    pub type PFNGLPAUSETRANSFORMFEEDBACKPROC = Option<unsafe extern "system" fn()>;
    pub type PFNGLRESUMETRANSFORMFEEDBACKPROC = Option<unsafe extern "system" fn()>;
    pub type PFNGLDRAWTRANSFORMFEEDBACKPROC = Option<unsafe extern "system" fn(mode: GLenum, id: GLuint)>;
    pub type PFNGLDRAWTRANSFORMFEEDBACKSTREAMPROC = Option<unsafe extern "system" fn(mode: GLenum, id: GLuint, stream: GLuint)>;
    pub type PFNGLBEGINQUERYINDEXEDPROC = Option<unsafe extern "system" fn(target: GLenum, index: GLuint, id: GLuint)>;
    pub type PFNGLENDQUERYINDEXEDPROC = Option<unsafe extern "system" fn(target: GLenum, index: GLuint)>;
    pub type PFNGLGETQUERYINDEXEDIVPROC = Option<unsafe extern "system" fn(target: GLenum, index: GLuint, pname: GLenum, params: *mut GLint)>;
    
    // GL 4.1 — Shader binary, viewport arrays, doubles
    // (incluido en gl41.rs existente)
    
    // GL 4.2 — Texture storage, atomic counters
    pub type PFNGLTEXSTORAGE1DPROC = Option<unsafe extern "system" fn(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei)>;
    pub type PFNGLTEXSTORAGE2DPROC = Option<unsafe extern "system" fn(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei)>;
    pub type PFNGLTEXSTORAGE3DPROC = Option<unsafe extern "system" fn(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei)>;
    pub type PFNGLDRAWTRANSFORMFEEDBACKINSTANCEDPROC = Option<unsafe extern "system" fn(mode: GLenum, id: GLuint, instancecount: GLsizei)>;
    pub type PFNGLDRAWTRANSFORMFEEDBACKSTREAMINSTANCEDPROC = Option<unsafe extern "system" fn(mode: GLenum, id: GLuint, stream: GLuint, instancecount: GLsizei)>;
    
    // GL 4.3 — Compute shaders, debug callbacks, explicit uniform location
    pub type PFNGLCLEARBUFFERDATAPROC = Option<unsafe extern "system" fn(target: GLenum, internalformat: GLenum, format: GLenum, type_: GLenum, data: *const GLvoid)>;
    pub type PFNGLCLEARBUFFERSUBDATAPROC = Option<unsafe extern "system" fn(target: GLenum, internalformat: GLenum, offset: GLintptr, size: GLsizeiptr, format: GLenum, type_: GLenum, data: *const GLvoid)>;
    pub type PFNGLDISPATCHCOMPUTEPROC = Option<unsafe extern "system" fn(num_groups_x: GLuint, num_groups_y: GLuint, num_groups_z: GLuint)>;
    pub type PFNGLDISPATCHCOMPUTEINDIRECTPROC = Option<unsafe extern "system" fn(indirect: GLintptr)>;
    pub type PFNGLCOPYIMAGESUBDATAPROC = Option<unsafe extern "system" fn(srcName: GLuint, srcTarget: GLenum, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: GLenum, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, srcWidth: GLsizei, srcHeight: GLsizei, srcDepth: GLsizei)>;
    pub type PFNGLFRAMEBUFFERPARAMETERIPROC = Option<unsafe extern "system" fn(target: GLenum, pname: GLenum, param: GLint)>;
    pub type PFNGLGETFRAMEBUFFERPARAMETERIVPROC = Option<unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint)>;
    pub type PFNGLGETINTERNALFORMATI64VPROC = Option<unsafe extern "system" fn(target: GLenum, internalformat: GLenum, pname: GLenum, bufSize: GLsizei, params: *mut GLint64)>;
    pub type PFNGLINVALIDATETEXSUBIMAGEPROC = Option<unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei)>;
    pub type PFNGLINVALIDATETEXIMAGEPROC = Option<unsafe extern "system" fn(texture: GLuint, level: GLint)>;
    pub type PFNGLINVALIDATEBUFFERSUBDATAPROC = Option<unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, length: GLsizeiptr)>;
    pub type PFNGLINVALIDATEBUFFERDATAPROC = Option<unsafe extern "system" fn(buffer: GLuint)>;
    pub type PFNGLINVALIDATEFRAMEBUFFERPROC = Option<unsafe extern "system" fn(target: GLenum, numAttachments: GLsizei, attachments: *const GLenum)>;
    pub type PFNGLINVALIDATESUBFRAMEBUFFERPROC = Option<unsafe extern "system" fn(target: GLenum, numAttachments: GLsizei, attachments: *const GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei)>;
    pub type PFNGLMULTIDRAWARRAYSINDIRECTPROC = Option<unsafe extern "system" fn(mode: GLenum, indirect: *const GLvoid, drawcount: GLsizei, stride: GLsizei)>;
    pub type PFNGLMULTIDRAWELEMENTSINDIRECTPROC = Option<unsafe extern "system" fn(mode: GLenum, type_: GLenum, indirect: *const GLvoid, drawcount: GLsizei, stride: GLsizei)>;
    pub type PFNGLGETPROGRAMINTERFACEIVPROC = Option<unsafe extern "system" fn(program: GLuint, programInterface: GLenum, pname: GLenum, params: *mut GLint)>;
    pub type PFNGLGETPROGRAMRESOURCEINDEXPROC = Option<unsafe extern "system" fn(program: GLuint, programInterface: GLenum, name: *const GLchar) -> GLuint>;
    pub type PFNGLGETPROGRAMRESOURCENAMEPROC = Option<unsafe extern "system" fn(program: GLuint, programInterface: GLenum, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar)>;
    pub type PFNGLGETPROGRAMRESOURCEIVPROC = Option<unsafe extern "system" fn(program: GLuint, programInterface: GLenum, index: GLuint, propCount: GLsizei, props: *const GLenum, bufSize: GLsizei, length: *mut GLsizei, params: *mut GLint)>;
    pub type PFNGLGETPROGRAMRESOURCELOCATIONPROC = Option<unsafe extern "system" fn(program: GLuint, programInterface: GLenum, name: *const GLchar) -> GLint>;
    pub type PFNGLGETPROGRAMRESOURCELOCATIONINDEXPROC = Option<unsafe extern "system" fn(program: GLuint, programInterface: GLenum, name: *const GLchar) -> GLint>;
    pub type PFNGLSHADERSTORAGEBLOCKBINDINGPROC = Option<unsafe extern "system" fn(program: GLuint, storageBlockIndex: GLuint, storageBlockBinding: GLuint)>;
    pub type PFNGLGETSTRINGIPROC = Option<unsafe extern "system" fn(name: GLenum, index: GLuint) -> *const GLubyte>;
    pub type PFNGLGETBOOLEANI_VPROC = Option<unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLboolean)>;
    pub type PFNGLGETINTEGERI_VPROC = Option<unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLint)>;
    
    // GL 4.4 — Buffer storage, clear tex image
    pub type PFNGLBUFFERSTORAGEPROC = Option<unsafe extern "system" fn(target: GLenum, size: GLsizeiptr, data: *const GLvoid, flags: GLbitfield)>;
    pub type PFNGLCLEARTEXIMAGEPROC = Option<unsafe extern "system" fn(texture: GLuint, level: GLint, format: GLenum, type_: GLenum, data: *const GLvoid)>;
    pub type PFNGLCLEARTEXSUBIMAGEPROC = Option<unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, data: *const GLvoid)>;
    pub type PFNGLBINDBUFFERSBASEPROC = Option<unsafe extern "system" fn(target: GLenum, first: GLuint, count: GLsizei, buffers: *const GLuint)>;
    pub type PFNGLBINDBUFFERSRANGEPROC = Option<unsafe extern "system" fn(target: GLenum, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, sizes: *const GLsizeiptr)>;
    pub type PFNGLBINDTEXTURESPROC = Option<unsafe extern "system" fn(first: GLuint, count: GLsizei, textures: *const GLuint)>;
    pub type PFNGLBINDSAMPLERSPROC = Option<unsafe extern "system" fn(first: GLuint, count: GLsizei, samplers: *const GLuint)>;
    pub type PFNGLBINDIMAGETEXTURESPROC = Option<unsafe extern "system" fn(first: GLuint, count: GLsizei, textures: *const GLuint)>;
    pub type PFNGLBINDVERTEXBUFFERSPROC = Option<unsafe extern "system" fn(first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei)>;
    
    // GL 4.5 — DSA (Direct State Access), spirv support
    pub type PFNGLCREATETRANSFORMFEEDBACKSPROC = Option<unsafe extern "system" fn(n: GLsizei, ids: *mut GLuint)>;
    pub type PFNGLTRANSFORMFEEDBACKBUFFERBASEPROC = Option<unsafe extern "system" fn(xfb: GLuint, index: GLuint, buffer: GLuint)>;
    pub type PFNGLTRANSFORMFEEDBACKBUFFERRANGEPROC = Option<unsafe extern "system" fn(xfb: GLuint, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr)>;
    pub type PFNGLGETTRANSFORMFEEDBACKIVPROC = Option<unsafe extern "system" fn(xfb: GLuint, pname: GLenum, param: *mut GLint)>;
    pub type PFNGLGETTRANSFORMFEEDBACKI_VPROC = Option<unsafe extern "system" fn(xfb: GLuint, pname: GLenum, index: GLuint, param: *mut GLint)>;
    pub type PFNGLGETTRANSFORMFEEDBACKI64_VPROC = Option<unsafe extern "system" fn(xfb: GLuint, pname: GLenum, index: GLuint, param: *mut GLint64)>;
    pub type PFNGLCREATEBUFFERSPROC = Option<unsafe extern "system" fn(n: GLsizei, buffers: *mut GLuint)>;
    pub type PFNGLNAMEDBUFFERSTORAGEPROC = Option<unsafe extern "system" fn(buffer: GLuint, size: GLsizeiptr, data: *const GLvoid, flags: GLbitfield)>;
    pub type PFNGLNAMEDBUFFERDATAPROC = Option<unsafe extern "system" fn(buffer: GLuint, size: GLsizeiptr, data: *const GLvoid, usage: GLenum)>;
    pub type PFNGLNAMEDBUFFERSUBDATAPROC = Option<unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *const GLvoid)>;
    pub type PFNGLCOPYNAMEDBUFFERSUBDATAPROC = Option<unsafe extern "system" fn(readBuffer: GLuint, writeBuffer: GLuint, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr)>;
    pub type PFNGLCLEARNAMEDBUFFERDATAPROC = Option<unsafe extern "system" fn(buffer: GLuint, internalformat: GLenum, format: GLenum, type_: GLenum, data: *const GLvoid)>;
    pub type PFNGLCLEARNAMEDBUFFERSUBDATAPROC = Option<unsafe extern "system" fn(buffer: GLuint, internalformat: GLenum, offset: GLintptr, size: GLsizeiptr, format: GLenum, type_: GLenum, data: *const GLvoid)>;
    pub type PFNGLMAPNAMEDBUFFERPROC = Option<unsafe extern "system" fn(buffer: GLuint, access: GLenum) -> *mut GLvoid>;
    pub type PFNGLMAPNAMEDBUFFERRANGEPROC = Option<unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut GLvoid>;
    pub type PFNGLUNMAPNAMEDBUFFERPROC = Option<unsafe extern "system" fn(buffer: GLuint) -> GLboolean>;
    pub type PFNGLFLUSHMAPPEDNAMEDBUFFERRANGEPROC = Option<unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, length: GLsizeiptr)>;
    pub type PFNGLGETNAMEDBUFFERPARAMETERIVPROC = Option<unsafe extern "system" fn(buffer: GLuint, pname: GLenum, params: *mut GLint)>;
    pub type PFNGLGETNAMEDBUFFERPARAMETERI64VPROC = Option<unsafe extern "system" fn(buffer: GLuint, pname: GLenum, params: *mut GLint64)>;
    pub type PFNGLGETNAMEDBUFFERPOINTERVPROC = Option<unsafe extern "system" fn(buffer: GLuint, pname: GLenum, params: *mut *mut GLvoid)>;
    pub type PFNGLGETNAMEDBUFFERSUBDATAPROC = Option<unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *mut GLvoid)>;
    pub type PFNGLCREATEFRAMEBUFFERSPROC = Option<unsafe extern "system" fn(n: GLsizei, framebuffers: *mut GLuint)>;
    pub type PFNGLNAMEDFRAMEBUFFERRENDERBUFFERPROC = Option<unsafe extern "system" fn(framebuffer: GLuint, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint)>;
    pub type PFNGLNAMEDFRAMEBUFFERPARAMETERIPROC = Option<unsafe extern "system" fn(framebuffer: GLuint, pname: GLenum, param: GLint)>;
    pub type PFNGLNAMEDFRAMEBUFFERTEXTUREPROC = Option<unsafe extern "system" fn(framebuffer: GLuint, attachment: GLenum, texture: GLuint, level: GLint)>;
    pub type PFNGLNAMEDFRAMEBUFFERTEXTURELAYERPROC = Option<unsafe extern "system" fn(framebuffer: GLuint, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint)>;
    pub type PFNGLNAMEDFRAMEBUFFERDRAWBUFFERSPROC = Option<unsafe extern "system" fn(framebuffer: GLuint, n: GLsizei, bufs: *const GLenum)>;
    pub type PFNGLNAMEDFRAMEBUFFERREADBUFFERPROC = Option<unsafe extern "system" fn(framebuffer: GLuint, src: GLenum)>;
    pub type PFNGLINVALIDATENAMEDFRAMEBUFFERDATAPROC = Option<unsafe extern "system" fn(framebuffer: GLuint, numAttachments: GLsizei, attachments: *const GLenum)>;
    pub type PFNGLINVALIDATENAMEDFRAMEBUFFERSUBDATAPROC = Option<unsafe extern "system" fn(framebuffer: GLuint, numAttachments: GLsizei, attachments: *const GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei)>;
    pub type PFNGLCLEARNAMEDFRAMEBUFFERIVPROC = Option<unsafe extern "system" fn(framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLint)>;
    pub type PFNGLCLEARNAMEDFRAMEBUFFERUIVPROC = Option<unsafe extern "system" fn(framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLuint)>;
    pub type PFNGLCLEARNAMEDFRAMEBUFFERFVPROC = Option<unsafe extern "system" fn(framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLfloat)>;
    pub type PFNGLCLEARNAMEDFRAMEBUFFERFIPROC = Option<unsafe extern "system" fn(framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint)>;
    pub type PFNGLBLITNAMEDFRAMEBUFFERPROC = Option<unsafe extern "system" fn(readFramebuffer: GLuint, drawFramebuffer: GLuint, srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum)>;
    pub type PFNGLCHECKNAMEDFRAMEBUFFERSTATUSPROC = Option<unsafe extern "system" fn(framebuffer: GLuint, target: GLenum) -> GLenum>;
    pub type PFNGLGETNAMEDFRAMEBUFFERPARAMETERIVPROC = Option<unsafe extern "system" fn(framebuffer: GLuint, pname: GLenum, param: *mut GLint)>;
    pub type PFNGLGETNAMEDFRAMEBUFFERATTACHMENTPARAMETERIVPROC = Option<unsafe extern "system" fn(framebuffer: GLuint, attachment: GLenum, pname: GLenum, params: *mut GLint)>;
    pub type PFNGLCREATERENDERBUFFERSPROC = Option<unsafe extern "system" fn(n: GLsizei, renderbuffers: *mut GLuint)>;
    pub type PFNGLNAMEDRENDERBUFFERSTORAGEPROC = Option<unsafe extern "system" fn(renderbuffer: GLuint, internalformat: GLenum, width: GLsizei, height: GLsizei)>;
    pub type PFNGLNAMEDRENDERBUFFERSTORAGEMULTISAMPLEPROC = Option<unsafe extern "system" fn(renderbuffer: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei)>;
    pub type PFNGLGETNAMEDRENDERBUFFERPARAMETERIVPROC = Option<unsafe extern "system" fn(renderbuffer: GLuint, pname: GLenum, params: *mut GLint)>;
    pub type PFNGLCREATETEXTURESPROC = Option<unsafe extern "system" fn(target: GLenum, n: GLsizei, textures: *mut GLuint)>;
    pub type PFNGLTEXTUREBUFFERPROC = Option<unsafe extern "system" fn(texture: GLuint, internalformat: GLenum, buffer: GLuint)>;
    pub type PFNGLTEXTUREBUFFERRANGEPROC = Option<unsafe extern "system" fn(texture: GLuint, internalformat: GLenum, buffer: GLuint, offset: GLintptr, size: GLsizeiptr)>;
    pub type PFNGLTEXTURESTORAGE1DPROC = Option<unsafe extern "system" fn(texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei)>;
    pub type PFNGLTEXTURESTORAGE2DPROC = Option<unsafe extern "system" fn(texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei)>;
    pub type PFNGLTEXTURESTORAGE3DPROC = Option<unsafe extern "system" fn(texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei)>;
    pub type PFNGLTEXTURESTORAGE2DMULTISAMPLEPROC = Option<unsafe extern "system" fn(texture: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean)>;
    pub type PFNGLTEXTURESTORAGE3DMULTISAMPLEPROC = Option<unsafe extern "system" fn(texture: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean)>;
    pub type PFNGLTEXTURESUBIMAGE1DPROC = Option<unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *const GLvoid)>;
    pub type PFNGLTEXTURESUBIMAGE2DPROC = Option<unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const GLvoid)>;
    pub type PFNGLTEXTURESUBIMAGE3DPROC = Option<unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *const GLvoid)>;
    pub type PFNGLCOMPRESSEDTEXTURESUBIMAGE1DPROC = Option<unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *const GLvoid)>;
    pub type PFNGLCOMPRESSEDTEXTURESUBIMAGE2DPROC = Option<unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const GLvoid)>;
    pub type PFNGLCOMPRESSEDTEXTURESUBIMAGE3DPROC = Option<unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const GLvoid)>;
    pub type PFNGLCOPYTEXTURESUBIMAGE1DPROC = Option<unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei)>;
    pub type PFNGLCOPYTEXTURESUBIMAGE2DPROC = Option<unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei)>;
    pub type PFNGLCOPYTEXTURESUBIMAGE3DPROC = Option<unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei)>;
    pub type PFNGLTEXTUREPARAMETERFPROC = Option<unsafe extern "system" fn(texture: GLuint, pname: GLenum, param: GLfloat)>;
    pub type PFNGLTEXTUREPARAMETERFVPROC = Option<unsafe extern "system" fn(texture: GLuint, pname: GLenum, param: *const GLfloat)>;
    pub type PFNGLTEXTUREPARAMETERIPROC = Option<unsafe extern "system" fn(texture: GLuint, pname: GLenum, param: GLint)>;
    pub type PFNGLTEXTUREPARAMETERIIVPROC = Option<unsafe extern "system" fn(texture: GLuint, pname: GLenum, params: *const GLint)>;
    pub type PFNGLTEXTUREPARAMETERIUIVPROC = Option<unsafe extern "system" fn(texture: GLuint, pname: GLenum, params: *const GLuint)>;
    pub type PFNGLTEXTUREPARAMETERIVPROC = Option<unsafe extern "system" fn(texture: GLuint, pname: GLenum, param: *const GLint)>;
    pub type PFNGLGENERATETEXTUREMIPMAPPROC = Option<unsafe extern "system" fn(texture: GLenum)>;
    pub type PFNGLBINDTEXTUREUNITPROC = Option<unsafe extern "system" fn(unit: GLuint, texture: GLuint)>;
    pub type PFNGLGETTEXTUREIMAGEPROC = Option<unsafe extern "system" fn(texture: GLuint, level: GLint, format: GLenum, type_: GLenum, bufSize: GLsizei, pixels: *mut GLvoid)>;
    pub type PFNGLGETCOMPRESSEDTEXTUREIMAGEPROC = Option<unsafe extern "system" fn(texture: GLuint, level: GLint, bufSize: GLsizei, pixels: *mut GLvoid)>;
    pub type PFNGLGETTEXTURELEVELPARAMETERFVPROC = Option<unsafe extern "system" fn(texture: GLuint, level: GLint, pname: GLenum, params: *mut GLfloat)>;
    pub type PFNGLGETTEXTURELEVELPARAMETERIVPROC = Option<unsafe extern "system" fn(texture: GLuint, level: GLint, pname: GLenum, params: *mut GLint)>;
    pub type PFNGLGETTEXTUREPARAMETERFVPROC = Option<unsafe extern "system" fn(texture: GLuint, pname: GLenum, params: *mut GLfloat)>;
    pub type PFNGLGETTEXTUREPARAMETERIIVPROC = Option<unsafe extern "system" fn(texture: GLuint, pname: GLenum, params: *mut GLint)>;
    pub type PFNGLGETTEXTUREPARAMETERIUIVPROC = Option<unsafe extern "system" fn(texture: GLuint, pname: GLenum, params: *mut GLuint)>;
    pub type PFNGLGETTEXTUREPARAMETERIVPROC = Option<unsafe extern "system" fn(texture: GLuint, pname: GLenum, params: *mut GLint)>;
    pub type PFNGLCREATEVERTEXARRAYSPROC = Option<unsafe extern "system" fn(n: GLsizei, arrays: *mut GLuint)>;
    pub type PFNGLDISABLEVERTEXARRAYATTRIBPROC = Option<unsafe extern "system" fn(vaobj: GLuint, index: GLuint)>;
    pub type PFNGLENABLEVERTEXARRAYATTRIBPROC = Option<unsafe extern "system" fn(vaobj: GLuint, index: GLuint)>;
    pub type PFNGLVERTEXARRAYELEMENTBUFFERPROC = Option<unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint)>;
    pub type PFNGLVERTEXARRAYVERTEXBUFFERPROC = Option<unsafe extern "system" fn(vaobj: GLuint, bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei)>;
    pub type PFNGLVERTEXARRAYVERTEXBUFFERSPROC = Option<unsafe extern "system" fn(vaobj: GLuint, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei)>;
    pub type PFNGLVERTEXARRAYATTRIBBINDINGPROC = Option<unsafe extern "system" fn(vaobj: GLuint, attribindex: GLuint, bindingindex: GLuint)>;
    pub type PFNGLVERTEXARRAYATTRIBFORMATPROC = Option<unsafe extern "system" fn(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, relativeoffset: GLuint)>;
    pub type PFNGLVERTEXARRAYATTRIBIFORMATPROC = Option<unsafe extern "system" fn(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint)>;
    pub type PFNGLVERTEXARRAYATTRIBLFORMATPROC = Option<unsafe extern "system" fn(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint)>;
    pub type PFNGLVERTEXARRAYBINDINGDIVISORPROC = Option<unsafe extern "system" fn(vaobj: GLuint, bindingindex: GLuint, divisor: GLuint)>;
    pub type PFNGLGETVERTEXARRAYIVPROC = Option<unsafe extern "system" fn(vaobj: GLuint, pname: GLenum, param: *mut GLint)>;
    pub type PFNGLGETVERTEXARRAYINDEXEDIVPROC = Option<unsafe extern "system" fn(vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut GLint)>;
    pub type PFNGLGETVERTEXARRAYINDEXED64IVPROC = Option<unsafe extern "system" fn(vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut GLint64)>;
    pub type PFNGLCREATESAMPLERSPROC = Option<unsafe extern "system" fn(n: GLsizei, samplers: *mut GLuint)>;
    pub type PFNGLCREATEPROGRAMPIPELINESPROC = Option<unsafe extern "system" fn(n: GLsizei, pipelines: *mut GLuint)>;
    pub type PFNGLCREATEQUERIESPROC = Option<unsafe extern "system" fn(target: GLenum, n: GLsizei, ids: *mut GLuint)>;
    pub type PFNGLGETQUERYBUFFEROBJECTI64VPROC = Option<unsafe extern "system" fn(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr)>;
    pub type PFNGLGETQUERYBUFFEROBJECTIVPROC = Option<unsafe extern "system" fn(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr)>;
    pub type PFNGLGETQUERYBUFFEROBJECTUI64VPROC = Option<unsafe extern "system" fn(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr)>;
    pub type PFNGLGETQUERYBUFFEROBJECTUIVPROC = Option<unsafe extern "system" fn(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr)>;
    pub type PFNGLMEMORYBARRIERBYREGIONPROC = Option<unsafe extern "system" fn(barriers: GLbitfield)>;
    pub type PFNGLGETGRAPHICSRESETSTATUSPROC = Option<unsafe extern "system" fn() -> GLenum>;
    pub type PFNGLGETNCOMPRESSEDTEXIMAGEPROC = Option<unsafe extern "system" fn(target: GLenum, lod: GLint, bufSize: GLsizei, pixels: *mut GLvoid)>;
    pub type PFNGLGETNTEXIMAGEPROC = Option<unsafe extern "system" fn(target: GLenum, level: GLint, format: GLenum, type_: GLenum, bufSize: GLsizei, pixels: *mut GLvoid)>;
    pub type PFNGLGETNUNIFORMDVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLdouble)>;
    pub type PFNGLGETNUNIFORMFVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLfloat)>;
    pub type PFNGLGETNUNIFORMIVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLint)>;
    pub type PFNGLGETNUNIFORMUIVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLuint)>;
    pub type PFNGLREADNPIXELSPROC = Option<unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, bufSize: GLsizei, data: *mut GLvoid)>;
    pub type PFNGLTEXTUREBARRIERPROC = Option<unsafe extern "system" fn()>;
    
    // GL 4.6 — SPIR-V support, polygon offset clamp
    pub type PFNGLSPECIALIZESHADERPROC = Option<unsafe extern "system" fn(shader: GLuint, pEntryPoint: *const GLchar, numSpecializationConstants: GLuint, pConstantIndex: *const GLuint, pConstantValue: *const GLuint)>;
    pub type PFNGLPOLYGONOFFSETCLAMPPROC = Option<unsafe extern "system" fn(factor: GLfloat, units: GLfloat, clamp: GLfloat)>;
}

// ============================================================================
// SECTION 5: Constantes Consolidadas OpenGL
// ============================================================================

pub mod constants {
    //! Todas las constantes OpenGL de GL 1.0 a 4.6 consolidadas
    
    // === GL_VERSION ===
    pub const GL_VERSION_1_0: u32 = 1;
    pub const GL_VERSION_1_1: u32 = 1;
    pub const GL_VERSION_1_2: u32 = 1;
    pub const GL_VERSION_1_3: u32 = 1;
    pub const GL_VERSION_1_4: u32 = 1;
    pub const GL_VERSION_1_5: u32 = 1;
    pub const GL_VERSION_2_0: u32 = 1;
    pub const GL_VERSION_2_1: u32 = 1;
    pub const GL_VERSION_3_0: u32 = 1;
    pub const GL_VERSION_3_1: u32 = 1;
    pub const GL_VERSION_3_2: u32 = 1;
    pub const GL_VERSION_3_3: u32 = 1;
    pub const GL_VERSION_4_0: u32 = 1;
    pub const GL_VERSION_4_1: u32 = 1;
    pub const GL_VERSION_4_2: u32 = 1;
    pub const GL_VERSION_4_3: u32 = 1;
    pub const GL_VERSION_4_4: u32 = 1;
    pub const GL_VERSION_4_5: u32 = 1;
    pub const GL_VERSION_4_6: u32 = 1;
    
    // === GL 1.0: Errors ===
    pub const GL_NO_ERROR: u32 = 0;
    pub const GL_INVALID_ENUM: u32 = 0x0500;
    pub const GL_INVALID_VALUE: u32 = 0x0501;
    pub const GL_INVALID_OPERATION: u32 = 0x0502;
    pub const GL_STACK_OVERFLOW: u32 = 0x0503;
    pub const GL_STACK_UNDERFLOW: u32 = 0x0504;
    pub const GL_OUT_OF_MEMORY: u32 = 0x0505;
    pub const GL_INVALID_FRAMEBUFFER_OPERATION: u32 = 0x0506;
    pub const GL_CONTEXT_LOST: u32 = 0x0507;
    
    // === GL 1.0: Types ===
    pub const GL_BYTE: u32 = 0x1400;
    pub const GL_UNSIGNED_BYTE: u32 = 0x1401;
    pub const GL_SHORT: u32 = 0x1402;
    pub const GL_UNSIGNED_SHORT: u32 = 0x1403;
    pub const GL_INT: u32 = 0x1404;
    pub const GL_UNSIGNED_INT: u32 = 0x1405;
    pub const GL_FLOAT: u32 = 0x1406;
    pub const GL_DOUBLE: u32 = 0x140A;
    pub const GL_HALF_FLOAT: u32 = 0x140B;
    pub const GL_FIXED: u32 = 0x140C;
    
    // === GL 1.0: Primitives ===
    pub const GL_POINTS: u32 = 0x0000;
    pub const GL_LINES: u32 = 0x0001;
    pub const GL_LINE_LOOP: u32 = 0x0002;
    pub const GL_LINE_STRIP: u32 = 0x0003;
    pub const GL_TRIANGLES: u32 = 0x0004;
    pub const GL_TRIANGLE_STRIP: u32 = 0x0005;
    pub const GL_TRIANGLE_FAN: u32 = 0x0006;
    pub const GL_QUADS: u32 = 0x0007;
    pub const GL_QUAD_STRIP: u32 = 0x0008;
    pub const GL_POLYGON: u32 = 0x0009;
    pub const GL_LINES_ADJACENCY: u32 = 0x000A;
    pub const GL_LINE_STRIP_ADJACENCY: u32 = 0x000B;
    pub const GL_TRIANGLES_ADJACENCY: u32 = 0x000C;
    pub const GL_TRIANGLE_STRIP_ADJACENCY: u32 = 0x000D;
    pub const GL_PATCHES: u32 = 0x000E;
    
    // === GL 1.0: Blending ===
    pub const GL_ZERO: u32 = 0;
    pub const GL_ONE: u32 = 1;
    pub const GL_SRC_COLOR: u32 = 0x0300;
    pub const GL_ONE_MINUS_SRC_COLOR: u32 = 0x0301;
    pub const GL_SRC_ALPHA: u32 = 0x0302;
    pub const GL_ONE_MINUS_SRC_ALPHA: u32 = 0x0303;
    pub const GL_DST_ALPHA: u32 = 0x0304;
    pub const GL_ONE_MINUS_DST_ALPHA: u32 = 0x0305;
    pub const GL_DST_COLOR: u32 = 0x0306;
    pub const GL_ONE_MINUS_DST_COLOR: u32 = 0x0307;
    pub const GL_SRC_ALPHA_SATURATE: u32 = 0x0308;
    pub const GL_CONSTANT_COLOR: u32 = 0x8001;
    pub const GL_ONE_MINUS_CONSTANT_COLOR: u32 = 0x8002;
    pub const GL_CONSTANT_ALPHA: u32 = 0x8003;
    pub const GL_ONE_MINUS_CONSTANT_ALPHA: u32 = 0x8004;
    
    // === GL 1.0: Buffers ===
    pub const GL_NONE: u32 = 0;
    pub const GL_FRONT_LEFT: u32 = 0x0400;
    pub const GL_FRONT_RIGHT: u32 = 0x0401;
    pub const GL_BACK_LEFT: u32 = 0x0402;
    pub const GL_BACK_RIGHT: u32 = 0x0403;
    pub const GL_FRONT: u32 = 0x0404;
    pub const GL_BACK: u32 = 0x0405;
    pub const GL_LEFT: u32 = 0x0406;
    pub const GL_RIGHT: u32 = 0x0407;
    pub const GL_FRONT_AND_BACK: u32 = 0x0408;
    
    // === GL 1.0: Depth Test ===
    pub const GL_NEVER: u32 = 0x0200;
    pub const GL_LESS: u32 = 0x0201;
    pub const GL_EQUAL: u32 = 0x0202;
    pub const GL_LEQUAL: u32 = 0x0203;
    pub const GL_GREATER: u32 = 0x0204;
    pub const GL_NOTEQUAL: u32 = 0x0205;
    pub const GL_GEQUAL: u32 = 0x0206;
    pub const GL_ALWAYS: u32 = 0x0207;
    
    // === GL 1.1: Capabilities ===
    pub const GL_DEPTH_TEST: u32 = 0x0B71;
    pub const GL_BLEND: u32 = 0x0BE2;
    pub const GL_CULL_FACE: u32 = 0x0B44;
    pub const GL_SCISSOR_TEST: u32 = 0x0C11;
    pub const GL_STENCIL_TEST: u32 = 0x0B90;
    pub const GL_DITHER: u32 = 0x0BD0;
    pub const GL_POLYGON_OFFSET_FILL: u32 = 0x8037;
    pub const GL_POLYGON_OFFSET_LINE: u32 = 0x2A02;
    pub const GL_POLYGON_OFFSET_POINT: u32 = 0x2A01;
    
    // === GL 1.3: Texture Units ===
    pub const GL_TEXTURE0: u32 = 0x84C0;
    pub const GL_TEXTURE1: u32 = 0x84C1;
    pub const GL_TEXTURE2: u32 = 0x84C2;
    pub const GL_TEXTURE3: u32 = 0x84C3;
    pub const GL_TEXTURE4: u32 = 0x84C4;
    pub const GL_TEXTURE5: u32 = 0x84C5;
    pub const GL_TEXTURE6: u32 = 0x84C6;
    pub const GL_TEXTURE7: u32 = 0x84C7;
    pub const GL_TEXTURE8: u32 = 0x84C8;
    pub const GL_TEXTURE9: u32 = 0x84C9;
    pub const GL_TEXTURE10: u32 = 0x84CA;
    pub const GL_TEXTURE11: u32 = 0x84CB;
    pub const GL_TEXTURE12: u32 = 0x84CC;
    pub const GL_TEXTURE13: u32 = 0x84CD;
    pub const GL_TEXTURE14: u32 = 0x84CE;
    pub const GL_TEXTURE15: u32 = 0x84CF;
    pub const GL_TEXTURE16: u32 = 0x84D0;
    pub const GL_TEXTURE17: u32 = 0x84D1;
    pub const GL_TEXTURE18: u32 = 0x84D2;
    pub const GL_TEXTURE19: u32 = 0x84D3;
    pub const GL_TEXTURE20: u32 = 0x84D4;
    pub const GL_TEXTURE21: u32 = 0x84D5;
    pub const GL_TEXTURE22: u32 = 0x84D6;
    pub const GL_TEXTURE23: u32 = 0x84D7;
    pub const GL_TEXTURE24: u32 = 0x84D8;
    pub const GL_TEXTURE25: u32 = 0x84D9;
    pub const GL_TEXTURE26: u32 = 0x84DA;
    pub const GL_TEXTURE27: u32 = 0x84DB;
    pub const GL_TEXTURE28: u32 = 0x84DC;
    pub const GL_TEXTURE29: u32 = 0x84DD;
    pub const GL_TEXTURE30: u32 = 0x84DE;
    pub const GL_TEXTURE31: u32 = 0x84DF;
    pub const GL_ACTIVE_TEXTURE: u32 = 0x84E0;
    
    // === GL 2.0: Shader Types ===
    pub const GL_FRAGMENT_SHADER: u32 = 0x8B30;
    pub const GL_VERTEX_SHADER: u32 = 0x8B31;
    pub const GL_GEOMETRY_SHADER: u32 = 0x8DD9;
    pub const GL_TESS_CONTROL_SHADER: u32 = 0x8E88;
    pub const GL_TESS_EVALUATION_SHADER: u32 = 0x8E87;
    pub const GL_COMPUTE_SHADER: u32 = 0x91B9;
    
    // === GL 2.0: Shader Status ===
    pub const GL_COMPILE_STATUS: u32 = 0x8B81;
    pub const GL_LINK_STATUS: u32 = 0x8B82;
    pub const GL_VALIDATE_STATUS: u32 = 0x8B83;
    pub const GL_INFO_LOG_LENGTH: u32 = 0x8B84;
    pub const GL_SHADER_SOURCE_LENGTH: u32 = 0x8B88;
    
    // === GL 3.0: Framebuffers ===
    pub const GL_FRAMEBUFFER: u32 = 0x8D40;
    pub const GL_RENDERBUFFER: u32 = 0x8D41;
    pub const GL_COLOR_ATTACHMENT0: u32 = 0x8CE0;
    pub const GL_COLOR_ATTACHMENT1: u32 = 0x8CE1;
    pub const GL_COLOR_ATTACHMENT2: u32 = 0x8CE2;
    pub const GL_COLOR_ATTACHMENT3: u32 = 0x8CE3;
    pub const GL_COLOR_ATTACHMENT4: u32 = 0x8CE4;
    pub const GL_COLOR_ATTACHMENT5: u32 = 0x8CE5;
    pub const GL_COLOR_ATTACHMENT6: u32 = 0x8CE6;
    pub const GL_COLOR_ATTACHMENT7: u32 = 0x8CE7;
    pub const GL_DEPTH_ATTACHMENT: u32 = 0x8D00;
    pub const GL_STENCIL_ATTACHMENT: u32 = 0x8D20;
    pub const GL_DEPTH_STENCIL_ATTACHMENT: u32 = 0x821A;
    pub const GL_FRAMEBUFFER_COMPLETE: u32 = 0x8CD5;
    
    // === GL 3.2: Context Flags ===
    pub const GL_CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT: u32 = 0x00000001;
    pub const GL_CONTEXT_FLAG_DEBUG_BIT: u32 = 0x00000002;
    pub const GL_CONTEXT_FLAG_ROBUST_ACCESS_BIT: u32 = 0x00000004;
    pub const GL_CONTEXT_FLAG_NO_ERROR_BIT: u32 = 0x00000008;
    
    // === GL 4.0: Tessellation ===
    pub const GL_PATCH_VERTICES: u32 = 0x8E72;
    pub const GL_PATCH_DEFAULT_INNER_LEVEL: u32 = 0x8E73;
    pub const GL_PATCH_DEFAULT_OUTER_LEVEL: u32 = 0x8E74;
    
    // === GL 4.3: Compute ===
    pub const GL_COMPUTE_WORK_GROUP_SIZE: u32 = 0x8267;
    pub const GL_MAX_COMPUTE_WORK_GROUP_INVOCATIONS: u32 = 0x90EB;
    pub const GL_MAX_COMPUTE_WORK_GROUP_COUNT: u32 = 0x91BE;
    pub const GL_MAX_COMPUTE_WORK_GROUP_SIZE: u32 = 0x91BF;
    
    // === GL 4.6: SPIR-V ===
    pub const GL_SHADER_BINARY_FORMAT_SPIR_V: u32 = 0x9551;
    pub const GL_SPIR_V_BINARY: u32 = 0x9552;
}

