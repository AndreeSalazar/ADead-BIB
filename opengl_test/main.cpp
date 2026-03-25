// main.cpp — ADead-BIB OpenGL Cube v3
// Cubo 3D con SHADERS reales — GL 2.0+ pipeline completo
// Compilar: cargo run --bin adb -- cxx opengl_test/main.cpp -o opengl_cube.exe
//
// Autor: Eddi Andreé Salazar Matos — Marzo 2026
// v3: Shaders GLSL embebidos, compilación+linkeo en runtime,
//     iluminación Phong real, VBO con vertex data,
//     matrices manuales 4x4, todo via ADead-BIB

extern "C" {
int printf(const char *, ...);
void *malloc(unsigned long long);
void free(void *);
void *memset(void *, int, unsigned long long);
void *memcpy(void *, const void *, unsigned long long);
int strlen(const char *);

// Win32
void *GetModuleHandleA(const char *name);
unsigned short RegisterClassA(const void *wc);
void *CreateWindowExA(unsigned long dwExStyle, const char *cls,
                      const char *title, unsigned long dwStyle, int x, int y,
                      int w, int h, void *parent, void *menu, void *hInst,
                      void *param);
int ShowWindow(void *hwnd, int cmd);
int PeekMessageA(void *msg, void *hwnd, unsigned int min, unsigned int max,
                 unsigned int rm);
int TranslateMessage(const void *msg);
long long DispatchMessageA(const void *msg);
void PostQuitMessage(int code);
long long DefWindowProcA(void *hwnd, unsigned int msg, unsigned long long wp,
                         long long lp);
int DestroyWindow(void *hwnd);
void Sleep(unsigned int ms);
void *LoadLibraryA(const char *name);
void *GetProcAddress(void *hmod, const char *name);

// GDI
void *GetDC(void *hwnd);
int ReleaseDC(void *hwnd, void *hdc);
int SwapBuffers(void *hdc);
int ChoosePixelFormat(void *hdc, const void *pfd);
int SetPixelFormat(void *hdc, int fmt, const void *pfd);

// WGL
void *wglCreateContext(void *hdc);
int wglMakeCurrent(void *hdc, void *hglrc);
int wglDeleteContext(void *hglrc);
void *wglGetProcAddress(const char *name);

// OpenGL 1.1 — opengl32.dll exports
void glClear(unsigned int mask);
void glClearColor(float r, float g, float b, float a);
void glEnable(unsigned int cap);
void glDisable(unsigned int cap);
void glDepthFunc(unsigned int func);
void glViewport(int x, int y, int w, int h);
void glFlush();
void glDrawArrays(unsigned int mode, int first, int count);
const char *glGetString(unsigned int name);
unsigned int glGetError();
}

// ── Constantes ──
unsigned int WM_QUIT = 18;
unsigned int GL_COLOR_BUFFER_BIT = 0x00004000;
unsigned int GL_DEPTH_BUFFER_BIT = 0x00000100;
unsigned int GL_DEPTH_TEST = 0x0B71;
unsigned int GL_LEQUAL = 0x0203;
unsigned int GL_TRIANGLES = 0x0004;
unsigned int GL_FLOAT = 0x1406;
unsigned int GL_FALSE = 0;
unsigned int GL_TRUE = 1;
unsigned int GL_VENDOR = 0x1F00;
unsigned int GL_RENDERER = 0x1F01;
unsigned int GL_VERSION = 0x1F02;
unsigned int GL_NO_ERROR = 0;
unsigned int GL_VERTEX_SHADER = 0x8B31;
unsigned int GL_FRAGMENT_SHADER = 0x8B30;
unsigned int GL_COMPILE_STATUS = 0x8B81;
unsigned int GL_LINK_STATUS = 0x8B82;
unsigned int GL_INFO_LOG_LENGTH = 0x8B84;
unsigned int GL_ARRAY_BUFFER = 0x8889;
unsigned int GL_STATIC_DRAW = 0x88E4;

