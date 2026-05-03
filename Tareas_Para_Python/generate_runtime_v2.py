#!/usr/bin/env python3
"""
Genera código Rust del runtime desde knowledge.json
Versión 2.0 - Con validación robusta
"""

import json
import re
from pathlib import Path
from collections import defaultdict

# Configuración
KNOWLEDGE_JSON = Path(r"C:\Users\andre\OneDrive\Documentos\ADead-BIB\knowledge.json")
RUNTIME_PATH = Path(r"C:\Users\andre\OneDrive\Documentos\ADead-BIB\C_Real_Optimo\runtime")

# Mapeo de categoría a archivo
CATEGORY_TO_FILE = {
    "memory": RUNTIME_PATH / "memory" / "mod.rs",
    "io": RUNTIME_PATH / "io" / "mod.rs",
    "string": RUNTIME_PATH / "string" / "mod.rs",
    "math": RUNTIME_PATH / "math" / "mod.rs",
    "thread": RUNTIME_PATH / "thread" / "mod.rs",
    "win32": RUNTIME_PATH / "win32" / "mod.rs",
    "vulkan": RUNTIME_PATH / "vulkan" / "mod.rs",
    "dx12": RUNTIME_PATH / "dx12" / "mod.rs",
    "opengl": RUNTIME_PATH / "opengl" / "mod.rs",
}

# Regex para validar identificadores Rust
VALID_IDENT = re.compile(r'^[a-zA-Z_][a-zA-Z0-9_]*$')

# Palabras reservadas de Rust + nombres C inválidos
RUST_KEYWORDS = {
    # Rust keywords
    'as', 'async', 'await', 'break', 'const', 'continue', 'crate', 'dyn',
    'else', 'enum', 'extern', 'false', 'fn', 'for', 'if', 'impl', 'in',
    'let', 'loop', 'match', 'mod', 'move', 'mut', 'pub', 'ref', 'return',
    'self', 'Self', 'static', 'struct', 'super', 'trait', 'true', 'type',
    'union', 'unsafe', 'use', 'where', 'while', 'abstract', 'become',
    'box', 'do', 'final', 'macro', 'override', 'priv', 'try', 'typeof',
    'unsized', 'virtual', 'yield',
    # C/C++ keywords que no deben ser funciones
    'void', 'int', 'char', 'short', 'long', 'float', 'double', 'signed',
    'unsigned', 'sizeof', 'typedef', 'register', 'volatile', 'inline',
    # Macros/atributos comunes que no son funciones
    'WINAPI', 'CALLBACK', 'APIENTRY', '__attribute__', '__declspec',
    '__cdecl', '__stdcall', '__fastcall', 'EXTERN_C', 'FORCEINLINE',
    '__inline', '__forceinline', 'DECLARE_HANDLE', 'DEFINE_GUID',
}

