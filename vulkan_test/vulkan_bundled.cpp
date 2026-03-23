// vulkan.h — ADead-BIB Vulkan Header
// Compatible con Vulkan 1.3 API oficial de Khronos
// Sin dependencias externas — todo integrado en el compilador
//
// Autor: Eddi Andreé Salazar Matos
// Fecha: Marzo 2026

#ifndef ADEAD_VULKAN_H
#define ADEAD_VULKAN_H

// ============================================================================
// TIPOS BÁSICOS
// ============================================================================












#define VK_TRUE  1
#define VK_FALSE 0
#define VK_NULL_HANDLE 0

// ============================================================================
// HANDLES (opaque pointers)
// ============================================================================

struct VkInstance_T* VkInstance;
typedef struct VkPhysicalDevice_T* VkPhysicalDevice;
typedef struct VkDevice_T* VkDevice;
typedef struct VkQueue_T* VkQueue;
typedef struct VkCommandPool_T* VkCommandPool;
typedef struct VkCommandBuffer_T* VkCommandBuffer;
typedef struct VkFence_T* VkFence;
typedef struct VkSemaphore_T* VkSemaphore;
typedef struct VkBuffer_T* VkBuffer;
typedef struct VkDeviceMemory_T* VkDeviceMemory;
typedef struct VkImage_T* VkImage;
typedef struct VkImageView_T* VkImageView;
typedef struct VkSampler_T* VkSampler;
typedef struct VkShaderModule_T* VkShaderModule;
typedef struct VkPipeline_T* VkPipeline;
typedef struct VkPipelineLayout_T* VkPipelineLayout;
typedef struct VkPipelineCache_T* VkPipelineCache;
typedef struct VkRenderPass_T* VkRenderPass;
typedef struct VkFramebuffer_T* VkFramebuffer;
typedef struct VkDescriptorSetLayout_T* VkDescriptorSetLayout;
typedef struct VkDescriptorPool_T* VkDescriptorPool;
typedef struct VkDescriptorSet_T* VkDescriptorSet;
typedef struct VkSurfaceKHR_T* VkSurfaceKHR;
typedef struct VkSwapchainKHR_T* VkSwapchainKHR;

// ============================================================================
// ENUMS - VkResult
// ============================================================================

enum VkResult {

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
    VK_ERROR_SURFACE_LOST_KHR = -1000000000,
    VK_ERROR_NATIVE_WINDOW_IN_USE_KHR = -1000000001,
    VK_SUBOPTIMAL_KHR = 1000001003,
    VK_ERROR_OUT_OF_DATE_KHR = -1000001004,

};

// ============================================================================
// ENUMS - Estructura de tipos
// ============================================================================

enum VkStructureType {

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
    VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR = 1000009000,

};

// ============================================================================
// ENUMS - Formatos
// ============================================================================

enum VkFormat {

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

};

// ============================================================================
// ENUMS - Pipeline
// ============================================================================

enum VkPrimitiveTopology {

    VK_PRIMITIVE_TOPOLOGY_POINT_LIST = 0,
    VK_PRIMITIVE_TOPOLOGY_LINE_LIST = 1,
    VK_PRIMITIVE_TOPOLOGY_LINE_STRIP = 2,
    VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST = 3,
    VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP = 4,
    VK_PRIMITIVE_TOPOLOGY_TRIANGLE_FAN = 5,

};

enum VkPolygonMode {

    VK_POLYGON_MODE_FILL = 0,
    VK_POLYGON_MODE_LINE = 1,
    VK_POLYGON_MODE_POINT = 2,

};

enum VkCullModeFlagBits {

    VK_CULL_MODE_NONE = 0,
    VK_CULL_MODE_FRONT_BIT = 1,
    VK_CULL_MODE_BACK_BIT = 2,
    VK_CULL_MODE_FRONT_AND_BACK = 3,

};

enum VkFrontFace {

    VK_FRONT_FACE_COUNTER_CLOCKWISE = 0,
    VK_FRONT_FACE_CLOCKWISE = 1,

};

enum VkShaderStageFlagBits {

    VK_SHADER_STAGE_VERTEX_BIT = 0x00000001,
    VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT = 0x00000002,
    VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT = 0x00000004,
    VK_SHADER_STAGE_GEOMETRY_BIT = 0x00000008,
    VK_SHADER_STAGE_FRAGMENT_BIT = 0x00000010,
    VK_SHADER_STAGE_COMPUTE_BIT = 0x00000020,
    VK_SHADER_STAGE_ALL_GRAPHICS = 0x0000001F,
    VK_SHADER_STAGE_ALL = 0x7FFFFFFF,

};

// ============================================================================
// ENUMS - Memory & Buffer
// ============================================================================

enum VkMemoryPropertyFlagBits {

    VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT = 0x00000001,
    VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT = 0x00000002,
    VK_MEMORY_PROPERTY_HOST_COHERENT_BIT = 0x00000004,
    VK_MEMORY_PROPERTY_HOST_CACHED_BIT = 0x00000008,
    VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT = 0x00000010,

};

enum VkBufferUsageFlagBits {

    VK_BUFFER_USAGE_TRANSFER_SRC_BIT = 0x00000001,
    VK_BUFFER_USAGE_TRANSFER_DST_BIT = 0x00000002,
    VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT = 0x00000004,
    VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT = 0x00000008,
    VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT = 0x00000010,
    VK_BUFFER_USAGE_STORAGE_BUFFER_BIT = 0x00000020,
    VK_BUFFER_USAGE_INDEX_BUFFER_BIT = 0x00000040,
    VK_BUFFER_USAGE_VERTEX_BUFFER_BIT = 0x00000080,
    VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT = 0x00000100,

};

enum VkSharingMode {

    VK_SHARING_MODE_EXCLUSIVE = 0,
    VK_SHARING_MODE_CONCURRENT = 1,

};

// ============================================================================
// ENUMS - Image
// ============================================================================

enum VkImageType {

    VK_IMAGE_TYPE_1D = 0,
    VK_IMAGE_TYPE_2D = 1,
    VK_IMAGE_TYPE_3D = 2,

};

enum VkImageViewType {

    VK_IMAGE_VIEW_TYPE_1D = 0,
    VK_IMAGE_VIEW_TYPE_2D = 1,
    VK_IMAGE_VIEW_TYPE_3D = 2,
    VK_IMAGE_VIEW_TYPE_CUBE = 3,
    VK_IMAGE_VIEW_TYPE_1D_ARRAY = 4,
    VK_IMAGE_VIEW_TYPE_2D_ARRAY = 5,
    VK_IMAGE_VIEW_TYPE_CUBE_ARRAY = 6,

};

enum VkImageLayout {

    VK_IMAGE_LAYOUT_UNDEFINED = 0,
    VK_IMAGE_LAYOUT_GENERAL = 1,
    VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL = 2,
    VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL = 3,
    VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL = 4,
    VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL = 5,
    VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL = 6,
    VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL = 7,
    VK_IMAGE_LAYOUT_PREINITIALIZED = 8,
    VK_IMAGE_LAYOUT_PRESENT_SRC_KHR = 1000001002,

};

enum VkImageUsageFlagBits {

    VK_IMAGE_USAGE_TRANSFER_SRC_BIT = 0x00000001,
    VK_IMAGE_USAGE_TRANSFER_DST_BIT = 0x00000002,
    VK_IMAGE_USAGE_SAMPLED_BIT = 0x00000004,
    VK_IMAGE_USAGE_STORAGE_BIT = 0x00000008,
    VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT = 0x00000010,
    VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT = 0x00000020,
    VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT = 0x00000040,
    VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT = 0x00000080,

};

// ============================================================================
// ENUMS - Queue
// ============================================================================

enum VkQueueFlagBits {

    VK_QUEUE_GRAPHICS_BIT = 0x00000001,
    VK_QUEUE_COMPUTE_BIT = 0x00000002,
    VK_QUEUE_TRANSFER_BIT = 0x00000004,
    VK_QUEUE_SPARSE_BINDING_BIT = 0x00000008,

};

// ============================================================================
// ENUMS - Command Buffer
// ============================================================================

enum VkCommandBufferLevel {

    VK_COMMAND_BUFFER_LEVEL_PRIMARY = 0,
    VK_COMMAND_BUFFER_LEVEL_SECONDARY = 1,

};

enum VkCommandBufferUsageFlagBits {

    VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT = 0x00000001,
    VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT = 0x00000002,
    VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT = 0x00000004,

};

// ============================================================================
// ENUMS - Attachment
// ============================================================================

enum VkAttachmentLoadOp {

    VK_ATTACHMENT_LOAD_OP_LOAD = 0,
    VK_ATTACHMENT_LOAD_OP_CLEAR = 1,
    VK_ATTACHMENT_LOAD_OP_DONT_CARE = 2,

};

enum VkAttachmentStoreOp {

    VK_ATTACHMENT_STORE_OP_STORE = 0,
    VK_ATTACHMENT_STORE_OP_DONT_CARE = 1,

};

// ============================================================================
// ENUMS - Pipeline Bind Point
// ============================================================================

enum VkPipelineBindPoint {

    VK_PIPELINE_BIND_POINT_GRAPHICS = 0,
    VK_PIPELINE_BIND_POINT_COMPUTE = 1,

};

// ============================================================================
// ENUMS - Blend
// ============================================================================

enum VkBlendFactor {

    VK_BLEND_FACTOR_ZERO = 0,
    VK_BLEND_FACTOR_ONE = 1,
    VK_BLEND_FACTOR_SRC_COLOR = 2,
    VK_BLEND_FACTOR_ONE_MINUS_SRC_COLOR = 3,
    VK_BLEND_FACTOR_DST_COLOR = 4,
    VK_BLEND_FACTOR_ONE_MINUS_DST_COLOR = 5,
    VK_BLEND_FACTOR_SRC_ALPHA = 6,
    VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA = 7,
    VK_BLEND_FACTOR_DST_ALPHA = 8,
    VK_BLEND_FACTOR_ONE_MINUS_DST_ALPHA = 9,

};

enum VkBlendOp {

    VK_BLEND_OP_ADD = 0,
    VK_BLEND_OP_SUBTRACT = 1,
    VK_BLEND_OP_REVERSE_SUBTRACT = 2,
    VK_BLEND_OP_MIN = 3,
    VK_BLEND_OP_MAX = 4,

};

enum VkColorComponentFlagBits {

    VK_COLOR_COMPONENT_R_BIT = 0x00000001,
    VK_COLOR_COMPONENT_G_BIT = 0x00000002,
    VK_COLOR_COMPONENT_B_BIT = 0x00000004,
    VK_COLOR_COMPONENT_A_BIT = 0x00000008,

};

// ============================================================================
// ENUMS - Present Mode
// ============================================================================

enum VkPresentModeKHR {

    VK_PRESENT_MODE_IMMEDIATE_KHR = 0,
    VK_PRESENT_MODE_MAILBOX_KHR = 1,
    VK_PRESENT_MODE_FIFO_KHR = 2,
    VK_PRESENT_MODE_FIFO_RELAXED_KHR = 3,

};

enum VkColorSpaceKHR {

    VK_COLOR_SPACE_SRGB_NONLINEAR_KHR = 0,

};

enum VkCompositeAlphaFlagBitsKHR {

    VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR = 0x00000001,
    VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR = 0x00000002,
    VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR = 0x00000004,
    VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR = 0x00000008,

};

// ============================================================================
// STRUCTS - Basic
// ============================================================================

struct VkExtent2D {

    unsigned int width;
    unsigned int height;

};

struct VkExtent3D {

    unsigned int width;
    unsigned int height;
    unsigned int depth;

};

struct VkOffset2D {

    int x;
    int y;

};

struct VkOffset3D {

    int x;
    int y;
    int z;

};

struct VkRect2D {

    VkOffset2D offset;
    VkExtent2D extent;

};

struct VkViewport {

    float x;
    float y;
    float width;
    float height;
    float minDepth;
    float maxDepth;

};

struct VkClearColorValue {

    float float32[4];
    // // // // int int32[4];
    // // // // unsigned int uint32[4];

};

struct VkClearDepthStencilValue {

    float depth;
    unsigned int stencil;

};

struct VkClearValue {

    VkClearColorValue color;
    // // // // VkClearDepthStencilValue depthStencil;

};

// ============================================================================
// STRUCTS - Application & Instance
// ============================================================================

struct VkApplicationInfo {

    VkStructureType sType;
    const void* pNext;
    const char* pApplicationName;
    unsigned int applicationVersion;
    const char* pEngineName;
    unsigned int engineVersion;
    unsigned int apiVersion;

};

struct VkInstanceCreateInfo {

    VkStructureType sType;
    const void* pNext;
    unsigned int flags;
    const VkApplicationInfo* pApplicationInfo;
    unsigned int enabledLayerCount;
    const char* const* ppEnabledLayerNames;
    unsigned int enabledExtensionCount;
    const char* const* ppEnabledExtensionNames;

};

// ============================================================================
// STRUCTS - Device
// ============================================================================

struct VkPhysicalDeviceProperties {

    unsigned int apiVersion;
    unsigned int driverVersion;
    unsigned int vendorID;
    unsigned int deviceID;
    unsigned int deviceType;
    char deviceName[256];
    uint8_t pipelineCacheUUID[16];
    // VkPhysicalDeviceLimits limits; // Simplified
    // VkPhysicalDeviceSparseProperties sparseProperties;

};

struct VkQueueFamilyProperties {

