//! Vulkan 1.3 — Function Names (C ABI)

/// Core Vulkan 1.0 functions (vulkan-1.dll / libvulkan.so.1)
pub const VK_CORE_FUNCTIONS: &[&str] = &[
    // Instance
    "vkCreateInstance", "vkDestroyInstance",
    "vkEnumeratePhysicalDevices",
    "vkGetPhysicalDeviceProperties", "vkGetPhysicalDeviceFeatures",
    "vkGetPhysicalDeviceQueueFamilyProperties",
    "vkGetPhysicalDeviceMemoryProperties",
    "vkGetPhysicalDeviceFormatProperties",
    "vkEnumerateInstanceExtensionProperties",
    "vkEnumerateInstanceLayerProperties",
    "vkEnumerateDeviceExtensionProperties",
    "vkGetInstanceProcAddr",
    // Device
    "vkCreateDevice", "vkDestroyDevice",
    "vkGetDeviceQueue", "vkDeviceWaitIdle", "vkQueueWaitIdle",
    "vkGetDeviceProcAddr",
    // Memory
    "vkAllocateMemory", "vkFreeMemory",
    "vkMapMemory", "vkUnmapMemory",
    "vkFlushMappedMemoryRanges", "vkInvalidateMappedMemoryRanges",
    // Buffer
    "vkCreateBuffer", "vkDestroyBuffer",
    "vkGetBufferMemoryRequirements", "vkBindBufferMemory",
    // Image
    "vkCreateImage", "vkDestroyImage",
    "vkGetImageMemoryRequirements", "vkBindImageMemory",
    "vkCreateImageView", "vkDestroyImageView",
    // Sampler
    "vkCreateSampler", "vkDestroySampler",
    // Descriptor
    "vkCreateDescriptorSetLayout", "vkDestroyDescriptorSetLayout",
    "vkCreateDescriptorPool", "vkDestroyDescriptorPool",
    "vkAllocateDescriptorSets", "vkFreeDescriptorSets",
    "vkUpdateDescriptorSets",
    // Shader / Pipeline
    "vkCreateShaderModule", "vkDestroyShaderModule",
    "vkCreateGraphicsPipelines", "vkCreateComputePipelines",
    "vkDestroyPipeline",
    "vkCreatePipelineLayout", "vkDestroyPipelineLayout",
    "vkCreatePipelineCache", "vkDestroyPipelineCache",
    "vkGetPipelineCacheData", "vkMergePipelineCaches",
    // Render pass
    "vkCreateRenderPass", "vkDestroyRenderPass",
    "vkCreateFramebuffer", "vkDestroyFramebuffer",
    // Command
    "vkCreateCommandPool", "vkDestroyCommandPool",
    "vkResetCommandPool",
    "vkAllocateCommandBuffers", "vkFreeCommandBuffers",
    "vkBeginCommandBuffer", "vkEndCommandBuffer", "vkResetCommandBuffer",
    // Draw commands
    "vkCmdBeginRenderPass", "vkCmdEndRenderPass", "vkCmdNextSubpass",
    "vkCmdBindPipeline",
    "vkCmdSetViewport", "vkCmdSetScissor", "vkCmdSetLineWidth",
    "vkCmdSetDepthBias", "vkCmdSetBlendConstants", "vkCmdSetStencilReference",
    "vkCmdDraw", "vkCmdDrawIndexed",
    "vkCmdDrawIndirect", "vkCmdDrawIndexedIndirect",
    "vkCmdDispatch", "vkCmdDispatchIndirect",
    "vkCmdBindVertexBuffers", "vkCmdBindIndexBuffer",
    "vkCmdBindDescriptorSets",
    "vkCmdPushConstants",
    "vkCmdCopyBuffer", "vkCmdCopyImage", "vkCmdBlitImage",
    "vkCmdCopyBufferToImage", "vkCmdCopyImageToBuffer",
    "vkCmdUpdateBuffer", "vkCmdFillBuffer",
    "vkCmdClearColorImage", "vkCmdClearDepthStencilImage",
    "vkCmdClearAttachments",
    "vkCmdPipelineBarrier",
    "vkCmdExecuteCommands",
    // Sync
    "vkCreateFence", "vkDestroyFence",
    "vkWaitForFences", "vkResetFences", "vkGetFenceStatus",
    "vkCreateSemaphore", "vkDestroySemaphore",
    "vkCreateEvent", "vkDestroyEvent", "vkGetEventStatus",
    "vkSetEvent", "vkResetEvent",
    "vkCmdSetEvent", "vkCmdResetEvent", "vkCmdWaitEvents",
    "vkQueueSubmit",
    // Query
    "vkCreateQueryPool", "vkDestroyQueryPool",
    "vkGetQueryPoolResults",
    "vkCmdBeginQuery", "vkCmdEndQuery",
    "vkCmdResetQueryPool", "vkCmdWriteTimestamp",
    "vkCmdCopyQueryPoolResults",
];

/// KHR extension functions (swap chain, surfaces)
pub const VK_KHR_FUNCTIONS: &[&str] = &[
    // Surface
    "vkDestroySurfaceKHR",
    "vkGetPhysicalDeviceSurfaceSupportKHR",
    "vkGetPhysicalDeviceSurfaceCapabilitiesKHR",
    "vkGetPhysicalDeviceSurfaceFormatsKHR",
    "vkGetPhysicalDeviceSurfacePresentModesKHR",
    // Win32 surface
    "vkCreateWin32SurfaceKHR",
    // Xlib surface (Linux)
    "vkCreateXlibSurfaceKHR",
    // Wayland surface (Linux)
    "vkCreateWaylandSurfaceKHR",
    // Swapchain
    "vkCreateSwapchainKHR", "vkDestroySwapchainKHR",
    "vkGetSwapchainImagesKHR",
    "vkAcquireNextImageKHR",
    "vkQueuePresentKHR",
];

/// Vulkan 1.3 core promoted functions
pub const VK_13_FUNCTIONS: &[&str] = &[
    "vkCmdBeginRendering", "vkCmdEndRendering",
    "vkCmdSetCullMode", "vkCmdSetFrontFace",
    "vkCmdSetPrimitiveTopology",
    "vkCmdSetViewportWithCount", "vkCmdSetScissorWithCount",
    "vkCmdBindVertexBuffers2",
    "vkCmdSetDepthTestEnable", "vkCmdSetDepthWriteEnable",
    "vkCmdSetDepthCompareOp",
    "vkCmdSetStencilTestEnable", "vkCmdSetStencilOp",
    "vkCmdSetRasterizerDiscardEnable",
    "vkCmdSetDepthBiasEnable",
    "vkCmdSetPrimitiveRestartEnable",
];

pub fn is_vk_function(name: &str) -> bool {
    VK_CORE_FUNCTIONS.contains(&name)
        || VK_KHR_FUNCTIONS.contains(&name)
        || VK_13_FUNCTIONS.contains(&name)
        || name.starts_with("PFN_vk")
}
