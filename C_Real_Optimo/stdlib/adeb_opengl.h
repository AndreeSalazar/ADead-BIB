/* adeb_opengl.h — OpenGL 1.0 a 4.6 (proxy a runtime/opengl) */
#ifndef ADEB_OPENGL_H
#define ADEB_OPENGL_H

#include "adeb_types.h"

typedef uint32_t GLenum;
typedef uint32_t GLbitfield;
typedef uint32_t GLuint;
typedef int32_t  GLint;
typedef int32_t  GLsizei;
typedef uint8_t  GLboolean;
typedef int8_t   GLbyte;
typedef int16_t  GLshort;
typedef float    GLfloat;
typedef double   GLdouble;

#define GL_COLOR_BUFFER_BIT 0x00004000
#define GL_DEPTH_BUFFER_BIT 0x00000100
#define GL_TRIANGLES        0x0004
#define GL_QUADS            0x0007
#define GL_LINES            0x0001

extern void glClear(GLbitfield mask);
extern void glClearColor(GLfloat r, GLfloat g, GLfloat b, GLfloat a);
extern void glViewport(GLint x, GLint y, GLsizei w, GLsizei h);
extern void glDrawArrays(GLenum mode, GLint first, GLsizei count);
extern void glDrawElements(GLenum mode, GLsizei count, GLenum type, const void* indices);
extern void glGenBuffers(GLsizei n, GLuint* bufs);
extern void glBindBuffer(GLenum target, GLuint buf);
extern void glBufferData(GLenum target, intptr_t size, const void* data, GLenum usage);
extern GLuint glCreateShader(GLenum type);
extern void glShaderSource(GLuint sh, GLsizei count, const char* const* str, const GLint* len);
extern void glCompileShader(GLuint sh);
extern GLuint glCreateProgram(void);
extern void glAttachShader(GLuint prog, GLuint sh);
extern void glLinkProgram(GLuint prog);
extern void glUseProgram(GLuint prog);

#endif /* ADEB_OPENGL_H */