    unsigned int queueFlags;
    unsigned int queueCount;
    unsigned int timestampValidBits;
    VkExtent3D minImageTransferGranularity;

};

struct VkDeviceQueueCreateInfo {

    VkStructureType sType;
    const void* pNext;
    unsigned int flags;
    unsigned int queueFamilyIndex;
    unsigned int queueCount;
    const float* pQueuePriorities;

};

struct VkDeviceCreateInfo {

    VkStructureType sType;
    const void* pNext;
    unsigned int flags;
    unsigned int queueCreateInfoCount;
    const VkDeviceQueueCreateInfo* pQueueCreateInfos;
    unsigned int enabledLayerCount;
    const char* const* ppEnabledLayerNames;
    unsigned int enabledExtensionCount;
    const char* const* ppEnabledExtensionNames;
    const void* pEnabledFeatures;

};

// ============================================================================
// STRUCTS - Memory
// ============================================================================

struct VkMemoryRequirements {

    unsigned long long size;
    unsigned long long alignment;
    unsigned int memoryTypeBits;

};

struct VkMemoryAllocateInfo {

    VkStructureType sType;
    const void* pNext;
    unsigned long long allocationSize;
    unsigned int memoryTypeIndex;

};

struct VkMemoryType {

    unsigned int propertyFlags;
    unsigned int heapIndex;

};

struct VkMemoryHeap {

    unsigned long long size;
    unsigned int flags;

};

struct VkPhysicalDeviceMemoryProperties {

    unsigned int memoryTypeCount;
    VkMemoryType memoryTypes[32];
    unsigned int memoryHeapCount;
    VkMemoryHeap memoryHeaps[16];

};

// ============================================================================
// STRUCTS - Buffer
// ============================================================================

struct VkBufferCreateInfo {

    VkStructureType sType;
    const void* pNext;
    unsigned int flags;
    unsigned long long size;
    unsigned int usage;
    VkSharingMode sharingMode;
    unsigned int queueFamilyIndexCount;
    const unsigned int* pQueueFamilyIndices;

};

// ============================================================================
// STRUCTS - Image
// ============================================================================

struct VkImageCreateInfo {

    VkStructureType sType;
    const void* pNext;
    unsigned int flags;
    VkImageType imageType;
    VkFormat format;
    VkExtent3D extent;
    unsigned int mipLevels;
    unsigned int arrayLayers;
    unsigned int samples;
    unsigned int tiling;
    unsigned int usage;
    VkSharingMode sharingMode;
    unsigned int queueFamilyIndexCount;
    const unsigned int* pQueueFamilyIndices;
    VkImageLayout initialLayout;

};

struct VkComponentMapping {

    unsigned int r;
    unsigned int g;
    unsigned int b;
    unsigned int a;

};

struct VkImageSubresourceRange {

    unsigned int aspectMask;
    unsigned int baseMipLevel;
    unsigned int levelCount;
    unsigned int baseArrayLayer;
    unsigned int layerCount;

};

struct VkImageViewCreateInfo {

    VkStructureType sType;
    const void* pNext;
    unsigned int flags;
    VkImage image;
    VkImageViewType viewType;
    VkFormat format;
    VkComponentMapping components;
    VkImageSubresourceRange subresourceRange;

};

// ============================================================================
// STRUCTS - Shader & Pipeline
// ============================================================================

struct VkShaderModuleCreateInfo {

    VkStructureType sType;
    const void* pNext;
    unsigned int flags;
    unsigned long long codeSize;
    const unsigned int* pCode;

};

struct VkPipelineShaderStageCreateInfo {

    VkStructureType sType;
    const void* pNext;
    unsigned int flags;
    VkShaderStageFlagBits stage;
    VkShaderModule module;
    const char* pName;
    const void* pSpecializationInfo;

};

struct VkVertexInputBindingDescription {

    unsigned int binding;
    unsigned int stride;
    unsigned int inputRate;

};

struct VkVertexInputAttributeDescription {

    unsigned int location;
    unsigned int binding;
    VkFormat format;
    unsigned int offset;

};

struct VkPipelineVertexInputStateCreateInfo {

    VkStructureType sType;
    const void* pNext;
    unsigned int flags;
    unsigned int vertexBindingDescriptionCount;
    const VkVertexInputBindingDescription* pVertexBindingDescriptions;
    unsigned int vertexAttributeDescriptionCount;
    const VkVertexInputAttributeDescription* pVertexAttributeDescriptions;

};

struct VkPipelineInputAssemblyStateCreateInfo {

    VkStructureType sType;
    const void* pNext;
    unsigned int flags;
    VkPrimitiveTopology topology;
    unsigned int primitiveRestartEnable;

};

struct VkPipelineViewportStateCreateInfo {

    VkStructureType sType;
    const void* pNext;
    unsigned int flags;
    unsigned int viewportCount;
    const VkViewport* pViewports;
    unsigned int scissorCount;
    const VkRect2D* pScissors;

};

struct VkPipelineRasterizationStateCreateInfo {

    VkStructureType sType;
    const void* pNext;
    unsigned int flags;
    unsigned int depthClampEnable;
    unsigned int rasterizerDiscardEnable;
    VkPolygonMode polygonMode;
    unsigned int cullMode;
    VkFrontFace frontFace;
    unsigned int depthBiasEnable;
    float depthBiasConstantFactor;
    float depthBiasClamp;
    float depthBiasSlopeFactor;
    float lineWidth;

};

struct VkPipelineMultisampleStateCreateInfo {

    VkStructureType sType;
    const void* pNext;
    unsigned int flags;
    unsigned int rasterizationSamples;
    unsigned int sampleShadingEnable;
    float minSampleShading;
    const unsigned int* pSampleMask;
    unsigned int alphaToCoverageEnable;
    unsigned int alphaToOneEnable;

};

struct VkPipelineColorBlendAttachmentState {

    unsigned int blendEnable;
    VkBlendFactor srcColorBlendFactor;
    VkBlendFactor dstColorBlendFactor;
    VkBlendOp colorBlendOp;
    VkBlendFactor srcAlphaBlendFactor;
    VkBlendFactor dstAlphaBlendFactor;
    VkBlendOp alphaBlendOp;
    unsigned int colorWriteMask;

};

struct VkPipelineColorBlendStateCreateInfo {

    VkStructureType sType;
    const void* pNext;
    unsigned int flags;
    unsigned int logicOpEnable;
    unsigned int logicOp;
    unsigned int attachmentCount;
    const VkPipelineColorBlendAttachmentState* pAttachments;
    float blendConstants[4];

};

struct VkPipelineLayoutCreateInfo {

    VkStructureType sType;
    const void* pNext;
    unsigned int flags;
    unsigned int setLayoutCount;
    const VkDescriptorSetLayout* pSetLayouts;
    unsigned int pushConstantRangeCount;
    const void* pPushConstantRanges;

};

struct VkGraphicsPipelineCreateInfo {

    VkStructureType sType;
    const void* pNext;
    unsigned int flags;
    unsigned int stageCount;
    const VkPipelineShaderStageCreateInfo* pStages;
    const VkPipelineVertexInputStateCreateInfo* pVertexInputState;
    const VkPipelineInputAssemblyStateCreateInfo* pInputAssemblyState;
    const void* pTessellationState;
    const VkPipelineViewportStateCreateInfo* pViewportState;
    const VkPipelineRasterizationStateCreateInfo* pRasterizationState;
    const VkPipelineMultisampleStateCreateInfo* pMultisampleState;
    const void* pDepthStencilState;
    const VkPipelineColorBlendStateCreateInfo* pColorBlendState;
    const void* pDynamicState;
    VkPipelineLayout layout;
    VkRenderPass renderPass;
    unsigned int subpass;
    VkPipeline basePipelineHandle;
    int basePipelineIndex;

};

// ============================================================================
// STRUCTS - Render Pass
// ============================================================================

struct VkAttachmentDescription {

    unsigned int flags;
    VkFormat format;
    unsigned int samples;
    VkAttachmentLoadOp loadOp;
    VkAttachmentStoreOp storeOp;
    VkAttachmentLoadOp stencilLoadOp;
    VkAttachmentStoreOp stencilStoreOp;
    VkImageLayout initialLayout;
    VkImageLayout finalLayout;

};

struct VkAttachmentReference {

    unsigned int attachment;
    VkImageLayout layout;

};

struct VkSubpassDescription {

    unsigned int flags;
    VkPipelineBindPoint pipelineBindPoint;
    unsigned int inputAttachmentCount;
    const VkAttachmentReference* pInputAttachments;
    unsigned int colorAttachmentCount;
    const VkAttachmentReference* pColorAttachments;
    const VkAttachmentReference* pResolveAttachments;
    const VkAttachmentReference* pDepthStencilAttachment;
    unsigned int preserveAttachmentCount;
    const unsigned int* pPreserveAttachments;

};

struct VkSubpassDependency {

    unsigned int srcSubpass;
    unsigned int dstSubpass;
    unsigned int srcStageMask;
    unsigned int dstStageMask;
    unsigned int srcAccessMask;
    unsigned int dstAccessMask;
    unsigned int dependencyFlags;

};

struct VkRenderPassCreateInfo {

    VkStructureType sType;
    const void* pNext;
    unsigned int flags;
    unsigned int attachmentCount;
    const VkAttachmentDescription* pAttachments;
    unsigned int subpassCount;
    const VkSubpassDescription* pSubpasses;
    unsigned int dependencyCount;
    const VkSubpassDependency* pDependencies;

};

struct VkRenderPassBeginInfo {

    VkStructureType sType;
    const void* pNext;
    VkRenderPass renderPass;
    VkFramebuffer framebuffer;
    VkRect2D renderArea;
    unsigned int clearValueCount;
    const VkClearValue* pClearValues;

};

// ============================================================================
// STRUCTS - Framebuffer
// ============================================================================

struct VkFramebufferCreateInfo {

    VkStructureType sType;
    const void* pNext;
    unsigned int flags;
    VkRenderPass renderPass;
    unsigned int attachmentCount;
    const VkImageView* pAttachments;
    unsigned int width;
    unsigned int height;
    unsigned int layers;

};

// ============================================================================
// STRUCTS - Command Buffer
// ============================================================================

struct VkCommandPoolCreateInfo {

    VkStructureType sType;
    const void* pNext;
    unsigned int flags;
    unsigned int queueFamilyIndex;

};

struct VkCommandBufferAllocateInfo {

    VkStructureType sType;
    const void* pNext;
    VkCommandPool commandPool;
    VkCommandBufferLevel level;
    unsigned int commandBufferCount;

};

struct VkCommandBufferBeginInfo {

    VkStructureType sType;
    const void* pNext;
    unsigned int flags;
    const void* pInheritanceInfo;

};

// ============================================================================
// STRUCTS - Synchronization
// ============================================================================

struct VkFenceCreateInfo {

    VkStructureType sType;
    const void* pNext;
    unsigned int flags;

};

struct VkSemaphoreCreateInfo {

    VkStructureType sType;
    const void* pNext;
    unsigned int flags;

};

struct VkSubmitInfo {

    VkStructureType sType;
    const void* pNext;
    unsigned int waitSemaphoreCount;
    const VkSemaphore* pWaitSemaphores;
    const unsigned int* pWaitDstStageMask;
    unsigned int commandBufferCount;
    const VkCommandBuffer* pCommandBuffers;
    unsigned int signalSemaphoreCount;
    const VkSemaphore* pSignalSemaphores;

};

// ============================================================================
// STRUCTS - Swapchain (KHR Extension)
// ============================================================================

struct VkSurfaceCapabilitiesKHR {

    unsigned int minImageCount;
    unsigned int maxImageCount;
    VkExtent2D currentExtent;
    VkExtent2D minImageExtent;
    VkExtent2D maxImageExtent;
    unsigned int maxImageArrayLayers;
    unsigned int supportedTransforms;
    unsigned int currentTransform;
    unsigned int supportedCompositeAlpha;
    unsigned int supportedUsageFlags;

};

struct VkSurfaceFormatKHR {

    VkFormat format;
    VkColorSpaceKHR colorSpace;

};

struct VkSwapchainCreateInfoKHR {

    VkStructureType sType;
    const void* pNext;
    unsigned int flags;
    VkSurfaceKHR surface;
    unsigned int minImageCount;
    VkFormat imageFormat;
    VkColorSpaceKHR imageColorSpace;
    VkExtent2D imageExtent;
    unsigned int imageArrayLayers;
    unsigned int imageUsage;
    VkSharingMode imageSharingMode;
    unsigned int queueFamilyIndexCount;
    const unsigned int* pQueueFamilyIndices;
    unsigned int preTransform;
    VkCompositeAlphaFlagBitsKHR compositeAlpha;
    VkPresentModeKHR presentMode;
    unsigned int clipped;
    VkSwapchainKHR oldSwapchain;

};

struct VkPresentInfoKHR {

    VkStructureType sType;
    const void* pNext;
    unsigned int waitSemaphoreCount;
    const VkSemaphore* pWaitSemaphores;
    unsigned int swapchainCount;
    const VkSwapchainKHR* pSwapchains;
    const unsigned int* pImageIndices;
    VkResult* pResults;

};

// ============================================================================
// STRUCTS - Win32 Surface (Windows)
// ============================================================================

struct VkWin32SurfaceCreateInfoKHR {

    VkStructureType sType;
    const void* pNext;
    unsigned int flags;
    void* hinstance;
    void* hwnd;

};

// ============================================================================
// API VERSION MACROS
// ============================================================================

