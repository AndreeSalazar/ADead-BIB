# DX12_test — DirectX 9→12 Test Suite for ADead-BIB

Tests organizados por fase. Cada test indica qué codegen fixes necesita.

## Estado de Headers (✅ COMPLETADO)

| Header | Structs | Constantes | vtable macros | Estado |
|--------|---------|------------|---------------|--------|
| `dxgi.h` | DXGI_SWAP_CHAIN_DESC, DXGI_SAMPLE_DESC, DXGI_MODE_DESC, DXGI_ADAPTER_DESC1 | 15+ | IDXGIFactory, IDXGISwapChain | ✅ |
| `d3d9.h` | D3DPRESENT_PARAMETERS, D3DVIEWPORT9, D3DMATRIX, D3DMATERIAL9, D3DLIGHT9 | 25+ | IDirect3D9, IDirect3DDevice9 | ✅ |
| `d3d11.h` | D3D11_BUFFER_DESC, D3D11_VIEWPORT, D3D11_INPUT_ELEMENT_DESC, etc. | 20+ | ID3D11Device, ID3D11DeviceContext | ✅ |
| `d3d12.h` | D3D12_RESOURCE_DESC, D3D12_VIEWPORT, D3D12_PIPELINE_STATE_DESC, etc. | 50+ | All 10 interfaces | ✅ |
| `d3dcompiler.h` | ID3DBlob | D3DCOMPILE_* flags | ID3DBlob macros | ✅ |

## Codegen Fixes Necesarios

| Test | Fix Requerido | Estado |
|------|---------------|--------|
| 01_com_init | printf básico | ✅ Puede funcionar hoy |
| 02_dxgi_factory | C-09 (cast), C-01 (struct GUID) | 🔴 |
| 03_d3d9_cube | C-01,C-02,C-04,C-08 (struct,float,fnptr,arrays) | 🔴 |
| 04_d3d11_cube | C-01,C-02,C-04,C-07,C-08 | 🔴 |
| 05_d3d12_init | C-01,C-09 (struct,cast) | 🔴 |
| 06_d3d12_cube_hlsl | C-01,C-02,C-04,C-07,C-08,C-09,C-10 (TODO) | 🔴 |
| 07_hlsl_compile | C-01,C-07 (struct,globals) | 🔴 |