// ── GL 2.0+ function pointer types ──
typedef unsigned int (*PFN_glCreateShader)(unsigned int);
typedef unsigned int (*PFN_glCreateProgram)(void);
typedef void (*PFN_glShaderSource)(unsigned int, int, const char **, const int *);
typedef void (*PFN_glCompileShader)(unsigned int);
typedef void (*PFN_glAttachShader)(unsigned int, unsigned int);
typedef void (*PFN_glLinkProgram)(unsigned int);
typedef void (*PFN_glUseProgram)(unsigned int);
typedef void (*PFN_glDeleteShader)(unsigned int);
typedef void (*PFN_glGetShaderiv)(unsigned int, unsigned int, int *);
typedef void (*PFN_glGetProgramiv)(unsigned int, unsigned int, int *);
typedef void (*PFN_glGetShaderInfoLog)(unsigned int, int, int *, char *);
typedef void (*PFN_glGetProgramInfoLog)(unsigned int, int, int *, char *);
typedef int  (*PFN_glGetUniformLocation)(unsigned int, const char *);
typedef void (*PFN_glUniform3f)(int, float, float, float);
typedef void (*PFN_glUniformMatrix4fv)(int, int, unsigned char, const float *);
// VBO/VAO
typedef void (*PFN_glGenBuffers)(int, unsigned int *);
typedef void (*PFN_glBindBuffer)(unsigned int, unsigned int);
typedef void (*PFN_glBufferData)(unsigned int, long long, const void *, unsigned int);
typedef void (*PFN_glGenVertexArrays)(int, unsigned int *);
typedef void (*PFN_glBindVertexArray)(unsigned int);
typedef void (*PFN_glEnableVertexAttribArray)(unsigned int);
typedef void (*PFN_glVertexAttribPointer)(unsigned int, int, unsigned int, unsigned char, int, const void *);
typedef void (*PFN_glDeleteBuffers)(int, const unsigned int *);
typedef void (*PFN_glDeleteVertexArrays)(int, const unsigned int *);
typedef void (*PFN_glDeleteProgram)(unsigned int);

// ── Function pointers ──
PFN_glCreateShader fn_glCreateShader = nullptr;
PFN_glCreateProgram fn_glCreateProgram = nullptr;
PFN_glShaderSource fn_glShaderSource = nullptr;
PFN_glCompileShader fn_glCompileShader = nullptr;
PFN_glAttachShader fn_glAttachShader = nullptr;
PFN_glLinkProgram fn_glLinkProgram = nullptr;
PFN_glUseProgram fn_glUseProgram = nullptr;
PFN_glDeleteShader fn_glDeleteShader = nullptr;
PFN_glGetShaderiv fn_glGetShaderiv = nullptr;
PFN_glGetProgramiv fn_glGetProgramiv = nullptr;
PFN_glGetShaderInfoLog fn_glGetShaderInfoLog = nullptr;
PFN_glGetProgramInfoLog fn_glGetProgramInfoLog = nullptr;
PFN_glGetUniformLocation fn_glGetUniformLocation = nullptr;
PFN_glUniform3f fn_glUniform3f = nullptr;
PFN_glUniformMatrix4fv fn_glUniformMatrix4fv = nullptr;
PFN_glGenBuffers fn_glGenBuffers = nullptr;
PFN_glBindBuffer fn_glBindBuffer = nullptr;
PFN_glBufferData fn_glBufferData = nullptr;
PFN_glGenVertexArrays fn_glGenVertexArrays = nullptr;
PFN_glBindVertexArray fn_glBindVertexArray = nullptr;
PFN_glEnableVertexAttribArray fn_glEnableVertexAttribArray = nullptr;
PFN_glVertexAttribPointer fn_glVertexAttribPointer = nullptr;
PFN_glDeleteBuffers fn_glDeleteBuffers = nullptr;
PFN_glDeleteVertexArrays fn_glDeleteVertexArrays = nullptr;
PFN_glDeleteProgram fn_glDeleteProgram = nullptr;

// ── Estado global ──
float angleY = 0.0f;
float angleX = 25.0f;
void *g_glrc = nullptr;
void *g_hdc = nullptr;
void *g_hwnd = nullptr;
int g_gl20 = 0;
unsigned int g_program = 0;
unsigned int g_vao = 0;
unsigned int g_vbo = 0;

// ── Shaders GLSL embebidos ──
// Vertex: transforma posición, pasa color y normal al fragment
const char *vertexShaderSrc =
  "#version 130\n"
  "in vec3 aPosition;\n"
  "in vec3 aColor;\n"
  "in vec3 aNormal;\n"
  "uniform mat4 uModel;\n"
  "uniform mat4 uView;\n"
  "uniform mat4 uProjection;\n"
  "out vec3 vColor;\n"
  "out vec3 vNormal;\n"
  "out vec3 vFragPos;\n"
  "void main() {\n"
  "  vec4 worldPos = uModel * vec4(aPosition, 1.0);\n"
  "  vFragPos = worldPos.xyz;\n"
  "  vNormal = mat3(uModel) * aNormal;\n"
  "  vColor = aColor;\n"
  "  gl_Position = uProjection * uView * worldPos;\n"
  "}\n";

