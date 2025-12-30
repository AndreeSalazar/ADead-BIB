"""
ADead-BIB FFI for Python
========================
Python = Cabeza 🧠 | ADead-BIB = Cuerpo 💪

Uso:
    from adead_ffi import ADeadLib
    lib = ADeadLib()
    result = lib.count_billion()
"""

import ctypes
import os
import subprocess
import time

class ADeadLib:
    """Wrapper Python para librería ADead-BIB nativa"""
    
    def __init__(self, lib_path=None):
        """
        Inicializa la librería ADead-BIB.
        Si no existe el binario, lo compila automáticamente.
        """
        self.lib_path = lib_path or self._get_default_path()
        self._lib = None
        
        # Por ahora, ejecutamos directamente el compilador ADead-BIB
        # En el futuro: cargar DLL con ctypes
        self._adead_source = os.path.join(os.path.dirname(__file__), "adead_lib.adB")
    
    def _get_default_path(self):
        """Obtiene el path por defecto de la librería"""
        if os.name == 'nt':
            return "adead_lib.dll"
        else:
            return "libadead.so"
    
    def _run_adead(self, code):
        """Ejecuta código ADead-BIB y retorna el resultado"""
        # Crear archivo temporal con el código
        temp_file = "_temp_adead.adB"
        with open(temp_file, 'w') as f:
            f.write(code)
        
        try:
            # Ejecutar con adeadc
            result = subprocess.run(
                ["adeadc", "run", temp_file],
                capture_output=True,
                text=True,
                timeout=60
            )
            output = result.stdout.strip()
            # Filtrar mensajes del compilador (🚀 Running..., warnings, etc.)
            lines = output.split('\n')
            # Tomar solo la última línea que contiene el resultado
            for line in reversed(lines):
                line = line.strip()
                if line and not line.startswith('🚀') and not line.startswith('Running') and not 'Warning' in line:
                    return line
            return output
        finally:
            if os.path.exists(temp_file):
                os.remove(temp_file)
    
    # ==========================================
    # Funciones de la librería
    # ==========================================
    
    def count_to(self, limit: int) -> int:
        """Cuenta hasta el límite especificado"""
        code = f'''
fn main() {{
    let counter = 0
    while counter < {limit} {{
        counter = counter + 1
    }}
    println(counter)
}}
'''
        result = self._run_adead(code)
        return int(result) if result else 0
    
    def count_billion(self) -> int:
        """Cuenta hasta 1 billón - benchmark principal"""
        return self.count_to(1000000000)
    
    def fibonacci(self, n: int) -> int:
        """Calcula el n-ésimo número de Fibonacci"""
        code = f'''
fn main() {{
    let n = {n}
    if n <= 1 {{
        println(n)
    }} else {{
        let a = 0
        let b = 1
        let i = 2
        while i <= n {{
            let temp = a + b
            a = b
            b = temp
            i = i + 1
        }}
        println(b)
    }}
}}
'''
        result = self._run_adead(code)
        return int(result) if result else 0
    
    def factorial(self, n: int) -> int:
        """Calcula el factorial de n"""
        code = f'''
fn main() {{
    let result = 1
    let i = 1
    while i <= {n} {{
        result = result * i
        i = i + 1
    }}
    println(result)
}}
'''
        result = self._run_adead(code)
        return int(result) if result else 0
    
    def multiply(self, a: int, b: int) -> int:
        """Multiplica dos números"""
        code = f'''
fn main() {{
    println({a} * {b})
}}
'''
        result = self._run_adead(code)
        return int(result) if result else 0
    
    def power(self, base: int, exp: int) -> int:
        """Calcula base^exp"""
        code = f'''
fn main() {{
    let result = 1
    let i = 0
    while i < {exp} {{
        result = result * {base}
        i = i + 1
    }}
    println(result)
}}
'''
        result = self._run_adead(code)
        return int(result) if result else 0


def benchmark():
    """Ejecuta benchmark comparativo"""
    print("=" * 50)
    print("ADead-BIB FFI Benchmark")
    print("Python = Cabeza 🧠 | ADead-BIB = Cuerpo 💪")
    print("=" * 50)
    print()
    
    lib = ADeadLib()
    
    # Test funciones básicas
    print("[1] Funciones básicas:")
    print(f"    fibonacci(10) = {lib.fibonacci(10)}")
    print(f"    factorial(5) = {lib.factorial(5)}")
    print(f"    multiply(7, 8) = {lib.multiply(7, 8)}")
    print(f"    power(2, 10) = {lib.power(2, 10)}")
    print()
    
    # Benchmark: Python puro vs ADead-BIB
    print("[2] Benchmark: Contador")
    
    # Python puro
    print("    Python puro (1M)...", end=" ")
    start = time.time()
    counter = 0
    while counter < 1000000:
        counter += 1
    python_time = time.time() - start
    print(f"{python_time:.3f}s")
    
    # ADead-BIB
    print("    ADead-BIB (1M)...", end=" ")
    start = time.time()
    result = lib.count_to(1000000)
    adead_time = time.time() - start
    print(f"{adead_time:.3f}s")
    
    print()
    print(f"    Speedup: {python_time/adead_time:.1f}x más rápido con ADead-BIB")
    print()
    print("=" * 50)
    print("¡Librería ADead-BIB lista para Python!")
    print("=" * 50)


if __name__ == "__main__":
    benchmark()