#define VK_API_VERSION_1_0 ((1 << 22) | (0 << 12) | 0)
#define VK_API_VERSION_1_1 ((1 << 22) | (1 << 12) | 0)
#define VK_API_VERSION_1_2 ((1 << 22) | (2 << 12) | 0)
#define VK_API_VERSION_1_3 ((1 << 22) | (3 << 12) | 0)

#define VK_MAKE_VERSION(major, minor, patch) \
    (((major) << 22) | ((minor) << 12) | (patch))

#define VK_VERSION_MAJOR(version) ((unsigned int)(version) >> 22)
#define VK_VERSION_MINOR(version) (((unsigned int)(version) >> 12) & 0x3FF)
#define VK_VERSION_PATCH(version) ((unsigned int)(version) & 0xFFF)

// ============================================================================
// // ============================================================================
// FUNCTION POINTERS (Dynamic Loading)
// ============================================================================

typedef VkResult (*PFN_vkCreateInstance)(const VkInstanceCreateInfo* pCreateInfo, const void* pAllocator, VkInstance* pInstance);
typedef void (*PFN_vkDestroyInstance)(VkInstance instance, const void* pAllocator);
typedef VkResult (*PFN_vkEnumeratePhysicalDevices)(VkInstance instance, unsigned int* pPhysicalDeviceCount, VkPhysicalDevice* pPhysicalDevices);
typedef void (*PFN_vkGetPhysicalDeviceProperties)(VkPhysicalDevice physicalDevice, VkPhysicalDeviceProperties* pProperties);
typedef void (*PFN_vkGetPhysicalDeviceQueueFamilyProperties)(VkPhysicalDevice physicalDevice, unsigned int* pQueueFamilyPropertyCount, VkQueueFamilyProperties* pQueueFamilyProperties);
typedef void (*PFN_vkGetPhysicalDeviceMemoryProperties)(VkPhysicalDevice physicalDevice, VkPhysicalDeviceMemoryProperties* pMemoryProperties);
typedef VkResult (*PFN_vkCreateDevice)(VkPhysicalDevice physicalDevice, const VkDeviceCreateInfo* pCreateInfo, const void* pAllocator, VkDevice* pDevice);
typedef void (*PFN_vkDestroyDevice)(VkDevice device, const void* pAllocator);
typedef void (*PFN_vkGetDeviceQueue)(VkDevice device, unsigned int queueFamilyIndex, unsigned int queueIndex, VkQueue* pQueue);
typedef VkResult (*PFN_vkDeviceWaitIdle)(VkDevice device);
typedef VkResult (*PFN_vkAllocateMemory)(VkDevice device, const VkMemoryAllocateInfo* pAllocateInfo, const void* pAllocator, VkDeviceMemory* pMemory);
typedef void (*PFN_vkFreeMemory)(VkDevice device, VkDeviceMemory memory, const void* pAllocator);
typedef VkResult (*PFN_vkMapMemory)(VkDevice device, VkDeviceMemory memory, unsigned long long offset, unsigned long long size, unsigned int flags, void** ppData);
typedef void (*PFN_vkUnmapMemory)(VkDevice device, VkDeviceMemory memory);
typedef VkResult (*PFN_vkCreateBuffer)(VkDevice device, const VkBufferCreateInfo* pCreateInfo, const void* pAllocator, VkBuffer* pBuffer);
typedef void (*PFN_vkDestroyBuffer)(VkDevice device, VkBuffer buffer, const void* pAllocator);
typedef void (*PFN_vkGetBufferMemoryRequirements)(VkDevice device, VkBuffer buffer, VkMemoryRequirements* pMemoryRequirements);
typedef VkResult (*PFN_vkBindBufferMemory)(VkDevice device, VkBuffer buffer, VkDeviceMemory memory, unsigned long long memoryOffset);
typedef VkResult (*PFN_vkCreateImage)(VkDevice device, const VkImageCreateInfo* pCreateInfo, const void* pAllocator, VkImage* pImage);
typedef void (*PFN_vkDestroyImage)(VkDevice device, VkImage image, const void* pAllocator);
typedef VkResult (*PFN_vkCreateImageView)(VkDevice device, const VkImageViewCreateInfo* pCreateInfo, const void* pAllocator, VkImageView* pView);
typedef void (*PFN_vkDestroyImageView)(VkDevice device, VkImageView imageView, const void* pAllocator);
typedef VkResult (*PFN_vkCreateShaderModule)(VkDevice device, const VkShaderModuleCreateInfo* pCreateInfo, const void* pAllocator, VkShaderModule* pShaderModule);
typedef void (*PFN_vkDestroyShaderModule)(VkDevice device, VkShaderModule shaderModule, const void* pAllocator);
typedef VkResult (*PFN_vkCreateGraphicsPipelines)(VkDevice device, VkPipelineCache pipelineCache, unsigned int createInfoCount, const VkGraphicsPipelineCreateInfo* pCreateInfos, const void* pAllocator, VkPipeline* pPipelines);
typedef void (*PFN_vkDestroyPipeline)(VkDevice device, VkPipeline pipeline, const void* pAllocator);
typedef VkResult (*PFN_vkCreatePipelineLayout)(VkDevice device, const VkPipelineLayoutCreateInfo* pCreateInfo, const void* pAllocator, VkPipelineLayout* pPipelineLayout);
typedef void (*PFN_vkDestroyPipelineLayout)(VkDevice device, VkPipelineLayout pipelineLayout, const void* pAllocator);
typedef VkResult (*PFN_vkCreateRenderPass)(VkDevice device, const VkRenderPassCreateInfo* pCreateInfo, const void* pAllocator, VkRenderPass* pRenderPass);
typedef void (*PFN_vkDestroyRenderPass)(VkDevice device, VkRenderPass renderPass, const void* pAllocator);
typedef VkResult (*PFN_vkCreateFramebuffer)(VkDevice device, const VkFramebufferCreateInfo* pCreateInfo, const void* pAllocator, VkFramebuffer* pFramebuffer);
typedef void (*PFN_vkDestroyFramebuffer)(VkDevice device, VkFramebuffer framebuffer, const void* pAllocator);
typedef VkResult (*PFN_vkCreateCommandPool)(VkDevice device, const VkCommandPoolCreateInfo* pCreateInfo, const void* pAllocator, VkCommandPool* pCommandPool);
typedef void (*PFN_vkDestroyCommandPool)(VkDevice device, VkCommandPool commandPool, const void* pAllocator);
typedef VkResult (*PFN_vkAllocateCommandBuffers)(VkDevice device, const VkCommandBufferAllocateInfo* pAllocateInfo, VkCommandBuffer* pCommandBuffers);
typedef void (*PFN_vkFreeCommandBuffers)(VkDevice device, VkCommandPool commandPool, unsigned int commandBufferCount, const VkCommandBuffer* pCommandBuffers);
typedef VkResult (*PFN_vkBeginCommandBuffer)(VkCommandBuffer commandBuffer, const VkCommandBufferBeginInfo* pBeginInfo);
typedef VkResult (*PFN_vkEndCommandBuffer)(VkCommandBuffer commandBuffer);
typedef VkResult (*PFN_vkResetCommandBuffer)(VkCommandBuffer commandBuffer, unsigned int flags);
typedef void (*PFN_vkCmdBeginRenderPass)(VkCommandBuffer commandBuffer, const VkRenderPassBeginInfo* pRenderPassBegin, unsigned int contents);
typedef void (*PFN_vkCmdEndRenderPass)(VkCommandBuffer commandBuffer);
typedef void (*PFN_vkCmdBindPipeline)(VkCommandBuffer commandBuffer, VkPipelineBindPoint pipelineBindPoint, VkPipeline pipeline);
typedef void (*PFN_vkCmdSetViewport)(VkCommandBuffer commandBuffer, unsigned int firstViewport, unsigned int viewportCount, const VkViewport* pViewports);
typedef void (*PFN_vkCmdSetScissor)(VkCommandBuffer commandBuffer, unsigned int firstScissor, unsigned int scissorCount, const VkRect2D* pScissors);
typedef void (*PFN_vkCmdDraw)(VkCommandBuffer commandBuffer, unsigned int vertexCount, unsigned int instanceCount, unsigned int firstVertex, unsigned int firstInstance);
typedef void (*PFN_vkCmdDrawIndexed)(VkCommandBuffer commandBuffer, unsigned int indexCount, unsigned int instanceCount, unsigned int firstIndex, int vertexOffset, unsigned int firstInstance);
typedef void (*PFN_vkCmdBindVertexBuffers)(VkCommandBuffer commandBuffer, unsigned int firstBinding, unsigned int bindingCount, const VkBuffer* pBuffers, const unsigned long long* pOffsets);
typedef void (*PFN_vkCmdBindIndexBuffer)(VkCommandBuffer commandBuffer, VkBuffer buffer, unsigned long long offset, unsigned int indexType);
typedef void (*PFN_vkCmdCopyBuffer)(VkCommandBuffer commandBuffer, VkBuffer srcBuffer, VkBuffer dstBuffer, unsigned int regionCount, const void* pRegions);
typedef VkResult (*PFN_vkCreateFence)(VkDevice device, const VkFenceCreateInfo* pCreateInfo, const void* pAllocator, VkFence* pFence);
typedef void (*PFN_vkDestroyFence)(VkDevice device, VkFence fence, const void* pAllocator);
typedef VkResult (*PFN_vkWaitForFences)(VkDevice device, unsigned int fenceCount, const VkFence* pFences, unsigned int waitAll, unsigned long long timeout);
typedef VkResult (*PFN_vkResetFences)(VkDevice device, unsigned int fenceCount, const VkFence* pFences);
typedef VkResult (*PFN_vkCreateSemaphore)(VkDevice device, const VkSemaphoreCreateInfo* pCreateInfo, const void* pAllocator, VkSemaphore* pSemaphore);
typedef void (*PFN_vkDestroySemaphore)(VkDevice device, VkSemaphore semaphore, const void* pAllocator);
typedef VkResult (*PFN_vkQueueSubmit)(VkQueue queue, unsigned int submitCount, const VkSubmitInfo* pSubmits, VkFence fence);
typedef VkResult (*PFN_vkQueueWaitIdle)(VkQueue queue);
typedef void (*PFN_vkDestroySurfaceKHR)(VkInstance instance, VkSurfaceKHR surface, const void* pAllocator);
typedef VkResult (*PFN_vkGetPhysicalDeviceSurfaceSupportKHR)(VkPhysicalDevice physicalDevice, unsigned int queueFamilyIndex, VkSurfaceKHR surface, unsigned int* pSupported);
typedef VkResult (*PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR)(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, VkSurfaceCapabilitiesKHR* pSurfaceCapabilities);
typedef VkResult (*PFN_vkGetPhysicalDeviceSurfaceFormatsKHR)(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, unsigned int* pSurfaceFormatCount, VkSurfaceFormatKHR* pSurfaceFormats);
typedef VkResult (*PFN_vkGetPhysicalDeviceSurfacePresentModesKHR)(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, unsigned int* pPresentModeCount, VkPresentModeKHR* pPresentModes);
typedef VkResult (*PFN_vkCreateWin32SurfaceKHR)(VkInstance instance, const VkWin32SurfaceCreateInfoKHR* pCreateInfo, const void* pAllocator, VkSurfaceKHR* pSurface);
typedef VkResult (*PFN_vkCreateSwapchainKHR)(VkDevice device, const VkSwapchainCreateInfoKHR* pCreateInfo, const void* pAllocator, VkSwapchainKHR* pSwapchain);
typedef void (*PFN_vkDestroySwapchainKHR)(VkDevice device, VkSwapchainKHR swapchain, const void* pAllocator);
typedef VkResult (*PFN_vkGetSwapchainImagesKHR)(VkDevice device, VkSwapchainKHR swapchain, unsigned int* pSwapchainImageCount, VkImage* pSwapchainImages);
typedef VkResult (*PFN_vkAcquireNextImageKHR)(VkDevice device, VkSwapchainKHR swapchain, unsigned long long timeout, VkSemaphore semaphore, VkFence fence, unsigned int* pImageIndex);
typedef VkResult (*PFN_vkQueuePresentKHR)(VkQueue queue, const VkPresentInfoKHR* pPresentInfo);

