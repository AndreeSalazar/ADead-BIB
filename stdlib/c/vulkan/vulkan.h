/*
 * ADead-BIB Standard Library
 * vulkan/vulkan.h - Vulkan Graphics API
 * 
 * Based on: Vulkan 1.3 Specification
 * Optimized for ADead-BIB C compiler
 */

#ifndef _ADEAD_VULKAN_H
#define _ADEAD_VULKAN_H

#include "../stdint.h"
#include "../stddef.h"

/* Vulkan version */
#define VK_API_VERSION_1_0 ((1 << 22) | (0 << 12) | 0)
#define VK_API_VERSION_1_1 ((1 << 22) | (1 << 12) | 0)
#define VK_API_VERSION_1_2 ((1 << 22) | (2 << 12) | 0)
#define VK_API_VERSION_1_3 ((1 << 22) | (3 << 12) | 0)

#define VK_MAKE_VERSION(major, minor, patch) \
    (((major) << 22) | ((minor) << 12) | (patch))

#define VK_VERSION_MAJOR(version) ((uint32_t)(version) >> 22)
#define VK_VERSION_MINOR(version) (((uint32_t)(version) >> 12) & 0x3FF)
#define VK_VERSION_PATCH(version) ((uint32_t)(version) & 0xFFF)

/* Handle types */
#define VK_DEFINE_HANDLE(object) typedef struct object##_T* object;
#define VK_DEFINE_NON_DISPATCHABLE_HANDLE(object) typedef uint64_t object;

VK_DEFINE_HANDLE(VkInstance)
VK_DEFINE_HANDLE(VkPhysicalDevice)
VK_DEFINE_HANDLE(VkDevice)
VK_DEFINE_HANDLE(VkQueue)
VK_DEFINE_HANDLE(VkCommandBuffer)

VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkSemaphore)
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkFence)
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkDeviceMemory)
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkBuffer)
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkImage)
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkEvent)
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkQueryPool)
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkBufferView)
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkImageView)
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkShaderModule)
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkPipelineCache)
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkPipelineLayout)
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkRenderPass)
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkPipeline)
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkDescriptorSetLayout)
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkSampler)
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkDescriptorPool)
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkDescriptorSet)
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkFramebuffer)
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkCommandPool)
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkSurfaceKHR)
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkSwapchainKHR)

/* Null handle */
#define VK_NULL_HANDLE 0

/* Boolean */
typedef uint32_t VkBool32;
typedef uint32_t VkFlags;
typedef uint64_t VkDeviceSize;
typedef uint64_t VkDeviceAddress;

#define VK_TRUE  1
#define VK_FALSE 0

/* Result codes */
typedef enum VkResult {
    VK_SUCCESS = 0,
    VK_NOT_READY = 1,
    VK_TIMEOUT = 2,
    VK_EVENT_SET = 3,
    VK_EVENT_RESET = 4,
    VK_INCOMPLETE = 5,
    VK_ERROR_OUT_OF_HOST_MEMORY = -1,
    VK_ERROR_OUT_OF_DEVICE_MEMORY = -2,
    VK_ERROR_INITIALIZATION_FAILED = -3,
    VK_ERROR_DEVICE_LOST = -4,
    VK_ERROR_MEMORY_MAP_FAILED = -5,
    VK_ERROR_LAYER_NOT_PRESENT = -6,
    VK_ERROR_EXTENSION_NOT_PRESENT = -7,
    VK_ERROR_FEATURE_NOT_PRESENT = -8,
    VK_ERROR_INCOMPATIBLE_DRIVER = -9,
    VK_ERROR_TOO_MANY_OBJECTS = -10,
    VK_ERROR_FORMAT_NOT_SUPPORTED = -11,
    VK_ERROR_FRAGMENTED_POOL = -12,
    VK_ERROR_UNKNOWN = -13,
    VK_ERROR_SURFACE_LOST_KHR = -1000000000,
    VK_ERROR_OUT_OF_DATE_KHR = -1000001004,
    VK_SUBOPTIMAL_KHR = 1000001003,
} VkResult;

