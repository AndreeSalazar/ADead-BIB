// main.cpp — ADead-BIB OpenGL Cube
// Cubo 3D rotante — Self-contained, sin dependencias externas
// Compilar: cargo run --bin adb -- cxx opengl_test/main.cpp -o opengl_cube.exe
//
// Autor: Eddi Andreé Salazar Matos — Marzo 2026

extern "C" {
    int printf(const char*, ...);
    void* malloc(unsigned long long);
    void free(void*);
    void* memset(void*, int, unsigned long long);

    // Win32 native
    void* GetModuleHandleA(const char* name);
    unsigned short RegisterClassA(const void* wc);
    void* CreateWindowExA(unsigned long dwExStyle, const char* cls, const char* title,
        unsigned long dwStyle, int x, int y, int w, int h,
        void* parent, void* menu, void* hInst, void* param);
    int ShowWindow(void* hwnd, int cmd);
    int PeekMessageA(void* msg, void* hwnd, unsigned int min, unsigned int max, unsigned int rm);
    int TranslateMessage(const void* msg);
    long long DispatchMessageA(const void* msg);
    void PostQuitMessage(int code);
    long long DefWindowProcA(void* hwnd, unsigned int msg, unsigned long long wp, long long lp);
    int DestroyWindow(void* hwnd);
    void Sleep(unsigned int ms);
    void* LoadLibraryA(const char* name);
    void* GetProcAddress(void* hmod, const char* name);

    // GDI
    void* GetDC(void* hwnd);
    int ReleaseDC(void* hwnd, void* hdc);
    int SwapBuffers(void* hdc);
    int ChoosePixelFormat(void* hdc, const void* pfd);
    int SetPixelFormat(void* hdc, int fmt, const void* pfd);

    // WGL
    void* wglCreateContext(void* hdc);
    int wglMakeCurrent(void* hdc, void* hglrc);
    int wglDeleteContext(void* hglrc);

    // OpenGL 1.1
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
    void glBegin(unsigned int mode);
    void glEnd();
    void glVertex3f(float x, float y, float z);
    void glColor3f(float r, float g, float b);
    void glNormal3f(float x, float y, float z);
    const char* glGetString(unsigned int name);
}

// GL constants
unsigned int GL_COLOR_BUFFER_BIT = 0x00004000;
unsigned int GL_DEPTH_BUFFER_BIT = 0x00000100;
unsigned int GL_DEPTH_TEST = 0x0B71;
unsigned int GL_LEQUAL = 0x0203;
unsigned int GL_SMOOTH = 0x1D01;
unsigned int GL_PROJECTION = 0x1701;
unsigned int GL_MODELVIEW = 0x1700;
unsigned int GL_QUADS = 0x0007;
unsigned int GL_VENDOR = 0x1F00;
unsigned int GL_RENDERER = 0x1F01;
unsigned int GL_VERSION = 0x1F02;

// Estado global
float angleY = 0.0f;
void* g_glrc = nullptr;
void* g_hdc  = nullptr;
void* g_hwnd = nullptr;

// ── Dibujar cubo ──
void drawCube() {
    glBegin(GL_QUADS);

    glColor3f(1.0f, 0.2f, 0.2f);
    glNormal3f(0.0f, 0.0f, 1.0f);
    glVertex3f(-1.0f, -1.0f,  1.0f);
    glVertex3f( 1.0f, -1.0f,  1.0f);
    glVertex3f( 1.0f,  1.0f,  1.0f);
    glVertex3f(-1.0f,  1.0f,  1.0f);

    glColor3f(0.2f, 1.0f, 0.2f);
    glNormal3f(0.0f, 0.0f, -1.0f);
    glVertex3f(-1.0f, -1.0f, -1.0f);
    glVertex3f(-1.0f,  1.0f, -1.0f);
    glVertex3f( 1.0f,  1.0f, -1.0f);
    glVertex3f( 1.0f, -1.0f, -1.0f);

    glColor3f(0.2f, 0.2f, 1.0f);
    glNormal3f(0.0f, 1.0f, 0.0f);
    glVertex3f(-1.0f, 1.0f, -1.0f);
    glVertex3f(-1.0f, 1.0f,  1.0f);
    glVertex3f( 1.0f, 1.0f,  1.0f);
    glVertex3f( 1.0f, 1.0f, -1.0f);

    glColor3f(1.0f, 1.0f, 0.2f);
    glNormal3f(0.0f, -1.0f, 0.0f);
    glVertex3f(-1.0f, -1.0f, -1.0f);
    glVertex3f( 1.0f, -1.0f, -1.0f);
    glVertex3f( 1.0f, -1.0f,  1.0f);
    glVertex3f(-1.0f, -1.0f,  1.0f);

    glColor3f(1.0f, 0.2f, 1.0f);
    glNormal3f(1.0f, 0.0f, 0.0f);
    glVertex3f(1.0f, -1.0f, -1.0f);
    glVertex3f(1.0f,  1.0f, -1.0f);
    glVertex3f(1.0f,  1.0f,  1.0f);
    glVertex3f(1.0f, -1.0f,  1.0f);

    glColor3f(0.2f, 1.0f, 1.0f);
    glNormal3f(-1.0f, 0.0f, 0.0f);
    glVertex3f(-1.0f, -1.0f, -1.0f);
    glVertex3f(-1.0f, -1.0f,  1.0f);
    glVertex3f(-1.0f,  1.0f,  1.0f);
    glVertex3f(-1.0f,  1.0f, -1.0f);

    glEnd();
}