# Mapeo de tipos C a Rust
TYPE_MAP = {
    # Básicos
    "void": "()",
    "char": "i8",
    "signed char": "i8",
    "unsigned char": "u8",
    "short": "i16",
    "short int": "i16",
    "signed short": "i16",
    "unsigned short": "u16",
    "int": "i32",
    "signed int": "i32",
    "unsigned": "u32",
    "unsigned int": "u32",
    "long": "i64",
    "long int": "i64",
    "signed long": "i64",
    "unsigned long": "u64",
    "long long": "i64",
    "long long int": "i64",
    "unsigned long long": "u64",
    "float": "f32",
    "double": "f64",
    "long double": "f64",
    # Stdint
    "int8_t": "i8",
    "uint8_t": "u8",
    "int16_t": "i16",
    "uint16_t": "u16",
    "int32_t": "i32",
    "uint32_t": "u32",
    "int64_t": "i64",
    "uint64_t": "u64",
    "size_t": "usize",
    "ssize_t": "isize",
    "ptrdiff_t": "isize",
    "intptr_t": "isize",
    "uintptr_t": "usize",
    # Windows básicos
    "BOOL": "i32",
    "BYTE": "u8",
    "WORD": "u16",
    "DWORD": "u32",
    "QWORD": "u64",
    "LONG": "i32",
    "ULONG": "u32",
    "LONGLONG": "i64",
    "ULONGLONG": "u64",
    "INT": "i32",
    "UINT": "u32",
    "SHORT": "i16",
    "USHORT": "u16",
    "CHAR": "i8",
    "UCHAR": "u8",
    "WCHAR": "u16",
    "FLOAT": "f32",
    "DOUBLE": "f64",
    "HRESULT": "i32",
    # Windows handles (todos son punteros opacos)
    "HANDLE": "*mut core::ffi::c_void",
    "HWND": "*mut core::ffi::c_void",
    "HINSTANCE": "*mut core::ffi::c_void",
    "HDC": "*mut core::ffi::c_void",
    "HMODULE": "*mut core::ffi::c_void",
    "HKEY": "*mut core::ffi::c_void",
    "HBITMAP": "*mut core::ffi::c_void",
    "HBRUSH": "*mut core::ffi::c_void",
    "HFONT": "*mut core::ffi::c_void",
    "HICON": "*mut core::ffi::c_void",
    "HMENU": "*mut core::ffi::c_void",
    "HPEN": "*mut core::ffi::c_void",
    "HRGN": "*mut core::ffi::c_void",
    "HCURSOR": "*mut core::ffi::c_void",
    "HGLOBAL": "*mut core::ffi::c_void",
    "HLOCAL": "*mut core::ffi::c_void",
    "LPVOID": "*mut core::ffi::c_void",
    "LPCVOID": "*const core::ffi::c_void",
    "LPSTR": "*mut i8",
    "LPCSTR": "*const i8",
    "LPWSTR": "*mut u16",
    "LPCWSTR": "*const u16",
    "PVOID": "*mut core::ffi::c_void",
    # Windows status codes (todos son enteros)
    "CONFIGRET": "u32",
    "NTSTATUS": "i32",
    "LSTATUS": "i32",
    "MMRESULT": "u32",
    "LRESULT": "isize",
    "WPARAM": "usize",
    "LPARAM": "isize",
    "ATOM": "u16",
    "COLORREF": "u32",
    # Vulkan
    "VkResult": "i32",
    "VkBool32": "u32",
    "VkFlags": "u32",
    "VkDeviceSize": "u64",
    "VkSampleMask": "u32",
}

def is_valid_identifier(name: str) -> bool:
    """Verifica si es un identificador Rust válido"""
    if not name or not VALID_IDENT.match(name):
        return False
    if name in RUST_KEYWORDS:
        return False
    return True

def clean_type(c_type: str) -> str:
    """Limpia y normaliza un tipo C"""
    # Remover keywords de calling convention
    for kw in ['WINAPI', 'APIENTRY', 'CALLBACK', '__cdecl', '__stdcall', '__fastcall',
               'WINAPIV', 'NTAPI', 'CDECL', 'PASCAL', 'FAR', 'NEAR', '__inline',
               'inline', 'static', 'extern', 'const', 'volatile', 'register']:
        c_type = re.sub(rf'\b{kw}\b', '', c_type)
    
    # Limpiar espacios múltiples
    c_type = ' '.join(c_type.split())
    return c_type.strip()

def convert_type(c_type: str, is_return: bool = False) -> str:
    """Convierte tipo C a Rust"""
    c_type = clean_type(c_type)
    
    if not c_type:
        return "()"
    
    # Contar punteros
    ptr_count = c_type.count('*')
    base = c_type.replace('*', '').strip()
    
    # void sin punteros = () en Rust
    if base == "void" and ptr_count == 0:
        return "()"
    
    # void* = *mut c_void
    if base == "void" and ptr_count > 0:
        rust_type = "core::ffi::c_void"
        for _ in range(ptr_count):
            rust_type = f"*mut {rust_type}"
        return rust_type
    
    # Buscar en mapeo
    if base in TYPE_MAP:
        rust_type = TYPE_MAP[base]
    else:
        # Tipo desconocido sin puntero = usize (opaque value)
        # Tipo desconocido con puntero = *mut c_void
        if ptr_count == 0:
            rust_type = "usize"  # Valor opaco
        else:
            rust_type = "core::ffi::c_void"
    
    # Agregar punteros
    for _ in range(ptr_count):
        rust_type = f"*mut {rust_type}"
    
    return rust_type