/* Structure types */
typedef enum VkStructureType {
    VK_STRUCTURE_TYPE_APPLICATION_INFO = 0,
    VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO = 1,
    VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO = 2,
    VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO = 3,
    VK_STRUCTURE_TYPE_SUBMIT_INFO = 4,
    VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO = 5,
    VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE = 6,
    VK_STRUCTURE_TYPE_BIND_SPARSE_INFO = 7,
    VK_STRUCTURE_TYPE_FENCE_CREATE_INFO = 8,
    VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO = 9,
    VK_STRUCTURE_TYPE_EVENT_CREATE_INFO = 10,
    VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO = 11,
    VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO = 12,
    VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO = 13,
    VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO = 14,
    VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO = 15,
    VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO = 16,
    VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO = 17,
    VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO = 18,
    VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO = 19,
    VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO = 20,
    VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO = 21,
    VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO = 22,
    VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO = 23,
    VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO = 24,
    VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO = 25,
    VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO = 26,
    VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO = 27,
    VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO = 28,
    VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO = 29,
    VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO = 30,
    VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO = 31,
    VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO = 32,
    VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO = 33,
    VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO = 34,
    VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET = 35,
    VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET = 36,
    VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO = 37,
    VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO = 38,
    VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO = 39,
    VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO = 40,
    VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO = 41,
    VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO = 42,
    VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO = 43,
    VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER = 44,
    VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER = 45,
    VK_STRUCTURE_TYPE_MEMORY_BARRIER = 46,
    VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR = 1000001000,
    VK_STRUCTURE_TYPE_PRESENT_INFO_KHR = 1000001001,
} VkStructureType;

/* Format */
typedef enum VkFormat {
    VK_FORMAT_UNDEFINED = 0,
    VK_FORMAT_R8_UNORM = 9,
    VK_FORMAT_R8G8_UNORM = 16,
    VK_FORMAT_R8G8B8_UNORM = 23,
    VK_FORMAT_R8G8B8A8_UNORM = 37,
    VK_FORMAT_R8G8B8A8_SRGB = 43,
    VK_FORMAT_B8G8R8A8_UNORM = 44,
    VK_FORMAT_B8G8R8A8_SRGB = 50,
    VK_FORMAT_R16_SFLOAT = 76,
    VK_FORMAT_R16G16_SFLOAT = 83,
    VK_FORMAT_R16G16B16A16_SFLOAT = 97,
    VK_FORMAT_R32_SFLOAT = 100,
    VK_FORMAT_R32G32_SFLOAT = 103,
    VK_FORMAT_R32G32B32_SFLOAT = 106,
    VK_FORMAT_R32G32B32A32_SFLOAT = 109,
    VK_FORMAT_D16_UNORM = 124,
    VK_FORMAT_D32_SFLOAT = 126,
    VK_FORMAT_D24_UNORM_S8_UINT = 129,
    VK_FORMAT_D32_SFLOAT_S8_UINT = 130,
} VkFormat;

/* Application info */
typedef struct VkApplicationInfo {
    VkStructureType sType;
    const void* pNext;
    const char* pApplicationName;
    uint32_t applicationVersion;
    const char* pEngineName;
    uint32_t engineVersion;
    uint32_t apiVersion;
} VkApplicationInfo;

/* Instance create info */
typedef struct VkInstanceCreateInfo {
    VkStructureType sType;
    const void* pNext;
    VkFlags flags;
    const VkApplicationInfo* pApplicationInfo;
    uint32_t enabledLayerCount;
    const char* const* ppEnabledLayerNames;
    uint32_t enabledExtensionCount;
    const char* const* ppEnabledExtensionNames;
} VkInstanceCreateInfo;

/* Physical device properties */
typedef struct VkPhysicalDeviceProperties {
    uint32_t apiVersion;
    uint32_t driverVersion;
    uint32_t vendorID;
    uint32_t deviceID;
    uint32_t deviceType;
    char deviceName[256];
    uint8_t pipelineCacheUUID[16];
    /* VkPhysicalDeviceLimits limits; */
    /* VkPhysicalDeviceSparseProperties sparseProperties; */
} VkPhysicalDeviceProperties;

/* Queue family properties */
typedef struct VkQueueFamilyProperties {
    VkFlags queueFlags;
    uint32_t queueCount;
    uint32_t timestampValidBits;
    struct { uint32_t width, height, depth; } minImageTransferGranularity;
} VkQueueFamilyProperties;

/* Queue flags */
#define VK_QUEUE_GRAPHICS_BIT       0x00000001
#define VK_QUEUE_COMPUTE_BIT        0x00000002
#define VK_QUEUE_TRANSFER_BIT       0x00000004
#define VK_QUEUE_SPARSE_BINDING_BIT 0x00000008

/* Device queue create info */
typedef struct VkDeviceQueueCreateInfo {
    VkStructureType sType;
    const void* pNext;
    VkFlags flags;
    uint32_t queueFamilyIndex;
    uint32_t queueCount;
    const float* pQueuePriorities;
} VkDeviceQueueCreateInfo;

