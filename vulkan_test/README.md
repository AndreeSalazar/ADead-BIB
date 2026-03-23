# ADead-BIB Vulkan Test

## Descripción

Ejemplo completo de Vulkan para ADead-BIB que dibuja un triángulo con gradiente RGB.

## Estructura

```
vulkan_test/
├── main.cpp              # Código principal C++
├── include/
│   └── vulkan.h          # Header Vulkan completo (1000+ líneas)
├── shaders/
│   ├── triangle.vert     # Vertex shader GLSL
│   └── triangle.frag     # Fragment shader GLSL
└── README.md
```

## Compilar

```bash
# Con ADead-BIB
adb cxx vulkan_test/main.cpp -o vulkan_triangle.exe

# Step mode para análisis
adb step vulkan_test/main.cpp
```

## Requisitos

- **Vulkan SDK** instalado (para vulkan-1.dll)
- **GPU compatible** con Vulkan 1.0+
- **Windows 10/11** (usa Win32 surface)

## Características

### Header vulkan.h

El header incluye:

- **Tipos básicos**: `VkInstance`, `VkDevice`, `VkQueue`, etc.
- **Enums**: `VkResult`, `VkFormat`, `VkPrimitiveTopology`, etc.
- **Structs**: `VkApplicationInfo`, `VkDeviceCreateInfo`, `VkPipelineCreateInfo`, etc.
- **Funciones**: 50+ funciones de Vulkan Core + KHR extensions

### Shaders SPIR-V

Los shaders están pre-compilados como arrays de `uint32_t` en el código:

```cpp
const uint32_t vertShaderCode[] = { 0x07230203, ... };
const uint32_t fragShaderCode[] = { 0x07230203, ... };
```

Para recompilar los shaders:

```bash
glslangValidator -V shaders/triangle.vert -o vert.spv
glslangValidator -V shaders/triangle.frag -o frag.spv
```

## Pipeline Vulkan

1. **Instance** → Crear instancia Vulkan
2. **Surface** → Crear superficie Win32
3. **Physical Device** → Seleccionar GPU
4. **Logical Device** → Crear dispositivo lógico
5. **Swapchain** → Crear cadena de intercambio
6. **Render Pass** → Definir pases de renderizado
7. **Pipeline** → Crear pipeline gráfico
8. **Command Buffer** → Grabar comandos
9. **Draw** → Dibujar triángulo

## Resultado Esperado

Ventana 800x600 con triángulo RGB:
- Vértice superior: **Rojo**
- Vértice inferior derecho: **Verde**
- Vértice inferior izquierdo: **Azul**
- Gradiente interpolado entre vértices

## Notas

- El código usa SPIR-V pre-compilado para evitar dependencia de glslang
- Compatible con Vulkan 1.0+ (no requiere features avanzados)
- Usa `STATIC` window class para evitar RegisterClassExA

## Autor

Eddi Andreé Salazar Matos — Marzo 2026