#ifdef __cplusplus
extern "C" {
#endif

extern PFN_vkCreateInstance vkCreateInstance;
extern PFN_vkDestroyInstance vkDestroyInstance;
extern PFN_vkEnumeratePhysicalDevices vkEnumeratePhysicalDevices;
extern PFN_vkGetPhysicalDeviceProperties vkGetPhysicalDeviceProperties;
extern PFN_vkGetPhysicalDeviceQueueFamilyProperties vkGetPhysicalDeviceQueueFamilyProperties;
extern PFN_vkGetPhysicalDeviceMemoryProperties vkGetPhysicalDeviceMemoryProperties;
extern PFN_vkCreateDevice vkCreateDevice;
extern PFN_vkDestroyDevice vkDestroyDevice;
extern PFN_vkGetDeviceQueue vkGetDeviceQueue;
extern PFN_vkDeviceWaitIdle vkDeviceWaitIdle;
extern PFN_vkAllocateMemory vkAllocateMemory;
extern PFN_vkFreeMemory vkFreeMemory;
extern PFN_vkMapMemory vkMapMemory;
extern PFN_vkUnmapMemory vkUnmapMemory;
extern PFN_vkCreateBuffer vkCreateBuffer;
extern PFN_vkDestroyBuffer vkDestroyBuffer;
extern PFN_vkGetBufferMemoryRequirements vkGetBufferMemoryRequirements;
extern PFN_vkBindBufferMemory vkBindBufferMemory;
extern PFN_vkCreateImage vkCreateImage;
extern PFN_vkDestroyImage vkDestroyImage;
extern PFN_vkCreateImageView vkCreateImageView;
extern PFN_vkDestroyImageView vkDestroyImageView;
extern PFN_vkCreateShaderModule vkCreateShaderModule;
extern PFN_vkDestroyShaderModule vkDestroyShaderModule;
extern PFN_vkCreateGraphicsPipelines vkCreateGraphicsPipelines;
extern PFN_vkDestroyPipeline vkDestroyPipeline;
extern PFN_vkCreatePipelineLayout vkCreatePipelineLayout;
extern PFN_vkDestroyPipelineLayout vkDestroyPipelineLayout;
extern PFN_vkCreateRenderPass vkCreateRenderPass;
extern PFN_vkDestroyRenderPass vkDestroyRenderPass;
extern PFN_vkCreateFramebuffer vkCreateFramebuffer;
extern PFN_vkDestroyFramebuffer vkDestroyFramebuffer;
extern PFN_vkCreateCommandPool vkCreateCommandPool;
extern PFN_vkDestroyCommandPool vkDestroyCommandPool;
extern PFN_vkAllocateCommandBuffers vkAllocateCommandBuffers;
extern PFN_vkFreeCommandBuffers vkFreeCommandBuffers;
extern PFN_vkBeginCommandBuffer vkBeginCommandBuffer;
extern PFN_vkEndCommandBuffer vkEndCommandBuffer;
extern PFN_vkResetCommandBuffer vkResetCommandBuffer;
extern PFN_vkCmdBeginRenderPass vkCmdBeginRenderPass;
extern PFN_vkCmdEndRenderPass vkCmdEndRenderPass;
extern PFN_vkCmdBindPipeline vkCmdBindPipeline;
extern PFN_vkCmdSetViewport vkCmdSetViewport;
extern PFN_vkCmdSetScissor vkCmdSetScissor;
extern PFN_vkCmdDraw vkCmdDraw;
extern PFN_vkCmdDrawIndexed vkCmdDrawIndexed;
extern PFN_vkCmdBindVertexBuffers vkCmdBindVertexBuffers;
extern PFN_vkCmdBindIndexBuffer vkCmdBindIndexBuffer;
extern PFN_vkCmdCopyBuffer vkCmdCopyBuffer;
extern PFN_vkCreateFence vkCreateFence;
extern PFN_vkDestroyFence vkDestroyFence;
extern PFN_vkWaitForFences vkWaitForFences;
extern PFN_vkResetFences vkResetFences;
extern PFN_vkCreateSemaphore vkCreateSemaphore;
extern PFN_vkDestroySemaphore vkDestroySemaphore;
extern PFN_vkQueueSubmit vkQueueSubmit;
extern PFN_vkQueueWaitIdle vkQueueWaitIdle;
extern PFN_vkDestroySurfaceKHR vkDestroySurfaceKHR;
extern PFN_vkGetPhysicalDeviceSurfaceSupportKHR vkGetPhysicalDeviceSurfaceSupportKHR;
extern PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR vkGetPhysicalDeviceSurfaceCapabilitiesKHR;
extern PFN_vkGetPhysicalDeviceSurfaceFormatsKHR vkGetPhysicalDeviceSurfaceFormatsKHR;
extern PFN_vkGetPhysicalDeviceSurfacePresentModesKHR vkGetPhysicalDeviceSurfacePresentModesKHR;
extern PFN_vkCreateWin32SurfaceKHR vkCreateWin32SurfaceKHR;
extern PFN_vkCreateSwapchainKHR vkCreateSwapchainKHR;
extern PFN_vkDestroySwapchainKHR vkDestroySwapchainKHR;
extern PFN_vkGetSwapchainImagesKHR vkGetSwapchainImagesKHR;
extern PFN_vkAcquireNextImageKHR vkAcquireNextImageKHR;
extern PFN_vkQueuePresentKHR vkQueuePresentKHR;

int adLoadVulkanLibrary();
void adLoadInstanceFunctions(VkInstance instance);

#ifdef __cplusplus
}
#endif

#endif // ADEAD_VULKAN_H

// ad_vulkan.hpp - ADead-BIB Vulkan C++ Wrapper
// Autor: Eddi Andreé Salazar Matos (ADead-BIB ecosystem)

#pragma once


extern "C" {
    void* malloc(unsigned long long);
    void free(void*);
    int printf(const char*, ...);
    
    // Win32 Minimal Definitions for ADead-BIB Native
    typedef void* HWND;
    typedef void* HINSTANCE;
    typedef void* HICON;
    typedef void* HCURSOR;
    typedef void* HBRUSH;
    typedef void* HMODULE;
    
    struct WNDCLASSA {
        unsigned int style;
        long long lpfnWndProc; // function pointer
        int cbClsExtra;
        int cbWndExtra;
        HINSTANCE hInstance;
        HICON hIcon;
        HCURSOR hCursor;
        HBRUSH hbrBackground;
        const char* lpszMenuName;
        const char* lpszClassName;
    };
    
    struct POINT { long x; long y; };
    struct MSG {
        HWND hwnd;
        unsigned int message;
        unsigned long long wParam;
        long long lParam;
        unsigned int time;
        POINT pt;
        unsigned int lPrivate;
    };

    unsigned short RegisterClassA(const WNDCLASSA* lpWndClass);
    HWND CreateWindowExA(unsigned int dwExStyle, const char* lpClassName, const char* lpWindowName, unsigned int dwStyle, int X, int Y, int nWidth, int nHeight, HWND hWndParent, void* hMenu, HINSTANCE hInstance, void* lpParam);
    int ShowWindow(HWND hWnd, int nCmdShow);
    int UpdateWindow(HWND hWnd);
    int PeekMessageA(MSG* lpMsg, HWND hWnd, unsigned int wMsgFilterMin, unsigned int wMsgFilterMax, unsigned int wRemoveMsg);
    int TranslateMessage(const MSG* lpMsg);
    long long DispatchMessageA(const MSG* lpMsg);
    long long DefWindowProcA(HWND hWnd, unsigned int Msg, unsigned long long wParam, long long lParam);
    void PostQuitMessage(int nExitCode);
    void* GetModuleHandleA(const char* lpModuleName);
    HMODULE LoadLibraryA(const char* lpLibFileName);
    void* GetProcAddress(HMODULE hModule, const char* lpProcName);
}

// Win32 Constants
#define WS_OVERLAPPEDWINDOW 0x00CF0000
#define SW_SHOW 5
#define PM_REMOVE 1
#define WM_DESTROY (unsigned)0x0002
#define WM_CLOSE (unsigned)0x0010

static bool global_window_running = true;

extern "C" long long WindowProc(HWND hwnd, unsigned int uMsg, unsigned long long wParam, long long lParam) {
    if (uMsg == WM_DESTROY || uMsg == WM_CLOSE) {
        global_window_running = false;
        PostQuitMessage(0);
        return 0;
    }
    return DefWindowProcA(hwnd, uMsg, wParam, lParam);
}

namespace ad {

    inline void Check(VkResult result, const char* msg) {
        if ((int)result != (int)VK_SUCCESS) {
            printf("[ADead-BIB Vulkan Error] %s\n", msg);
        }
    }

    class Window {
    public:
        HWND handle = nullptr;
        HINSTANCE hInstance = nullptr;
        unsigned int width;
        unsigned int height;

        Window(unsigned int w, unsigned int h, const char* title) : width(w), height(h) {
            hInstance = (HINSTANCE)GetModuleHandleA(nullptr);
            
            WNDCLASSA wc;
            wc.style = 0;
            wc.lpfnWndProc = (long long)WindowProc;
            wc.cbClsExtra = 0;
            wc.cbWndExtra = 0;
            wc.hInstance = hInstance;
            wc.hIcon = nullptr;
            wc.hCursor = nullptr;
            wc.hbrBackground = nullptr;
            wc.lpszMenuName = nullptr;
            wc.lpszClassName = "ADeadBIBWindowClass";
            
            RegisterClassA(&wc);
            
            handle = CreateWindowExA(
                0, "ADeadBIBWindowClass", title, WS_OVERLAPPEDWINDOW,
                100, 100, width, height, nullptr, nullptr, hInstance, nullptr
            );
            
            ShowWindow(handle, SW_SHOW);
            UpdateWindow(handle);
            printf("ADead-BIB Native Window Created!\n");
        }

        bool update() {
            MSG msg;
            while (PeekMessageA(&msg, nullptr, 0, 0, PM_REMOVE)) {
                TranslateMessage(&msg);
                DispatchMessageA(&msg);
            }
            return global_window_running;
        }
    };

} // namespace ad



// Manual Win32 declarations for ADead-BIB Native C++ Compiler
extern "C" {
    typedef void* HMODULE;
    HMODULE LoadLibraryA(const char* lpLibFileName);
    void* GetProcAddress(HMODULE hModule, const char* lpProcName);
}
PFN_vkCreateInstance vkCreateInstance = 0;
PFN_vkDestroyInstance vkDestroyInstance = 0;
PFN_vkEnumeratePhysicalDevices vkEnumeratePhysicalDevices = 0;
PFN_vkGetPhysicalDeviceProperties vkGetPhysicalDeviceProperties = 0;
PFN_vkGetPhysicalDeviceQueueFamilyProperties vkGetPhysicalDeviceQueueFamilyProperties = 0;
PFN_vkGetPhysicalDeviceMemoryProperties vkGetPhysicalDeviceMemoryProperties = 0;
PFN_vkCreateDevice vkCreateDevice = 0;
PFN_vkDestroyDevice vkDestroyDevice = 0;
PFN_vkGetDeviceQueue vkGetDeviceQueue = 0;
PFN_vkDeviceWaitIdle vkDeviceWaitIdle = 0;
PFN_vkAllocateMemory vkAllocateMemory = 0;
PFN_vkFreeMemory vkFreeMemory = 0;
PFN_vkMapMemory vkMapMemory = 0;
PFN_vkUnmapMemory vkUnmapMemory = 0;
PFN_vkCreateBuffer vkCreateBuffer = 0;
PFN_vkDestroyBuffer vkDestroyBuffer = 0;
PFN_vkGetBufferMemoryRequirements vkGetBufferMemoryRequirements = 0;
PFN_vkBindBufferMemory vkBindBufferMemory = 0;
PFN_vkCreateImage vkCreateImage = 0;
PFN_vkDestroyImage vkDestroyImage = 0;
PFN_vkCreateImageView vkCreateImageView = 0;
PFN_vkDestroyImageView vkDestroyImageView = 0;
PFN_vkCreateShaderModule vkCreateShaderModule = 0;
PFN_vkDestroyShaderModule vkDestroyShaderModule = 0;
PFN_vkCreateGraphicsPipelines vkCreateGraphicsPipelines = 0;
PFN_vkDestroyPipeline vkDestroyPipeline = 0;
PFN_vkCreatePipelineLayout vkCreatePipelineLayout = 0;
PFN_vkDestroyPipelineLayout vkDestroyPipelineLayout = 0;
PFN_vkCreateRenderPass vkCreateRenderPass = 0;
PFN_vkDestroyRenderPass vkDestroyRenderPass = 0;
PFN_vkCreateFramebuffer vkCreateFramebuffer = 0;
PFN_vkDestroyFramebuffer vkDestroyFramebuffer = 0;
PFN_vkCreateCommandPool vkCreateCommandPool = 0;
PFN_vkDestroyCommandPool vkDestroyCommandPool = 0;
PFN_vkAllocateCommandBuffers vkAllocateCommandBuffers = 0;
PFN_vkFreeCommandBuffers vkFreeCommandBuffers = 0;
PFN_vkBeginCommandBuffer vkBeginCommandBuffer = 0;
PFN_vkEndCommandBuffer vkEndCommandBuffer = 0;
PFN_vkResetCommandBuffer vkResetCommandBuffer = 0;
PFN_vkCmdBeginRenderPass vkCmdBeginRenderPass = 0;
PFN_vkCmdEndRenderPass vkCmdEndRenderPass = 0;
PFN_vkCmdBindPipeline vkCmdBindPipeline = 0;
PFN_vkCmdSetViewport vkCmdSetViewport = 0;
PFN_vkCmdSetScissor vkCmdSetScissor = 0;
PFN_vkCmdDraw vkCmdDraw = 0;
PFN_vkCmdDrawIndexed vkCmdDrawIndexed = 0;
PFN_vkCmdBindVertexBuffers vkCmdBindVertexBuffers = 0;
PFN_vkCmdBindIndexBuffer vkCmdBindIndexBuffer = 0;
PFN_vkCmdCopyBuffer vkCmdCopyBuffer = 0;
PFN_vkCreateFence vkCreateFence = 0;
PFN_vkDestroyFence vkDestroyFence = 0;
PFN_vkWaitForFences vkWaitForFences = 0;
PFN_vkResetFences vkResetFences = 0;
PFN_vkCreateSemaphore vkCreateSemaphore = 0;
PFN_vkDestroySemaphore vkDestroySemaphore = 0;
PFN_vkQueueSubmit vkQueueSubmit = 0;
PFN_vkQueueWaitIdle vkQueueWaitIdle = 0;
PFN_vkDestroySurfaceKHR vkDestroySurfaceKHR = 0;
PFN_vkGetPhysicalDeviceSurfaceSupportKHR vkGetPhysicalDeviceSurfaceSupportKHR = 0;
PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR vkGetPhysicalDeviceSurfaceCapabilitiesKHR = 0;
PFN_vkGetPhysicalDeviceSurfaceFormatsKHR vkGetPhysicalDeviceSurfaceFormatsKHR = 0;
PFN_vkGetPhysicalDeviceSurfacePresentModesKHR vkGetPhysicalDeviceSurfacePresentModesKHR = 0;
PFN_vkCreateWin32SurfaceKHR vkCreateWin32SurfaceKHR = 0;
PFN_vkCreateSwapchainKHR vkCreateSwapchainKHR = 0;
PFN_vkDestroySwapchainKHR vkDestroySwapchainKHR = 0;
PFN_vkGetSwapchainImagesKHR vkGetSwapchainImagesKHR = 0;
PFN_vkAcquireNextImageKHR vkAcquireNextImageKHR = 0;
PFN_vkQueuePresentKHR vkQueuePresentKHR = 0;


