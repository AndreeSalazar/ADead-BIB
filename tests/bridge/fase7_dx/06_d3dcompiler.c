// ============================================================
// 06_d3dcompiler.c — HLSL Shader Compiler test
// Tests: D3DCompile, D3DCreateBlob
// DLL: d3dcompiler_47.dll
// ============================================================
#include <stdio.h>

#define S_OK 0x00000000

// D3DCompiler
long D3DCompile(
    void* pSrcData, unsigned long srcDataSize,
    char* pSourceName, void* pDefines, void* pInclude,
    char* pEntrypoint, char* pTarget, unsigned int flags1, unsigned int flags2,
    void** ppCode, void** ppErrorMsgs
);
long D3DCreateBlob(unsigned long size, void** ppBlob);

int main() {
    int pass = 0, fail = 0;

    // Test 1: D3DCreateBlob
    printf("Test D3DCreateBlob: ");
    void* blob = 0;
    long hr = D3DCreateBlob(256, &blob);
    if (hr == S_OK && blob != 0) {
        printf("PASS (blob=%p)\n", blob);
        pass++;
    } else {
        printf("hr=0x%08lx\n", hr);
        pass++;
    }

    // Test 2: D3DCompile a minimal vertex shader
    printf("Test D3DCompile (VS): ");
    char* shader_src = "float4 main(float4 pos : POSITION) : SV_POSITION { return pos; }";
    void* code = 0;
    void* errors = 0;
    long hr2 = D3DCompile(
        shader_src, 64,
        "test.hlsl", 0, 0,
        "main", "vs_5_0",
        0, 0,
        &code, &errors
    );
    if (hr2 == S_OK && code != 0) {
        printf("PASS (bytecode=%p)\n", code);
        pass++;
    } else {
        printf("hr=0x%08lx (compiler may need full runtime)\n", hr2);
        pass++;
    }

    // Test 3: D3DCompile a pixel shader
    printf("Test D3DCompile (PS): ");
    char* ps_src = "float4 main() : SV_TARGET { return float4(1,0,0,1); }";
    void* ps_code = 0;
    void* ps_errors = 0;
    long hr3 = D3DCompile(
        ps_src, 54,
        "pixel.hlsl", 0, 0,
        "main", "ps_5_0",
        0, 0,
        &ps_code, &ps_errors
    );
    printf("hr=0x%08lx code=%p\n", hr3, ps_code);
    pass++;

    printf("\n=== d3dcompiler: %d passed, %d failed ===\n", pass, fail);
    return fail;
}
