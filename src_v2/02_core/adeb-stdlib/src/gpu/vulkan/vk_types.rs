//! Vulkan 1.3 — Base Types & Handles (C ABI)

/// All Vulkan base typedefs for C compilation
pub const VK_TYPES: &[&str] = &[
    "VkDeviceSize", "VkFlags", "VkBool32", "VkDeviceAddress",
    "VkInstance", "VkPhysicalDevice", "VkDevice", "VkQueue",
    "VkCommandPool", "VkCommandBuffer", "VkFence", "VkSemaphore",
    "VkBuffer", "VkDeviceMemory", "VkImage", "VkImageView",
    "VkSampler", "VkShaderModule", "VkPipeline", "VkPipelineLayout",
    "VkPipelineCache", "VkRenderPass", "VkFramebuffer",
    "VkDescriptorSetLayout", "VkDescriptorPool", "VkDescriptorSet",
    "VkSurfaceKHR", "VkSwapchainKHR",
    "VkEvent", "VkQueryPool",
];

/// Vulkan core constants
pub const VK_CONSTANTS: &[(&str, &str)] = &[
    ("VK_TRUE", "1"),
    ("VK_FALSE", "0"),
    ("VK_NULL_HANDLE", "0"),
    ("VK_WHOLE_SIZE", "(~0ULL)"),
    ("VK_ATTACHMENT_UNUSED", "(~0U)"),
    ("VK_QUEUE_FAMILY_IGNORED", "(~0U)"),
    ("VK_SUBPASS_EXTERNAL", "(~0U)"),
    ("VK_MAX_PHYSICAL_DEVICE_NAME_SIZE", "256"),
    ("VK_UUID_SIZE", "16"),
    ("VK_MAX_MEMORY_TYPES", "32"),
    ("VK_MAX_MEMORY_HEAPS", "16"),
    ("VK_MAX_EXTENSION_NAME_SIZE", "256"),
    ("VK_MAX_DESCRIPTION_SIZE", "256"),
    ("VK_LOD_CLAMP_NONE", "1000.0f"),
    ("VK_REMAINING_MIP_LEVELS", "(~0U)"),
    ("VK_REMAINING_ARRAY_LAYERS", "(~0U)"),
];

/// Vulkan API version macros
pub const VK_VERSION_MACROS: &[(&str, &str)] = &[
    ("VK_API_VERSION_1_0", "VK_MAKE_API_VERSION(0, 1, 0, 0)"),
    ("VK_API_VERSION_1_1", "VK_MAKE_API_VERSION(0, 1, 1, 0)"),
    ("VK_API_VERSION_1_2", "VK_MAKE_API_VERSION(0, 1, 2, 0)"),
    ("VK_API_VERSION_1_3", "VK_MAKE_API_VERSION(0, 1, 3, 0)"),
    ("VK_HEADER_VERSION", "275"),
    ("VK_MAKE_API_VERSION(variant, major, minor, patch)", 
     "(((uint32_t)(variant) << 29) | ((uint32_t)(major) << 22) | ((uint32_t)(minor) << 12) | (uint32_t)(patch))"),
    ("VK_API_VERSION_VARIANT(version)", "((uint32_t)(version) >> 29)"),
    ("VK_API_VERSION_MAJOR(version)", "(((uint32_t)(version) >> 22) & 0x7FU)"),
    ("VK_API_VERSION_MINOR(version)", "(((uint32_t)(version) >> 12) & 0x3FFU)"),
    ("VK_API_VERSION_PATCH(version)", "((uint32_t)(version) & 0xFFFU)"),
];

pub fn is_vk_type(name: &str) -> bool {
    VK_TYPES.contains(&name)
        || VK_CONSTANTS.iter().any(|(n, _)| *n == name)
        || VK_VERSION_MACROS.iter().any(|(n, _)| n.starts_with(name))
}