typedef void (*PFN_vkVoidFunction)(void);
typedef PFN_vkVoidFunction (*PFN_vkGetInstanceProcAddrType)(VkInstance instance, const char* pName);
static PFN_vkGetInstanceProcAddrType real_vkGetInstanceProcAddr = 0;

int adLoadVulkanLibrary() {
    HMODULE lib = LoadLibraryA("vulkan-1.dll");
    if (lib == nullptr) return 0;
    
    real_vkGetInstanceProcAddr = (PFN_vkGetInstanceProcAddrType)GetProcAddress(lib, "vkGetInstanceProcAddr");
    if (real_vkGetInstanceProcAddr == nullptr) return 0;
    
    // Load vkCreateInstance and vkEnumerateInstanceExtensionProperties properly (instance = VK_NULL_HANDLE)
    vkCreateInstance = (PFN_vkCreateInstance)real_vkGetInstanceProcAddr(0, "vkCreateInstance");
    
    return 1;
}

void adLoadInstanceFunctions(VkInstance instance) {
    if (real_vkGetInstanceProcAddr == nullptr || instance == nullptr) return;
    vkCreateInstance = (PFN_vkCreateInstance)real_vkGetInstanceProcAddr(instance, "vkCreateInstance");
    vkDestroyInstance = (PFN_vkDestroyInstance)real_vkGetInstanceProcAddr(instance, "vkDestroyInstance");
    vkEnumeratePhysicalDevices = (PFN_vkEnumeratePhysicalDevices)real_vkGetInstanceProcAddr(instance, "vkEnumeratePhysicalDevices");
    vkGetPhysicalDeviceProperties = (PFN_vkGetPhysicalDeviceProperties)real_vkGetInstanceProcAddr(instance, "vkGetPhysicalDeviceProperties");
    vkGetPhysicalDeviceQueueFamilyProperties = (PFN_vkGetPhysicalDeviceQueueFamilyProperties)real_vkGetInstanceProcAddr(instance, "vkGetPhysicalDeviceQueueFamilyProperties");
    vkGetPhysicalDeviceMemoryProperties = (PFN_vkGetPhysicalDeviceMemoryProperties)real_vkGetInstanceProcAddr(instance, "vkGetPhysicalDeviceMemoryProperties");
    vkCreateDevice = (PFN_vkCreateDevice)real_vkGetInstanceProcAddr(instance, "vkCreateDevice");
    vkDestroyDevice = (PFN_vkDestroyDevice)real_vkGetInstanceProcAddr(instance, "vkDestroyDevice");
    vkGetDeviceQueue = (PFN_vkGetDeviceQueue)real_vkGetInstanceProcAddr(instance, "vkGetDeviceQueue");
    vkDeviceWaitIdle = (PFN_vkDeviceWaitIdle)real_vkGetInstanceProcAddr(instance, "vkDeviceWaitIdle");
    vkAllocateMemory = (PFN_vkAllocateMemory)real_vkGetInstanceProcAddr(instance, "vkAllocateMemory");
    vkFreeMemory = (PFN_vkFreeMemory)real_vkGetInstanceProcAddr(instance, "vkFreeMemory");
    vkMapMemory = (PFN_vkMapMemory)real_vkGetInstanceProcAddr(instance, "vkMapMemory");
    vkUnmapMemory = (PFN_vkUnmapMemory)real_vkGetInstanceProcAddr(instance, "vkUnmapMemory");
    vkCreateBuffer = (PFN_vkCreateBuffer)real_vkGetInstanceProcAddr(instance, "vkCreateBuffer");
    vkDestroyBuffer = (PFN_vkDestroyBuffer)real_vkGetInstanceProcAddr(instance, "vkDestroyBuffer");
    vkGetBufferMemoryRequirements = (PFN_vkGetBufferMemoryRequirements)real_vkGetInstanceProcAddr(instance, "vkGetBufferMemoryRequirements");
    vkBindBufferMemory = (PFN_vkBindBufferMemory)real_vkGetInstanceProcAddr(instance, "vkBindBufferMemory");
    vkCreateImage = (PFN_vkCreateImage)real_vkGetInstanceProcAddr(instance, "vkCreateImage");
    vkDestroyImage = (PFN_vkDestroyImage)real_vkGetInstanceProcAddr(instance, "vkDestroyImage");
    vkCreateImageView = (PFN_vkCreateImageView)real_vkGetInstanceProcAddr(instance, "vkCreateImageView");
    vkDestroyImageView = (PFN_vkDestroyImageView)real_vkGetInstanceProcAddr(instance, "vkDestroyImageView");
    vkCreateShaderModule = (PFN_vkCreateShaderModule)real_vkGetInstanceProcAddr(instance, "vkCreateShaderModule");
    vkDestroyShaderModule = (PFN_vkDestroyShaderModule)real_vkGetInstanceProcAddr(instance, "vkDestroyShaderModule");
    vkCreateGraphicsPipelines = (PFN_vkCreateGraphicsPipelines)real_vkGetInstanceProcAddr(instance, "vkCreateGraphicsPipelines");
    vkDestroyPipeline = (PFN_vkDestroyPipeline)real_vkGetInstanceProcAddr(instance, "vkDestroyPipeline");
    vkCreatePipelineLayout = (PFN_vkCreatePipelineLayout)real_vkGetInstanceProcAddr(instance, "vkCreatePipelineLayout");
    vkDestroyPipelineLayout = (PFN_vkDestroyPipelineLayout)real_vkGetInstanceProcAddr(instance, "vkDestroyPipelineLayout");
    vkCreateRenderPass = (PFN_vkCreateRenderPass)real_vkGetInstanceProcAddr(instance, "vkCreateRenderPass");
    vkDestroyRenderPass = (PFN_vkDestroyRenderPass)real_vkGetInstanceProcAddr(instance, "vkDestroyRenderPass");
    vkCreateFramebuffer = (PFN_vkCreateFramebuffer)real_vkGetInstanceProcAddr(instance, "vkCreateFramebuffer");
    vkDestroyFramebuffer = (PFN_vkDestroyFramebuffer)real_vkGetInstanceProcAddr(instance, "vkDestroyFramebuffer");
    vkCreateCommandPool = (PFN_vkCreateCommandPool)real_vkGetInstanceProcAddr(instance, "vkCreateCommandPool");
    vkDestroyCommandPool = (PFN_vkDestroyCommandPool)real_vkGetInstanceProcAddr(instance, "vkDestroyCommandPool");
    vkAllocateCommandBuffers = (PFN_vkAllocateCommandBuffers)real_vkGetInstanceProcAddr(instance, "vkAllocateCommandBuffers");
    vkFreeCommandBuffers = (PFN_vkFreeCommandBuffers)real_vkGetInstanceProcAddr(instance, "vkFreeCommandBuffers");
    vkBeginCommandBuffer = (PFN_vkBeginCommandBuffer)real_vkGetInstanceProcAddr(instance, "vkBeginCommandBuffer");
    vkEndCommandBuffer = (PFN_vkEndCommandBuffer)real_vkGetInstanceProcAddr(instance, "vkEndCommandBuffer");
    vkResetCommandBuffer = (PFN_vkResetCommandBuffer)real_vkGetInstanceProcAddr(instance, "vkResetCommandBuffer");
    vkCmdBeginRenderPass = (PFN_vkCmdBeginRenderPass)real_vkGetInstanceProcAddr(instance, "vkCmdBeginRenderPass");
    vkCmdEndRenderPass = (PFN_vkCmdEndRenderPass)real_vkGetInstanceProcAddr(instance, "vkCmdEndRenderPass");
    vkCmdBindPipeline = (PFN_vkCmdBindPipeline)real_vkGetInstanceProcAddr(instance, "vkCmdBindPipeline");
    vkCmdSetViewport = (PFN_vkCmdSetViewport)real_vkGetInstanceProcAddr(instance, "vkCmdSetViewport");
    vkCmdSetScissor = (PFN_vkCmdSetScissor)real_vkGetInstanceProcAddr(instance, "vkCmdSetScissor");
    vkCmdDraw = (PFN_vkCmdDraw)real_vkGetInstanceProcAddr(instance, "vkCmdDraw");
    vkCmdDrawIndexed = (PFN_vkCmdDrawIndexed)real_vkGetInstanceProcAddr(instance, "vkCmdDrawIndexed");
    vkCmdBindVertexBuffers = (PFN_vkCmdBindVertexBuffers)real_vkGetInstanceProcAddr(instance, "vkCmdBindVertexBuffers");
    vkCmdBindIndexBuffer = (PFN_vkCmdBindIndexBuffer)real_vkGetInstanceProcAddr(instance, "vkCmdBindIndexBuffer");
    vkCmdCopyBuffer = (PFN_vkCmdCopyBuffer)real_vkGetInstanceProcAddr(instance, "vkCmdCopyBuffer");
    vkCreateFence = (PFN_vkCreateFence)real_vkGetInstanceProcAddr(instance, "vkCreateFence");
    vkDestroyFence = (PFN_vkDestroyFence)real_vkGetInstanceProcAddr(instance, "vkDestroyFence");
    vkWaitForFences = (PFN_vkWaitForFences)real_vkGetInstanceProcAddr(instance, "vkWaitForFences");
    vkResetFences = (PFN_vkResetFences)real_vkGetInstanceProcAddr(instance, "vkResetFences");
    vkCreateSemaphore = (PFN_vkCreateSemaphore)real_vkGetInstanceProcAddr(instance, "vkCreateSemaphore");
    vkDestroySemaphore = (PFN_vkDestroySemaphore)real_vkGetInstanceProcAddr(instance, "vkDestroySemaphore");
    vkQueueSubmit = (PFN_vkQueueSubmit)real_vkGetInstanceProcAddr(instance, "vkQueueSubmit");
    vkQueueWaitIdle = (PFN_vkQueueWaitIdle)real_vkGetInstanceProcAddr(instance, "vkQueueWaitIdle");
    vkDestroySurfaceKHR = (PFN_vkDestroySurfaceKHR)real_vkGetInstanceProcAddr(instance, "vkDestroySurfaceKHR");
    vkGetPhysicalDeviceSurfaceSupportKHR = (PFN_vkGetPhysicalDeviceSurfaceSupportKHR)real_vkGetInstanceProcAddr(instance, "vkGetPhysicalDeviceSurfaceSupportKHR");
    vkGetPhysicalDeviceSurfaceCapabilitiesKHR = (PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR)real_vkGetInstanceProcAddr(instance, "vkGetPhysicalDeviceSurfaceCapabilitiesKHR");
    vkGetPhysicalDeviceSurfaceFormatsKHR = (PFN_vkGetPhysicalDeviceSurfaceFormatsKHR)real_vkGetInstanceProcAddr(instance, "vkGetPhysicalDeviceSurfaceFormatsKHR");
    vkGetPhysicalDeviceSurfacePresentModesKHR = (PFN_vkGetPhysicalDeviceSurfacePresentModesKHR)real_vkGetInstanceProcAddr(instance, "vkGetPhysicalDeviceSurfacePresentModesKHR");
    vkCreateWin32SurfaceKHR = (PFN_vkCreateWin32SurfaceKHR)real_vkGetInstanceProcAddr(instance, "vkCreateWin32SurfaceKHR");
    vkCreateSwapchainKHR = (PFN_vkCreateSwapchainKHR)real_vkGetInstanceProcAddr(instance, "vkCreateSwapchainKHR");
    vkDestroySwapchainKHR = (PFN_vkDestroySwapchainKHR)real_vkGetInstanceProcAddr(instance, "vkDestroySwapchainKHR");
    vkGetSwapchainImagesKHR = (PFN_vkGetSwapchainImagesKHR)real_vkGetInstanceProcAddr(instance, "vkGetSwapchainImagesKHR");
    vkAcquireNextImageKHR = (PFN_vkAcquireNextImageKHR)real_vkGetInstanceProcAddr(instance, "vkAcquireNextImageKHR");
    vkQueuePresentKHR = (PFN_vkQueuePresentKHR)real_vkGetInstanceProcAddr(instance, "vkQueuePresentKHR");

}

// main.cpp — ADead-BIB Vulkan Triangle Example
// Dibuja un triángulo con gradiente RGB usando Vulkan
// Sin SDK externo — usa vulkan.h interno de ADead-BIB
//
// Compilar: adb cxx vulkan_test/main.cpp -o vulkan_triangle.exe
//
// Autor: Eddi Andreé Salazar Matos
// Fecha: Marzo 2026



