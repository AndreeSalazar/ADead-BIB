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

typedef unsigned int uint32_t;
typedef unsigned long long uint64_t;
typedef long long int64_t;
typedef int int32_t;
typedef float float32_t;
typedef double float64_t;
typedef unsigned long long VkDeviceSize;
typedef uint32_t VkFlags;
typedef uint32_t VkBool32;
typedef uint64_t VkDeviceAddress;

#define VK_TRUE  1
#define VK_FALSE 0
#define VK_NULL_HANDLE 0

// ============================================================================
// HANDLES (opaque pointers)
// ============================================================================

typedef void* VkInstance;
typedef void* VkPhysicalDevice;
typedef void* VkDevice;
typedef void* VkQueue;
typedef void* VkCommandPool;
typedef void* VkCommandBuffer;
typedef void* VkFence;
typedef void* VkSemaphore;
typedef void* VkBuffer;
typedef void* VkDeviceMemory;
typedef void* VkImage;
typedef void* VkImageView;
typedef void* VkSampler;
typedef void* VkShaderModule;
typedef void* VkPipeline;
typedef void* VkPipelineLayout;
typedef void* VkPipelineCache;
typedef void* VkRenderPass;
typedef void* VkFramebuffer;
typedef void* VkDescriptorSetLayout;
typedef void* VkDescriptorPool;
typedef void* VkDescriptorSet;
typedef void* VkSurfaceKHR;
typedef void* VkSwapchainKHR;

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

    uint32_t width;
    uint32_t height;

};

struct VkExtent3D {

    uint32_t width;
    uint32_t height;
    uint32_t depth;

};

struct VkOffset2D {

    int32_t x;
    int32_t y;

};

struct VkOffset3D {

    int32_t x;
    int32_t y;
    int32_t z;

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
    // // // // int32_t int32[4];
    // // // // uint32_t uint32[4];

};

struct VkClearDepthStencilValue {

    float depth;
    uint32_t stencil;

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
    uint32_t applicationVersion;
    const char* pEngineName;
    uint32_t engineVersion;
    uint32_t apiVersion;

};

struct VkInstanceCreateInfo {

    VkStructureType sType;
    const void* pNext;
    VkFlags flags;
    const VkApplicationInfo* pApplicationInfo;
    uint32_t enabledLayerCount;
    const char* const* ppEnabledLayerNames;
    uint32_t enabledExtensionCount;
    const char* const* ppEnabledExtensionNames;

};

// ============================================================================
// STRUCTS - Device
// ============================================================================

struct VkPhysicalDeviceProperties {

    uint32_t apiVersion;
    uint32_t driverVersion;
    uint32_t vendorID;
    uint32_t deviceID;
    uint32_t deviceType;
    char deviceName[256];
    uint8_t pipelineCacheUUID[16];
    // VkPhysicalDeviceLimits limits; // Simplified
    // VkPhysicalDeviceSparseProperties sparseProperties;

};

struct VkQueueFamilyProperties {

    VkFlags queueFlags;
    uint32_t queueCount;
    uint32_t timestampValidBits;
    VkExtent3D minImageTransferGranularity;

};

struct VkDeviceQueueCreateInfo {

    VkStructureType sType;
    const void* pNext;
    VkFlags flags;
    uint32_t queueFamilyIndex;
    uint32_t queueCount;
    const float* pQueuePriorities;

};

struct VkDeviceCreateInfo {

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

};

// ============================================================================
// STRUCTS - Memory
// ============================================================================

struct VkMemoryRequirements {

    VkDeviceSize size;
    VkDeviceSize alignment;
    uint32_t memoryTypeBits;

};

struct VkMemoryAllocateInfo {

    VkStructureType sType;
    const void* pNext;
    VkDeviceSize allocationSize;
    uint32_t memoryTypeIndex;

};

struct VkMemoryType {

    VkFlags propertyFlags;
    uint32_t heapIndex;

};

struct VkMemoryHeap {

    VkDeviceSize size;
    VkFlags flags;

};

struct VkPhysicalDeviceMemoryProperties {

    uint32_t memoryTypeCount;
    VkMemoryType memoryTypes[32];
    uint32_t memoryHeapCount;
    VkMemoryHeap memoryHeaps[16];

};

