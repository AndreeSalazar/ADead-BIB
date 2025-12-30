"""
Benchmark Comparativo: Python + FFI Libraries
==============================================
Python = Cabeza 🧠 | Librería Nativa = Cuerpo 💪

Compara:
- Python puro
- Python + ADead-BIB
- Python + C++
- Python + Rust
"""

import time
import sys
import os

# Agregar paths
sys.path.insert(0, os.path.join(os.path.dirname(__file__), 'adead'))
sys.path.insert(0, os.path.join(os.path.dirname(__file__), 'cpp'))
sys.path.insert(0, os.path.join(os.path.dirname(__file__), 'rust'))


def benchmark_python_pure(iterations=1000000):
    """Benchmark Python puro"""
    counter = 0
    while counter < iterations:
        counter += 1
    return counter


def run_benchmarks():
    print("=" * 60)
    print("BENCHMARK: Python + FFI Libraries")
    print("Python = Cabeza 🧠 | Librería Nativa = Cuerpo 💪")
    print("=" * 60)
    print()
    
    results = {}
    iterations = 10_000_000  # 10 millones para benchmark rápido
    
    # 1. Python Puro
    print("[1] Python Puro...")
    start = time.time()
    result = benchmark_python_pure(iterations)
    python_time = time.time() - start
    results['Python'] = python_time
    print(f"    Resultado: {result}")
    print(f"    Tiempo: {python_time:.3f}s")
    print()
    
    # 2. ADead-BIB
    print("[2] Python + ADead-BIB...")
    try:
        from adead_ffi import ADeadLib
        lib = ADeadLib()
        start = time.time()
        result = lib.count_to(iterations)
        adead_time = time.time() - start
        results['ADead-BIB'] = adead_time
        print(f"    Resultado: {result}")
        print(f"    Tiempo: {adead_time:.3f}s")
    except Exception as e:
        print(f"    Error: {e}")
        results['ADead-BIB'] = None
    print()
    
    # 3. C++
    print("[3] Python + C++...")
    try:
        from cpp_ffi import CppLib
        lib = CppLib()
        start = time.time()
        result = lib.count_to(iterations)
        cpp_time = time.time() - start
        results['C++'] = cpp_time
        print(f"    Resultado: {result}")
        print(f"    Tiempo: {cpp_time:.3f}s")
    except Exception as e:
        print(f"    Error: {e}")
        print("    (Compila con: g++ -shared -o cpp_lib.dll -fPIC cpp_lib.cpp)")
        results['C++'] = None
    print()
    
    # 4. Rust
    print("[4] Python + Rust...")
    try:
        from rust_ffi import RustLib
        lib = RustLib()
        start = time.time()
        result = lib.count_to(iterations)
        rust_time = time.time() - start
        results['Rust'] = rust_time
        print(f"    Resultado: {result}")
        print(f"    Tiempo: {rust_time:.3f}s")
    except Exception as e:
        print(f"    Error: {e}")
        print("    (Compila con: cargo build --release)")
        results['Rust'] = None
    print()
    
    # Resumen
    print("=" * 60)
    print("RESUMEN")
    print("=" * 60)
    print()
    print(f"{'Lenguaje':<15} {'Tiempo':<12} {'Speedup':<10}")
    print("-" * 40)
    
    base_time = results.get('Python', 1)
    for lang, t in results.items():
        if t is not None:
            speedup = base_time / t if t > 0 else 0
            print(f"{lang:<15} {t:.3f}s       {speedup:.1f}x")
        else:
            print(f"{lang:<15} {'N/A':<12} {'N/A':<10}")
    
    print()
    print("=" * 60)
    print("CONCLUSIÓN")
    print("=" * 60)
    print()
    print("ADead-BIB ofrece:")
    print("  ✅ Sintaxis simple (Python-style + Rust-style)")
    print("  ✅ Compilación a binario nativo")
    print("  ✅ FFI fácil para Python")
    print("  ✅ Binarios pequeños (~2KB)")
    print("  ✅ Sin dependencias de runtime")
    print()


if __name__ == "__main__":
    run_benchmarks()