def parse_param(param: str, index: int) -> tuple:
    """Parsea un parámetro C y retorna (nombre, tipo_rust)"""
    param = clean_type(param)
    
    if not param or param == "void" or param == "...":
        return None
    
    # Detectar punteros en el nombre
    parts = param.split()
    if not parts:
        return None
    
    # Caso: solo tipo (sin nombre)
    if len(parts) == 1:
        c_type = parts[0]
        name = f"arg{index}"
        rust_type = convert_type(c_type)
        return (name, rust_type)
    
    # Último elemento es el nombre (posiblemente con *)
    name = parts[-1]
    c_type = ' '.join(parts[:-1])
    
    # Mover * del nombre al tipo
    while name.startswith('*'):
        c_type += '*'
        name = name[1:]
    
    # Limpiar nombre
    name = re.sub(r'[^a-zA-Z0-9_]', '', name)
    if not name:
        name = f"arg{index}"
    
    # Validar nombre
    if not is_valid_identifier(name):
        name = f"arg{index}"
    
    rust_type = convert_type(c_type)
    return (name, rust_type)

def generate_function(func: dict, seen_names: set) -> str:
    """Genera código Rust para una función"""
    name = func.get("name", "")
    
    # Validar nombre
    if not is_valid_identifier(name):
        return None
    
    # Evitar duplicados
    if name in seen_names:
        return None
    seen_names.add(name)
    
    # Parsear tipo de retorno
    return_type = convert_type(func.get("return", "void"))
    
    # Parsear parámetros
    params = func.get("params", [])
    rust_params = []
    param_names_seen = set()
    
    for i, p in enumerate(params):
        result = parse_param(p, i)
        if result:
            pname, ptype = result
            # Evitar nombres duplicados
            if pname in param_names_seen:
                pname = f"{pname}_{i}"
            param_names_seen.add(pname)
            rust_params.append(f"{pname}: {ptype}")
    
    params_str = ", ".join(rust_params)
    
    # Determinar valor de retorno
    if return_type == "()":
        return_val = ""
        return_type_str = ""
    elif return_type.startswith("*"):
        return_val = "    core::ptr::null_mut()"
        return_type_str = f" -> {return_type}"
    elif return_type in ["i8", "i16", "i32", "i64", "isize"]:
        return_val = "    0"
        return_type_str = f" -> {return_type}"
    elif return_type in ["u8", "u16", "u32", "u64", "usize"]:
        return_val = "    0"
        return_type_str = f" -> {return_type}"
    elif return_type in ["f32", "f64"]:
        return_val = "    0.0"
        return_type_str = f" -> {return_type}"
    else:
        return_val = "    core::ptr::null_mut()"
        return_type_str = f" -> {return_type}"
    
    source = func.get("source", "unknown")
    header = func.get("header", "unknown")
    
    code = f"""/// {name} - from {source}/{header}
#[no_mangle]
pub unsafe extern "C" fn {name}({params_str}){return_type_str} {{
{return_val}
}}

"""
    return code

def generate_module(category: str, functions: list) -> tuple:
    """Genera el código de un módulo completo"""
    header = f"""//! ADead Runtime - {category.upper()} Module
//!
//! Funciones generadas automáticamente desde knowledge.json
//! Categoría: {category}

#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

"""
    
    seen_names = set()
    code = ""
    valid_count = 0
    
    for func in functions:
        func_code = generate_function(func, seen_names)
        if func_code:
            code += func_code
            valid_count += 1
    
    return header + code, valid_count

def main():
    print("🔍 Leyendo knowledge.json...")
    
    with open(KNOWLEDGE_JSON, 'r', encoding='utf-8') as f:
        data = json.load(f)
    
    functions = data.get("functions", [])
    print(f"📊 Total funciones en JSON: {len(functions)}")
    
    # Agrupar por categoría
    by_category = defaultdict(list)
    for func in functions:
        category = func.get("category", "other")
        if category in CATEGORY_TO_FILE:
            by_category[category].append(func)
    
    # Generar módulos
    stats = {}
    total_valid = 0
    
    for category, funcs in by_category.items():
        output_file = CATEGORY_TO_FILE[category]
        output_file.parent.mkdir(parents=True, exist_ok=True)
        
        code, valid_count = generate_module(category, funcs)
        output_file.write_text(code, encoding='utf-8')
        
        stats[category] = (valid_count, len(funcs))
        total_valid += valid_count
        print(f"  ✅ {category:10s}: {valid_count:5d} válidas (de {len(funcs)})")
    
    print(f"\n📈 Resumen:")
    print(f"  Total válidas: {total_valid}")
    print(f"  Módulos: {len(stats)}")

if __name__ == "__main__":
    main()