/* Device create info */
typedef struct VkDeviceCreateInfo {
    VkStructureType sType;
    const void* pNext;
    VkFlags flags;
    uint32_t queueCreateInfoCount;
    const VkDeviceQueueCreateInfo* pQueueCreateInfos;
    uint32_t enabledLayerCount;
    const char* const* ppEnabledLayerNames;
    uint32_t enabledExtensionCount;
    const char* const* ppEnabledExtensionNames;
    const void* pEnabledFeatures;
} VkDeviceCreateInfo;

/* Extent */
typedef struct VkExtent2D {
    uint32_t width;
    uint32_t height;
} VkExtent2D;

typedef struct VkExtent3D {
    uint32_t width;
    uint32_t height;
    uint32_t depth;
} VkExtent3D;

/* Viewport */
typedef struct VkViewport {
    float x;
    float y;
    float width;
    float height;
    float minDepth;
    float maxDepth;
} VkViewport;

/* Rect */
typedef struct VkRect2D {
    struct { int32_t x, y; } offset;
    VkExtent2D extent;
} VkRect2D;

/* Clear values */
typedef union VkClearColorValue {
    float float32[4];
    int32_t int32[4];
    uint32_t uint32[4];
} VkClearColorValue;

typedef struct VkClearDepthStencilValue {
    float depth;
    uint32_t stencil;
} VkClearDepthStencilValue;

typedef union VkClearValue {
    VkClearColorValue color;
    VkClearDepthStencilValue depthStencil;
} VkClearValue;

/* Core functions */
VkResult vkCreateInstance(const VkInstanceCreateInfo* pCreateInfo,
                          const void* pAllocator, VkInstance* pInstance);
void vkDestroyInstance(VkInstance instance, const void* pAllocator);

VkResult vkEnumeratePhysicalDevices(VkInstance instance, uint32_t* pPhysicalDeviceCount,
                                    VkPhysicalDevice* pPhysicalDevices);
void vkGetPhysicalDeviceProperties(VkPhysicalDevice physicalDevice,
                                   VkPhysicalDeviceProperties* pProperties);
void vkGetPhysicalDeviceQueueFamilyProperties(VkPhysicalDevice physicalDevice,
                                              uint32_t* pQueueFamilyPropertyCount,
                                              VkQueueFamilyProperties* pQueueFamilyProperties);

VkResult vkCreateDevice(VkPhysicalDevice physicalDevice, const VkDeviceCreateInfo* pCreateInfo,
                        const void* pAllocator, VkDevice* pDevice);
void vkDestroyDevice(VkDevice device, const void* pAllocator);
void vkGetDeviceQueue(VkDevice device, uint32_t queueFamilyIndex, uint32_t queueIndex,
                      VkQueue* pQueue);

VkResult vkDeviceWaitIdle(VkDevice device);
VkResult vkQueueWaitIdle(VkQueue queue);

/* Memory */
VkResult vkAllocateMemory(VkDevice device, const void* pAllocateInfo,
                          const void* pAllocator, VkDeviceMemory* pMemory);
void vkFreeMemory(VkDevice device, VkDeviceMemory memory, const void* pAllocator);
VkResult vkMapMemory(VkDevice device, VkDeviceMemory memory, VkDeviceSize offset,
                     VkDeviceSize size, VkFlags flags, void** ppData);
void vkUnmapMemory(VkDevice device, VkDeviceMemory memory);

/* Buffer */
VkResult vkCreateBuffer(VkDevice device, const void* pCreateInfo,
                        const void* pAllocator, VkBuffer* pBuffer);
void vkDestroyBuffer(VkDevice device, VkBuffer buffer, const void* pAllocator);
VkResult vkBindBufferMemory(VkDevice device, VkBuffer buffer, VkDeviceMemory memory,
                            VkDeviceSize memoryOffset);

/* Image */
VkResult vkCreateImage(VkDevice device, const void* pCreateInfo,
                       const void* pAllocator, VkImage* pImage);
void vkDestroyImage(VkDevice device, VkImage image, const void* pAllocator);
VkResult vkBindImageMemory(VkDevice device, VkImage image, VkDeviceMemory memory,
                           VkDeviceSize memoryOffset);

/* Image view */
VkResult vkCreateImageView(VkDevice device, const void* pCreateInfo,
                           const void* pAllocator, VkImageView* pView);
void vkDestroyImageView(VkDevice device, VkImageView imageView, const void* pAllocator);

/* Shader module */
VkResult vkCreateShaderModule(VkDevice device, const void* pCreateInfo,
                              const void* pAllocator, VkShaderModule* pShaderModule);
void vkDestroyShaderModule(VkDevice device, VkShaderModule shaderModule, const void* pAllocator);

