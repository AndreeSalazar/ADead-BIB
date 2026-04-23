//! Vulkan 1.3 — Unified Symbol Recognition

use super::{vk_types, vk_enums, vk_structs, vk_functions};

/// Check if a name is any Vulkan symbol
pub fn is_vulkan_symbol(name: &str) -> bool {
    vk_types::is_vk_type(name)
        || vk_enums::is_vk_enum(name)
        || vk_structs::is_vk_struct(name)
        || vk_functions::is_vk_function(name)
        || name.starts_with("VK_")
        || name.starts_with("Vk")
        || name.starts_with("vk")
        || name.starts_with("PFN_vk")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vulkan_symbols() {
        assert!(is_vulkan_symbol("vkCreateInstance"));
        assert!(is_vulkan_symbol("VkDevice"));
        assert!(is_vulkan_symbol("VK_SUCCESS"));
        assert!(is_vulkan_symbol("VkGraphicsPipelineCreateInfo"));
        assert!(is_vulkan_symbol("PFN_vkCreateDevice"));
        assert!(!is_vulkan_symbol("printf"));
        assert!(!is_vulkan_symbol("malloc"));
    }
}
