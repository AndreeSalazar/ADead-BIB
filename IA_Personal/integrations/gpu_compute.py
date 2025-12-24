"""
GPU Compute para IA-Personal
=============================
Author: Eddi Andreé Salazar Matos
Email: eddi.salazar.dev@gmail.com
Made with ❤️ in Peru 🇵🇪

Integración GPU completa para IA-Personal:
- Detección automática de hardware
- Operaciones aceleradas (matmul, attention, softmax)
- Transformer completo en GPU
- Benchmark CPU vs GPU

Uso:
    from IA_Personal.integrations.gpu_compute import GPUCompute, IAPersonalGPU
    
    gpu = GPUCompute()
    gpu.benchmark()
    
    ia = IAPersonalGPU()
    ia.chat("Hola")
"""

import os
import sys
import time
import platform
from pathlib import Path
from typing import Dict, List, Optional, Tuple, Any
from dataclasses import dataclass
from enum import Enum

import numpy as np

# Detectar backends disponibles
HAS_TORCH = False
TORCH_CUDA = False
HAS_CUPY = False

try:
    import torch
    HAS_TORCH = True
    TORCH_CUDA = torch.cuda.is_available()
    if TORCH_CUDA:
        TORCH_DEVICE = torch.device("cuda")
        TORCH_GPU_NAME = torch.cuda.get_device_name(0)
        TORCH_GPU_MEMORY = torch.cuda.get_device_properties(0).total_memory / (1024**3)
    else:
        TORCH_DEVICE = torch.device("cpu")
        TORCH_GPU_NAME = None
        TORCH_GPU_MEMORY = 0
except ImportError:
    pass

try:
    import cupy as cp
    HAS_CUPY = True
except ImportError:
    pass


class ComputeBackend(Enum):
    """Backend de cómputo."""
    CPU = "cpu"
    CUDA_TORCH = "cuda_torch"
    CUDA_CUPY = "cuda_cupy"
    AUTO = "auto"


@dataclass
class GPUInfo:
    """Información de GPU."""
    available: bool
    name: str
    memory_gb: float
    backend: str
    cuda_version: str


@dataclass 
class BenchmarkResult:
    """Resultado de benchmark."""
    operation: str
    size: str
    cpu_ms: float
    gpu_ms: Optional[float]
    speedup: float


