"""
ADead-BIB FFI Python Bindings
=============================
Author: Eddi Andreé Salazar Matos
Email: eddi.salazar.dev@gmail.com
Made with ❤️ in Peru 🇵🇪

ADead-BIB como cabeza principal ABI para Python.
Permite usar funciones compiladas de ADead-BIB desde Python.
"""

import subprocess
import os
import ctypes
from pathlib import Path
from typing import Optional, List, Any, Callable


class ADeadFFI:
    """
    Foreign Function Interface para ADead-BIB.
    ADead-BIB actúa como cabeza principal ABI.
    """
    
    def __init__(self, compiler_path: Optional[str] = None):
        """
        Inicializa el FFI de ADead-BIB.
        
        Args:
            compiler_path: Ruta al compilador adeadc.exe
        """
        self.base_dir = Path(__file__).parent.parent.parent
        
        if compiler_path:
            self.compiler = Path(compiler_path)
        else:
            possible_paths = [
                self.base_dir / "target" / "release" / "adeadc.exe",
                self.base_dir / "target" / "debug" / "adeadc.exe",
            ]
            for p in possible_paths:
                if p.exists():
                    self.compiler = p
                    break
            else:
                self.compiler = None
        
        self._cache = {}
    
    def compile(self, source_file: str) -> str:
        """Compila archivo .adB a ejecutable."""
        if not self.compiler:
            raise RuntimeError("Compilador ADead-BIB no encontrado")
        
        source = Path(source_file).resolve()
        result = subprocess.run(
            [str(self.compiler), "build", str(source)],
            capture_output=True,
            encoding='utf-8',
            errors='replace'
        )
        
        if result.returncode != 0:
            raise RuntimeError(f"Error de compilación: {result.stderr}")
        
        return str(source.with_suffix('.exe'))
    
    def run(self, source_file: str, *args) -> str:
        """Compila y ejecuta archivo .adB."""
        if not self.compiler:
            raise RuntimeError("Compilador ADead-BIB no encontrado")
        
        source = Path(source_file).resolve()
        result = subprocess.run(
            [str(self.compiler), "run", str(source)],
            capture_output=True,
            encoding='utf-8',
            errors='replace'
        )
        
        return result.stdout
    
    def call(self, func_name: str, *args) -> Any:
        """
        Llama una función exportada de ADead-BIB.
        
        Args:
            func_name: Nombre de la función
            *args: Argumentos
            
        Returns:
            Resultado de la función
        """
        # Generar código ADead-BIB dinámico
        args_str = ", ".join(str(a) for a in args)
        code = f"""
int main() {{
    int result = {func_name}({args_str})
    printf(result)
    return 0
}}
"""
        return self.run_code(code)
    
    def run_code(self, code: str) -> str:
        """Ejecuta código ADead-BIB desde string."""
        temp_file = self.base_dir / "FFI" / "examples" / "_temp.adB"
        
        with open(temp_file, 'w', encoding='utf-8') as f:
            f.write(code)
        
        try:
            return self.run(str(temp_file))
        finally:
            try:
                os.unlink(temp_file)
            except:
                pass
    
    # ============================================
    # FUNCIONES MATEMÁTICAS (usando ABI)
    # ============================================
    
    def sum(self, a: int, b: int) -> int:
        """Suma usando ADead-BIB ABI."""
        code = f"""
int main() {{
    printf({a} + {b})
    return 0
}}
"""
        result = self.run_code(code)
        return int(result.strip()) if result.strip() else 0
    
    def mul(self, a: int, b: int) -> int:
        """Multiplicación usando ADead-BIB ABI."""
        code = f"""
int main() {{
    printf({a} * {b})
    return 0
}}
"""
        result = self.run_code(code)
        return int(result.strip()) if result.strip() else 0
    
    def max(self, a: int, b: int) -> int:
        """Máximo usando ADead-BIB ABI."""
        code = f"""
int main() {{
    if {a} > {b} {{
        printf({a})
    }} else {{
        printf({b})
    }}
    return 0
}}
"""
        result = self.run_code(code)
        return int(result.strip()) if result.strip() else 0
    
    def factorial(self, n: int) -> int:
        """Factorial usando ADead-BIB ABI."""
        code = f"""
int factorial(int n) {{
    if n <= 1 {{
        return 1
    }}
    return n * factorial(n - 1)
}}

int main() {{
    printf(factorial({n}))
    return 0
}}
"""
        result = self.run_code(code)
        return int(result.strip()) if result.strip() else 0
    
    def fib(self, n: int) -> int:
        """Fibonacci usando ADead-BIB ABI."""
        code = f"""
int fib(int n) {{
    if n <= 1 {{
        return n
    }}
    return fib(n - 1) + fib(n - 2)
}}

int main() {{
    printf(fib({n}))
    return 0
}}
"""
        result = self.run_code(code)
        return int(result.strip()) if result.strip() else 0
    
    # ============================================
    # INTEGRACIÓN CON METAL_DEAD
    # ============================================
    
    def get_compute_kernel(self, operation: str) -> str:
        """
        Genera kernel de cómputo optimizado para Metal_Dead.
        
        Args:
            operation: Tipo de operación (matmul, vecadd, etc.)
            
        Returns:
            Código del kernel
        """
        kernels = {
            "vecadd": """
int main() {
    // Vector addition kernel
    printf("VecAdd Kernel Ready\\n")
    return 0
}
""",
            "matmul": """
int main() {
    // Matrix multiplication kernel
    printf("MatMul Kernel Ready\\n")
    return 0
}
""",
            "reduce": """
int main() {
    // Reduction kernel
    printf("Reduce Kernel Ready\\n")
    return 0
}
"""
        }
        return kernels.get(operation, kernels["vecadd"])


class ADeadMetalBridge:
    """
    Puente entre ADead-BIB y Metal_Dead.
    ADead-BIB compila kernels, Metal_Dead los ejecuta en GPU.
    """
    
    def __init__(self):
        self.ffi = ADeadFFI()
        self.metal_dead = None
        
        # Intentar importar Metal_Dead
        try:
            import sys
            sys.path.insert(0, str(Path(__file__).parent.parent.parent / "Metal_Dead"))
            from core.metal_dead import MetalDead
            self.metal_dead = MetalDead()
        except ImportError:
            pass
    
    def compute(self, operation: str, data: List[int]) -> Any:
        """
        Ejecuta operación usando ADead-BIB + Metal_Dead.
        
        Args:
            operation: Tipo de operación
            data: Datos de entrada
            
        Returns:
            Resultado
        """
        if operation == "sum":
            # Usar ADead-BIB para suma rápida
            total = 0
            for i in range(0, len(data), 2):
                if i + 1 < len(data):
                    total += self.ffi.sum(data[i], data[i+1])
                else:
                    total += data[i]
            return total
        
        return None


# ============================================
# EJEMPLO DE USO
# ============================================

if __name__ == "__main__":
    print("=== ADead-BIB FFI Python Demo ===\n")
    
    try:
        ffi = ADeadFFI()
        print("✓ FFI inicializado")
        
        # Test funciones matemáticas
        print(f"\nffi.sum(10, 20) = {ffi.sum(10, 20)}")
        print(f"ffi.mul(5, 6) = {ffi.mul(5, 6)}")
        print(f"ffi.max(15, 8) = {ffi.max(15, 8)}")
        print(f"ffi.factorial(5) = {ffi.factorial(5)}")
        
        print("\n=== FFI Python Funcionando ===")
        
    except Exception as e:
        print(f"Error: {e}")
