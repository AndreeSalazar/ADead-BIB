// Test: Production OpenGL — header parsing + type verification
// Expected: Compile OK — OpenGL types, constants, and functions recognized
// Strict: Verifies GL types, constant values, function declarations

#include <GL/gl.h>
#include <GL/glext.h>
#include <stdio.h>

// Verify GL type sizes
int test_gl_types() {
    int pass = 0;
    if (sizeof(GLenum) == 4) pass++;
    if (sizeof(GLuint) == 4) pass++;
    if (sizeof(GLint) == 4) pass++;
    if (sizeof(GLfloat) == 4) pass++;
    if (sizeof(GLboolean) == 1) pass++;
    if (sizeof(GLsizei) == 4) pass++;
    return pass;
}

// Verify GL constants
int test_gl_constants() {
    int pass = 0;
    if (GL_TRIANGLES == 0x0004) pass++;
    if (GL_FLOAT == 0x1406) pass++;
    if (GL_FALSE == 0) pass++;
    if (GL_TRUE == 1) pass++;
    if (GL_DEPTH_TEST == 0x0B71) pass++;
    if (GL_BLEND == 0x0BE2) pass++;
    if (GL_COLOR_BUFFER_BIT == 0x00004000) pass++;
    if (GL_DEPTH_BUFFER_BIT == 0x00000100) pass++;
    if (GL_ARRAY_BUFFER == 0x8892) pass++;
    if (GL_STATIC_DRAW == 0x88E4) pass++;
    if (GL_VERTEX_SHADER == 0x8B31) pass++;
    if (GL_FRAGMENT_SHADER == 0x8B30) pass++;
    if (GL_COMPILE_STATUS == 0x8B81) pass++;
    if (GL_LINK_STATUS == 0x8B82) pass++;
    if (GL_FRAMEBUFFER == 0x8D40) pass++;
    if (GL_FRAMEBUFFER_COMPLETE == 0x8CD5) pass++;
    if (GL_NO_ERROR == 0) pass++;
    if (GL_RGBA == 0x1908) pass++;
    return pass;
}

// Typical OpenGL init pattern (parse test — functions declared correctly)
void demo_gl_pipeline(void) {
    // This function tests that all GL function declarations parse correctly
    // It would crash if actually called (no GL context), but it compiles ✅

    // State setup
    glEnable(GL_DEPTH_TEST);
    glEnable(GL_BLEND);
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    glCullFace(GL_BACK);
    glFrontFace(GL_CCW);
    glDepthFunc(GL_LESS);
    glClearColor(0.1f, 0.1f, 0.1f, 1.0f);
    glViewport(0, 0, 800, 600);

    // Buffer operations
    GLuint vao, vbo, ebo;
    glGenVertexArrays(1, &vao);
    glGenBuffers(1, &vbo);
    glGenBuffers(1, &ebo);
    glBindVertexArray(vao);
    glBindBuffer(GL_ARRAY_BUFFER, vbo);
    glEnableVertexAttribArray(0);
    glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 0, 0);

    // Shader operations
    GLuint vs = glCreateShader(GL_VERTEX_SHADER);
    GLuint fs = glCreateShader(GL_FRAGMENT_SHADER);
    GLuint prog = glCreateProgram();
    glAttachShader(prog, vs);
    glAttachShader(prog, fs);
    glLinkProgram(prog);
    glUseProgram(prog);

    // Uniforms
    GLint loc = glGetUniformLocation(prog, "uMVP");
    glUniform1i(loc, 0);
    glUniform3f(loc, 1.0f, 2.0f, 3.0f);

    // Draw
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
    glDrawArrays(GL_TRIANGLES, 0, 36);
    glDrawElements(GL_TRIANGLES, 36, GL_UNSIGNED_INT, 0);

    // Framebuffer
    GLuint fbo;
    glGenFramebuffers(1, &fbo);
    glBindFramebuffer(GL_FRAMEBUFFER, fbo);

    // Cleanup
    glDeleteShader(vs);
    glDeleteShader(fs);
    glDeleteProgram(prog);
    glDeleteBuffers(1, &vbo);
    glDeleteBuffers(1, &ebo);
    glDeleteVertexArrays(1, &vao);
    glDeleteFramebuffers(1, &fbo);
}

int main() {
    printf("=== PRODUCTION: OpenGL Headers ===\n");

    int types = test_gl_types();
    int consts = test_gl_constants();
    printf("GL types: %d/6 passed\n", types);
    printf("GL constants: %d/18 passed\n", consts);
    printf("GL pipeline: compiles OK (all functions declared)\n");

    int total = types + consts;
    int expected = 6 + 18;
    printf("Results: %d/%d passed\n", total, expected);
    printf("=== PRODUCTION: OpenGL %s ===\n", total == expected ? "PASS" : "FAIL");
    return total == expected ? 0 : 1;
}