class GPUCompute:
    """
    Sistema de cómputo GPU para IA-Personal.
    Detecta hardware y proporciona operaciones aceleradas.
    """
    
    def __init__(self, backend: ComputeBackend = ComputeBackend.AUTO):
        self.requested_backend = backend
        self.active_backend = ComputeBackend.CPU
        self.gpu_info = self._detect_gpu()
        
        # Inicializar backend
        self._init_backend(backend)
        
        # Estadísticas
        self.stats = {
            "cpu_ops": 0,
            "gpu_ops": 0,
            "cpu_time_ms": 0,
            "gpu_time_ms": 0,
            "total_speedup": 0,
        }
        
        self._print_info()
    
    def _detect_gpu(self) -> GPUInfo:
        """Detecta GPU disponible."""
        if HAS_TORCH and TORCH_CUDA:
            return GPUInfo(
                available=True,
                name=TORCH_GPU_NAME,
                memory_gb=TORCH_GPU_MEMORY,
                backend="torch",
                cuda_version=torch.version.cuda or "N/A"
            )
        elif HAS_CUPY:
            try:
                device = cp.cuda.Device(0)
                mem = device.mem_info[1] / (1024**3)
                return GPUInfo(
                    available=True,
                    name=f"CUDA Device {device.id}",
                    memory_gb=mem,
                    backend="cupy",
                    cuda_version=str(cp.cuda.runtime.runtimeGetVersion())
                )
            except:
                pass
        
        return GPUInfo(
            available=False,
            name="No GPU",
            memory_gb=0,
            backend="none",
            cuda_version="N/A"
        )
    
    def _init_backend(self, backend: ComputeBackend):
        """Inicializa el backend de cómputo."""
        if backend == ComputeBackend.AUTO:
            if HAS_TORCH and TORCH_CUDA:
                self.active_backend = ComputeBackend.CUDA_TORCH
            elif HAS_CUPY:
                self.active_backend = ComputeBackend.CUDA_CUPY
            else:
                self.active_backend = ComputeBackend.CPU
        else:
            self.active_backend = backend
        
        # Warmup GPU
        if self.active_backend != ComputeBackend.CPU:
            self._warmup()
    
    def _warmup(self):
        """Calienta la GPU."""
        if self.active_backend == ComputeBackend.CUDA_TORCH:
            a = torch.randn(512, 512, device=TORCH_DEVICE)
            for _ in range(5):
                _ = torch.matmul(a, a)
            torch.cuda.synchronize()
        elif self.active_backend == ComputeBackend.CUDA_CUPY:
            a = cp.random.randn(512, 512, dtype=cp.float32)
            for _ in range(5):
                _ = cp.matmul(a, a)
            cp.cuda.Stream.null.synchronize()
    
    def _print_info(self):
        """Imprime información del sistema."""
        print("\n" + "=" * 60)
        print("   🎮 GPU Compute para IA-Personal")
        print("=" * 60)
        
        if self.gpu_info.available:
            print(f"\n✅ GPU Detectada:")
            print(f"   Modelo: {self.gpu_info.name}")
            print(f"   VRAM: {self.gpu_info.memory_gb:.1f} GB")
            print(f"   CUDA: {self.gpu_info.cuda_version}")
            print(f"   Backend: {self.active_backend.value}")
        else:
            print(f"\n⚠️ GPU no disponible")
            print(f"   Backend: CPU (NumPy)")
            print(f"\n💡 Para habilitar GPU:")
            print(f"   pip install torch --index-url https://download.pytorch.org/whl/cu121")
        
        print("=" * 60)
    
    def is_gpu_available(self) -> bool:
        """Verifica si GPU está disponible."""
        return self.gpu_info.available and self.active_backend != ComputeBackend.CPU
    
    # =========================================================================
    # OPERACIONES BÁSICAS
    # =========================================================================
    
    def to_device(self, x: np.ndarray) -> Any:
        """Transfiere array a GPU."""
        if self.active_backend == ComputeBackend.CUDA_TORCH:
            return torch.from_numpy(x.astype(np.float32)).to(TORCH_DEVICE)
        elif self.active_backend == ComputeBackend.CUDA_CUPY:
            return cp.asarray(x, dtype=cp.float32)
        return x.astype(np.float32)
    
    def to_numpy(self, x: Any) -> np.ndarray:
        """Transfiere de GPU a NumPy."""
        if self.active_backend == ComputeBackend.CUDA_TORCH:
            return x.cpu().numpy()
        elif self.active_backend == ComputeBackend.CUDA_CUPY:
            return cp.asnumpy(x)
        return np.asarray(x)
    
    def sync(self):
        """Sincroniza GPU."""
        if self.active_backend == ComputeBackend.CUDA_TORCH:
            torch.cuda.synchronize()
        elif self.active_backend == ComputeBackend.CUDA_CUPY:
            cp.cuda.Stream.null.synchronize()
    
    # =========================================================================
    # OPERACIONES MATEMÁTICAS ACELERADAS
    # =========================================================================
    
    def matmul(self, a: np.ndarray, b: np.ndarray, use_gpu: bool = True) -> np.ndarray:
        """Multiplicación de matrices."""
        if use_gpu and self.is_gpu_available():
            return self._matmul_gpu(a, b)
        return self._matmul_cpu(a, b)
    
    def _matmul_cpu(self, a: np.ndarray, b: np.ndarray) -> np.ndarray:
        """MatMul en CPU."""
        start = time.perf_counter()
        result = np.matmul(a.astype(np.float32), b.astype(np.float32))
        elapsed = (time.perf_counter() - start) * 1000
        self.stats["cpu_ops"] += 1
        self.stats["cpu_time_ms"] += elapsed
        return result
    
    def _matmul_gpu(self, a: np.ndarray, b: np.ndarray) -> np.ndarray:
        """MatMul en GPU."""
        start = time.perf_counter()
        
        if self.active_backend == ComputeBackend.CUDA_TORCH:
            a_gpu = torch.from_numpy(a.astype(np.float32)).to(TORCH_DEVICE)
            b_gpu = torch.from_numpy(b.astype(np.float32)).to(TORCH_DEVICE)
            c_gpu = torch.matmul(a_gpu, b_gpu)
            torch.cuda.synchronize()
            result = c_gpu.cpu().numpy()
        elif self.active_backend == ComputeBackend.CUDA_CUPY:
            a_gpu = cp.asarray(a, dtype=cp.float32)
            b_gpu = cp.asarray(b, dtype=cp.float32)
            c_gpu = cp.matmul(a_gpu, b_gpu)
            cp.cuda.Stream.null.synchronize()
            result = cp.asnumpy(c_gpu)
        else:
            return self._matmul_cpu(a, b)
        
        elapsed = (time.perf_counter() - start) * 1000
        self.stats["gpu_ops"] += 1
        self.stats["gpu_time_ms"] += elapsed
        return result
    
    def softmax(self, x: np.ndarray, axis: int = -1, use_gpu: bool = True) -> np.ndarray:
        """Softmax acelerado."""
        if use_gpu and self.is_gpu_available():
            return self._softmax_gpu(x, axis)
        return self._softmax_cpu(x, axis)
    
    def _softmax_cpu(self, x: np.ndarray, axis: int = -1) -> np.ndarray:
        """Softmax en CPU."""
        x = x.astype(np.float32)
        x_max = np.max(x, axis=axis, keepdims=True)
        exp_x = np.exp(x - x_max)
        return exp_x / (np.sum(exp_x, axis=axis, keepdims=True) + 1e-8)
    
    def _softmax_gpu(self, x: np.ndarray, axis: int = -1) -> np.ndarray:
        """Softmax en GPU."""
        if self.active_backend == ComputeBackend.CUDA_TORCH:
            x_gpu = torch.from_numpy(x.astype(np.float32)).to(TORCH_DEVICE)
            y_gpu = torch.softmax(x_gpu, dim=axis)
            torch.cuda.synchronize()
            return y_gpu.cpu().numpy()
        elif self.active_backend == ComputeBackend.CUDA_CUPY:
            x_gpu = cp.asarray(x, dtype=cp.float32)
            x_max = cp.max(x_gpu, axis=axis, keepdims=True)
            exp_x = cp.exp(x_gpu - x_max)
            y_gpu = exp_x / (cp.sum(exp_x, axis=axis, keepdims=True) + 1e-8)
            cp.cuda.Stream.null.synchronize()
            return cp.asnumpy(y_gpu)
        return self._softmax_cpu(x, axis)
    
    def gelu(self, x: np.ndarray, use_gpu: bool = True) -> np.ndarray:
        """GELU acelerado."""
        if use_gpu and self.is_gpu_available():
            return self._gelu_gpu(x)
        return self._gelu_cpu(x)
    
    def _gelu_cpu(self, x: np.ndarray) -> np.ndarray:
        """GELU en CPU."""
        x = x.astype(np.float32)
        return x * 0.5 * (1 + np.tanh(np.sqrt(2 / np.pi) * (x + 0.044715 * x**3)))
    
    def _gelu_gpu(self, x: np.ndarray) -> np.ndarray:
        """GELU en GPU."""
        if self.active_backend == ComputeBackend.CUDA_TORCH:
            x_gpu = torch.from_numpy(x.astype(np.float32)).to(TORCH_DEVICE)
            y_gpu = torch.nn.functional.gelu(x_gpu)
            torch.cuda.synchronize()
            return y_gpu.cpu().numpy()
        return self._gelu_cpu(x)
    
    def layer_norm(self, x: np.ndarray, eps: float = 1e-5, use_gpu: bool = True) -> np.ndarray:
        """Layer normalization acelerada."""
        if use_gpu and self.is_gpu_available():
            return self._layer_norm_gpu(x, eps)
        return self._layer_norm_cpu(x, eps)
    
    def _layer_norm_cpu(self, x: np.ndarray, eps: float = 1e-5) -> np.ndarray:
        """Layer norm en CPU."""
        x = x.astype(np.float32)
        mean = np.mean(x, axis=-1, keepdims=True)
        var = np.var(x, axis=-1, keepdims=True)
        return (x - mean) / np.sqrt(var + eps)
    
    def _layer_norm_gpu(self, x: np.ndarray, eps: float = 1e-5) -> np.ndarray:
        """Layer norm en GPU."""
        if self.active_backend == ComputeBackend.CUDA_TORCH:
            x_gpu = torch.from_numpy(x.astype(np.float32)).to(TORCH_DEVICE)
            y_gpu = torch.nn.functional.layer_norm(x_gpu, x_gpu.shape[-1:], eps=eps)
            torch.cuda.synchronize()
            return y_gpu.cpu().numpy()
        return self._layer_norm_cpu(x, eps)
    
    # =========================================================================
    # ATTENTION
    # =========================================================================
    
    def attention(self, q: np.ndarray, k: np.ndarray, v: np.ndarray,
                  mask: Optional[np.ndarray] = None, use_gpu: bool = True) -> np.ndarray:
        """Scaled dot-product attention."""
        if use_gpu and self.is_gpu_available():
            return self._attention_gpu(q, k, v, mask)
        return self._attention_cpu(q, k, v, mask)
    
    def _attention_cpu(self, q: np.ndarray, k: np.ndarray, v: np.ndarray,
                       mask: Optional[np.ndarray] = None) -> np.ndarray:
        """Attention en CPU."""
        q, k, v = q.astype(np.float32), k.astype(np.float32), v.astype(np.float32)
        d_k = q.shape[-1]
        scores = np.matmul(q, k.swapaxes(-2, -1)) / np.sqrt(d_k)
        
        if mask is not None:
            scores = scores + mask
        
        weights = self._softmax_cpu(scores, axis=-1)
        return np.matmul(weights, v)
    
    def _attention_gpu(self, q: np.ndarray, k: np.ndarray, v: np.ndarray,
                       mask: Optional[np.ndarray] = None) -> np.ndarray:
        """Attention en GPU."""
        if self.active_backend == ComputeBackend.CUDA_TORCH:
            q_gpu = torch.from_numpy(q.astype(np.float32)).to(TORCH_DEVICE)
            k_gpu = torch.from_numpy(k.astype(np.float32)).to(TORCH_DEVICE)
            v_gpu = torch.from_numpy(v.astype(np.float32)).to(TORCH_DEVICE)
            
            d_k = q_gpu.shape[-1]
            scores = torch.matmul(q_gpu, k_gpu.transpose(-2, -1)) / (d_k ** 0.5)
            
            if mask is not None:
                mask_gpu = torch.from_numpy(mask.astype(np.float32)).to(TORCH_DEVICE)
                scores = scores + mask_gpu
            
            weights = torch.softmax(scores, dim=-1)
            output = torch.matmul(weights, v_gpu)
            torch.cuda.synchronize()
            return output.cpu().numpy()
        
        return self._attention_cpu(q, k, v, mask)
    
    # =========================================================================
    # TRANSFORMER FORWARD PASS
    # =========================================================================
    
    def transformer_layer(self, x: np.ndarray, 
                          w_q: np.ndarray, w_k: np.ndarray, w_v: np.ndarray, w_o: np.ndarray,
                          w1: np.ndarray, w2: np.ndarray,
                          use_gpu: bool = True) -> np.ndarray:
        """Una capa de transformer completa en GPU."""
        if use_gpu and self.is_gpu_available():
            return self._transformer_layer_gpu(x, w_q, w_k, w_v, w_o, w1, w2)
        return self._transformer_layer_cpu(x, w_q, w_k, w_v, w_o, w1, w2)
    
    def _transformer_layer_cpu(self, x: np.ndarray,
                               w_q: np.ndarray, w_k: np.ndarray, w_v: np.ndarray, w_o: np.ndarray,
                               w1: np.ndarray, w2: np.ndarray) -> np.ndarray:
        """Transformer layer en CPU."""
        # Attention
        q = self._matmul_cpu(x, w_q)
        k = self._matmul_cpu(x, w_k)
        v = self._matmul_cpu(x, w_v)
        
        attn = self._attention_cpu(q, k, v)
        attn_out = self._matmul_cpu(attn, w_o)
        x = x + attn_out
        
        # FFN
        hidden = self._gelu_cpu(self._matmul_cpu(x, w1))
        ffn_out = self._matmul_cpu(hidden, w2)
        x = x + ffn_out
        
        return x
    
    def _transformer_layer_gpu(self, x: np.ndarray,
                               w_q: np.ndarray, w_k: np.ndarray, w_v: np.ndarray, w_o: np.ndarray,
                               w1: np.ndarray, w2: np.ndarray) -> np.ndarray:
        """Transformer layer en GPU (todo en un solo kernel)."""
        if self.active_backend == ComputeBackend.CUDA_TORCH:
            # Transferir todo a GPU de una vez
            x_gpu = torch.from_numpy(x.astype(np.float32)).to(TORCH_DEVICE)
            w_q_gpu = torch.from_numpy(w_q.astype(np.float32)).to(TORCH_DEVICE)
            w_k_gpu = torch.from_numpy(w_k.astype(np.float32)).to(TORCH_DEVICE)
            w_v_gpu = torch.from_numpy(w_v.astype(np.float32)).to(TORCH_DEVICE)
            w_o_gpu = torch.from_numpy(w_o.astype(np.float32)).to(TORCH_DEVICE)
            w1_gpu = torch.from_numpy(w1.astype(np.float32)).to(TORCH_DEVICE)
            w2_gpu = torch.from_numpy(w2.astype(np.float32)).to(TORCH_DEVICE)
            
            # Attention
            q = torch.matmul(x_gpu, w_q_gpu)
            k = torch.matmul(x_gpu, w_k_gpu)
            v = torch.matmul(x_gpu, w_v_gpu)
            
            d_k = q.shape[-1]
            scores = torch.matmul(q, k.transpose(-2, -1)) / (d_k ** 0.5)
            
            # Máscara causal
            seq_len = x.shape[0]
            mask = torch.triu(torch.ones(seq_len, seq_len, device=TORCH_DEVICE) * -1e9, diagonal=1)
            scores = scores + mask
            
            weights = torch.softmax(scores, dim=-1)
            attn = torch.matmul(weights, v)
            attn_out = torch.matmul(attn, w_o_gpu)
            x_gpu = x_gpu + attn_out
            
            # FFN con GELU
            hidden = torch.nn.functional.gelu(torch.matmul(x_gpu, w1_gpu))
            ffn_out = torch.matmul(hidden, w2_gpu)
            x_gpu = x_gpu + ffn_out
            
            torch.cuda.synchronize()
            return x_gpu.cpu().numpy()
        
        return self._transformer_layer_cpu(x, w_q, w_k, w_v, w_o, w1, w2)
    
    # =========================================================================
    # BENCHMARK
    # =========================================================================
    
    def benchmark(self, sizes: List[int] = None) -> List[BenchmarkResult]:
        """Ejecuta benchmark CPU vs GPU."""
        if sizes is None:
            sizes = [256, 512, 1024, 2048]
        
        results = []
        
        print("\n" + "=" * 70)
        print("   ⚡ BENCHMARK: CPU vs GPU")
        print("=" * 70)
        
        # MatMul Benchmark
        print("\n📊 Multiplicación de Matrices (MatMul)")
        print("-" * 70)
        print(f"{'Tamaño':<15} {'CPU (ms)':<15} {'GPU (ms)':<15} {'Speedup':<15}")
        print("-" * 70)
        
        for size in sizes:
            a = np.random.randn(size, size).astype(np.float32)
            b = np.random.randn(size, size).astype(np.float32)
            
            # CPU
            start = time.perf_counter()
            for _ in range(3):
                _ = self._matmul_cpu(a, b)
            cpu_ms = ((time.perf_counter() - start) / 3) * 1000
            
            # GPU
            if self.is_gpu_available():
                # Warmup
                _ = self._matmul_gpu(a, b)
                
                start = time.perf_counter()
                for _ in range(3):
                    _ = self._matmul_gpu(a, b)
                gpu_ms = ((time.perf_counter() - start) / 3) * 1000
                speedup = cpu_ms / gpu_ms
            else:
                gpu_ms = None
                speedup = 1.0
            
            gpu_str = f"{gpu_ms:.2f}" if gpu_ms else "N/A"
            print(f"{size}x{size:<9} {cpu_ms:<15.2f} {gpu_str:<15} {speedup:.1f}x")
            
            results.append(BenchmarkResult(
                operation="matmul",
                size=f"{size}x{size}",
                cpu_ms=cpu_ms,
                gpu_ms=gpu_ms,
                speedup=speedup
            ))
        
        # Attention Benchmark
        print("\n📊 Attention (seq_len x dim)")
        print("-" * 70)
        print(f"{'Tamaño':<15} {'CPU (ms)':<15} {'GPU (ms)':<15} {'Speedup':<15}")
        print("-" * 70)
        
        attention_sizes = [(128, 64), (256, 128), (512, 256)]
        
        for seq_len, dim in attention_sizes:
            q = np.random.randn(seq_len, dim).astype(np.float32)
            k = np.random.randn(seq_len, dim).astype(np.float32)
            v = np.random.randn(seq_len, dim).astype(np.float32)
            
            # CPU
            start = time.perf_counter()
            for _ in range(3):
                _ = self._attention_cpu(q, k, v)
            cpu_ms = ((time.perf_counter() - start) / 3) * 1000
            
            # GPU
            if self.is_gpu_available():
                _ = self._attention_gpu(q, k, v)
                
                start = time.perf_counter()
                for _ in range(3):
                    _ = self._attention_gpu(q, k, v)
                gpu_ms = ((time.perf_counter() - start) / 3) * 1000
                speedup = cpu_ms / gpu_ms
            else:
                gpu_ms = None
                speedup = 1.0
            
            gpu_str = f"{gpu_ms:.2f}" if gpu_ms else "N/A"
            print(f"{seq_len}x{dim:<9} {cpu_ms:<15.2f} {gpu_str:<15} {speedup:.1f}x")
            
            results.append(BenchmarkResult(
                operation="attention",
                size=f"{seq_len}x{dim}",
                cpu_ms=cpu_ms,
                gpu_ms=gpu_ms,
                speedup=speedup
            ))
        
        print("-" * 70)
        
        # Resumen
        if self.is_gpu_available():
            avg_speedup = np.mean([r.speedup for r in results if r.gpu_ms])
            print(f"\n🚀 Speedup promedio: {avg_speedup:.1f}x")
        
        return results
    
    def get_stats(self) -> Dict:
        """Obtiene estadísticas de uso."""
        total_ops = self.stats["cpu_ops"] + self.stats["gpu_ops"]
        gpu_ratio = self.stats["gpu_ops"] / total_ops if total_ops > 0 else 0
        
        return {
            "gpu_available": self.is_gpu_available(),
            "gpu_name": self.gpu_info.name,
            "gpu_memory_gb": self.gpu_info.memory_gb,
            "backend": self.active_backend.value,
            "cpu_ops": self.stats["cpu_ops"],
            "gpu_ops": self.stats["gpu_ops"],
            "gpu_usage_ratio": gpu_ratio,
            "cpu_time_ms": self.stats["cpu_time_ms"],
            "gpu_time_ms": self.stats["gpu_time_ms"],
        }