/* Pipeline */
VkResult vkCreateGraphicsPipelines(VkDevice device, VkPipelineCache pipelineCache,
                                   uint32_t createInfoCount, const void* pCreateInfos,
                                   const void* pAllocator, VkPipeline* pPipelines);
VkResult vkCreateComputePipelines(VkDevice device, VkPipelineCache pipelineCache,
                                  uint32_t createInfoCount, const void* pCreateInfos,
                                  const void* pAllocator, VkPipeline* pPipelines);
void vkDestroyPipeline(VkDevice device, VkPipeline pipeline, const void* pAllocator);

/* Render pass */
VkResult vkCreateRenderPass(VkDevice device, const void* pCreateInfo,
                            const void* pAllocator, VkRenderPass* pRenderPass);
void vkDestroyRenderPass(VkDevice device, VkRenderPass renderPass, const void* pAllocator);

/* Framebuffer */
VkResult vkCreateFramebuffer(VkDevice device, const void* pCreateInfo,
                             const void* pAllocator, VkFramebuffer* pFramebuffer);
void vkDestroyFramebuffer(VkDevice device, VkFramebuffer framebuffer, const void* pAllocator);

/* Command pool */
VkResult vkCreateCommandPool(VkDevice device, const void* pCreateInfo,
                             const void* pAllocator, VkCommandPool* pCommandPool);
void vkDestroyCommandPool(VkDevice device, VkCommandPool commandPool, const void* pAllocator);
VkResult vkResetCommandPool(VkDevice device, VkCommandPool commandPool, VkFlags flags);

/* Command buffer */
VkResult vkAllocateCommandBuffers(VkDevice device, const void* pAllocateInfo,
                                  VkCommandBuffer* pCommandBuffers);
void vkFreeCommandBuffers(VkDevice device, VkCommandPool commandPool,
                          uint32_t commandBufferCount, const VkCommandBuffer* pCommandBuffers);
VkResult vkBeginCommandBuffer(VkCommandBuffer commandBuffer, const void* pBeginInfo);
VkResult vkEndCommandBuffer(VkCommandBuffer commandBuffer);
VkResult vkResetCommandBuffer(VkCommandBuffer commandBuffer, VkFlags flags);

/* Command buffer commands */
void vkCmdBindPipeline(VkCommandBuffer commandBuffer, uint32_t pipelineBindPoint,
                       VkPipeline pipeline);
void vkCmdSetViewport(VkCommandBuffer commandBuffer, uint32_t firstViewport,
                      uint32_t viewportCount, const VkViewport* pViewports);
void vkCmdSetScissor(VkCommandBuffer commandBuffer, uint32_t firstScissor,
                     uint32_t scissorCount, const VkRect2D* pScissors);
void vkCmdDraw(VkCommandBuffer commandBuffer, uint32_t vertexCount, uint32_t instanceCount,
               uint32_t firstVertex, uint32_t firstInstance);
void vkCmdDrawIndexed(VkCommandBuffer commandBuffer, uint32_t indexCount, uint32_t instanceCount,
                      uint32_t firstIndex, int32_t vertexOffset, uint32_t firstInstance);
void vkCmdDispatch(VkCommandBuffer commandBuffer, uint32_t groupCountX,
                   uint32_t groupCountY, uint32_t groupCountZ);
void vkCmdCopyBuffer(VkCommandBuffer commandBuffer, VkBuffer srcBuffer, VkBuffer dstBuffer,
                     uint32_t regionCount, const void* pRegions);
void vkCmdBeginRenderPass(VkCommandBuffer commandBuffer, const void* pRenderPassBegin,
                          uint32_t contents);
void vkCmdEndRenderPass(VkCommandBuffer commandBuffer);

/* Synchronization */
VkResult vkCreateSemaphore(VkDevice device, const void* pCreateInfo,
                           const void* pAllocator, VkSemaphore* pSemaphore);
void vkDestroySemaphore(VkDevice device, VkSemaphore semaphore, const void* pAllocator);
VkResult vkCreateFence(VkDevice device, const void* pCreateInfo,
                       const void* pAllocator, VkFence* pFence);
void vkDestroyFence(VkDevice device, VkFence fence, const void* pAllocator);
VkResult vkWaitForFences(VkDevice device, uint32_t fenceCount, const VkFence* pFences,
                         VkBool32 waitAll, uint64_t timeout);
VkResult vkResetFences(VkDevice device, uint32_t fenceCount, const VkFence* pFences);

/* Queue submit */
VkResult vkQueueSubmit(VkQueue queue, uint32_t submitCount, const void* pSubmits, VkFence fence);

#endif /* _ADEAD_VULKAN_H */