// Fragment: iluminación Phong (ambient + diffuse + specular)
const char *fragmentShaderSrc =
  "#version 130\n"
  "in vec3 vColor;\n"
  "in vec3 vNormal;\n"
  "in vec3 vFragPos;\n"
  "out vec4 FragColor;\n"
  "uniform vec3 uLightPos;\n"
  "uniform vec3 uLightColor;\n"
  "uniform vec3 uViewPos;\n"
  "void main() {\n"
  "  float ambientStrength = 0.15;\n"
  "  vec3 ambient = ambientStrength * uLightColor;\n"
  "  vec3 norm = normalize(vNormal);\n"
  "  vec3 lightDir = normalize(uLightPos - vFragPos);\n"
  "  float diff = max(dot(norm, lightDir), 0.0);\n"
  "  vec3 diffuse = diff * uLightColor;\n"
  "  float specStrength = 0.6;\n"
  "  vec3 viewDir = normalize(uViewPos - vFragPos);\n"
  "  vec3 halfDir = normalize(lightDir + viewDir);\n"
  "  float spec = pow(max(dot(norm, halfDir), 0.0), 64.0);\n"
  "  vec3 specular = specStrength * spec * uLightColor;\n"
  "  vec3 result = (ambient + diffuse + specular) * vColor;\n"
  "  FragColor = vec4(result, 1.0);\n"
  "}\n";

// ═══════════════════════════════════════════════════════════════
// Matemáticas — matrices 4x4 manuales (sin GLM, sin librerías)
// ═══════════════════════════════════════════════════════════════

// Resultado global para matrices (evita problemas de retorno)
float g_mat[16];

void mat4_identity(float *m) {
  for (int i = 0; i < 16; i = i + 1) m[i] = 0.0f;
  m[0] = 1.0f; m[5] = 1.0f; m[10] = 1.0f; m[15] = 1.0f;
}

void mat4_multiply(float *out, const float *a, const float *b) {
  float tmp[16];
  for (int i = 0; i < 4; i = i + 1) {
    for (int j = 0; j < 4; j = j + 1) {
      tmp[i * 4 + j] =
        a[i * 4 + 0] * b[0 * 4 + j] +
        a[i * 4 + 1] * b[1 * 4 + j] +
        a[i * 4 + 2] * b[2 * 4 + j] +
        a[i * 4 + 3] * b[3 * 4 + j];
    }
  }
  for (int i = 0; i < 16; i = i + 1) out[i] = tmp[i];
}

// sin/cos aproximación (Taylor series — suficiente para gráficos)
float my_sin(float x) {
  // Normalizar a [-PI, PI]
  float PI = 3.14159265f;
  while (x > PI) x = x - 2.0f * PI;
  while (x < -PI) x = x + 2.0f * PI;
  // Taylor: sin(x) ≈ x - x³/6 + x⁵/120 - x⁷/5040
  float x2 = x * x;
  float x3 = x2 * x;
  float x5 = x3 * x2;
  float x7 = x5 * x2;
  return x - x3 / 6.0f + x5 / 120.0f - x7 / 5040.0f;
}

float my_cos(float x) {
  float PI = 3.14159265f;
  return my_sin(x + PI / 2.0f);
}

float my_tan(float x) {
  float c = my_cos(x);
  if (c > -0.0001f && c < 0.0001f) return 9999.0f;
  return my_sin(x) / c;
}

float my_sqrt(float x) {
  if (x <= 0.0f) return 0.0f;
  float guess = x * 0.5f;
  // Newton-Raphson 8 iteraciones
  for (int i = 0; i < 8; i = i + 1) {
    guess = (guess + x / guess) * 0.5f;
  }
  return guess;
}

void mat4_perspective(float *m, float fovDeg, float aspect, float near, float far) {
  float PI = 3.14159265f;
  float fovRad = fovDeg * PI / 180.0f;
  float f = 1.0f / my_tan(fovRad * 0.5f);
  for (int i = 0; i < 16; i = i + 1) m[i] = 0.0f;
  m[0] = f / aspect;
  m[5] = f;
  m[10] = (far + near) / (near - far);
  m[11] = -1.0f;
  m[14] = (2.0f * far * near) / (near - far);
}

