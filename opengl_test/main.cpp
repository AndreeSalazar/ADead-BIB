// main.cpp — ADead-BIB OpenGL Cube v3
// Cubo 3D rotante con colores por cara — GL 1.1 pipeline
// Compilar: cargo run --bin adb -- cxx opengl_test/main.cpp -o opengl_cube.exe --warn-ub
//
// Autor: Eddi Andreé Salazar Matos — Marzo 2026
// v3: GL 1.1 pipeline (funciona en CUALQUIER GPU)
//     Iluminación via GL fixed-function, colores por cara,
//     wglGetProcAddress para detectar capacidades futuras

extern "C" {
int printf(const char *, ...);
void *malloc(unsigned long long);
void free(void *);
void *memset(void *, int, unsigned long long);

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

// OpenGL 1.1 — opengl32.dll
void glClear(unsigned int mask);
void glClearColor(float r, float g, float b, float a);
void glEnable(unsigned int cap);
void glDisable(unsigned int cap);
void glDepthFunc(unsigned int func);
void glShadeModel(unsigned int mode);
void glViewport(int x, int y, int w, int h);
void glMatrixMode(unsigned int mode);
void glLoadIdentity();
void glTranslatef(float x, float y, float z);
void glRotatef(float angle, float x, float y, float z);
void glScalef(float x, float y, float z);
void glFrustum(double left, double right, double bottom, double top, double near, double far);
void glBegin(unsigned int mode);
void glEnd();
void glVertex3f(float x, float y, float z);
void glColor3f(float r, float g, float b);
void glColor4f(float r, float g, float b, float a);
void glNormal3f(float x, float y, float z);
void glLightfv(unsigned int light, unsigned int pname, const float *params);
void glMaterialfv(unsigned int face, unsigned int pname, const float *params);
void glMaterialf(unsigned int face, unsigned int pname, float param);
void glColorMaterial(unsigned int face, unsigned int mode);
void glFlush();
const char *glGetString(unsigned int name);
unsigned int glGetError();
}

// ── GL constants ──
unsigned int WM_QUIT = 18;
unsigned int GL_COLOR_BUFFER_BIT = 0x00004000;
unsigned int GL_DEPTH_BUFFER_BIT = 0x00000100;
unsigned int GL_DEPTH_TEST = 0x0B71;
unsigned int GL_LEQUAL = 0x0203;
unsigned int GL_LESS = 0x0201;
unsigned int GL_SMOOTH = 0x1D01;
unsigned int GL_PROJECTION = 0x1701;
unsigned int GL_MODELVIEW = 0x1700;
unsigned int GL_QUADS = 0x0007;
unsigned int GL_TRIANGLES = 0x0004;
unsigned int GL_VENDOR = 0x1F00;
unsigned int GL_RENDERER = 0x1F01;
unsigned int GL_VERSION = 0x1F02;
unsigned int GL_NO_ERROR = 0;
// Lighting
unsigned int GL_LIGHTING = 0x0B50;
unsigned int GL_LIGHT0 = 0x4000;
unsigned int GL_POSITION = 0x1203;
unsigned int GL_AMBIENT = 0x1200;
unsigned int GL_DIFFUSE = 0x1201;
unsigned int GL_SPECULAR = 0x1202;
unsigned int GL_SHININESS = 0x1601;
unsigned int GL_FRONT = 0x0404;
unsigned int GL_FRONT_AND_BACK = 0x0408;
unsigned int GL_AMBIENT_AND_DIFFUSE = 0x1602;
unsigned int GL_COLOR_MATERIAL = 0x0B57;
unsigned int GL_NORMALIZE = 0x0BA1;

// ── Estado global ──
float angleY = 0.0f;
float angleX = 25.0f;
void *g_glrc = nullptr;
void *g_hdc = nullptr;
void *g_hwnd = nullptr;
int g_gl20 = 0;

// ═══════════════════════════════════════════════════════════
// Math
// ═══════════════════════════════════════════════════════════

