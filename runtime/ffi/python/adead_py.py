"""
ADead-BIB Universal Runtime - Python FFI
=========================================
Author: Eddi Andreé Salazar Matos
Email: eddi.salazar.dev@gmail.com
Made with love in Peru

Wrapper Python para el runtime universal.
"""

import ctypes
import numpy as np
from pathlib import Path
from typing import Optional, Tuple, List
from enum import IntEnum


class Backend(IntEnum):
    CPU = 0
    CUDA = 1
    VULKAN = 2
    AUTO = 255


class DType(IntEnum):
    F32 = 0
    F64 = 1
    I32 = 2
    I64 = 3
    U8 = 4
    I8 = 5
    F16 = 6
    BF16 = 7


class ADeadError(Exception):
    """Error del runtime ADead-BIB."""
    pass


class ADeadRuntime:
    """
    Runtime universal ADead-BIB.
    
    Ejemplo:
        rt = ADeadRuntime(Backend.AUTO)
        a = rt.tensor([1024, 1024])
        b = rt.tensor([1024, 1024])
        c = rt.tensor([1024, 1024])
        
        a.copy_from(np.random.randn(1024, 1024).astype(np.float32))
        b.copy_from(np.random.randn(1024, 1024).astype(np.float32))
        
        rt.matmul(a, b, c)
        result = c.to_numpy()
    """
    
    def __init__(self, backend: Backend = Backend.AUTO, lib_path: Optional[str] = None):
        """
        Inicializa el runtime.
        
        Args:
            backend: Backend a usar (CPU, CUDA, VULKAN, AUTO)
            lib_path: Ruta a la librería compartida (opcional)
        """
        self._lib = self._load_library(lib_path)
        self._runtime = None
        self._tensors = []  # Track tensors for cleanup
        
        if self._lib:
            self._setup_functions()
            # Inicializar runtime nativo
            self._runtime = ctypes.c_void_p()
            err = self._lib.adead_init(ctypes.byref(self._runtime), backend)
            if err != 0:
                raise ADeadError(f"Failed to initialize runtime: error {err}")
        # Si no hay librería, usa fallback Python puro
    
    def _load_library(self, lib_path: Optional[str]) -> ctypes.CDLL:
        """Carga la librería del runtime."""
        if lib_path:
            return ctypes.CDLL(lib_path)
        
        # Buscar en ubicaciones comunes
        search_paths = [
            Path(__file__).parent / "libadead_runtime.so",
            Path(__file__).parent / "libadead_runtime.dylib",
            Path(__file__).parent / "adead_runtime.dll",
            Path(__file__).parent.parent.parent / "build" / "libadead_runtime.so",
        ]
        
        for path in search_paths:
            if path.exists():
                return ctypes.CDLL(str(path))
        
        # Si no se encuentra, usar implementación Python pura
        return None
    
    def _setup_functions(self):
        """Configura los tipos de las funciones."""
        if self._lib is None:
            return
        
        # adead_init
        self._lib.adead_init.argtypes = [ctypes.c_void_p, ctypes.c_int]
        self._lib.adead_init.restype = ctypes.c_int
        
        # adead_shutdown
        self._lib.adead_shutdown.argtypes = [ctypes.c_void_p]
        self._lib.adead_shutdown.restype = None
        
        # etc...
    
    def __del__(self):
        """Destructor."""
        if hasattr(self, '_runtime') and self._runtime and self._lib:
            self._lib.adead_shutdown(self._runtime)
    
    @property
    def backend(self) -> Backend:
        """Obtiene el backend activo."""
        if self._lib:
            return Backend(self._lib.adead_get_backend(self._runtime))
        return Backend.CPU
    
    @property
    def backend_name(self) -> str:
        """Obtiene el nombre del backend."""
        if self._lib:
            return self._lib.adead_get_backend_name(self._runtime).decode()
        return "Python (Pure)"
    
    def tensor(self, shape: List[int], dtype: DType = DType.F32) -> 'Tensor':
        """Crea un tensor."""
        t = Tensor(self, shape, dtype)
        self._tensors.append(t)
        return t
    
    def matmul(self, a: 'Tensor', b: 'Tensor', c: 'Tensor'):
        """Multiplicación de matrices: C = A @ B."""
        if self._lib:
            err = self._lib.adead_matmul(self._runtime, a._ptr, b._ptr, c._ptr)
            if err != 0:
                raise ADeadError(f"MatMul failed: error {err}")
        else:
            # Fallback Python
            c._data[:] = np.matmul(a._data, b._data)
    
    def add(self, a: 'Tensor', b: 'Tensor', c: 'Tensor'):
        """Suma: C = A + B."""
        if self._lib:
            err = self._lib.adead_add(self._runtime, a._ptr, b._ptr, c._ptr)
            if err != 0:
                raise ADeadError(f"Add failed: error {err}")
        else:
            c._data[:] = a._data + b._data
    
    def relu(self, inp: 'Tensor', out: 'Tensor'):
        """ReLU: out = max(0, in)."""
        if self._lib:
            err = self._lib.adead_relu(self._runtime, inp._ptr, out._ptr)
            if err != 0:
                raise ADeadError(f"ReLU failed: error {err}")
        else:
            out._data[:] = np.maximum(0, inp._data)
    
    def softmax(self, inp: 'Tensor', out: 'Tensor'):
        """Softmax por filas."""
        if self._lib:
            err = self._lib.adead_softmax(self._runtime, inp._ptr, out._ptr)
            if err != 0:
                raise ADeadError(f"Softmax failed: error {err}")
        else:
            exp_x = np.exp(inp._data - np.max(inp._data, axis=-1, keepdims=True))
            out._data[:] = exp_x / np.sum(exp_x, axis=-1, keepdims=True)
    
    def sync(self):
        """Sincroniza operaciones pendientes."""
        if self._lib:
            self._lib.adead_sync(self._runtime)
    
    def memory_available(self) -> int:
        """Memoria disponible en bytes."""
        if self._lib:
            return self._lib.adead_memory_available(self._runtime)
        return 16 * 1024 * 1024 * 1024  # 16GB default
    
    def memory_used(self) -> int:
        """Memoria usada en bytes."""
        if self._lib:
            return self._lib.adead_memory_used(self._runtime)
        return sum(t._data.nbytes for t in self._tensors if t._data is not None)