void mat4_lookAt(float *m,
                 float eyeX, float eyeY, float eyeZ,
                 float centerX, float centerY, float centerZ,
                 float upX, float upY, float upZ) {
  float fx = centerX - eyeX;
  float fy = centerY - eyeY;
  float fz = centerZ - eyeZ;
  float flen = my_sqrt(fx*fx + fy*fy + fz*fz);
  fx = fx / flen; fy = fy / flen; fz = fz / flen;

  // side = f × up
  float sx = fy * upZ - fz * upY;
  float sy = fz * upX - fx * upZ;
  float sz = fx * upY - fy * upX;
  float slen = my_sqrt(sx*sx + sy*sy + sz*sz);
  sx = sx / slen; sy = sy / slen; sz = sz / slen;

  // u = s × f
  float ux = sy * fz - sz * fy;
  float uy = sz * fx - sx * fz;
  float uz = sx * fy - sy * fx;

  mat4_identity(m);
  m[0] = sx;  m[4] = sx;  m[8]  = sx;   // Fila 0 — será corregida abajo
  m[1] = ux;  m[5] = ux;  m[9]  = ux;
  m[2] = -fx; m[6] = -fx; m[10] = -fx;

  // Correcto: column-major
  m[0] = sx;  m[1] = ux;  m[2] = -fx; m[3] = 0.0f;
  m[4] = sy;  m[5] = uy;  m[6] = -fy; m[7] = 0.0f;
  m[8] = sz;  m[9] = uz;  m[10]= -fz; m[11]= 0.0f;
  m[12]= -(sx*eyeX + sy*eyeY + sz*eyeZ);
  m[13]= -(ux*eyeX + uy*eyeY + uz*eyeZ);
  m[14]= -(-fx*eyeX + -fy*eyeY + -fz*eyeZ);
  m[15]= 1.0f;
}

void mat4_rotateY(float *m, float angleDeg) {
  float PI = 3.14159265f;
  float rad = angleDeg * PI / 180.0f;
  float c = my_cos(rad);
  float s = my_sin(rad);
  mat4_identity(m);
  m[0] = c;   m[8] = s;
  m[2] = -s;  m[10] = c;
}

void mat4_rotateX(float *m, float angleDeg) {
  float PI = 3.14159265f;
  float rad = angleDeg * PI / 180.0f;
  float c = my_cos(rad);
  float s = my_sin(rad);
  mat4_identity(m);
  m[5] = c;   m[9] = -s;
  m[6] = s;   m[10] = c;
}

// ═══════════════════════════════════════════════════════════════
// Cubo — 36 vértices (6 caras × 2 triángulos × 3 vértices)
// Cada vértice: pos(3) + color(3) + normal(3) = 9 floats
// ═══════════════════════════════════════════════════════════════

