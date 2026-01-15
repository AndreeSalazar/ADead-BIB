// ============================================================================
// build.adB — Configuración de Build
// ============================================================================
// Define cómo se compila el proyecto.
// ============================================================================

#![project("ADead-BIB Demo")]
#![version("1.0.0")]
#![author("Eddi Andreé Salazar Matos")]

// Archivos del proyecto
#![main("main.adB")]
#![modules(
    "call.adB",
    "core/init.adB",
    "core/shutdown.adB"
)]

// Configuración de salida
#![output("demo.exe")]
#![format("pe")]           // pe, elf, raw, ahyb

// Optimizaciones
#![clean("normal")]        // none, normal, aggressive

// Target
#![target("windows-x64")]  // windows-x64, linux-x64