// ============================================================================
// STRUCTS - Buffer
// ============================================================================

struct VkBufferCreateInfo {

    VkStructureType sType;
    const void* pNext;
    VkFlags flags;
    VkDeviceSize size;
    VkFlags usage;
    VkSharingMode sharingMode;
    uint32_t queueFamilyIndexCount;
    const uint32_t* pQueueFamilyIndices;

};

// ============================================================================
// STRUCTS - Image
// ============================================================================

struct VkImageCreateInfo {

    VkStructureType sType;
    const void* pNext;
    VkFlags flags;
    VkImageType imageType;
    VkFormat format;
    VkExtent3D extent;
    uint32_t mipLevels;
    uint32_t arrayLayers;
    uint32_t samples;
    uint32_t tiling;
    VkFlags usage;
    VkSharingMode sharingMode;
    uint32_t queueFamilyIndexCount;
    const uint32_t* pQueueFamilyIndices;
    VkImageLayout initialLayout;

};

struct VkComponentMapping {

    uint32_t r;
    uint32_t g;
    uint32_t b;
    uint32_t a;

};

struct VkImageSubresourceRange {

    VkFlags aspectMask;
    uint32_t baseMipLevel;
    uint32_t levelCount;
    uint32_t baseArrayLayer;
    uint32_t layerCount;

};

struct VkImageViewCreateInfo {

    VkStructureType sType;
    const void* pNext;
    VkFlags flags;
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
    VkFlags flags;
    uint64_t codeSize;
    const uint32_t* pCode;

};

struct VkPipelineShaderStageCreateInfo {

    VkStructureType sType;
    const void* pNext;
    VkFlags flags;
    VkShaderStageFlagBits stage;
    VkShaderModule module;
    const char* pName;
    const void* pSpecializationInfo;

};

struct VkVertexInputBindingDescription {

    uint32_t binding;
    uint32_t stride;
    uint32_t inputRate;

};

struct VkVertexInputAttributeDescription {

    uint32_t location;
    uint32_t binding;
    VkFormat format;
    uint32_t offset;

};

struct VkPipelineVertexInputStateCreateInfo {

    VkStructureType sType;
    const void* pNext;
    VkFlags flags;
    uint32_t vertexBindingDescriptionCount;
    const VkVertexInputBindingDescription* pVertexBindingDescriptions;
    uint32_t vertexAttributeDescriptionCount;
    const VkVertexInputAttributeDescription* pVertexAttributeDescriptions;

};

struct VkPipelineInputAssemblyStateCreateInfo {

    VkStructureType sType;
    const void* pNext;
    VkFlags flags;
    VkPrimitiveTopology topology;
    VkBool32 primitiveRestartEnable;

};

struct VkPipelineViewportStateCreateInfo {

    VkStructureType sType;
    const void* pNext;
    VkFlags flags;
    uint32_t viewportCount;
    const VkViewport* pViewports;
    uint32_t scissorCount;
    const VkRect2D* pScissors;

};

struct VkPipelineRasterizationStateCreateInfo {

    VkStructureType sType;
    const void* pNext;
    VkFlags flags;
    VkBool32 depthClampEnable;
    VkBool32 rasterizerDiscardEnable;
    VkPolygonMode polygonMode;
    VkFlags cullMode;
    VkFrontFace frontFace;
    VkBool32 depthBiasEnable;
    float depthBiasConstantFactor;
    float depthBiasClamp;
    float depthBiasSlopeFactor;
    float lineWidth;

};

struct VkPipelineMultisampleStateCreateInfo {

    VkStructureType sType;
    const void* pNext;
    VkFlags flags;
    uint32_t rasterizationSamples;
    VkBool32 sampleShadingEnable;
    float minSampleShading;
    const uint32_t* pSampleMask;
    VkBool32 alphaToCoverageEnable;
    VkBool32 alphaToOneEnable;

};

struct VkPipelineColorBlendAttachmentState {

    VkBool32 blendEnable;
    VkBlendFactor srcColorBlendFactor;
    VkBlendFactor dstColorBlendFactor;
    VkBlendOp colorBlendOp;
    VkBlendFactor srcAlphaBlendFactor;
    VkBlendFactor dstAlphaBlendFactor;
    VkBlendOp alphaBlendOp;
    VkFlags colorWriteMask;

};

