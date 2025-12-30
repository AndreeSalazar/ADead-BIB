"""
Rust FFI for Python
===================
Requiere compilar con: cargo build --release
La DLL estará en: target/release/rust_lib.dll
"""

import ctypes
import os
import time

class RustLib:
    """Wrapper Python para librería Rust nativa"""
    
    def __init__(self, lib_path=None):
        self.lib_path = lib_path or os.path.join(
            os.path.dirname(__file__), 
            "target", "release", "rust_lib.dll"
        )
        
        if not os.path.exists(self.lib_path):
            raise FileNotFoundError(
                f"Librería no encontrada: {self.lib_path}\n"
                "Compila con: cargo build --release"
            )
        
        self._lib = ctypes.CDLL(self.lib_path)
        self._setup_functions()
    
    def _setup_functions(self):
        """Configura los tipos de las funciones"""
        # count_to
        self._lib.count_to.argtypes = [ctypes.c_int64]
        self._lib.count_to.restype = ctypes.c_int64
        
        # count_billion
        self._lib.count_billion.argtypes = []
        self._lib.count_billion.restype = ctypes.c_int64
        
        # fibonacci
        self._lib.fibonacci.argtypes = [ctypes.c_int64]
        self._lib.fibonacci.restype = ctypes.c_int64
        
        # factorial
        self._lib.factorial.argtypes = [ctypes.c_int64]
        self._lib.factorial.restype = ctypes.c_int64
        
        # multiply
        self._lib.multiply.argtypes = [ctypes.c_int64, ctypes.c_int64]
        self._lib.multiply.restype = ctypes.c_int64
        
        # power
        self._lib.power.argtypes = [ctypes.c_int64, ctypes.c_int64]
        self._lib.power.restype = ctypes.c_int64
    
    def count_to(self, limit: int) -> int:
        return self._lib.count_to(limit)
    
    def count_billion(self) -> int:
        return self._lib.count_billion()
    
    def fibonacci(self, n: int) -> int:
        return self._lib.fibonacci(n)
    
    def factorial(self, n: int) -> int:
        return self._lib.factorial(n)
    
    def multiply(self, a: int, b: int) -> int:
        return self._lib.multiply(a, b)
    
    def power(self, base: int, exp: int) -> int:
        return self._lib.power(base, exp)


def benchmark():
    """Ejecuta benchmark"""
    print("=" * 50)
    print("Rust FFI Benchmark")
    print("=" * 50)
    
    try:
        lib = RustLib()
    except FileNotFoundError as e:
        print(f"Error: {e}")
        return
    
    print(f"fibonacci(10) = {lib.fibonacci(10)}")
    print(f"factorial(5) = {lib.factorial(5)}")
    print(f"multiply(7, 8) = {lib.multiply(7, 8)}")
    print(f"power(2, 10) = {lib.power(2, 10)}")
    
    print()
    print("Benchmark: count_billion()...")
    start = time.time()
    result = lib.count_billion()
    elapsed = time.time() - start
    print(f"Resultado: {result}")
    print(f"Tiempo: {elapsed:.3f}s")


if __name__ == "__main__":
    benchmark()