# =============================================================================
# TRANSFORMER GPU
# =============================================================================

class GPUTransformer:
    """Transformer optimizado para GPU."""
    
    def __init__(self, vocab_size: int, embed_dim: int, num_heads: int,
                 hidden_dim: int, num_layers: int, gpu_compute: GPUCompute):
        self.vocab_size = vocab_size
        self.embed_dim = embed_dim
        self.num_heads = num_heads
        self.hidden_dim = hidden_dim
        self.num_layers = num_layers
        self.gpu = gpu_compute
        
        # Inicializar pesos
        self.embeddings = np.random.randn(vocab_size, embed_dim).astype(np.float32) * 0.02
        
        self.layers = []
        for _ in range(num_layers):
            layer = {
                "W_q": np.random.randn(embed_dim, embed_dim).astype(np.float32) * 0.02,
                "W_k": np.random.randn(embed_dim, embed_dim).astype(np.float32) * 0.02,
                "W_v": np.random.randn(embed_dim, embed_dim).astype(np.float32) * 0.02,
                "W_o": np.random.randn(embed_dim, embed_dim).astype(np.float32) * 0.02,
                "W1": np.random.randn(embed_dim, hidden_dim).astype(np.float32) * 0.02,
                "W2": np.random.randn(hidden_dim, embed_dim).astype(np.float32) * 0.02,
            }
            self.layers.append(layer)
        
        self.output_proj = np.random.randn(embed_dim, vocab_size).astype(np.float32) * 0.02
        
        # Calcular RAM
        self._calc_memory()
    
    def _calc_memory(self):
        """Calcula memoria usada."""
        embed_mem = self.vocab_size * self.embed_dim * 4
        layer_mem = self.num_layers * (
            4 * self.embed_dim * self.embed_dim +
            2 * self.embed_dim * self.hidden_dim
        ) * 4
        output_mem = self.embed_dim * self.vocab_size * 4
        
        self.memory_mb = (embed_mem + layer_mem + output_mem) / (1024 * 1024)
    
    def forward(self, token_ids: List[int], use_gpu: bool = True) -> np.ndarray:
        """Forward pass."""
        # Embeddings
        safe_ids = [min(max(0, t), self.vocab_size - 1) for t in token_ids]
        x = self.embeddings[safe_ids]
        
        # Capas transformer
        for layer in self.layers:
            x = self.gpu.transformer_layer(
                x,
                layer["W_q"], layer["W_k"], layer["W_v"], layer["W_o"],
                layer["W1"], layer["W2"],
                use_gpu=use_gpu
            )
        
        # Logits
        logits = self.gpu.matmul(x[-1:], self.output_proj, use_gpu=use_gpu)
        return logits[0]
    
    def generate_token(self, token_ids: List[int], temperature: float = 0.7,
                       top_k: int = 50, use_gpu: bool = True) -> int:
        """Genera el siguiente token."""
        logits = self.forward(token_ids, use_gpu=use_gpu)
        
        # Temperatura
        logits = logits / max(temperature, 0.1)
        
        # Top-k
        if top_k > 0:
            indices = np.argsort(logits)[-top_k:]
            mask = np.ones_like(logits) * -1e9
            mask[indices] = 0
            logits = logits + mask
        
        # Softmax
        probs = self.gpu.softmax(logits, use_gpu=use_gpu)
        
        # Muestrear
        return int(np.random.choice(len(probs), p=probs.astype(np.float64)))