// Windows API para ventana
typedef void* HWND;
typedef void* HINSTANCE;
typedef unsigned int UINT;
typedef unsigned long DWORD;
typedef long LRESULT;
typedef unsigned long long WPARAM;
typedef long long LPARAM;

#define WS_OVERLAPPEDWINDOW 0x00CF0000
#define WM_DESTROY 0x0002
#define WM_CLOSE 0x0010
#define CS_HREDRAW 0x0002
#define CS_VREDRAW 0x0001
#define IDC_ARROW 32512
#define COLOR_WINDOW 5
#define SW_SHOW 5
#define PM_REMOVE 0x0001

extern "C" {
    HWND CreateWindowExA(DWORD, const char*, const char*, DWORD, int, int, int, int, HWND, void*, HINSTANCE, void*);
    int ShowWindow(HWND, int);
    int UpdateWindow(HWND);
    int DestroyWindow(HWND);
    LRESULT DefWindowProcA(HWND, UINT, WPARAM, LPARAM);
    int PeekMessageA(void*, HWND, UINT, UINT, UINT);
    int TranslateMessage(const void*);
    LRESULT DispatchMessageA(const void*);
    void PostQuitMessage(int);
    HINSTANCE GetModuleHandleA(const char*);
    void* LoadCursorW(HINSTANCE, int);
    int printf(const char*, ...);
    void* malloc(unsigned long long);
    void free(void*);
}

// Constantes
const int WIDTH = 800;
const int HEIGHT = 600;
const char* APP_NAME = "ADead-BIB Vulkan Triangle";

// Variables globales
VkInstance instance;
VkPhysicalDevice physicalDevice;
VkDevice device;
VkQueue graphicsQueue;
VkSurfaceKHR surface;
VkSwapchainKHR swapchain;
VkRenderPass renderPass;
VkPipelineLayout pipelineLayout;
VkPipeline graphicsPipeline;
VkCommandPool commandPool;
VkCommandBuffer commandBuffer;
VkSemaphore imageAvailableSemaphore;
VkSemaphore renderFinishedSemaphore;
VkFence inFlightFence;

unsigned int graphicsQueueFamily = (unsigned)0;
unsigned int swapchainImageCount = (unsigned)0;
VkImage* swapchainImages;
VkImageView* swapchainImageViews;
VkFramebuffer* swapchainFramebuffers;
VkFormat swapchainImageFormat;
VkExtent2D swapchainExtent;

bool running = true;

// SPIR-V compilado del vertex shader (triangle.vert)
// Generado con: glslangValidator -V triangle.vert -o vert.spv
const unsigned int vertShaderCode[] = {
    0x07230203, 0x00010000, 0x0008000b, 0x00000036,
    0x00000000, 0x00020011, 0x00000001, 0x0006000b,
    0x00000001, 0x4c534c47, 0x6474732e, 0x3035342e,
    0x00000000, 0x0003000e, 0x00000000, 0x00000001,
    0x0008000f, 0x00000000, 0x00000004, 0x6e69616d,
    0x00000000, 0x00000022, 0x00000026, 0x00000031,
    0x00030003, 0x00000002, 0x000001c2, 0x00040005,
    0x00000004, 0x6e69616d, 0x00000000, 0x00050005,
    0x0000000c, 0x69736f70, 0x6e6f6974, 0x00000073,
    0x00040005, 0x00000017, 0x6f6c6f63, 0x00007372,
    0x00060005, 0x00000020, 0x505f6c67, 0x65567265,
    0x78657472, 0x00000000, 0x00060006, 0x00000020,
    0x00000000, 0x505f6c67, 0x7469736f, 0x006e6f69,
    0x00030005, 0x00000022, 0x00000000, 0x00060005,
    0x00000026, 0x565f6c67, 0x65747265, 0x646e4978,
    0x00007865, 0x00050005, 0x00000031, 0x67617266,
    0x6f6c6f43, 0x00000072, 0x00050048, 0x00000020,
    0x00000000, 0x0000000b, 0x00000000, 0x00030047,
    0x00000020, 0x00000002, 0x00040047, 0x00000026,
    0x0000000b, 0x0000002a, 0x00040047, 0x00000031,
    0x0000001e, 0x00000000, 0x00020013, 0x00000002,
    0x00030021, 0x00000003, 0x00000002, 0x00030016,
    0x00000006, 0x00000020, 0x00040017, 0x00000007,
    0x00000006, 0x00000002, 0x00040015, 0x00000008,
    0x00000020, 0x00000000, 0x0004002b, 0x00000008,
    0x00000009, 0x00000003, 0x0004001c, 0x0000000a,
    0x00000007, 0x00000009, 0x00040020, 0x0000000b,
    0x00000006, 0x0000000a, 0x0004003b, 0x0000000b,
    0x0000000c, 0x00000006, 0x0004002b, 0x00000006,
    0x0000000d, 0x00000000, 0x0004002b, 0x00000006,
    0x0000000e, 0xbf000000, 0x0005002c, 0x00000007,
    0x0000000f, 0x0000000d, 0x0000000e, 0x0004002b,
    0x00000006, 0x00000010, 0x3f000000, 0x0005002c,
    0x00000007, 0x00000011, 0x00000010, 0x00000010,
    0x0005002c, 0x00000007, 0x00000012, 0x0000000e,
    0x00000010, 0x0006002c, 0x0000000a, 0x00000013,
    0x0000000f, 0x00000011, 0x00000012, 0x00040017,
    0x00000014, 0x00000006, 0x00000003, 0x0004001c,
    0x00000015, 0x00000014, 0x00000009, 0x00040020,
    0x00000016, 0x00000006, 0x00000015, 0x0004003b,
    0x00000016, 0x00000017, 0x00000006, 0x0004002b,
    0x00000006, 0x00000018, 0x3f800000, 0x0006002c,
    0x00000014, 0x00000019, 0x00000018, 0x0000000d,
    0x0000000d, 0x0006002c, 0x00000014, 0x0000001a,
    0x0000000d, 0x00000018, 0x0000000d, 0x0006002c,
    0x00000014, 0x0000001b, 0x0000000d, 0x0000000d,
    0x00000018, 0x0006002c, 0x00000015, 0x0000001c,
    0x00000019, 0x0000001a, 0x0000001b, 0x00040017,
    0x0000001d, 0x00000006, 0x00000004, 0x0003001e,
    0x00000020, 0x0000001d, 0x00040020, 0x00000021,
    0x00000003, 0x00000020, 0x0004003b, 0x00000021,
    0x00000022, 0x00000003, 0x00040015, 0x00000023,
    0x00000020, 0x00000001, 0x0004002b, 0x00000023,
    0x00000024, 0x00000000, 0x00040020, 0x00000025,
    0x00000001, 0x00000023, 0x0004003b, 0x00000025,
    0x00000026, 0x00000001, 0x00040020, 0x00000028,
    0x00000006, 0x00000007, 0x00040020, 0x0000002e,
    0x00000003, 0x0000001d, 0x00040020, 0x00000030,
    0x00000003, 0x00000014, 0x0004003b, 0x00000030,
    0x00000031, 0x00000003, 0x00040020, 0x00000033,
    0x00000006, 0x00000014, 0x00050036, 0x00000002,
    0x00000004, 0x00000000, 0x00000003, 0x000200f8,
    0x00000005, 0x0003003e, 0x0000000c, 0x00000013,
    0x0003003e, 0x00000017, 0x0000001c, 0x0004003d,
    0x00000023, 0x00000027, 0x00000026, 0x00050041,
    0x00000028, 0x00000029, 0x0000000c, 0x00000027,
    0x0004003d, 0x00000007, 0x0000002a, 0x00000029,
    0x00050051, 0x00000006, 0x0000002b, 0x0000002a,
    0x00000000, 0x00050051, 0x00000006, 0x0000002c,
    0x0000002a, 0x00000001, 0x00070050, 0x0000001d,
    0x0000002d, 0x0000002b, 0x0000002c, 0x0000000d,
    0x00000018, 0x00050041, 0x0000002e, 0x0000002f,
    0x00000022, 0x00000024, 0x0003003e, 0x0000002f,
    0x0000002d, 0x0004003d, 0x00000023, 0x00000032,
    0x00000026, 0x00050041, 0x00000033, 0x00000034,
    0x00000017, 0x00000032, 0x0004003d, 0x00000014,
    0x00000035, 0x00000034, 0x0003003e, 0x00000031,
    0x00000035, 0x000100fd, 0x00010038
};

// SPIR-V compilado del fragment shader (triangle.frag)
const unsigned int fragShaderCode[] = {
    0x07230203, 0x00010000, 0x0008000b, 0x00000013,
    0x00000000, 0x00020011, 0x00000001, 0x0006000b,
    0x00000001, 0x4c534c47, 0x6474732e, 0x3035342e,
    0x00000000, 0x0003000e, 0x00000000, 0x00000001,
    0x0007000f, 0x00000004, 0x00000004, 0x6e69616d,
    0x00000000, 0x00000009, 0x0000000b, 0x00030010,
    0x00000004, 0x00000007, 0x00030003, 0x00000002,
    0x000001c2, 0x00040005, 0x00000004, 0x6e69616d,
    0x00000000, 0x00050005, 0x00000009, 0x4374756f,
    0x726f6c6f, 0x00000000, 0x00050005, 0x0000000b,
    0x67617266, 0x6f6c6f43, 0x00000072, 0x00040047,
    0x00000009, 0x0000001e, 0x00000000, 0x00040047,
    0x0000000b, 0x0000001e, 0x00000000, 0x00020013,
    0x00000002, 0x00030021, 0x00000003, 0x00000002,
    0x00030016, 0x00000006, 0x00000020, 0x00040017,
    0x00000007, 0x00000006, 0x00000004, 0x00040020,
    0x00000008, 0x00000003, 0x00000007, 0x0004003b,
    0x00000008, 0x00000009, 0x00000003, 0x00040017,
    0x0000000a, 0x00000006, 0x00000003, 0x00040020,
    0x0000000c, 0x00000001, 0x0000000a, 0x0004003b,
    0x0000000c, 0x0000000b, 0x00000001, 0x0004002b,
    0x00000006, 0x0000000e, 0x3f800000, 0x00050036,
    0x00000002, 0x00000004, 0x00000000, 0x00000003,
    0x000200f8, 0x00000005, 0x0004003d, 0x0000000a,
    0x0000000d, 0x0000000b, 0x00050051, 0x00000006,
    0x0000000f, 0x0000000d, 0x00000000, 0x00050051,
    0x00000006, 0x00000010, 0x0000000d, 0x00000001,
    0x00050051, 0x00000006, 0x00000011, 0x0000000d,
    0x00000002, 0x00070050, 0x00000007, 0x00000012,
    0x0000000f, 0x00000010, 0x00000011, 0x0000000e,
    0x0003003e, 0x00000009, 0x00000012, 0x000100fd,
    0x00010038
};

// Helper: Check VkResult
void checkVk(VkResult result, const char* msg) {
    if ((int)result != (int)VK_SUCCESS) {
        printf("Vulkan Error: %s (code %d)\n", msg, result);
    }
}

// Create Vulkan Instance
void createInstance() {
    VkApplicationInfo appInfo;
    const char* extensions[2];
    VkInstanceCreateInfo createInfo;
    VkResult result;

    if (!adLoadVulkanLibrary()) {
        printf("Failed to load vulkan-1.dll. Install Vulkan Runtime.\n");
        return;
    }

    appInfo.sType = VK_STRUCTURE_TYPE_APPLICATION_INFO;
    appInfo.pNext = 0;
    appInfo.pApplicationName = APP_NAME;
    appInfo.applicationVersion = VK_MAKE_VERSION(1, 0, 0);
    appInfo.pEngineName = "ADead-BIB Engine";
    appInfo.engineVersion = VK_MAKE_VERSION(1, 0, 0);
    appInfo.apiVersion = VK_API_VERSION_1_0;

    extensions[0] = "VK_KHR_surface";
    extensions[1] = "VK_KHR_win32_surface";

    createInfo.sType = VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;
    createInfo.pNext = 0;
    createInfo.flags = 0;
    createInfo.pApplicationInfo = &appInfo;
    createInfo.enabledLayerCount = 0;
    createInfo.ppEnabledLayerNames = 0;
    createInfo.enabledExtensionCount = 2;
    createInfo.ppEnabledExtensionNames = extensions;

    result = vkCreateInstance(&createInfo, 0, &instance);
    checkVk(result, "vkCreateInstance");
    adLoadInstanceFunctions(instance);
    printf("Vulkan instance created and functions loaded dynamically\n");
}

// Pick Physical Device
void pickPhysicalDevice() {
    unsigned int deviceCount = (unsigned)0;
    vkEnumeratePhysicalDevices(instance, &deviceCount, nullptr);
    
    if (deviceCount == (unsigned)0) {
        printf("No Vulkan devices found!\n");
        return;
    }

    VkPhysicalDevice* devices = (VkPhysicalDevice*)malloc(deviceCount * sizeof(VkPhysicalDevice));
    vkEnumeratePhysicalDevices(instance, &deviceCount, devices);
    
    physicalDevice = devices[0];
    
    VkPhysicalDeviceProperties props;
    vkGetPhysicalDeviceProperties(physicalDevice, &props);
    printf("Using GPU: %s\n", props.deviceName);
    
    free(devices);
}

