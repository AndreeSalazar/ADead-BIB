// ADead-Bindgen: Native Header Abstractor for ADead-BIB
// This module acts as a compatibility layer for loading complex C headers mathematically.
// It uses regex to transform unsupported C structures into purely parseable ADead-BIB AST inputs.

use regex::Regex;

pub struct AdBindgen;

impl AdBindgen {
    /// Applies massive structural translations to C header files to bypass ADead-BIB limitations
    pub fn process_header(mut content: String) -> String {
        // 1. Remove opaque typedefs for basic types
        let basic_types = [
            ("uint32_t", "unsigned int"),
            ("uint64_t", "unsigned long long"),
            ("int64_t", "long long"),
            ("int32_t", "int"),
            ("float32_t", "float"),
            ("float64_t", "double"),
        ];
        
        for (k, v) in &basic_types {
            let re = Regex::new(&format!(r"\b{}\b", k)).unwrap();
            content = re.replace_all(&content, *v).to_string();
        }

        // Convert standard typedefs to #define macros for ADead-BIB AST safety
        content = Regex::new(r"typedef\s+unsigned\s+int\s+([A-Za-z0-9_]+)\s*;").unwrap()
            .replace_all(&content, "\n#define $1 unsigned int\n")
            .to_string();
        content = Regex::new(r"typedef\s+unsigned\s+long\s+long\s+([A-Za-z0-9_]+)\s*;").unwrap()
            .replace_all(&content, "\n#define $1 unsigned long long\n")
            .to_string();
        content = Regex::new(r"typedef\s+long\s+long\s+([A-Za-z0-9_]+)\s*;").unwrap()
            .replace_all(&content, "\n#define $1 long long\n")
            .to_string();
        content = Regex::new(r"typedef\s+int\s+([A-Za-z0-9_]+)\s*;").unwrap()
            .replace_all(&content, "\n#define $1 int\n")
            .to_string();
        content = Regex::new(r"typedef\s+float\s+([A-Za-z0-9_]+)\s*;").unwrap()
            .replace_all(&content, "\n#define $1 float\n")
            .to_string();
        content = Regex::new(r"typedef\s+double\s+([A-Za-z0-9_]+)\s*;").unwrap()
            .replace_all(&content, "\n#define $1 double\n")
            .to_string();

        // Convert Opaque Handles (Pointers) typedefs to typedef void*
        content = Regex::new(r"typedef\s+struct\s+[A-Za-z0-9_]+\s*\*\s*([A-Za-z0-9_]+)\s*;").unwrap()
            .replace_all(&content, "\ntypedef void* $1;\n")
            .to_string();
            
        // Clean up resulting phantom typedefs just in case any fell through: `typedef unsigned int unsigned int;`
        content = Regex::new(r"typedef\s+[a-zA-Z0-9_\s]+\s+unsigned\s+int\s*;").unwrap().replace_all(&content, "").to_string();
        content = Regex::new(r"typedef\s+[a-zA-Z0-9_\s]+\s+unsigned\s+long\s+long\s*;").unwrap().replace_all(&content, "").to_string();
        content = Regex::new(r"typedef\s+[a-zA-Z0-9_\s]+\s+long\s+long\s*;").unwrap().replace_all(&content, "").to_string();
        content = Regex::new(r"typedef\s+[a-zA-Z0-9_\s]+\s+int\s*;").unwrap().replace_all(&content, "").to_string();
        content = Regex::new(r"typedef\s+[a-zA-Z0-9_\s]+\s+float\s*;").unwrap().replace_all(&content, "").to_string();
        content = Regex::new(r"typedef\s+[a-zA-Z0-9_\s]+\s+double\s*;").unwrap().replace_all(&content, "").to_string();

        // 2. Wrap typedef struct { ... } Name; into struct Name { ... };
        content = Regex::new(r"typedef\s+struct\s+[A-Za-z0-9_]*\s*\{([\s\S]*?)\}\s*([A-Za-z0-9_]+)\s*;").unwrap()
            .replace_all(&content, "struct $2 {$1};")
            .to_string();
            
        content = Regex::new(r"typedef\s+enum\s+[A-Za-z0-9_]*\s*\{([\s\S]*?)\}\s*([A-Za-z0-9_]+)\s*;").unwrap()
            .replace_all(&content, "enum $2 {$1};")
            .to_string();

        content = Regex::new(r"typedef\s+union\s+[A-Za-z0-9_]*\s*\{([\s\S]*?)\}\s*([A-Za-z0-9_]+)\s*;").unwrap()
            .replace_all(&content, "union $2 {$1};")
            .to_string();

        // 3. Resolve Unaligned Unions (specifically Vulkan ones) to ABI-sized structs
        // VkClearColorValue is 16 bytes
        let struct_clear_color = r"struct VkClearColorValue {
    float float32[4];
};";
        content = Regex::new(r"union\s+VkClearColorValue\s*\{[\s\S]*?\}\s*;").unwrap().replace_all(&content, struct_clear_color).to_string();

        // VkClearValue is 16 bytes
        let struct_clear_value = r"struct VkClearValue {
    struct VkClearColorValue color;
};";
        content = Regex::new(r"union\s+VkClearValue\s*\{[\s\S]*?\}\s*;").unwrap().replace_all(&content, struct_clear_value).to_string();
        
        // Remove VK_DEFINE_HANDLE / VK_DEFINE_NON_DISPATCHABLE_HANDLE macros effectively if they cause struct/union confusion
        // Typically they define pointers. ADead-BIB compiles pointers fine.

        // 4. Resolve unsupported Literal Casting `(unsigned int)0` -> `0` (Since ADead-BIB parser handles 0 implicitly as 32-bit untyped token if cast fails on arrays)
        // Wait, (unsigned)0 works for strict assignments, but nullptr is preferred for handles.
        // We will leave casting replacement to user-side code for now or perform basic heuristics:
        content = content.replace("(PFN_vkGetInstanceProcAddrType)0", "nullptr");
        content = content.replace("(VkInstance)0", "nullptr");
        content = content.replace("(VkPhysicalDevice*)0", "nullptr");
        content = content.replace("(VkQueueFamilyProperties*)0", "nullptr");
        content = content.replace("(VkImage*)0", "nullptr");
        content = content.replace("(HMODULE)0", "nullptr");

        content
    }
}
