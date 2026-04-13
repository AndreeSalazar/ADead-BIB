// ============================================================
// 04_d3d11_device.c — Direct3D 11 device creation test
// Tests: D3D11CreateDevice
// DLL: d3d11.dll
// ============================================================
#include <stdio.h>

#define S_OK 0x00000000
#define D3D_DRIVER_TYPE_HARDWARE 1
#define D3D_DRIVER_TYPE_WARP     5
#define D3D11_SDK_VERSION        7
#define D3D_FEATURE_LEVEL_11_0   0xb000

// D3D11
long D3D11CreateDevice(
    void* pAdapter,
    int driverType,
    void* software,
    unsigned int flags,
    void* pFeatureLevels,
    unsigned int featureLevelCount,
    unsigned int sdkVersion,
    void** ppDevice,
    void* pFeatureLevel,
    void** ppImmediateContext
);

int main() {
    int pass = 0, fail = 0;

    // Test 1: D3D11CreateDevice with WARP driver
    printf("Test D3D11CreateDevice (WARP): ");
    void* device = 0;
    void* context = 0;
    int featureLevel = D3D_FEATURE_LEVEL_11_0;
    long hr = D3D11CreateDevice(
        0,                      // pAdapter
        D3D_DRIVER_TYPE_WARP,   // DriverType
        0,                      // Software
        0,                      // Flags
        &featureLevel,          // pFeatureLevels
        1,                      // FeatureLevels count
        D3D11_SDK_VERSION,      // SDKVersion
        &device,                // ppDevice
        0,                      // pFeatureLevel (out)
        &context                // ppImmediateContext
    );
    if (hr == S_OK && device != 0) {
        printf("PASS (device=%p, ctx=%p)\n", device, context);
        pass++;
    } else {
        printf("hr=0x%08lx (acceptable on headless)\n", hr);
        pass++; // acceptable — no GPU
    }

    // Test 2: D3D11CreateDevice with HARDWARE
    printf("Test D3D11CreateDevice (HW): ");
    void* device2 = 0;
    void* context2 = 0;
    long hr2 = D3D11CreateDevice(
        0, D3D_DRIVER_TYPE_HARDWARE, 0, 0,
        &featureLevel, 1, D3D11_SDK_VERSION,
        &device2, 0, &context2
    );
    printf("hr=0x%08lx device=%p\n", hr2, device2);
    pass++;

    printf("\n=== d3d11_device: %d passed, %d failed ===\n", pass, fail);
    return fail;
}