// Find Queue Families
void findQueueFamilies() {
    unsigned int queueFamilyCount = (unsigned)0;
    vkGetPhysicalDeviceQueueFamilyProperties(physicalDevice, &queueFamilyCount, nullptr);
    
    VkQueueFamilyProperties* queueFamilies = (VkQueueFamilyProperties*)malloc(
        queueFamilyCount * sizeof(VkQueueFamilyProperties));
    vkGetPhysicalDeviceQueueFamilyProperties(physicalDevice, &queueFamilyCount, queueFamilies);
    
    for (unsigned int i = (unsigned)0; i < queueFamilyCount; i++) {
        if (queueFamilies[i].queueFlags & VK_QUEUE_GRAPHICS_BIT) {
            graphicsQueueFamily = i;
            break;
        }
    }
    
    free(queueFamilies);
    printf("Graphics queue family: %d\n", graphicsQueueFamily);
}

// Create Logical Device
void createLogicalDevice() {
    float queuePriority = 1.0f;
    
    VkDeviceQueueCreateInfo queueCreateInfo;
    queueCreateInfo.sType = VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO;
    queueCreateInfo.pNext = 0;
    queueCreateInfo.flags = 0;
    queueCreateInfo.queueFamilyIndex = graphicsQueueFamily;
    queueCreateInfo.queueCount = 1;
    queueCreateInfo.pQueuePriorities = &queuePriority;

    const char* deviceExtensions[1];
    deviceExtensions[0] = "VK_KHR_swapchain";

    VkDeviceCreateInfo createInfo;
    createInfo.sType = VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO;
    createInfo.pNext = 0;
    createInfo.flags = 0;
    createInfo.queueCreateInfoCount = 1;
    createInfo.pQueueCreateInfos = &queueCreateInfo;
    createInfo.enabledLayerCount = 0;
    createInfo.ppEnabledLayerNames = 0;
    createInfo.enabledExtensionCount = 1;
    createInfo.ppEnabledExtensionNames = deviceExtensions;
    createInfo.pEnabledFeatures = 0;

    VkResult result = vkCreateDevice(physicalDevice, &createInfo, 0, &device);
    checkVk(result, "vkCreateDevice");
    
    vkGetDeviceQueue(device, graphicsQueueFamily, 0, &graphicsQueue);
    printf("Logical device created\n");
}

// Create Surface (Win32)
void createSurface(HWND hwnd, HINSTANCE hInstance) {
    VkWin32SurfaceCreateInfoKHR createInfo;
    createInfo.sType = VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR;
    createInfo.pNext = 0;
    createInfo.flags = 0;
    createInfo.hinstance = hInstance;
    createInfo.hwnd = hwnd;

    VkResult result = vkCreateWin32SurfaceKHR(instance, &createInfo, 0, &surface);
    checkVk(result, "vkCreateWin32SurfaceKHR");
    printf("Win32 surface created\n");
}

// Create Swapchain
void createSwapchain() {
    VkSurfaceCapabilitiesKHR capabilities;
    vkGetPhysicalDeviceSurfaceCapabilitiesKHR(physicalDevice, surface, &capabilities);

    swapchainExtent.width = WIDTH;
    swapchainExtent.height = HEIGHT;
    swapchainImageFormat = VK_FORMAT_B8G8R8A8_SRGB;

    VkSwapchainCreateInfoKHR createInfo;
    createInfo.sType = VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR;
    createInfo.pNext = 0;
    createInfo.flags = 0;
    createInfo.surface = surface;
    createInfo.minImageCount = 2;
    createInfo.imageFormat = swapchainImageFormat;
    createInfo.imageColorSpace = VK_COLOR_SPACE_SRGB_NONLINEAR_KHR;
    createInfo.imageExtent = swapchainExtent;
    createInfo.imageArrayLayers = 1;
    createInfo.imageUsage = VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT;
    createInfo.imageSharingMode = VK_SHARING_MODE_EXCLUSIVE;
    createInfo.queueFamilyIndexCount = 0;
    createInfo.pQueueFamilyIndices = 0;
    createInfo.preTransform = 1;
    createInfo.compositeAlpha = VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR;
    createInfo.presentMode = VK_PRESENT_MODE_FIFO_KHR;
    createInfo.clipped = VK_TRUE;
    createInfo.oldSwapchain = VK_NULL_HANDLE;

    VkResult result = vkCreateSwapchainKHR(device, &createInfo, 0, &swapchain);
    checkVk(result, "vkCreateSwapchainKHR");

    vkGetSwapchainImagesKHR(device, swapchain, &swapchainImageCount, nullptr);
    swapchainImages = (VkImage*)malloc(swapchainImageCount * sizeof(VkImage));
    vkGetSwapchainImagesKHR(device, swapchain, &swapchainImageCount, swapchainImages);

    printf("Swapchain created with %d images\n", swapchainImageCount);
}

// Create Image Views
void createImageViews() {
    swapchainImageViews = (VkImageView*)malloc(swapchainImageCount * sizeof(VkImageView));

    for (unsigned int i = (unsigned)0; i < swapchainImageCount; i++) {
        VkImageViewCreateInfo createInfo;
        createInfo.sType = VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO;
        createInfo.pNext = 0;
        createInfo.flags = 0;
        createInfo.image = swapchainImages[i];
        createInfo.viewType = VK_IMAGE_VIEW_TYPE_2D;
        createInfo.format = swapchainImageFormat;
        createInfo.components.r = 0;
        createInfo.components.g = 0;
        createInfo.components.b = 0;
        createInfo.components.a = 0;
        createInfo.subresourceRange.aspectMask = 1;
        createInfo.subresourceRange.baseMipLevel = 0;
        createInfo.subresourceRange.levelCount = 1;
        createInfo.subresourceRange.baseArrayLayer = 0;
        createInfo.subresourceRange.layerCount = 1;

        vkCreateImageView(device, &createInfo, 0, &swapchainImageViews[i]);
    }
    printf("Image views created\n");
}

// Create Render Pass
void createRenderPass() {
    VkAttachmentDescription colorAttachment;
    colorAttachment.flags = 0;
    colorAttachment.format = swapchainImageFormat;
    colorAttachment.samples = 1;
    colorAttachment.loadOp = VK_ATTACHMENT_LOAD_OP_CLEAR;
    colorAttachment.storeOp = VK_ATTACHMENT_STORE_OP_STORE;
    colorAttachment.stencilLoadOp = VK_ATTACHMENT_LOAD_OP_DONT_CARE;
    colorAttachment.stencilStoreOp = VK_ATTACHMENT_STORE_OP_DONT_CARE;
    colorAttachment.initialLayout = VK_IMAGE_LAYOUT_UNDEFINED;
    colorAttachment.finalLayout = VK_IMAGE_LAYOUT_PRESENT_SRC_KHR;

    VkAttachmentReference colorAttachmentRef;
    colorAttachmentRef.attachment = 0;
    colorAttachmentRef.layout = VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL;

    VkSubpassDescription subpass;
    subpass.flags = 0;
    subpass.pipelineBindPoint = VK_PIPELINE_BIND_POINT_GRAPHICS;
    subpass.inputAttachmentCount = 0;
    subpass.pInputAttachments = 0;
    subpass.colorAttachmentCount = 1;
    subpass.pColorAttachments = &colorAttachmentRef;
    subpass.pResolveAttachments = 0;
    subpass.pDepthStencilAttachment = 0;
    subpass.preserveAttachmentCount = 0;
    subpass.pPreserveAttachments = 0;

    VkRenderPassCreateInfo renderPassInfo;
    renderPassInfo.sType = VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO;
    renderPassInfo.pNext = 0;
    renderPassInfo.flags = 0;
    renderPassInfo.attachmentCount = 1;
    renderPassInfo.pAttachments = &colorAttachment;
    renderPassInfo.subpassCount = 1;
    renderPassInfo.pSubpasses = &subpass;
    renderPassInfo.dependencyCount = 0;
    renderPassInfo.pDependencies = 0;

    VkResult result = vkCreateRenderPass(device, &renderPassInfo, 0, &renderPass);
    checkVk(result, "vkCreateRenderPass");
    printf("Render pass created\n");
}

// Create Shader Module
VkShaderModule createShaderModule(const unsigned int* code, unsigned long long codeSize) {
    VkShaderModuleCreateInfo createInfo;
    createInfo.sType = VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO;
    createInfo.pNext = 0;
    createInfo.flags = 0;
    createInfo.codeSize = codeSize;
    createInfo.pCode = code;

    VkShaderModule shaderModule;
    vkCreateShaderModule(device, &createInfo, 0, &shaderModule);
    return shaderModule;
}

// Create Graphics Pipeline
void createGraphicsPipeline() {
    VkShaderModule vertShaderModule = createShaderModule(vertShaderCode, sizeof(vertShaderCode));
    VkShaderModule fragShaderModule = createShaderModule(fragShaderCode, sizeof(fragShaderCode));

    VkPipelineShaderStageCreateInfo vertShaderStageInfo;
    vertShaderStageInfo.sType = VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO;
    vertShaderStageInfo.pNext = 0;
    vertShaderStageInfo.flags = 0;
    vertShaderStageInfo.stage = VK_SHADER_STAGE_VERTEX_BIT;
    vertShaderStageInfo.module = vertShaderModule;
    vertShaderStageInfo.pName = "main";
    vertShaderStageInfo.pSpecializationInfo = 0;

    VkPipelineShaderStageCreateInfo fragShaderStageInfo;
    fragShaderStageInfo.sType = VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO;
    fragShaderStageInfo.pNext = 0;
    fragShaderStageInfo.flags = 0;
    fragShaderStageInfo.stage = VK_SHADER_STAGE_FRAGMENT_BIT;
    fragShaderStageInfo.module = fragShaderModule;
    fragShaderStageInfo.pName = "main";
    fragShaderStageInfo.pSpecializationInfo = 0;

    VkPipelineShaderStageCreateInfo shaderStages[2];
    shaderStages[0] = vertShaderStageInfo;
    shaderStages[1] = fragShaderStageInfo;

    VkPipelineVertexInputStateCreateInfo vertexInputInfo;
    vertexInputInfo.sType = VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO;
    vertexInputInfo.pNext = 0;
    vertexInputInfo.flags = 0;
    vertexInputInfo.vertexBindingDescriptionCount = 0;
    vertexInputInfo.pVertexBindingDescriptions = 0;
    vertexInputInfo.vertexAttributeDescriptionCount = 0;
    vertexInputInfo.pVertexAttributeDescriptions = 0;

    VkPipelineInputAssemblyStateCreateInfo inputAssembly;
    inputAssembly.sType = VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO;
    inputAssembly.pNext = 0;
    inputAssembly.flags = 0;
    inputAssembly.topology = VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST;
    inputAssembly.primitiveRestartEnable = VK_FALSE;

    VkViewport viewport;
    viewport.x = 0.0f;
    viewport.y = 0.0f;
    viewport.width = (float)WIDTH;
    viewport.height = (float)HEIGHT;
    viewport.minDepth = 0.0f;
    viewport.maxDepth = 1.0f;

    VkRect2D scissor;
    scissor.offset.x = 0;
    scissor.offset.y = 0;
    scissor.extent = swapchainExtent;

    VkPipelineViewportStateCreateInfo viewportState;
    viewportState.sType = VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO;
    viewportState.pNext = 0;
    viewportState.flags = 0;
    viewportState.viewportCount = 1;
    viewportState.pViewports = &viewport;
    viewportState.scissorCount = 1;
    viewportState.pScissors = &scissor;

    VkPipelineRasterizationStateCreateInfo rasterizer;
    rasterizer.sType = VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO;
    rasterizer.pNext = 0;
    rasterizer.flags = 0;
    rasterizer.depthClampEnable = VK_FALSE;
    rasterizer.rasterizerDiscardEnable = VK_FALSE;
    rasterizer.polygonMode = VK_POLYGON_MODE_FILL;
    rasterizer.cullMode = VK_CULL_MODE_BACK_BIT;
    rasterizer.frontFace = VK_FRONT_FACE_CLOCKWISE;
    rasterizer.depthBiasEnable = VK_FALSE;
    rasterizer.depthBiasConstantFactor = 0.0f;
    rasterizer.depthBiasClamp = 0.0f;
    rasterizer.depthBiasSlopeFactor = 0.0f;
    rasterizer.lineWidth = 1.0f;

    VkPipelineMultisampleStateCreateInfo multisampling;
    multisampling.sType = VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO;
    multisampling.pNext = 0;
    multisampling.flags = 0;
    multisampling.rasterizationSamples = 1;
    multisampling.sampleShadingEnable = VK_FALSE;
    multisampling.minSampleShading = 1.0f;
    multisampling.pSampleMask = 0;
    multisampling.alphaToCoverageEnable = VK_FALSE;
    multisampling.alphaToOneEnable = VK_FALSE;

    VkPipelineColorBlendAttachmentState colorBlendAttachment;
    colorBlendAttachment.blendEnable = VK_FALSE;
    colorBlendAttachment.srcColorBlendFactor = VK_BLEND_FACTOR_ONE;
    colorBlendAttachment.dstColorBlendFactor = VK_BLEND_FACTOR_ZERO;
    colorBlendAttachment.colorBlendOp = VK_BLEND_OP_ADD;
    colorBlendAttachment.srcAlphaBlendFactor = VK_BLEND_FACTOR_ONE;
    colorBlendAttachment.dstAlphaBlendFactor = VK_BLEND_FACTOR_ZERO;
    colorBlendAttachment.alphaBlendOp = VK_BLEND_OP_ADD;
    colorBlendAttachment.colorWriteMask = VK_COLOR_COMPONENT_R_BIT | VK_COLOR_COMPONENT_G_BIT |
                                          VK_COLOR_COMPONENT_B_BIT | VK_COLOR_COMPONENT_A_BIT;

    VkPipelineColorBlendStateCreateInfo colorBlending;
    colorBlending.sType = VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO;
    colorBlending.pNext = 0;
    colorBlending.flags = 0;
    colorBlending.logicOpEnable = VK_FALSE;
    colorBlending.logicOp = 0;
    colorBlending.attachmentCount = 1;
    colorBlending.pAttachments = &colorBlendAttachment;
    colorBlending.blendConstants[0] = 0.0f;
    colorBlending.blendConstants[1] = 0.0f;
    colorBlending.blendConstants[2] = 0.0f;
    colorBlending.blendConstants[3] = 0.0f;

    VkPipelineLayoutCreateInfo pipelineLayoutInfo;
    pipelineLayoutInfo.sType = VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO;
    pipelineLayoutInfo.pNext = 0;
    pipelineLayoutInfo.flags = 0;
    pipelineLayoutInfo.setLayoutCount = 0;
    pipelineLayoutInfo.pSetLayouts = 0;
    pipelineLayoutInfo.pushConstantRangeCount = 0;
    pipelineLayoutInfo.pPushConstantRanges = 0;

    vkCreatePipelineLayout(device, &pipelineLayoutInfo, 0, &pipelineLayout);

    VkGraphicsPipelineCreateInfo pipelineInfo;
    pipelineInfo.sType = VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO;
    pipelineInfo.pNext = 0;
    pipelineInfo.flags = 0;
    pipelineInfo.stageCount = 2;
    pipelineInfo.pStages = shaderStages;
    pipelineInfo.pVertexInputState = &vertexInputInfo;
    pipelineInfo.pInputAssemblyState = &inputAssembly;
    pipelineInfo.pTessellationState = 0;
    pipelineInfo.pViewportState = &viewportState;
    pipelineInfo.pRasterizationState = &rasterizer;
    pipelineInfo.pMultisampleState = &multisampling;
    pipelineInfo.pDepthStencilState = 0;
    pipelineInfo.pColorBlendState = &colorBlending;
    pipelineInfo.pDynamicState = 0;
    pipelineInfo.layout = pipelineLayout;
    pipelineInfo.renderPass = renderPass;
    pipelineInfo.subpass = 0;
    pipelineInfo.basePipelineHandle = VK_NULL_HANDLE;
    pipelineInfo.basePipelineIndex = -1;

    VkResult result = vkCreateGraphicsPipelines(device, VK_NULL_HANDLE, 1, &pipelineInfo, 0, &graphicsPipeline);
    checkVk(result, "vkCreateGraphicsPipelines");

    vkDestroyShaderModule(device, fragShaderModule, 0);
    vkDestroyShaderModule(device, vertShaderModule, 0);

    printf("Graphics pipeline created\n");
}

