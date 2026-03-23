#include "vulkan.h"

// Manual Win32 declarations for ADead-BIB Native C++ Compiler
extern "C" {
    typedef void* HMODULE;
    HMODULE LoadLibraryA(const char* lpLibFileName);
    void* GetProcAddress(HMODULE hModule, const char* lpProcName);
    int printf(const char* format, ...);
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

extern "C" int adLoadVulkanLibrary() {
    HMODULE lib = LoadLibraryA("vulkan-1.dll");
    if (lib == nullptr) return 2;
    
    real_vkGetInstanceProcAddr = (PFN_vkGetInstanceProcAddrType)GetProcAddress(lib, "vkGetInstanceProcAddr");
    if (real_vkGetInstanceProcAddr == nullptr) return 3;
    
    // Load vkCreateInstance and vkEnumerateInstanceExtensionProperties properly (instance = VK_NULL_HANDLE)
    vkCreateInstance = (PFN_vkCreateInstance)real_vkGetInstanceProcAddr(nullptr, "vkCreateInstance");
    if (vkCreateInstance == nullptr) return 4;
    
    return 1;
}

extern "C" void adLoadInstanceFunctions(VkInstance instance) {
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