struct VkPipelineColorBlendStateCreateInfo {

    VkStructureType sType;
    const void* pNext;
    VkFlags flags;
    VkBool32 logicOpEnable;
    uint32_t logicOp;
    uint32_t attachmentCount;
    const VkPipelineColorBlendAttachmentState* pAttachments;
    float blendConstants[4];

};

struct VkPipelineLayoutCreateInfo {

    VkStructureType sType;
    const void* pNext;
    VkFlags flags;
    uint32_t setLayoutCount;
    const VkDescriptorSetLayout* pSetLayouts;
    uint32_t pushConstantRangeCount;
    const void* pPushConstantRanges;

};

struct VkGraphicsPipelineCreateInfo {

    VkStructureType sType;
    const void* pNext;
    VkFlags flags;
    uint32_t stageCount;
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
    uint32_t subpass;
    VkPipeline basePipelineHandle;
    int32_t basePipelineIndex;

};

// ============================================================================
// STRUCTS - Render Pass
// ============================================================================

struct VkAttachmentDescription {

    VkFlags flags;
    VkFormat format;
    uint32_t samples;
    VkAttachmentLoadOp loadOp;
    VkAttachmentStoreOp storeOp;
    VkAttachmentLoadOp stencilLoadOp;
    VkAttachmentStoreOp stencilStoreOp;
    VkImageLayout initialLayout;
    VkImageLayout finalLayout;

};

struct VkAttachmentReference {

    uint32_t attachment;
    VkImageLayout layout;

};

struct VkSubpassDescription {

    VkFlags flags;
    VkPipelineBindPoint pipelineBindPoint;
    uint32_t inputAttachmentCount;
    const VkAttachmentReference* pInputAttachments;
    uint32_t colorAttachmentCount;
    const VkAttachmentReference* pColorAttachments;
    const VkAttachmentReference* pResolveAttachments;
    const VkAttachmentReference* pDepthStencilAttachment;
    uint32_t preserveAttachmentCount;
    const uint32_t* pPreserveAttachments;

};

struct VkSubpassDependency {

    uint32_t srcSubpass;
    uint32_t dstSubpass;
    VkFlags srcStageMask;
    VkFlags dstStageMask;
    VkFlags srcAccessMask;
    VkFlags dstAccessMask;
    VkFlags dependencyFlags;

};

struct VkRenderPassCreateInfo {

    VkStructureType sType;
    const void* pNext;
    VkFlags flags;
    uint32_t attachmentCount;
    const VkAttachmentDescription* pAttachments;
    uint32_t subpassCount;
    const VkSubpassDescription* pSubpasses;
    uint32_t dependencyCount;
    const VkSubpassDependency* pDependencies;

};

struct VkRenderPassBeginInfo {

    VkStructureType sType;
    const void* pNext;
    VkRenderPass renderPass;
    VkFramebuffer framebuffer;
    VkRect2D renderArea;
    uint32_t clearValueCount;
    const VkClearValue* pClearValues;

};

// ============================================================================
// STRUCTS - Framebuffer
// ============================================================================

struct VkFramebufferCreateInfo {

    VkStructureType sType;
    const void* pNext;
    VkFlags flags;
    VkRenderPass renderPass;
    uint32_t attachmentCount;
    const VkImageView* pAttachments;
    uint32_t width;
    uint32_t height;
    uint32_t layers;

};

// ============================================================================
// STRUCTS - Command Buffer
// ============================================================================

struct VkCommandPoolCreateInfo {

    VkStructureType sType;
    const void* pNext;
    VkFlags flags;
    uint32_t queueFamilyIndex;

};

struct VkCommandBufferAllocateInfo {

    VkStructureType sType;
    const void* pNext;
    VkCommandPool commandPool;
    VkCommandBufferLevel level;
    uint32_t commandBufferCount;

};

struct VkCommandBufferBeginInfo {

    VkStructureType sType;
    const void* pNext;
    VkFlags flags;
    const void* pInheritanceInfo;

};

// ============================================================================
// STRUCTS - Synchronization
// ============================================================================

struct VkFenceCreateInfo {

    VkStructureType sType;
    const void* pNext;
    VkFlags flags;

};

