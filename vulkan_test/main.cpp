// main.cpp — ADead-BIB Vulkan Triangle Example
// Dibuja un triángulo con gradiente RGB usando Vulkan
// Sin SDK externo — usa vulkan.h interno de ADead-BIB
//
// Compilar: adb cxx vulkan_test/main.cpp -o vulkan_triangle.exe
//
// Autor: Eddi Andreé Salazar Matos
// Fecha: Marzo 2026

#include "ad_std_window.hpp"
#include "ad_std_vulkan.hpp"
#include "ad_std_vulkan_loader.cpp"

extern "C" {
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

uint32_t graphicsQueueFamily = (uint32_t)0;
uint32_t swapchainImageCount = (uint32_t)0;
VkImage* swapchainImages;
VkImageView* swapchainImageViews;
VkFramebuffer* swapchainFramebuffers;
VkFormat swapchainImageFormat;
VkExtent2D swapchainExtent;

bool running = true;

// SPIR-V compilado del vertex shader (triangle.vert)
// Generado con: glslangValidator -V triangle.vert -o vert.spv
const uint32_t vertShaderCode[] = {
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
const uint32_t fragShaderCode[] = {
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

    int loadCode = adLoadVulkanLibrary();
    if (loadCode != 1) {
        printf("Failed to load vulkan-1.dll (Error Code: %d). Install Vulkan Runtime.\n", loadCode);
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
    uint32_t deviceCount = (uint32_t)0;
    vkEnumeratePhysicalDevices(instance, &deviceCount, (VkPhysicalDevice*)0);
    
    if (deviceCount == (uint32_t)0) {
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
    uint32_t queueFamilyCount = (uint32_t)0;
    vkGetPhysicalDeviceQueueFamilyProperties(physicalDevice, &queueFamilyCount, (VkQueueFamilyProperties*)0);
    
    VkQueueFamilyProperties* queueFamilies = (VkQueueFamilyProperties*)malloc(
        queueFamilyCount * sizeof(VkQueueFamilyProperties));
    vkGetPhysicalDeviceQueueFamilyProperties(physicalDevice, &queueFamilyCount, queueFamilies);
    
    for (uint32_t i = (uint32_t)0; i < queueFamilyCount; i++) {
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

    vkGetSwapchainImagesKHR(device, swapchain, &swapchainImageCount, (VkImage*)0);
    swapchainImages = (VkImage*)malloc(swapchainImageCount * sizeof(VkImage));
    vkGetSwapchainImagesKHR(device, swapchain, &swapchainImageCount, swapchainImages);

    printf("Swapchain created with %d images\n", swapchainImageCount);
}

// Create Image Views
void createImageViews() {
    swapchainImageViews = (VkImageView*)malloc(swapchainImageCount * sizeof(VkImageView));

    for (uint32_t i = (uint32_t)0; i < swapchainImageCount; i++) {
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
VkShaderModule createShaderModule(const uint32_t* code, uint64_t codeSize) {
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

    for (uint32_t i = (uint32_t)0; i < swapchainImageCount; i++) {
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
void recordCommandBuffer(uint32_t imageIndex) {
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

    uint32_t imageIndex;
    vkAcquireNextImageKHR(device, swapchain, 0xFFFFFFFFFFFFFFFF, imageAvailableSemaphore, VK_NULL_HANDLE, &imageIndex);

    vkResetCommandBuffer(commandBuffer, 0);
    recordCommandBuffer(imageIndex);

    VkSemaphore waitSemaphores[1];
    waitSemaphores[0] = imageAvailableSemaphore;
    VkFlags waitStages[1];
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

    for (uint32_t i = (uint32_t)0; i < swapchainImageCount; i++) {
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

// Create Surface
void createSurface(WindowInfo* win) {
    VkWin32SurfaceCreateInfoKHR createInfo;
    createInfo.sType = VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR;
    createInfo.pNext = nullptr;
    createInfo.flags = 0;
    createInfo.hinstance = GetModuleHandleA(nullptr);
    createInfo.hwnd = win->handle;
    
    VkResult result = vkCreateWin32SurfaceKHR(instance, &createInfo, nullptr, &surface);
    checkVk(result, "vkCreateWin32SurfaceKHR");
    printf("Surface created\n");
}

// Main
int main() {
    printf("=== ADead-BIB Vulkan Triangle ===\n");

    WindowInfo* win = ad_window_create(APP_NAME, WIDTH, HEIGHT);
    
    createInstance();
    createSurface(win);
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

    while (ad_window_update(win)) {
        drawFrame();
    }

    vkDeviceWaitIdle(device);
    cleanup();
    ad_window_destroy(win);

    return 0;
}
