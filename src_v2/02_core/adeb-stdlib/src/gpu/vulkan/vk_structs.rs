//! Vulkan 1.3 — Structure Names (C ABI)

/// Core Vulkan structures that the compiler must recognize
pub const VK_STRUCTS: &[&str] = &[
    // Application / Instance
    "VkApplicationInfo", "VkInstanceCreateInfo",
    // Device
    "VkDeviceQueueCreateInfo", "VkDeviceCreateInfo",
    "VkPhysicalDeviceProperties", "VkPhysicalDeviceFeatures",
    "VkPhysicalDeviceMemoryProperties",
    "VkQueueFamilyProperties",
    "VkExtensionProperties", "VkLayerProperties",
    // Memory
    "VkMemoryAllocateInfo", "VkMemoryRequirements",
    "VkMappedMemoryRange", "VkMemoryType", "VkMemoryHeap",
    // Buffer
    "VkBufferCreateInfo", "VkBufferCopy",
    // Image
    "VkImageCreateInfo", "VkImageViewCreateInfo",
    "VkImageSubresourceRange", "VkImageSubresourceLayers",
    "VkImageBlit", "VkImageCopy", "VkBufferImageCopy",
    "VkComponentMapping",
    // Sampler
    "VkSamplerCreateInfo",
    // Descriptor
    "VkDescriptorSetLayoutBinding", "VkDescriptorSetLayoutCreateInfo",
    "VkDescriptorPoolSize", "VkDescriptorPoolCreateInfo",
    "VkDescriptorSetAllocateInfo",
    "VkWriteDescriptorSet", "VkCopyDescriptorSet",
    "VkDescriptorBufferInfo", "VkDescriptorImageInfo",
    // Pipeline
    "VkShaderModuleCreateInfo",
    "VkPipelineShaderStageCreateInfo",
    "VkPipelineVertexInputStateCreateInfo",
    "VkPipelineInputAssemblyStateCreateInfo",
    "VkPipelineTessellationStateCreateInfo",
    "VkPipelineViewportStateCreateInfo",
    "VkPipelineRasterizationStateCreateInfo",
    "VkPipelineMultisampleStateCreateInfo",
    "VkPipelineDepthStencilStateCreateInfo",
    "VkPipelineColorBlendStateCreateInfo",
    "VkPipelineColorBlendAttachmentState",
    "VkPipelineDynamicStateCreateInfo",
    "VkPipelineLayoutCreateInfo",
    "VkPipelineCacheCreateInfo",
    "VkGraphicsPipelineCreateInfo",
    "VkComputePipelineCreateInfo",
    "VkVertexInputBindingDescription",
    "VkVertexInputAttributeDescription",
    "VkStencilOpState",
    "VkPushConstantRange",
    "VkSpecializationInfo", "VkSpecializationMapEntry",
    // Render pass
    "VkRenderPassCreateInfo", "VkRenderPassBeginInfo",
    "VkAttachmentDescription", "VkAttachmentReference",
    "VkSubpassDescription", "VkSubpassDependency",
    "VkFramebufferCreateInfo",
    "VkClearValue", "VkClearColorValue", "VkClearDepthStencilValue",
    // Command
    "VkCommandPoolCreateInfo",
    "VkCommandBufferAllocateInfo",
    "VkCommandBufferBeginInfo",
    "VkCommandBufferInheritanceInfo",
    "VkSubmitInfo",
    // Sync
    "VkFenceCreateInfo", "VkSemaphoreCreateInfo",
    // Geometry
    "VkViewport", "VkRect2D", "VkOffset2D", "VkOffset3D",
    "VkExtent2D", "VkExtent3D",
    // Barriers
    "VkMemoryBarrier", "VkBufferMemoryBarrier", "VkImageMemoryBarrier",
    // KHR extensions
    "VkSurfaceCapabilitiesKHR", "VkSurfaceFormatKHR",
    "VkSwapchainCreateInfoKHR", "VkPresentInfoKHR",
    "VkWin32SurfaceCreateInfoKHR",
    "VkXlibSurfaceCreateInfoKHR",
    "VkWaylandSurfaceCreateInfoKHR",
    // Vulkan 1.3 dynamic rendering
    "VkRenderingInfo", "VkRenderingAttachmentInfo",
    "VkPipelineRenderingCreateInfo",
];

pub fn is_vk_struct(name: &str) -> bool {
    VK_STRUCTS.contains(&name)
}
