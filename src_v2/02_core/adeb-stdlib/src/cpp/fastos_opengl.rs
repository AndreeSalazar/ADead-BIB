pub fn generate_opengl_hpp() -> String {
r#"// ad_opengl.hpp — ADead-BIB OpenGL Header
// OpenGL 1.0–1.1 + WGL — Sin dependencias externas
// Autor: Eddi Andreé Salazar Matos — Marzo 2026

#ifndef ADEAD_OPENGL_H
#define ADEAD_OPENGL_H

// ── Tipos básicos OpenGL ──
typedef unsigned int   GLenum;
typedef unsigned char  GLboolean;
typedef unsigned int   GLbitfield;
typedef void           GLvoid;
typedef int            GLint;
typedef unsigned int   GLuint;
typedef int            GLsizei;
typedef float          GLfloat;
typedef double         GLdouble;
typedef unsigned char  GLubyte;

typedef void* HGLRC;
typedef void* HDC;

#define GL_TRUE  1
#define GL_FALSE 0

// ── Constantes ──
#define GL_POINTS         0x0000
#define GL_LINES          0x0001
#define GL_TRIANGLES      0x0004
#define GL_TRIANGLE_STRIP 0x0005
#define GL_QUADS          0x0007

#define GL_MODELVIEW      0x1700
#define GL_PROJECTION     0x1701

#define GL_DEPTH_BUFFER_BIT   0x00000100
#define GL_COLOR_BUFFER_BIT   0x00004000

#define GL_DEPTH_TEST   0x0B71
#define GL_CULL_FACE    0x0B44
#define GL_BLEND        0x0BE2
#define GL_LIGHTING     0x0B50
#define GL_LIGHT0       0x4000
#define GL_TEXTURE_2D   0x0DE1

#define GL_LESS       0x0201
#define GL_LEQUAL     0x0203

#define GL_FLAT   0x1D00
#define GL_SMOOTH 0x1D01

#define GL_FLOAT          0x1406
#define GL_UNSIGNED_INT   0x1405

#define GL_VENDOR     0x1F00
#define GL_RENDERER   0x1F01
#define GL_VERSION    0x1F02

#define GL_FRONT      0x0404
#define GL_BACK       0x0405

#define GL_SRC_ALPHA           0x0302
#define GL_ONE_MINUS_SRC_ALPHA 0x0303

#define GL_FRAGMENT_SHADER 0x8B30
#define GL_VERTEX_SHADER   0x8B31
#define GL_COMPILE_STATUS  0x8B81
#define GL_LINK_STATUS     0x8B82

// ── Funciones OpenGL 1.0–1.1 — extern desde opengl32.dll ──
extern "C" {
    void glEnable(GLenum cap);
    void glDisable(GLenum cap);
    void glClearColor(GLfloat r, GLfloat g, GLfloat b, GLfloat a);
    void glClear(GLbitfield mask);
    void glViewport(GLint x, GLint y, GLsizei w, GLsizei h);
    void glDepthFunc(GLenum func);
    void glBlendFunc(GLenum sfactor, GLenum dfactor);
    void glCullFace(GLenum mode);
    void glFrontFace(GLenum mode);
    GLenum glGetError();
    const GLubyte* glGetString(GLenum name);
    void glFlush();
    void glFinish();
    void glDepthMask(GLboolean flag);
    void glHint(GLenum target, GLenum mode);
    void glShadeModel(GLenum mode);

    void glMatrixMode(GLenum mode);
    void glLoadIdentity();
    void glPushMatrix();
    void glPopMatrix();
    void glTranslatef(GLfloat x, GLfloat y, GLfloat z);
    void glRotatef(GLfloat angle, GLfloat x, GLfloat y, GLfloat z);
    void glScalef(GLfloat x, GLfloat y, GLfloat z);
    void glMultMatrixf(const GLfloat* m);

    void glBegin(GLenum mode);
    void glEnd();
    void glVertex3f(GLfloat x, GLfloat y, GLfloat z);
    void glVertex2f(GLfloat x, GLfloat y);
    void glColor3f(GLfloat r, GLfloat g, GLfloat b);
    void glColor4f(GLfloat r, GLfloat g, GLfloat b, GLfloat a);
    void glNormal3f(GLfloat x, GLfloat y, GLfloat z);

    // WGL
    HGLRC wglCreateContext(HDC hdc);
    int   wglDeleteContext(HGLRC hglrc);
    int   wglMakeCurrent(HDC hdc, HGLRC hglrc);

    // GDI pixel format
    int ChoosePixelFormat(HDC hdc, const void* ppfd);
    int SetPixelFormat(HDC hdc, int fmt, const void* ppfd);
    int SwapBuffers(HDC hdc);
    HDC GetDC(void* hwnd);
    int ReleaseDC(void* hwnd, HDC hdc);
}

// ── PFD flags ──
#define PFD_DRAW_TO_WINDOW  0x00000004
#define PFD_SUPPORT_OPENGL  0x00000020
#define PFD_DOUBLEBUFFER    0x00000001
#define PFD_TYPE_RGBA       0

#endif
"#.
    to_string()
}