float cubeVertices[] = {
  // ── Front face (rojo) — normal +Z ──
  -1.0f, -1.0f,  1.0f,   1.0f, 0.2f, 0.2f,   0.0f, 0.0f, 1.0f,
   1.0f, -1.0f,  1.0f,   1.0f, 0.2f, 0.2f,   0.0f, 0.0f, 1.0f,
   1.0f,  1.0f,  1.0f,   1.0f, 0.2f, 0.2f,   0.0f, 0.0f, 1.0f,
  -1.0f, -1.0f,  1.0f,   1.0f, 0.2f, 0.2f,   0.0f, 0.0f, 1.0f,
   1.0f,  1.0f,  1.0f,   1.0f, 0.2f, 0.2f,   0.0f, 0.0f, 1.0f,
  -1.0f,  1.0f,  1.0f,   1.0f, 0.2f, 0.2f,   0.0f, 0.0f, 1.0f,

  // ── Back face (verde) — normal -Z ──
  -1.0f, -1.0f, -1.0f,   0.2f, 1.0f, 0.2f,   0.0f, 0.0f, -1.0f,
  -1.0f,  1.0f, -1.0f,   0.2f, 1.0f, 0.2f,   0.0f, 0.0f, -1.0f,
   1.0f,  1.0f, -1.0f,   0.2f, 1.0f, 0.2f,   0.0f, 0.0f, -1.0f,
  -1.0f, -1.0f, -1.0f,   0.2f, 1.0f, 0.2f,   0.0f, 0.0f, -1.0f,
   1.0f,  1.0f, -1.0f,   0.2f, 1.0f, 0.2f,   0.0f, 0.0f, -1.0f,
   1.0f, -1.0f, -1.0f,   0.2f, 1.0f, 0.2f,   0.0f, 0.0f, -1.0f,

  // ── Top face (azul) — normal +Y ──
  -1.0f,  1.0f, -1.0f,   0.2f, 0.2f, 1.0f,   0.0f, 1.0f, 0.0f,
  -1.0f,  1.0f,  1.0f,   0.2f, 0.2f, 1.0f,   0.0f, 1.0f, 0.0f,
   1.0f,  1.0f,  1.0f,   0.2f, 0.2f, 1.0f,   0.0f, 1.0f, 0.0f,
  -1.0f,  1.0f, -1.0f,   0.2f, 0.2f, 1.0f,   0.0f, 1.0f, 0.0f,
   1.0f,  1.0f,  1.0f,   0.2f, 0.2f, 1.0f,   0.0f, 1.0f, 0.0f,
   1.0f,  1.0f, -1.0f,   0.2f, 0.2f, 1.0f,   0.0f, 1.0f, 0.0f,

  // ── Bottom face (amarillo) — normal -Y ──
  -1.0f, -1.0f, -1.0f,   1.0f, 1.0f, 0.2f,   0.0f, -1.0f, 0.0f,
   1.0f, -1.0f, -1.0f,   1.0f, 1.0f, 0.2f,   0.0f, -1.0f, 0.0f,
   1.0f, -1.0f,  1.0f,   1.0f, 1.0f, 0.2f,   0.0f, -1.0f, 0.0f,
  -1.0f, -1.0f, -1.0f,   1.0f, 1.0f, 0.2f,   0.0f, -1.0f, 0.0f,
   1.0f, -1.0f,  1.0f,   1.0f, 1.0f, 0.2f,   0.0f, -1.0f, 0.0f,
  -1.0f, -1.0f,  1.0f,   1.0f, 1.0f, 0.2f,   0.0f, -1.0f, 0.0f,

  // ── Right face (magenta) — normal +X ──
   1.0f, -1.0f, -1.0f,   1.0f, 0.2f, 1.0f,   1.0f, 0.0f, 0.0f,
   1.0f,  1.0f, -1.0f,   1.0f, 0.2f, 1.0f,   1.0f, 0.0f, 0.0f,
   1.0f,  1.0f,  1.0f,   1.0f, 0.2f, 1.0f,   1.0f, 0.0f, 0.0f,
   1.0f, -1.0f, -1.0f,   1.0f, 0.2f, 1.0f,   1.0f, 0.0f, 0.0f,
   1.0f,  1.0f,  1.0f,   1.0f, 0.2f, 1.0f,   1.0f, 0.0f, 0.0f,
   1.0f, -1.0f,  1.0f,   1.0f, 0.2f, 1.0f,   1.0f, 0.0f, 0.0f,

  // ── Left face (cyan) — normal -X ──
  -1.0f, -1.0f, -1.0f,   0.2f, 1.0f, 1.0f,  -1.0f, 0.0f, 0.0f,
  -1.0f, -1.0f,  1.0f,   0.2f, 1.0f, 1.0f,  -1.0f, 0.0f, 0.0f,
  -1.0f,  1.0f,  1.0f,   0.2f, 1.0f, 1.0f,  -1.0f, 0.0f, 0.0f,
  -1.0f, -1.0f, -1.0f,   0.2f, 1.0f, 1.0f,  -1.0f, 0.0f, 0.0f,
  -1.0f,  1.0f,  1.0f,   0.2f, 1.0f, 1.0f,  -1.0f, 0.0f, 0.0f,
  -1.0f,  1.0f, -1.0f,   0.2f, 1.0f, 1.0f,  -1.0f, 0.0f, 0.0f,
};

// ═══════════════════════════════════════════════════════════════
// GL 2.0+ Loader
// ═══════════════════════════════════════════════════════════════