// ── Renderizar ──
void render() {
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

    glMatrixMode(GL_PROJECTION);
    glLoadIdentity();
    glScalef(1.5f, 2.0f, 1.0f);

    glMatrixMode(GL_MODELVIEW);
    glLoadIdentity();
    glTranslatef(0.0f, 0.0f, -6.0f);
    glRotatef(25.0f, 1.0f, 0.0f, 0.0f);
    glRotatef(angleY, 0.0f, 1.0f, 0.0f);

    drawCube();

    angleY = angleY + 0.5f;
    if (angleY > 360.0f) {
        angleY = angleY - 360.0f;
    }
}

// ── Pump messages ──
int pumpMessages() {
    void* msg = malloc(64);
    int result = 1;
    memset(msg, 0, 64);
    while (PeekMessageA(msg, nullptr, 0, 0, 1) != 0) {
        unsigned int* msgFields = (unsigned int*)msg;
        int msgType = (int)msgFields[2];
        if (msgType == 18) {
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

// ── MAIN ──
int main() {
    printf("=== ADead-BIB OpenGL Cube ===\n");
    printf("Compiler: ADead-BIB v9.1\n");
    printf("API: OpenGL (native, sin SDK)\n\n");

    // 1. Get instance
    void* hInst = GetModuleHandleA(nullptr);
    printf("[1] hInstance: %p\n", hInst);

    // 2. Register window class using DefWindowProcA directly
    void* wc = malloc(80);
    memset(wc, 0, 80);
    unsigned long long* wc_ptrs = (unsigned long long*)wc;
    
    // WNDPROC at offset 8 (index 1) — use DefWindowProcA directly
    void* user32 = LoadLibraryA("user32.dll");
    void* defWndProc = GetProcAddress(user32, "DefWindowProcA");
    printf("[2] DefWindowProcA: %p\n", defWndProc);
    wc_ptrs[1] = (unsigned long long)defWndProc;
    // hInstance at offset 24 (index 3)
    wc_ptrs[3] = (unsigned long long)hInst;
    // lpszClassName at offset 64 (index 8)
    wc_ptrs[8] = (unsigned long long)"ADeadGL";

    unsigned short reg = RegisterClassA(wc);
    printf("[2] RegisterClass: %d\n", (int)reg);
    free(wc);

    if ((int)reg == 0) {
        printf("Error: RegisterClassA failed\n");
        return 1;
    }

    // 3. Create window — 12 arguments!
    printf("[3] Calling CreateWindowExA (12 args)...\n");
    g_hwnd = CreateWindowExA(
        0,
        "ADeadGL",
        "ADead-BIB OpenGL Cube",
        0x00CF0000,
        100, 100, 820, 640,
        nullptr, nullptr, hInst, nullptr
    );
    printf("[3] HWND: %p\n", g_hwnd);

    if (g_hwnd == nullptr) {
        printf("Error: CreateWindowExA failed\n");
        return 1;
    }

    ShowWindow(g_hwnd, 5);
    printf("[3] Window visible\n");

    // 4. Setup OpenGL context
    g_hdc = GetDC(g_hwnd);
    printf("[4] HDC: %p\n", g_hdc);

    // PFD — 40 bytes raw
    void* pfd = malloc(40);
    memset(pfd, 0, 40);
    unsigned short* pfd_s = (unsigned short*)pfd;
    pfd_s[0] = 40;
    pfd_s[1] = 1;
    unsigned int* pfd_i = (unsigned int*)pfd;
    pfd_i[1] = 0x00000025;
    unsigned char* pfd_b = (unsigned char*)pfd;
    pfd_b[9] = 32;
    pfd_b[23] = 24;
    pfd_b[24] = 8;

    int fmt = ChoosePixelFormat(g_hdc, pfd);
    printf("[4] PixelFormat: %d\n", fmt);
    SetPixelFormat(g_hdc, fmt, pfd);
    free(pfd);

    g_glrc = wglCreateContext(g_hdc);
    printf("[4] HGLRC: %p\n", g_glrc);

    if (g_glrc == nullptr) {
        printf("Error: wglCreateContext failed\n");
        return 1;
    }

    wglMakeCurrent(g_hdc, g_glrc);

    // Print GL info
    printf("\n--- OpenGL Info ---\n");
    printf("Vendor:   %s\n", glGetString(GL_VENDOR));
    printf("Renderer: %s\n", glGetString(GL_RENDERER));
    printf("Version:  %s\n", glGetString(GL_VERSION));
    printf("-------------------\n\n");

    // Configure GL
    glViewport(0, 0, 820, 640);
    glClearColor(0.08f, 0.08f, 0.12f, 1.0f);
    glEnable(GL_DEPTH_TEST);
    glDepthFunc(GL_LEQUAL);
    glShadeModel(GL_SMOOTH);

    printf("[5] Entering render loop...\n");

    // 5. Render loop
    int frames = 0;
    while (pumpMessages() != 0) {
        render();
        SwapBuffers(g_hdc);
        frames = frames + 1;
        if (frames == 1) {
            printf("[5] First frame rendered!\n");
        }
    }

    printf("[6] Rendered %d frames total\n", frames);

    // 6. Cleanup
    wglMakeCurrent(nullptr, nullptr);
    wglDeleteContext(g_glrc);
    ReleaseDC(g_hwnd, g_hdc);
    DestroyWindow(g_hwnd);
    printf("[6] Done!\n");

    return 0;
}