float my_sin(float x) {
  float PI = 3.14159265f;
  while (x > PI) x = x - 2.0f * PI;
  while (x < -PI) x = x + 2.0f * PI;
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

// ═══════════════════════════════════════════════════════════
// Setup iluminación GL 1.1 fixed-function
// ═══════════════════════════════════════════════════════════

void setupLighting() {
  glEnable(GL_LIGHTING);
  glEnable(GL_LIGHT0);
  glEnable(GL_COLOR_MATERIAL);
  glEnable(GL_NORMALIZE);

  // Posición de la luz
  float lightPos[4];
  lightPos[0] = 3.0f; lightPos[1] = 4.0f; lightPos[2] = 5.0f; lightPos[3] = 1.0f;
  glLightfv(GL_LIGHT0, GL_POSITION, lightPos);

  // Luz ambiente
  float lightAmb[4];
  lightAmb[0] = 0.2f; lightAmb[1] = 0.2f; lightAmb[2] = 0.2f; lightAmb[3] = 1.0f;
  glLightfv(GL_LIGHT0, GL_AMBIENT, lightAmb);

  // Luz difusa
  float lightDif[4];
  lightDif[0] = 0.9f; lightDif[1] = 0.9f; lightDif[2] = 0.9f; lightDif[3] = 1.0f;
  glLightfv(GL_LIGHT0, GL_DIFFUSE, lightDif);

  // Luz especular
  float lightSpec[4];
  lightSpec[0] = 1.0f; lightSpec[1] = 1.0f; lightSpec[2] = 1.0f; lightSpec[3] = 1.0f;
  glLightfv(GL_LIGHT0, GL_SPECULAR, lightSpec);

  // Material: usar glColor como ambient+diffuse
  glColorMaterial(GL_FRONT_AND_BACK, GL_AMBIENT_AND_DIFFUSE);

  // Specular del material
  float matSpec[4];
  matSpec[0] = 0.6f; matSpec[1] = 0.6f; matSpec[2] = 0.6f; matSpec[3] = 1.0f;
  glMaterialfv(GL_FRONT_AND_BACK, GL_SPECULAR, matSpec);
  glMaterialf(GL_FRONT_AND_BACK, GL_SHININESS, 64.0f);

  printf("[LIGHT] Phong lighting enabled (GL 1.1 fixed-function)\n");
}

// ═══════════════════════════════════════════════════════════
// Dibujar cubo — 6 caras coloreadas con normales
// ═══════════════════════════════════════════════════════════

void drawCube() {
  glBegin(GL_QUADS);

  // Front (rojo) +Z
  glColor3f(1.0f, 0.2f, 0.2f);
  glNormal3f(0.0f, 0.0f, 1.0f);
  glVertex3f(-1.0f, -1.0f,  1.0f);
  glVertex3f( 1.0f, -1.0f,  1.0f);
  glVertex3f( 1.0f,  1.0f,  1.0f);
  glVertex3f(-1.0f,  1.0f,  1.0f);

  // Back (verde) -Z
  glColor3f(0.2f, 1.0f, 0.2f);
  glNormal3f(0.0f, 0.0f, -1.0f);
  glVertex3f(-1.0f, -1.0f, -1.0f);
  glVertex3f(-1.0f,  1.0f, -1.0f);
  glVertex3f( 1.0f,  1.0f, -1.0f);
  glVertex3f( 1.0f, -1.0f, -1.0f);

  // Top (azul) +Y
  glColor3f(0.2f, 0.2f, 1.0f);
  glNormal3f(0.0f, 1.0f, 0.0f);
  glVertex3f(-1.0f,  1.0f, -1.0f);
  glVertex3f(-1.0f,  1.0f,  1.0f);
  glVertex3f( 1.0f,  1.0f,  1.0f);
  glVertex3f( 1.0f,  1.0f, -1.0f);

  // Bottom (amarillo) -Y
  glColor3f(1.0f, 1.0f, 0.2f);
  glNormal3f(0.0f, -1.0f, 0.0f);
  glVertex3f(-1.0f, -1.0f, -1.0f);
  glVertex3f( 1.0f, -1.0f, -1.0f);
  glVertex3f( 1.0f, -1.0f,  1.0f);
  glVertex3f(-1.0f, -1.0f,  1.0f);

  // Right (magenta) +X
  glColor3f(1.0f, 0.2f, 1.0f);
  glNormal3f(1.0f, 0.0f, 0.0f);
  glVertex3f( 1.0f, -1.0f, -1.0f);
  glVertex3f( 1.0f,  1.0f, -1.0f);
  glVertex3f( 1.0f,  1.0f,  1.0f);
  glVertex3f( 1.0f, -1.0f,  1.0f);

  // Left (cyan) -X
  glColor3f(0.2f, 1.0f, 1.0f);
  glNormal3f(-1.0f, 0.0f, 0.0f);
  glVertex3f(-1.0f, -1.0f, -1.0f);
  glVertex3f(-1.0f, -1.0f,  1.0f);
  glVertex3f(-1.0f,  1.0f,  1.0f);
  glVertex3f(-1.0f,  1.0f, -1.0f);

  glEnd();
}

// ═══════════════════════════════════════════════════════════
// Render
// ═══════════════════════════════════════════════════════════

void render() {
  glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

  // Projection: perspective manual via glFrustum
  glMatrixMode(GL_PROJECTION);
  glLoadIdentity();
  // fov=45°, aspect=820/640, near=0.5, far=100
  // top = near * tan(fov/2) = 0.5 * tan(22.5°) ≈ 0.2071
  // right = top * aspect = 0.2071 * 1.28125 ≈ 0.2653
  glFrustum(-0.2653, 0.2653, -0.2071, 0.2071, 0.5, 100.0);

  // Modelview
  glMatrixMode(GL_MODELVIEW);
  glLoadIdentity();
  glTranslatef(0.0f, 0.0f, -5.0f);
  glRotatef(angleX, 1.0f, 0.0f, 0.0f);
  glRotatef(angleY, 0.0f, 1.0f, 0.0f);

  // Re-posicionar la luz en espacio de vista
  float lightPos[4];
  lightPos[0] = 3.0f; lightPos[1] = 4.0f; lightPos[2] = 5.0f; lightPos[3] = 1.0f;
  glLightfv(GL_LIGHT0, GL_POSITION, lightPos);

  drawCube();

  angleY = angleY + 0.5f;
  if (angleY > 360.0f) angleY = angleY - 360.0f;

  glFlush();
}

// ═══════════════════════════════════════════════════════════
// Win32
// ═══════════════════════════════════════════════════════════

int pumpMessages() {
  void *msg = malloc(64);
  int result = 1;
  memset(msg, 0, 64);
  while (PeekMessageA(msg, nullptr, 0, 0, 1) != 0) {
    unsigned int *msgFields = (unsigned int *)msg;
    unsigned int msgType = msgFields[2];
    if (msgType == WM_QUIT) { result = 0; break; }
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
  if (vendor != nullptr) { printf("Vendor:   %s\n", vendor); }
  else { printf("Vendor:   (null)\n"); }
  if (renderer != nullptr) { printf("Renderer: %s\n", renderer); }
  else { printf("Renderer: (null)\n"); }
  if (version != nullptr) { printf("Version:  %s\n", version); }
  else { printf("Version:  (null)\n"); }
  printf("-------------------\n\n");
}

// ═══════════════════════════════════════════════════════════
// MAIN
// ═══════════════════════════════════════════════════════════

int main() {
  printf("=== ADead-BIB OpenGL Cube v3 ===\n");
  printf("Compiler: ADead-BIB\n");
  printf("GL 1.1 + fixed-function Phong lighting\n\n");

  // 1. Instance
  void *hInst = GetModuleHandleA(nullptr);
  printf("[1] hInstance: %p\n", hInst);

  // 2. Window class
  void *wc = malloc(80);
  memset(wc, 0, 80);
  unsigned long long *wc_ptrs = (unsigned long long *)wc;
  void *user32 = LoadLibraryA("user32.dll");
  void *defWndProc = GetProcAddress(user32, "DefWindowProcA");
  printf("[2] DefWindowProcA: %p\n", defWndProc);
  wc_ptrs[1] = (unsigned long long)defWndProc;
  wc_ptrs[3] = (unsigned long long)hInst;
  wc_ptrs[8] = (unsigned long long)"ADeadGL";
  unsigned short reg = RegisterClassA(wc);
  printf("[2] RegisterClass: %d\n", (int)reg);
  free(wc);
  if ((int)reg == 0) { printf("Error: RegisterClassA\n"); return 1; }

  // 3. Window
  g_hwnd = CreateWindowExA(0, "ADeadGL",
    "ADead-BIB OpenGL Cube v3 — Phong Lighting",
    0x00CF0000, 100, 100, 820, 640,
    nullptr, nullptr, hInst, nullptr);
  printf("[3] HWND: %p\n", g_hwnd);
  if (g_hwnd == nullptr) { printf("Error: CreateWindow\n"); return 1; }
  ShowWindow(g_hwnd, 5);
  printf("[3] Window visible\n");

  // 4. OpenGL context
  g_hdc = GetDC(g_hwnd);
  printf("[4] HDC: %p\n", g_hdc);

  void *pfd = malloc(40);
  memset(pfd, 0, 40);
  unsigned short *pfd_s = (unsigned short *)pfd;
  pfd_s[0] = 40; pfd_s[1] = 1;
  unsigned int *pfd_i = (unsigned int *)pfd;
  pfd_i[1] = 0x00000025;
  unsigned char *pfd_b = (unsigned char *)pfd;
  pfd_b[9] = 32; pfd_b[23] = 24; pfd_b[24] = 8;
  int fmt = ChoosePixelFormat(g_hdc, pfd);
  printf("[4] PixelFormat: %d\n", fmt);
  SetPixelFormat(g_hdc, fmt, pfd);
  free(pfd);

  g_glrc = wglCreateContext(g_hdc);
  printf("[4] HGLRC: %p\n", g_glrc);
  if (g_glrc == nullptr) { printf("Error: wglCreateContext\n"); return 1; }

  int mkr = wglMakeCurrent(g_hdc, g_glrc);
  printf("[4] wglMakeCurrent: %d\n", mkr);

  // Warm context
  glClearColor(0.06f, 0.06f, 0.10f, 1.0f);
  glClear(GL_COLOR_BUFFER_BIT);
  glFlush();
  SwapBuffers(g_hdc);

  printGLInfo();

  // 5. Check GL 2.0+
  void *test20 = wglGetProcAddress("glCreateShader");
  printf("[5] glCreateShader ptr: %p\n", test20);
  if (test20 != nullptr) {
    g_gl20 = 1;
    printf("[5] GL 2.0+ available (shaders en futuro)\n");
  } else {
    printf("[5] GL 2.0+ not available\n");
  }

  // 6. Setup GL
  glViewport(0, 0, 820, 640);
  glClearColor(0.06f, 0.06f, 0.10f, 1.0f);
  glEnable(GL_DEPTH_TEST);
  glDepthFunc(GL_LEQUAL);
  glShadeModel(GL_SMOOTH);

  // 7. Setup lighting
  setupLighting();

  printf("[OK] Entering render loop\n");

  // 8. Render loop
  int frames = 0;
  while (pumpMessages() != 0) {
    render();
    SwapBuffers(g_hdc);
    frames = frames + 1;
    if (frames == 1) {
      printf("[OK] First frame rendered!\n");
    }
    Sleep(16);
  }

  printf("[END] %d frames total\n", frames);

  // 9. Cleanup
  wglMakeCurrent(nullptr, nullptr);
  wglDeleteContext(g_glrc);
  ReleaseDC(g_hwnd, g_hdc);
  DestroyWindow(g_hwnd);
  printf("[END] Done!\n");
  return 0;
}