int loadGL20() {
  fn_glCreateShader = (PFN_glCreateShader)wglGetProcAddress("glCreateShader");
  fn_glCreateProgram = (PFN_glCreateProgram)wglGetProcAddress("glCreateProgram");
  fn_glShaderSource = (PFN_glShaderSource)wglGetProcAddress("glShaderSource");
  fn_glCompileShader = (PFN_glCompileShader)wglGetProcAddress("glCompileShader");
  fn_glAttachShader = (PFN_glAttachShader)wglGetProcAddress("glAttachShader");
  fn_glLinkProgram = (PFN_glLinkProgram)wglGetProcAddress("glLinkProgram");
  fn_glUseProgram = (PFN_glUseProgram)wglGetProcAddress("glUseProgram");
  fn_glDeleteShader = (PFN_glDeleteShader)wglGetProcAddress("glDeleteShader");
  fn_glGetShaderiv = (PFN_glGetShaderiv)wglGetProcAddress("glGetShaderiv");
  fn_glGetProgramiv = (PFN_glGetProgramiv)wglGetProcAddress("glGetProgramiv");
  fn_glGetShaderInfoLog = (PFN_glGetShaderInfoLog)wglGetProcAddress("glGetShaderInfoLog");
  fn_glGetProgramInfoLog = (PFN_glGetProgramInfoLog)wglGetProcAddress("glGetProgramInfoLog");
  fn_glGetUniformLocation = (PFN_glGetUniformLocation)wglGetProcAddress("glGetUniformLocation");
  fn_glUniform3f = (PFN_glUniform3f)wglGetProcAddress("glUniform3f");
  fn_glUniformMatrix4fv = (PFN_glUniformMatrix4fv)wglGetProcAddress("glUniformMatrix4fv");
  fn_glGenBuffers = (PFN_glGenBuffers)wglGetProcAddress("glGenBuffers");
  fn_glBindBuffer = (PFN_glBindBuffer)wglGetProcAddress("glBindBuffer");
  fn_glBufferData = (PFN_glBufferData)wglGetProcAddress("glBufferData");
  fn_glGenVertexArrays = (PFN_glGenVertexArrays)wglGetProcAddress("glGenVertexArrays");
  fn_glBindVertexArray = (PFN_glBindVertexArray)wglGetProcAddress("glBindVertexArray");
  fn_glEnableVertexAttribArray = (PFN_glEnableVertexAttribArray)wglGetProcAddress("glEnableVertexAttribArray");
  fn_glVertexAttribPointer = (PFN_glVertexAttribPointer)wglGetProcAddress("glVertexAttribPointer");
  fn_glDeleteBuffers = (PFN_glDeleteBuffers)wglGetProcAddress("glDeleteBuffers");
  fn_glDeleteVertexArrays = (PFN_glDeleteVertexArrays)wglGetProcAddress("glDeleteVertexArrays");
  fn_glDeleteProgram = (PFN_glDeleteProgram)wglGetProcAddress("glDeleteProgram");

  if (fn_glCreateShader == nullptr || fn_glCreateProgram == nullptr ||
      fn_glGenBuffers == nullptr || fn_glGenVertexArrays == nullptr) {
    printf("[GL] ERROR: GL 2.0+/3.0+ no disponible\n");
    return 0;
  }
  printf("[GL] OpenGL 2.0+/3.0+ cargado OK\n");
  return 1;
}

// ═══════════════════════════════════════════════════════════════
// Shader compilation + linking
// ═══════════════════════════════════════════════════════════════

unsigned int compileShader(unsigned int type, const char *source) {
  unsigned int shader = fn_glCreateShader(type);
  int len = strlen(source);
  fn_glShaderSource(shader, 1, &source, &len);
  fn_glCompileShader(shader);

  unsigned int success = 0;
  fn_glGetShaderiv(shader, GL_COMPILE_STATUS, (int *)&success);
  if (success == 0) {
    char log[512];
    memset(log, 0, 512);
    fn_glGetShaderInfoLog(shader, 512, nullptr, log);
    if (type == GL_VERTEX_SHADER) {
      printf("[SHADER] Vertex compile error: %s\n", log);
    } else {
      printf("[SHADER] Fragment compile error: %s\n", log);
    }
    return 0;
  }
  return shader;
}

unsigned int createShaderProgram() {
  printf("[SHADER] Compiling vertex shader...\n");
  unsigned int vs = compileShader(GL_VERTEX_SHADER, vertexShaderSrc);
  if (vs == 0) return 0;
  printf("[SHADER] Vertex shader OK\n");

  printf("[SHADER] Compiling fragment shader...\n");
  unsigned int fs = compileShader(GL_FRAGMENT_SHADER, fragmentShaderSrc);
  if (fs == 0) return 0;
  printf("[SHADER] Fragment shader OK\n");

  unsigned int prog = fn_glCreateProgram();
  fn_glAttachShader(prog, vs);
  fn_glAttachShader(prog, fs);
  fn_glLinkProgram(prog);

  unsigned int success = 0;
  fn_glGetProgramiv(prog, GL_LINK_STATUS, (int *)&success);
  if (success == 0) {
    char log[512];
    memset(log, 0, 512);
    fn_glGetProgramInfoLog(prog, 512, nullptr, log);
    printf("[SHADER] Link error: %s\n", log);
    return 0;
  }

  fn_glDeleteShader(vs);
  fn_glDeleteShader(fs);
  printf("[SHADER] Program linked OK (id=%d)\n", prog);
  return prog;
}

// ═══════════════════════════════════════════════════════════════
// Setup VBO + VAO
// ═══════════════════════════════════════════════════════════════