# =============================================================================
# IA PERSONAL CON GPU
# =============================================================================

# Importar después para evitar circular imports
sys.path.insert(0, str(Path(__file__).parent.parent))

from IA_Personal.core.ia_personal import IAPersonal, IAPersonalConfig
from IA_Personal.core.tokenizer import SmartTokenizer


class IAPersonalGPU(IAPersonal):
    """
    IA Personal con aceleración GPU completa.
    Usa GPU para todas las operaciones del transformer.
    """
    
    def __init__(self, config: IAPersonalConfig = None):
        # Inicializar GPU primero
        self.gpu_compute = GPUCompute(ComputeBackend.AUTO)
        
        # Llamar constructor padre
        super().__init__(config)
        
        # Reemplazar modelo con versión GPU
        self.model = GPUTransformer(
            vocab_size=len(self.tokenizer),
            embed_dim=self.config.embed_dim,
            num_heads=self.config.num_heads,
            hidden_dim=self.config.hidden_dim,
            num_layers=self.config.num_layers,
            gpu_compute=self.gpu_compute
        )
        
        self.use_gpu = self.gpu_compute.is_gpu_available()
        
        if self.use_gpu:
            print(f"🚀 GPU Activa: {self.gpu_compute.gpu_info.name}")
        else:
            print(f"⚠️ GPU no disponible, usando CPU")
    
    def _get_smart_response(self, message: str) -> str:
        """Genera respuesta usando GPU si está disponible."""
        # Usar la lógica del padre pero con GPU para operaciones pesadas
        return super()._get_smart_response(message)
    
    def benchmark_gpu(self) -> Dict:
        """Ejecuta benchmark de GPU."""
        print("\n🎮 Benchmark GPU para IA-Personal:")
        results = self.gpu_compute.benchmark([256, 512, 1024])
        return {
            "results": results,
            "gpu_stats": self.gpu_compute.get_stats()
        }
    
    def get_gpu_stats(self) -> Dict:
        """Obtiene estadísticas de GPU."""
        return self.gpu_compute.get_stats()
    
    def toggle_gpu(self, enabled: bool = None):
        """Activa/desactiva GPU."""
        if enabled is None:
            self.use_gpu = not self.use_gpu
        else:
            self.use_gpu = enabled and self.gpu_compute.is_gpu_available()
        
        status = "activada" if self.use_gpu else "desactivada"
        print(f"GPU {status}")


# =============================================================================
# DEMO
# =============================================================================

def demo():
    """Demo de GPU Compute."""
    print("\n" + "=" * 70)
    print("   🎮 GPU Compute Demo - IA-Personal")
    print("   Author: Eddi Andreé Salazar Matos")
    print("=" * 70)
    
    # Crear GPU Compute
    gpu = GPUCompute()
    
    # Benchmark
    gpu.benchmark([256, 512, 1024])
    
    # Estadísticas
    print("\n📊 Estadísticas:")
    stats = gpu.get_stats()
    for key, value in stats.items():
        print(f"   {key}: {value}")
    
    print("\n" + "=" * 70)
    print("   Demo completada")
    print("=" * 70)


if __name__ == "__main__":
    demo()
