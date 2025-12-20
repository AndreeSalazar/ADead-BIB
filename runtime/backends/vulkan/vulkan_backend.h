/**
 * ADead-BIB Universal Runtime - Vulkan Backend
 * =============================================
 * Author: Eddi Andreé Salazar Matos
 * Email: eddi.salazar.dev@gmail.com
 * Made with love in Peru
 * 
 * Backend Vulkan para compute shaders.
 * Compatible con NVIDIA, AMD, Intel.
 */

#ifndef ADEAD_VULKAN_BACKEND_H
#define ADEAD_VULKAN_BACKEND_H

#include "../../core/types.h"
#include "../../core/runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

/* ============================================================
 * Vulkan Backend Context
 * ============================================================ */

typedef struct {
    /* Vulkan handles (opaque) */
    void* instance;
    void* physical_device;
    void* device;
    void* compute_queue;
    void* command_pool;
    void* descriptor_pool;
    
    /* Memory */
    usize allocated;
    usize peak;
    
    /* Device info */
    char device_name[256];
    u32 compute_queue_family;
    usize device_memory;
    
    /* Shaders */
    void* matmul_pipeline;
    void* softmax_pipeline;
    void* relu_pipeline;
    void* attention_pipeline;
    
    /* Status */
    int initialized;
} ADeadVulkanContext;

/* ============================================================
 * Vulkan Backend API
 * ============================================================ */

/* Obtener vtable del backend Vulkan */
ADeadBackendVTable* adead_vulkan_get_vtable(void);

/* Verificar si Vulkan está disponible */
int adead_vulkan_available(void);

/* Obtener información del dispositivo */
const char* adead_vulkan_device_name(ADeadVulkanContext* ctx);
usize adead_vulkan_device_memory(ADeadVulkanContext* ctx);

/* ============================================================
 * Shader Management
 * ============================================================ */

/* Cargar shader SPIR-V */
ADeadError adead_vulkan_load_shader(ADeadVulkanContext* ctx,
                                    const char* name,
                                    const u32* spirv_code,
                                    usize spirv_size);

/* Crear pipeline de compute */
ADeadError adead_vulkan_create_pipeline(ADeadVulkanContext* ctx,
                                        const char* shader_name,
                                        void** pipeline);

/* ============================================================
 * Compute Operations
 * ============================================================ */

/* Dispatch compute shader */
ADeadError adead_vulkan_dispatch(ADeadVulkanContext* ctx,
                                 void* pipeline,
                                 u32 group_x, u32 group_y, u32 group_z);

/* Submit y esperar */
ADeadError adead_vulkan_submit_and_wait(ADeadVulkanContext* ctx);

/* ============================================================
 * Buffer Management
 * ============================================================ */

typedef struct {
    void* buffer;
    void* memory;
    usize size;
    void* mapped;  /* NULL si no está mapeado */
} ADeadVulkanBuffer;

/* Crear buffer */
ADeadError adead_vulkan_buffer_create(ADeadVulkanContext* ctx,
                                      ADeadVulkanBuffer* buf,
                                      usize size,
                                      int host_visible);

/* Destruir buffer */
void adead_vulkan_buffer_destroy(ADeadVulkanContext* ctx,
                                 ADeadVulkanBuffer* buf);

/* Copiar datos a buffer */
ADeadError adead_vulkan_buffer_upload(ADeadVulkanContext* ctx,
                                      ADeadVulkanBuffer* buf,
                                      const void* data,
                                      usize size);

/* Copiar datos desde buffer */
ADeadError adead_vulkan_buffer_download(ADeadVulkanContext* ctx,
                                        ADeadVulkanBuffer* buf,
                                        void* data,
                                        usize size);

#ifdef __cplusplus
}
#endif

#endif /* ADEAD_VULKAN_BACKEND_H */