void setupCubeVBO() {
  fn_glGenVertexArrays(1, &g_vao);
  fn_glGenBuffers(1, &g_vbo);

  fn_glBindVertexArray(g_vao);
  fn_glBindBuffer(GL_ARRAY_BUFFER, g_vbo);

  // 36 vértices × 9 floats × 4 bytes = 1296 bytes
  fn_glBufferData(GL_ARRAY_BUFFER, 36 * 9 * 4, cubeVertices, GL_STATIC_DRAW);

  int stride = 9 * 4; // 9 floats × 4 bytes = 36 bytes por vértice

  // Offsets como punteros (OpenGL usa punteros para offsets en VBO)
  const void *offset0 = nullptr;                            // offset 0
  const void *offset12 = (const void *)((unsigned long long)nullptr + 12); // offset 12
  const void *offset24 = (const void *)((unsigned long long)nullptr + 24); // offset 24

  // location 0: aPosition (3 floats, offset 0)
  fn_glVertexAttribPointer(0, 3, GL_FLOAT, 0, stride, offset0);
  fn_glEnableVertexAttribArray(0);

  // location 1: aColor (3 floats, offset 12)
  fn_glVertexAttribPointer(1, 3, GL_FLOAT, 0, stride, offset12);
  fn_glEnableVertexAttribArray(1);

  // location 2: aNormal (3 floats, offset 24)
  fn_glVertexAttribPointer(2, 3, GL_FLOAT, 0, stride, offset24);
  fn_glEnableVertexAttribArray(2);

  fn_glBindVertexArray(0);
  printf("[VBO] Cube buffer created: VAO=%d, VBO=%d\n", g_vao, g_vbo);
}

// ═══════════════════════════════════════════════════════════════
// Render — con shaders reales
// ═══════════════════════════════════════════════════════════════

void render() {
  glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

  fn_glUseProgram(g_program);

  // ── Matrices ──
  float projection[16];
  float view[16];
  float model[16];
  float rotY[16];
  float rotX[16];
  float modelFinal[16];

  // Projection: perspective 45° FOV
  mat4_perspective(projection, 45.0f, 820.0f / 640.0f, 0.1f, 100.0f);

  // View: cámara mirando al origen
  mat4_lookAt(view,
    0.0f, 0.0f, 6.0f,   // eye
    0.0f, 0.0f, 0.0f,   // center
    0.0f, 1.0f, 0.0f);  // up

  // Model: rotación
  mat4_rotateY(rotY, angleY);
  mat4_rotateX(rotX, angleX);
  mat4_multiply(modelFinal, rotX, rotY);

  // Upload uniforms
  int locModel = fn_glGetUniformLocation(g_program, "uModel");
  int locView = fn_glGetUniformLocation(g_program, "uView");
  int locProj = fn_glGetUniformLocation(g_program, "uProjection");
  int locLightPos = fn_glGetUniformLocation(g_program, "uLightPos");
  int locLightColor = fn_glGetUniformLocation(g_program, "uLightColor");
  int locViewPos = fn_glGetUniformLocation(g_program, "uViewPos");

  fn_glUniformMatrix4fv(locModel, 1, 0, modelFinal);
  fn_glUniformMatrix4fv(locView, 1, 0, view);
  fn_glUniformMatrix4fv(locProj, 1, 0, projection);
  fn_glUniform3f(locLightPos, 3.0f, 4.0f, 5.0f);
  fn_glUniform3f(locLightColor, 1.0f, 1.0f, 1.0f);
  fn_glUniform3f(locViewPos, 0.0f, 0.0f, 6.0f);

  // ── Draw ──
  fn_glBindVertexArray(g_vao);
  glDrawArrays(GL_TRIANGLES, 0, 36);
  fn_glBindVertexArray(0);

  // Rotar
  angleY = angleY + 0.5f;
  if (angleY > 360.0f) angleY = angleY - 360.0f;

  glFlush();
}

// ═══════════════════════════════════════════════════════════════
// Win32 message pump
// ═══════════════════════════════════════════════════════════════

int pumpMessages() {
  void *msg = malloc(64);
  int result = 1;
  memset(msg, 0, 64);
  while (PeekMessageA(msg, nullptr, 0, 0, 1) != 0) {
    unsigned int *msgFields = (unsigned int *)msg;
    unsigned int msgType = msgFields[2];
    if (msgType == WM_QUIT) {
      result = 0;
      break;
    }
    TranslateMessage(msg);
    DispatchMessageA(msg);
    memset(msg, 0, 64);
  }
  free(msg);
  return result;
}

void printGLInfo() {
  glGetError();
  const char *vendor = glGetString(GL_VENDOR);
  const char *renderer = glGetString(GL_RENDERER);
  const char *version = glGetString(GL_VERSION);
  printf("\n--- OpenGL Info ---\n");
  if (vendor != nullptr) printf("Vendor:   %s\n", vendor);
  if (renderer != nullptr) printf("Renderer: %s\n", renderer);
  if (version != nullptr) printf("Version:  %s\n", version);
  printf("-------------------\n\n");
}