// Create Framebuffers
void createFramebuffers() {
    swapchainFramebuffers = (VkFramebuffer*)malloc(swapchainImageCount * sizeof(VkFramebuffer));

    for (unsigned int i = (unsigned)0; i < swapchainImageCount; i++) {
        VkImageView attachments[1];
        attachments[0] = swapchainImageViews[i];

        VkFramebufferCreateInfo framebufferInfo;
        framebufferInfo.sType = VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO;
        framebufferInfo.pNext = 0;
        framebufferInfo.flags = 0;
        framebufferInfo.renderPass = renderPass;
        framebufferInfo.attachmentCount = 1;
        framebufferInfo.pAttachments = attachments;
        framebufferInfo.width = swapchainExtent.width;
        framebufferInfo.height = swapchainExtent.height;
        framebufferInfo.layers = 1;

        vkCreateFramebuffer(device, &framebufferInfo, 0, &swapchainFramebuffers[i]);
    }
    printf("Framebuffers created\n");
}

// Create Command Pool
void createCommandPool() {
    VkCommandPoolCreateInfo poolInfo;
    poolInfo.sType = VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO;
    poolInfo.pNext = 0;
    poolInfo.flags = 0;
    poolInfo.queueFamilyIndex = graphicsQueueFamily;

    vkCreateCommandPool(device, &poolInfo, 0, &commandPool);
    printf("Command pool created\n");
}

// Create Command Buffer
void createCommandBuffer() {
    VkCommandBufferAllocateInfo allocInfo;
    allocInfo.sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO;
    allocInfo.pNext = 0;
    allocInfo.commandPool = commandPool;
    allocInfo.level = VK_COMMAND_BUFFER_LEVEL_PRIMARY;
    allocInfo.commandBufferCount = 1;

    vkAllocateCommandBuffers(device, &allocInfo, &commandBuffer);
    printf("Command buffer allocated\n");
}

// Create Sync Objects
void createSyncObjects() {
    VkSemaphoreCreateInfo semaphoreInfo;
    semaphoreInfo.sType = VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO;
    semaphoreInfo.pNext = 0;
    semaphoreInfo.flags = 0;

    VkFenceCreateInfo fenceInfo;
    fenceInfo.sType = VK_STRUCTURE_TYPE_FENCE_CREATE_INFO;
    fenceInfo.pNext = 0;
    fenceInfo.flags = 1;

    vkCreateSemaphore(device, &semaphoreInfo, 0, &imageAvailableSemaphore);
    vkCreateSemaphore(device, &semaphoreInfo, 0, &renderFinishedSemaphore);
    vkCreateFence(device, &fenceInfo, 0, &inFlightFence);
    printf("Sync objects created\n");
}

// Record Command Buffer
void recordCommandBuffer(unsigned int imageIndex) {
    VkCommandBufferBeginInfo beginInfo;
    beginInfo.sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO;
    beginInfo.pNext = 0;
    beginInfo.flags = 0;
    beginInfo.pInheritanceInfo = 0;

    vkBeginCommandBuffer(commandBuffer, &beginInfo);

    VkClearValue clearColor;
    clearColor.color.float32[0] = 0.0f;
    clearColor.color.float32[1] = 0.0f;
    clearColor.color.float32[2] = 0.0f;
    clearColor.color.float32[3] = 1.0f;

    VkRenderPassBeginInfo renderPassInfo;
    renderPassInfo.sType = VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO;
    renderPassInfo.pNext = 0;
    renderPassInfo.renderPass = renderPass;
    renderPassInfo.framebuffer = swapchainFramebuffers[imageIndex];
    renderPassInfo.renderArea.offset.x = 0;
    renderPassInfo.renderArea.offset.y = 0;
    renderPassInfo.renderArea.extent = swapchainExtent;
    renderPassInfo.clearValueCount = 1;
    renderPassInfo.pClearValues = &clearColor;

    vkCmdBeginRenderPass(commandBuffer, &renderPassInfo, 0);
    vkCmdBindPipeline(commandBuffer, VK_PIPELINE_BIND_POINT_GRAPHICS, graphicsPipeline);

    VkViewport viewport;
    viewport.x = 0.0f;
    viewport.y = 0.0f;
    viewport.width = (float)swapchainExtent.width;
    viewport.height = (float)swapchainExtent.height;
    viewport.minDepth = 0.0f;
    viewport.maxDepth = 1.0f;
    vkCmdSetViewport(commandBuffer, 0, 1, &viewport);

    VkRect2D scissor;
    scissor.offset.x = 0;
    scissor.offset.y = 0;
    scissor.extent = swapchainExtent;
    vkCmdSetScissor(commandBuffer, 0, 1, &scissor);

    vkCmdDraw(commandBuffer, 3, 1, 0, 0);

    vkCmdEndRenderPass(commandBuffer);
    vkEndCommandBuffer(commandBuffer);
}

// Draw Frame
void drawFrame() {
    vkWaitForFences(device, 1, &inFlightFence, VK_TRUE, 0xFFFFFFFFFFFFFFFF);
    vkResetFences(device, 1, &inFlightFence);

    unsigned int imageIndex;
    vkAcquireNextImageKHR(device, swapchain, 0xFFFFFFFFFFFFFFFF, imageAvailableSemaphore, VK_NULL_HANDLE, &imageIndex);

    vkResetCommandBuffer(commandBuffer, 0);
    recordCommandBuffer(imageIndex);

    VkSemaphore waitSemaphores[1];
    waitSemaphores[0] = imageAvailableSemaphore;
    unsigned int waitStages[1];
    waitStages[0] = 0x00000400;
    VkSemaphore signalSemaphores[1];
    signalSemaphores[0] = renderFinishedSemaphore;

    VkSubmitInfo submitInfo;
    submitInfo.sType = VK_STRUCTURE_TYPE_SUBMIT_INFO;
    submitInfo.pNext = 0;
    submitInfo.waitSemaphoreCount = 1;
    submitInfo.pWaitSemaphores = waitSemaphores;
    submitInfo.pWaitDstStageMask = waitStages;
    submitInfo.commandBufferCount = 1;
    submitInfo.pCommandBuffers = &commandBuffer;
    submitInfo.signalSemaphoreCount = 1;
    submitInfo.pSignalSemaphores = signalSemaphores;

    vkQueueSubmit(graphicsQueue, 1, &submitInfo, inFlightFence);

    VkSwapchainKHR swapchains[1];
    swapchains[0] = swapchain;

    VkPresentInfoKHR presentInfo;
    presentInfo.sType = VK_STRUCTURE_TYPE_PRESENT_INFO_KHR;
    presentInfo.pNext = 0;
    presentInfo.waitSemaphoreCount = 1;
    presentInfo.pWaitSemaphores = signalSemaphores;
    presentInfo.swapchainCount = 1;
    presentInfo.pSwapchains = swapchains;
    presentInfo.pImageIndices = &imageIndex;
    presentInfo.pResults = 0;

    vkQueuePresentKHR(graphicsQueue, &presentInfo);
}

// Cleanup
void cleanup() {
    vkDeviceWaitIdle(device);

    vkDestroySemaphore(device, renderFinishedSemaphore, 0);
    vkDestroySemaphore(device, imageAvailableSemaphore, 0);
    vkDestroyFence(device, inFlightFence, 0);
    vkDestroyCommandPool(device, commandPool, 0);

    for (unsigned int i = (unsigned)0; i < swapchainImageCount; i++) {
        vkDestroyFramebuffer(device, swapchainFramebuffers[i], 0);
        vkDestroyImageView(device, swapchainImageViews[i], 0);
    }

    vkDestroyPipeline(device, graphicsPipeline, 0);
    vkDestroyPipelineLayout(device, pipelineLayout, 0);
    vkDestroyRenderPass(device, renderPass, 0);
    vkDestroySwapchainKHR(device, swapchain, 0);
    vkDestroySurfaceKHR(instance, surface, 0);
    vkDestroyDevice(device, 0);
    vkDestroyInstance(instance, 0);

    free(swapchainImages);
    free(swapchainImageViews);
    free(swapchainFramebuffers);

    printf("Cleanup complete\n");
}

struct VkWin32SurfaceCreateInfoKHR {
    unsigned int sType;
    const void* pNext;
    unsigned int flags;
    HINSTANCE hinstance;
    HWND hwnd;
};

// Create Surface
void createSurface(HWND hwnd, HINSTANCE hInstance) {
    VkWin32SurfaceCreateInfoKHR createInfo;
    createInfo.sType = 1000009000;
    createInfo.pNext = nullptr;
    createInfo.flags = 0;
    createInfo.hinstance = hInstance;
    createInfo.hwnd = hwnd;
    
    VkResult result = vkCreateWin32SurfaceKHR(instance, &createInfo, nullptr, &surface);
    checkVk(result, "vkCreateWin32SurfaceKHR");
    printf("Surface created\n");
}

// Main
int main() {
    printf("=== ADead-BIB Vulkan Triangle ===\n");

    HINSTANCE hInstance = (HINSTANCE)GetModuleHandleA(nullptr);

    WNDCLASSA wc;
    wc.style = 0;
    wc.lpfnWndProc = (long long)WindowProc;
    wc.cbClsExtra = 0;
    wc.cbWndExtra = 0;
    wc.hInstance = hInstance;
    wc.hIcon = nullptr;
    wc.hCursor = nullptr;
    wc.hbrBackground = nullptr;
    wc.lpszMenuName = nullptr;
    wc.lpszClassName = "ADeadBIBWindowClass";
    
    RegisterClassA(&wc);

    HWND hwnd = CreateWindowExA(
        0,
        "ADeadBIBWindowClass",
        APP_NAME,
        WS_OVERLAPPEDWINDOW,
        100, 100, WIDTH, HEIGHT,
        nullptr, nullptr, hInstance, nullptr
    );

    ShowWindow(hwnd, SW_SHOW);
    UpdateWindow(hwnd);

    createInstance();
    createSurface(hwnd, hInstance);
    pickPhysicalDevice();
    findQueueFamilies();
    createLogicalDevice();
    createSwapchain();
    createImageViews();
    createRenderPass();
    createGraphicsPipeline();
    createFramebuffers();
    createCommandPool();
    createCommandBuffer();
    createSyncObjects();

    printf("Vulkan initialized! Drawing triangle...\n");

    MSG msg;
    while (global_window_running) {
        if (PeekMessageA(&msg, nullptr, 0, 0, PM_REMOVE)) {
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        } else {
            drawFrame();
        }
    }

    vkDeviceWaitIdle(device);
    cleanup();

    return 0;
}
