//! ADead-BIB Vulkan Module — Vulkan 1.3 C ABI
//! Organized by initialization phases for clean compilation

pub mod vk_types;      // Phase 1: Base types, handles, macros
pub mod vk_enums;      // Phase 2: All enumerations (VkResult, VkFormat, etc)
pub mod vk_structs;    // Phase 3: All structures (CreateInfos, Descs, etc)
pub mod vk_functions;  // Phase 4: Function pointer types (PFN_vk*)
pub mod vk_loader;     // Phase 5: Dynamic loader (vulkan-1.dll / libvulkan.so)
pub mod vk_symbols;    // Symbol recognition for the compiler

pub use vk_types::*;
pub use vk_enums::*;
pub use vk_structs::*;
pub use vk_functions::*;
pub use vk_loader::*;
pub use vk_symbols::*;
