/* adeb_vulkan.h — Vulkan 1.3 C ABI (mínimo funcional) */
#ifndef ADEB_VULKAN_H
#define ADEB_VULKAN_H

#include "adeb_types.h"

typedef void* VkInstance;
typedef void* VkPhysicalDevice;
typedef void* VkDevice;
typedef void* VkQueue;
typedef void* VkCommandPool;
typedef void* VkCommandBuffer;
typedef void* VkSwapchainKHR;
typedef void* VkSurfaceKHR;
typedef int32_t VkResult;

#define VK_SUCCESS 0

typedef struct VkApplicationInfo {
    uint32_t sType;
    const void* pNext;
    const char* pApplicationName;
    uint32_t applicationVersion;
    const char* pEngineName;
    uint32_t engineVersion;
    uint32_t apiVersion;
} VkApplicationInfo;

typedef struct VkInstanceCreateInfo {
    uint32_t sType;
    const void* pNext;
    uint32_t flags;
    const VkApplicationInfo* pApplicationInfo;
    uint32_t enabledLayerCount;
    const char* const* ppEnabledLayerNames;
    uint32_t enabledExtensionCount;
    const char* const* ppEnabledExtensionNames;
} VkInstanceCreateInfo;

extern VkResult vkCreateInstance(const VkInstanceCreateInfo* info, const void* alloc, VkInstance* out);
extern void     vkDestroyInstance(VkInstance inst, const void* alloc);
extern VkResult vkEnumeratePhysicalDevices(VkInstance inst, uint32_t* count, VkPhysicalDevice* devs);
extern void     vkGetDeviceQueue(VkDevice dev, uint32_t fam, uint32_t idx, VkQueue* q);
extern VkResult vkQueueSubmit(VkQueue q, uint32_t n, const void* submits, void* fence);
extern VkResult vkQueueWaitIdle(VkQueue q);
extern VkResult vkDeviceWaitIdle(VkDevice dev);

#endif /* ADEB_VULKAN_H */
