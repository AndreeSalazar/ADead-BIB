/*
 * adeb_main.h — ADead-BIB v12.0 master include
 *
 * Un solo include para todo el ecosistema C ABI de ADead-BIB.
 *   #include <adeb_main.h>
 *
 * Subsystem headers:
 *   - adeb_types.h     int8_t..uint64_t, size_t, ptrdiff_t, NULL
 *   - adeb_stdio.h     printf, fopen, fread, fwrite, puts, fclose
 *   - adeb_stdlib.h    malloc, free, realloc, exit, atoi, qsort
 *   - adeb_string.h    memcpy, memset, memcmp, strlen, strcmp, strcpy
 *   - adeb_math.h      sin, cos, sqrt, pow, log, exp, floor, ceil
 *   - adeb_win32.h     CreateWindowEx, MessageBoxA, kernel32 + user32
 *   - adeb_vulkan.h    vkCreateInstance, vkQueueSubmit, vk_types
 *   - adeb_dx12.h      ID3D12Device, ID3D12Resource, COM
 *   - adeb_opengl.h    glDrawArrays, glClear, GLSL, wgl
 */
#ifndef ADEB_MAIN_H
#define ADEB_MAIN_H

#include "adeb_types.h"
#include "adeb_stdio.h"
#include "adeb_stdlib.h"
#include "adeb_string.h"
#include "adeb_math.h"

#ifdef ADEB_TARGET_WIN32
#include "adeb_win32.h"
#endif

#ifdef ADEB_TARGET_VULKAN
#include "adeb_vulkan.h"
#endif

#ifdef ADEB_TARGET_DX12
#include "adeb_dx12.h"
#endif

#ifdef ADEB_TARGET_OPENGL
#include "adeb_opengl.h"
#endif

#endif /* ADEB_MAIN_H */