struct VkSemaphoreCreateInfo {

    VkStructureType sType;
    const void* pNext;
    VkFlags flags;

};

struct VkSubmitInfo {

    VkStructureType sType;
    const void* pNext;
    uint32_t waitSemaphoreCount;
    const VkSemaphore* pWaitSemaphores;
    const VkFlags* pWaitDstStageMask;
    uint32_t commandBufferCount;
    const VkCommandBuffer* pCommandBuffers;
    uint32_t signalSemaphoreCount;
    const VkSemaphore* pSignalSemaphores;

};

// ============================================================================
// STRUCTS - Swapchain (KHR Extension)
// ============================================================================

struct VkSurfaceCapabilitiesKHR {

    uint32_t minImageCount;
    uint32_t maxImageCount;
    VkExtent2D currentExtent;
    VkExtent2D minImageExtent;
    VkExtent2D maxImageExtent;
    uint32_t maxImageArrayLayers;
    VkFlags supportedTransforms;
    uint32_t currentTransform;
    VkFlags supportedCompositeAlpha;
    VkFlags supportedUsageFlags;

};

struct VkSurfaceFormatKHR {

    VkFormat format;
    VkColorSpaceKHR colorSpace;

};

struct VkSwapchainCreateInfoKHR {

    VkStructureType sType;
    const void* pNext;
    VkFlags flags;
    VkSurfaceKHR surface;
    uint32_t minImageCount;
    VkFormat imageFormat;
    VkColorSpaceKHR imageColorSpace;
    VkExtent2D imageExtent;
    uint32_t imageArrayLayers;
    VkFlags imageUsage;
    VkSharingMode imageSharingMode;
    uint32_t queueFamilyIndexCount;
    const uint32_t* pQueueFamilyIndices;
    uint32_t preTransform;
    VkCompositeAlphaFlagBitsKHR compositeAlpha;
    VkPresentModeKHR presentMode;
    VkBool32 clipped;
    VkSwapchainKHR oldSwapchain;

};

struct VkPresentInfoKHR {

    VkStructureType sType;
    const void* pNext;
    uint32_t waitSemaphoreCount;
    const VkSemaphore* pWaitSemaphores;
    uint32_t swapchainCount;
    const VkSwapchainKHR* pSwapchains;
    const uint32_t* pImageIndices;
    VkResult* pResults;

};

// ============================================================================
// STRUCTS - Win32 Surface (Windows)
// ============================================================================

struct VkWin32SurfaceCreateInfoKHR {