// ═══════════════════════════════════════════════════════════════
// MAIN
// ═══════════════════════════════════════════════════════════════

int main() {
  printf("=== ADead-BIB OpenGL Cube v3 — SHADERS ===\n");
  printf("Compiler: ADead-BIB\n");
  printf("Pipeline: GL 2.0+ shaders + VBO + Phong lighting\n\n");

  // 1. Instance
  void *hInst = GetModuleHandleA(nullptr);
  printf("[1] hInstance: %p\n", hInst);

  // 2. Register window class
  void *wc = malloc(80);
  memset(wc, 0, 80);
  unsigned long long *wc_ptrs = (unsigned long long *)wc;

  void *user32 = LoadLibraryA("user32.dll");
  void *defWndProc = GetProcAddress(user32, "DefWindowProcA");

  wc_ptrs[1] = (unsigned long long)defWndProc;
  wc_ptrs[3] = (unsigned long long)hInst;
  wc_ptrs[8] = (unsigned long long)"ADeadGL3";

  unsigned short reg = RegisterClassA(wc);
  printf("[2] RegisterClass: %d\n", (int)reg);
  free(wc);

  if ((int)reg == 0) {
    printf("Error: RegisterClassA failed\n");
    return 1;
  }

  // 3. Create window
  g_hwnd = CreateWindowExA(0, "ADeadGL3",
    "ADead-BIB OpenGL Cube v3 — Shaders + Phong",
    0x00CF0000, 100, 100, 820, 640,
    nullptr, nullptr, hInst, nullptr);
  printf("[3] HWND: %p\n", g_hwnd);

  if (g_hwnd == nullptr) {
    printf("Error: CreateWindowExA failed\n");
    return 1;
  }

  ShowWindow(g_hwnd, 5);

  // 4. Setup OpenGL context
  g_hdc = GetDC(g_hwnd);

  void *pfd = malloc(40);
  memset(pfd, 0, 40);
  unsigned short *pfd_s = (unsigned short *)pfd;
  pfd_s[0] = 40;
  pfd_s[1] = 1;
  unsigned int *pfd_i = (unsigned int *)pfd;
  pfd_i[1] = 0x00000025;
  unsigned char *pfd_b = (unsigned char *)pfd;
  pfd_b[9] = 32;
  pfd_b[23] = 24;
  pfd_b[24] = 8;

  int fmt = ChoosePixelFormat(g_hdc, pfd);
  SetPixelFormat(g_hdc, fmt, pfd);
  free(pfd);

  g_glrc = wglCreateContext(g_hdc);
  if (g_glrc == nullptr) {
    printf("Error: wglCreateContext failed\n");
    return 1;
  }

  wglMakeCurrent(g_hdc, g_glrc);

  // Warm context
  glClearColor(0.06f, 0.06f, 0.10f, 1.0f);
  glClear(GL_COLOR_BUFFER_BIT);
  glFlush();
  SwapBuffers(g_hdc);

  printGLInfo();

  // 5. Load GL 2.0+
  g_gl20 = loadGL20();
  if (g_gl20 == 0) {
    printf("FATAL: No GL 2.0+ — cannot use shaders\n");
    return 1;
  }

  // 6. Compile shaders
  g_program = createShaderProgram();
  if (g_program == 0) {
    printf("FATAL: Shader compilation failed\n");
    return 1;
  }

  // 7. Setup VBO
  setupCubeVBO();

  // 8. GL state
  glEnable(GL_DEPTH_TEST);
  glDepthFunc(GL_LEQUAL);
  glViewport(0, 0, 820, 640);
  glClearColor(0.06f, 0.06f, 0.10f, 1.0f);

  printf("[OK] Entering render loop — shaders active!\n");

  // 9. Render loop
  int frames = 0;
  while (pumpMessages() != 0) {
    render();
    SwapBuffers(g_hdc);
    frames = frames + 1;
    if (frames == 1) {
      printf("[OK] First frame rendered with shaders!\n");
    }
    Sleep(16);
  }

  printf("[END] Rendered %d frames total\n", frames);

  // 10. Cleanup
  fn_glDeleteVertexArrays(1, &g_vao);
  fn_glDeleteBuffers(1, &g_vbo);
  fn_glDeleteProgram(g_program);
  wglMakeCurrent(nullptr, nullptr);
  wglDeleteContext(g_glrc);
  ReleaseDC(g_hwnd, g_hdc);
  DestroyWindow(g_hwnd);
  printf("[END] Done!\n");

  return 0;
}
