//! Vulkan 1.3 — Dynamic Loader (C ABI)
//! Provides the DLL/so names and entry point for Vulkan loading

/// Vulkan loader library names per platform
pub const VK_LOADER_WIN32: &str = "vulkan-1.dll";
pub const VK_LOADER_LINUX: &str = "libvulkan.so.1";

/// The global entry point to load all other functions
pub const VK_ENTRY_POINT: &str = "vkGetInstanceProcAddr";

/// Functions loadable without an instance (pre-instance)
pub const VK_GLOBAL_FUNCTIONS: &[&str] = &[
    "vkCreateInstance",
    "vkEnumerateInstanceExtensionProperties",
    "vkEnumerateInstanceLayerProperties",
    "vkEnumerateInstanceVersion",
];

/// Instance-level extensions needed for windowing
pub const VK_INSTANCE_EXTENSIONS: &[(&str, &str)] = &[
    ("VK_KHR_SURFACE_EXTENSION_NAME", "\"VK_KHR_surface\""),
    ("VK_KHR_WIN32_SURFACE_EXTENSION_NAME", "\"VK_KHR_win32_surface\""),
    ("VK_KHR_XLIB_SURFACE_EXTENSION_NAME", "\"VK_KHR_xlib_surface\""),
    ("VK_KHR_WAYLAND_SURFACE_EXTENSION_NAME", "\"VK_KHR_wayland_surface\""),
    ("VK_EXT_DEBUG_UTILS_EXTENSION_NAME", "\"VK_EXT_debug_utils\""),
    ("VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME", "\"VK_KHR_get_physical_device_properties2\""),
];

/// Device-level extensions
pub const VK_DEVICE_EXTENSIONS: &[(&str, &str)] = &[
    ("VK_KHR_SWAPCHAIN_EXTENSION_NAME", "\"VK_KHR_swapchain\""),
    ("VK_KHR_DYNAMIC_RENDERING_EXTENSION_NAME", "\"VK_KHR_dynamic_rendering\""),
    ("VK_KHR_MAINTENANCE1_EXTENSION_NAME", "\"VK_KHR_maintenance1\""),
];

/// Well-known Vulkan extension names the compiler should recognize
pub const VK_EXTENSION_NAMES: &[&str] = &[
    "VK_KHR_surface", "VK_KHR_win32_surface",
    "VK_KHR_xlib_surface", "VK_KHR_wayland_surface",
    "VK_KHR_swapchain", "VK_EXT_debug_utils",
    "VK_KHR_dynamic_rendering",
    "VK_KHR_get_physical_device_properties2",
    "VK_KHR_maintenance1",
];
