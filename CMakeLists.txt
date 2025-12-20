cmake_minimum_required(VERSION 3.10)
project(adead_emitter)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Directorio de headers
include_directories(src/cpp/emitter)

# Librería emitter
add_library(emitter STATIC
    src/cpp/emitter/emitter.cpp
    src/cpp/emitter/opcodes.cpp
)

# Opciones de compilación
target_compile_options(emitter PRIVATE
    -Wall
    -Wextra
    -O2
)