    VkStructureType sType;
    const void* pNext;
    VkFlags flags;
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

#define VK_VERSION_MAJOR(version) ((uint32_t)(version) >> 22)
#define VK_VERSION_MINOR(version) (((uint32_t)(version) >> 12) & 0x3FF)
#define VK_VERSION_PATCH(version) ((uint32_t)(version) & 0xFFF)

// ============================================================================
// // ============================================================================
// FUNCTION POINTERS (Dynamic Loading)
// ============================================================================

typedef VkResult (*PFN_vkCreateInstance)(const VkInstanceCreateInfo* pCreateInfo, const void* pAllocator, VkInstance* pInstance);
typedef void (*PFN_vkDestroyInstance)(VkInstance instance, const void* pAllocator);
typedef VkResult (*PFN_vkEnumeratePhysicalDevices)(VkInstance instance, uint32_t* pPhysicalDeviceCount, VkPhysicalDevice* pPhysicalDevices);
typedef void (*PFN_vkGetPhysicalDeviceProperties)(VkPhysicalDevice physicalDevice, VkPhysicalDeviceProperties* pProperties);
typedef void (*PFN_vkGetPhysicalDeviceQueueFamilyProperties)(VkPhysicalDevice physicalDevice, uint32_t* pQueueFamilyPropertyCount, VkQueueFamilyProperties* pQueueFamilyProperties);
typedef void (*PFN_vkGetPhysicalDeviceMemoryProperties)(VkPhysicalDevice physicalDevice, VkPhysicalDeviceMemoryProperties* pMemoryProperties);
typedef VkResult (*PFN_vkCreateDevice)(VkPhysicalDevice physicalDevice, const VkDeviceCreateInfo* pCreateInfo, const void* pAllocator, VkDevice* pDevice);
typedef void (*PFN_vkDestroyDevice)(VkDevice device, const void* pAllocator);
typedef void (*PFN_vkGetDeviceQueue)(VkDevice device, uint32_t queueFamilyIndex, uint32_t queueIndex, VkQueue* pQueue);
typedef VkResult (*PFN_vkDeviceWaitIdle)(VkDevice device);
typedef VkResult (*PFN_vkAllocateMemory)(VkDevice device, const VkMemoryAllocateInfo* pAllocateInfo, const void* pAllocator, VkDeviceMemory* pMemory);
typedef void (*PFN_vkFreeMemory)(VkDevice device, VkDeviceMemory memory, const void* pAllocator);
typedef VkResult (*PFN_vkMapMemory)(VkDevice device, VkDeviceMemory memory, VkDeviceSize offset, VkDeviceSize size, VkFlags flags, void** ppData);
typedef void (*PFN_vkUnmapMemory)(VkDevice device, VkDeviceMemory memory);
typedef VkResult (*PFN_vkCreateBuffer)(VkDevice device, const VkBufferCreateInfo* pCreateInfo, const void* pAllocator, VkBuffer* pBuffer);
typedef void (*PFN_vkDestroyBuffer)(VkDevice device, VkBuffer buffer, const void* pAllocator);
typedef void (*PFN_vkGetBufferMemoryRequirements)(VkDevice device, VkBuffer buffer, VkMemoryRequirements* pMemoryRequirements);
typedef VkResult (*PFN_vkBindBufferMemory)(VkDevice device, VkBuffer buffer, VkDeviceMemory memory, VkDeviceSize memoryOffset);
typedef VkResult (*PFN_vkCreateImage)(VkDevice device, const VkImageCreateInfo* pCreateInfo, const void* pAllocator, VkImage* pImage);
typedef void (*PFN_vkDestroyImage)(VkDevice device, VkImage image, const void* pAllocator);
typedef VkResult (*PFN_vkCreateImageView)(VkDevice device, const VkImageViewCreateInfo* pCreateInfo, const void* pAllocator, VkImageView* pView);
typedef void (*PFN_vkDestroyImageView)(VkDevice device, VkImageView imageView, const void* pAllocator);
typedef VkResult (*PFN_vkCreateShaderModule)(VkDevice device, const VkShaderModuleCreateInfo* pCreateInfo, const void* pAllocator, VkShaderModule* pShaderModule);
typedef void (*PFN_vkDestroyShaderModule)(VkDevice device, VkShaderModule shaderModule, const void* pAllocator);
typedef VkResult (*PFN_vkCreateGraphicsPipelines)(VkDevice device, VkPipelineCache pipelineCache, uint32_t createInfoCount, const VkGraphicsPipelineCreateInfo* pCreateInfos, const void* pAllocator, VkPipeline* pPipelines);
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
typedef void (*PFN_vkFreeCommandBuffers)(VkDevice device, VkCommandPool commandPool, uint32_t commandBufferCount, const VkCommandBuffer* pCommandBuffers);
typedef VkResult (*PFN_vkBeginCommandBuffer)(VkCommandBuffer commandBuffer, const VkCommandBufferBeginInfo* pBeginInfo);
typedef VkResult (*PFN_vkEndCommandBuffer)(VkCommandBuffer commandBuffer);
typedef VkResult (*PFN_vkResetCommandBuffer)(VkCommandBuffer commandBuffer, VkFlags flags);
typedef void (*PFN_vkCmdBeginRenderPass)(VkCommandBuffer commandBuffer, const VkRenderPassBeginInfo* pRenderPassBegin, uint32_t contents);
typedef void (*PFN_vkCmdEndRenderPass)(VkCommandBuffer commandBuffer);
typedef void (*PFN_vkCmdBindPipeline)(VkCommandBuffer commandBuffer, VkPipelineBindPoint pipelineBindPoint, VkPipeline pipeline);
typedef void (*PFN_vkCmdSetViewport)(VkCommandBuffer commandBuffer, uint32_t firstViewport, uint32_t viewportCount, const VkViewport* pViewports);
typedef void (*PFN_vkCmdSetScissor)(VkCommandBuffer commandBuffer, uint32_t firstScissor, uint32_t scissorCount, const VkRect2D* pScissors);
typedef void (*PFN_vkCmdDraw)(VkCommandBuffer commandBuffer, uint32_t vertexCount, uint32_t instanceCount, uint32_t firstVertex, uint32_t firstInstance);
typedef void (*PFN_vkCmdDrawIndexed)(VkCommandBuffer commandBuffer, uint32_t indexCount, uint32_t instanceCount, uint32_t firstIndex, int32_t vertexOffset, uint32_t firstInstance);
typedef void (*PFN_vkCmdBindVertexBuffers)(VkCommandBuffer commandBuffer, uint32_t firstBinding, uint32_t bindingCount, const VkBuffer* pBuffers, const VkDeviceSize* pOffsets);
typedef void (*PFN_vkCmdBindIndexBuffer)(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, uint32_t indexType);
typedef void (*PFN_vkCmdCopyBuffer)(VkCommandBuffer commandBuffer, VkBuffer srcBuffer, VkBuffer dstBuffer, uint32_t regionCount, const void* pRegions);
typedef VkResult (*PFN_vkCreateFence)(VkDevice device, const VkFenceCreateInfo* pCreateInfo, const void* pAllocator, VkFence* pFence);
typedef void (*PFN_vkDestroyFence)(VkDevice device, VkFence fence, const void* pAllocator);
typedef VkResult (*PFN_vkWaitForFences)(VkDevice device, uint32_t fenceCount, const VkFence* pFences, VkBool32 waitAll, uint64_t timeout);
typedef VkResult (*PFN_vkResetFences)(VkDevice device, uint32_t fenceCount, const VkFence* pFences);
typedef VkResult (*PFN_vkCreateSemaphore)(VkDevice device, const VkSemaphoreCreateInfo* pCreateInfo, const void* pAllocator, VkSemaphore* pSemaphore);
typedef void (*PFN_vkDestroySemaphore)(VkDevice device, VkSemaphore semaphore, const void* pAllocator);
typedef VkResult (*PFN_vkQueueSubmit)(VkQueue queue, uint32_t submitCount, const VkSubmitInfo* pSubmits, VkFence fence);
typedef VkResult (*PFN_vkQueueWaitIdle)(VkQueue queue);
typedef void (*PFN_vkDestroySurfaceKHR)(VkInstance instance, VkSurfaceKHR surface, const void* pAllocator);
typedef VkResult (*PFN_vkGetPhysicalDeviceSurfaceSupportKHR)(VkPhysicalDevice physicalDevice, uint32_t queueFamilyIndex, VkSurfaceKHR surface, VkBool32* pSupported);
typedef VkResult (*PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR)(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, VkSurfaceCapabilitiesKHR* pSurfaceCapabilities);
typedef VkResult (*PFN_vkGetPhysicalDeviceSurfaceFormatsKHR)(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, uint32_t* pSurfaceFormatCount, VkSurfaceFormatKHR* pSurfaceFormats);
typedef VkResult (*PFN_vkGetPhysicalDeviceSurfacePresentModesKHR)(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, uint32_t* pPresentModeCount, VkPresentModeKHR* pPresentModes);
typedef VkResult (*PFN_vkCreateWin32SurfaceKHR)(VkInstance instance, const VkWin32SurfaceCreateInfoKHR* pCreateInfo, const void* pAllocator, VkSurfaceKHR* pSurface);
typedef VkResult (*PFN_vkCreateSwapchainKHR)(VkDevice device, const VkSwapchainCreateInfoKHR* pCreateInfo, const void* pAllocator, VkSwapchainKHR* pSwapchain);
typedef void (*PFN_vkDestroySwapchainKHR)(VkDevice device, VkSwapchainKHR swapchain, const void* pAllocator);
typedef VkResult (*PFN_vkGetSwapchainImagesKHR)(VkDevice device, VkSwapchainKHR swapchain, uint32_t* pSwapchainImageCount, VkImage* pSwapchainImages);
typedef VkResult (*PFN_vkAcquireNextImageKHR)(VkDevice device, VkSwapchainKHR swapchain, uint64_t timeout, VkSemaphore semaphore, VkFence fence, uint32_t* pImageIndex);
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