class Tensor:
    """Tensor del runtime ADead-BIB."""
    
    def __init__(self, runtime: ADeadRuntime, shape: List[int], dtype: DType = DType.F32):
        self._runtime = runtime
        self._shape = tuple(shape)
        self._dtype = dtype
        self._ptr = None
        
        # Mapeo de dtype a numpy
        dtype_map = {
            DType.F32: np.float32,
            DType.F64: np.float64,
            DType.I32: np.int32,
            DType.I64: np.int64,
            DType.U8: np.uint8,
            DType.I8: np.int8,
        }
        
        np_dtype = dtype_map.get(dtype, np.float32)
        
        if runtime._lib:
            # Usar runtime nativo
            shape_arr = (ctypes.c_uint64 * len(shape))(*shape)
            self._ptr = ctypes.c_void_p()
            err = runtime._lib.adead_tensor_create(
                runtime._runtime, ctypes.byref(self._ptr),
                shape_arr, len(shape), dtype
            )
            if err != 0:
                raise ADeadError(f"Failed to create tensor: error {err}")
            self._data = None
        else:
            # Fallback Python
            self._data = np.zeros(shape, dtype=np_dtype)
    
    def __del__(self):
        if self._ptr and self._runtime._lib:
            self._runtime._lib.adead_tensor_destroy(self._runtime._runtime, self._ptr)
    
    @property
    def shape(self) -> Tuple[int, ...]:
        return self._shape
    
    @property
    def dtype(self) -> DType:
        return self._dtype
    
    @property
    def size(self) -> int:
        return int(np.prod(self._shape))
    
    def copy_from(self, data: np.ndarray):
        """Copia datos al tensor."""
        data = np.ascontiguousarray(data)
        
        if self._runtime._lib and self._ptr:
            self._runtime._lib.adead_tensor_copy_from(
                self._runtime._runtime, self._ptr,
                data.ctypes.data, data.nbytes
            )
        else:
            self._data[:] = data
    
    def to_numpy(self) -> np.ndarray:
        """Copia datos del tensor a numpy."""
        if self._runtime._lib and self._ptr:
            dtype_map = {
                DType.F32: np.float32,
                DType.F64: np.float64,
                DType.I32: np.int32,
                DType.I64: np.int64,
            }
            np_dtype = dtype_map.get(self._dtype, np.float32)
            result = np.zeros(self._shape, dtype=np_dtype)
            
            self._runtime._lib.adead_tensor_copy_to(
                self._runtime._runtime, self._ptr,
                result.ctypes.data, result.nbytes
            )
            return result
        else:
            return self._data.copy()


def version() -> str:
    """Obtiene la versión del runtime."""
    return "ADead-BIB Runtime 1.0.0 (Python)"


# ============================================================
# Demo
# ============================================================

def demo():
    """Demo del runtime Python."""
    print("=" * 60)
    print("   ADead-BIB Runtime - Python FFI Demo")
    print("   Author: Eddi Andree Salazar Matos")
    print("=" * 60)
    
    print(f"\nVersion: {version()}")
    
    # Crear runtime (usará fallback Python si no hay librería)
    rt = ADeadRuntime(Backend.AUTO)
    print(f"Backend: {rt.backend_name}")
    
    # Crear tensores
    print("\nCreando tensores 1024x1024...")
    a = rt.tensor([1024, 1024])
    b = rt.tensor([1024, 1024])
    c = rt.tensor([1024, 1024])
    
    # Inicializar con datos
    a.copy_from(np.random.randn(1024, 1024).astype(np.float32))
    b.copy_from(np.random.randn(1024, 1024).astype(np.float32))
    
    # MatMul
    import time
    print("Ejecutando MatMul...")
    start = time.perf_counter()
    rt.matmul(a, b, c)
    rt.sync()
    elapsed = (time.perf_counter() - start) * 1000
    print(f"Tiempo: {elapsed:.2f} ms")
    
    # Verificar resultado
    result = c.to_numpy()
    print(f"Shape resultado: {result.shape}")
    print(f"Suma resultado: {result.sum():.2f}")
    
    print("\n" + "=" * 60)
    print("   Demo completada")
    print("=" * 60)


if __name__ == "__main__":
    demo()
