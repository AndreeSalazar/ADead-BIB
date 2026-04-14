/* DX12_test/07_hlsl_compile.c — HLSL Shader Compilation Test
 *
 * CODEGEN NEEDED: C-07 (global strings), C-01 (struct for blob)
 * TESTS: D3DCompile with various shader models
 */
#include <stdio.h>
#include <windows.h>
#include <d3dcompiler.h>

/* Test various shader models */
const char *g_vs_simple =
    "float4 main(float4 pos : POSITION) : SV_Position { return pos; }\n";

const char *g_ps_simple =
    "float4 main() : SV_Target { return float4(1,0,0,1); }\n";

const char *g_ps_input =
    "struct PSInput { float4 pos : SV_Position; float4 col : COLOR; };\n"
    "float4 main(PSInput i) : SV_Target { return i.col; }\n";

const char *g_cs_simple =
    "RWStructuredBuffer<float> buf : register(u0);\n"
    "[numthreads(64,1,1)]\n"
    "void main(uint3 id : SV_DispatchThreadID) { buf[id.x] = id.x; }\n";

int test_compile(const char *code, unsigned long long len,
                 const char *entry, const char *target, const char *name) {
    ID3DBlob *blob;
    ID3DBlob *err;
    HRESULT hr;
    
    blob = 0;
    err = 0;
    
    hr = D3DCompile(code, len, name, 0, 0, entry, target, 0, 0, &blob, &err);
    
    if (hr >= 0 && blob != 0) {
        printf("  [OK] %s (%s) -> %llu bytes\n", name, target,
            ID3DBlob_GetBufferSize(blob));
        ID3DBlob_Release(blob);
        return 0;
    } else {
        printf("  [FAIL] %s (%s)\n", name, target);
        if (err) {
            printf("    Error: %s\n", (const char*)ID3DBlob_GetBufferPointer(err));
            ID3DBlob_Release(err);
        }
        return 1;
    }
}

int main(void) {
    int fails;
    fails = 0;
    
    printf("=== DX Test 07: HLSL Compilation ===\n");
    
    /* DX11 shader models */
    printf("--- DX11 shader models (vs_5_0 / ps_5_0) ---\n");
    fails = fails + test_compile(g_vs_simple, 64, "main", "vs_5_0", "simple_vs");
    fails = fails + test_compile(g_ps_simple, 53, "main", "ps_5_0", "simple_ps");
    fails = fails + test_compile(g_ps_input, 100, "main", "ps_5_0", "input_ps");
    
    /* DX12 shader models */
    printf("--- DX12 shader models (vs_5_1 / ps_5_1 / cs_5_1) ---\n");
    fails = fails + test_compile(g_vs_simple, 64, "main", "vs_5_1", "simple_vs_51");
    fails = fails + test_compile(g_ps_simple, 53, "main", "ps_5_1", "simple_ps_51");
    fails = fails + test_compile(g_cs_simple, 120, "main", "cs_5_1", "simple_cs_51");
    
    /* D3DCreateBlob test */
    printf("--- D3DCreateBlob ---\n");
    ID3DBlob *testBlob;
    HRESULT hr;
    testBlob = 0;
    hr = D3DCreateBlob(256, &testBlob);
    if (hr >= 0) {
        printf("  [OK] Created blob of 256 bytes (actual=%llu)\n",
            ID3DBlob_GetBufferSize(testBlob));
        ID3DBlob_Release(testBlob);
    } else {
        printf("  [FAIL] D3DCreateBlob\n");
        fails++;
    }
    
    printf("--- Results: %d failures ---\n", fails);
    if (fails == 0) {
        printf("=== Test 07: ALL PASSED ===\n");
    } else {
        printf("=== Test 07: %d FAILED ===\n", fails);
    }
    
    return fails;
}
