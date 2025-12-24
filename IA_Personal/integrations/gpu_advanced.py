"""
GPU Advanced - Optimizaciones Avanzadas para IA-Personal
=========================================================
Author: Eddi Andreé Salazar Matos
Email: eddi.salazar.dev@gmail.com
Made with ❤️ in Peru 🇵🇪

Optimizaciones avanzadas para máximo rendimiento en GPU:
- Flash Attention (memoria eficiente)
- Mixed Precision (FP16/BF16)
- Persistent GPU Weights (sin transferencias)
- Batch Processing
- KV Cache para generación
- Métricas en tiempo real
- Auto-tuning por GPU

Optimizado para: RTX 3060 12GB (Ampere)
"""

import os
import sys
import time
import threading
from pathlib import Path
from typing import Dict, List, Optional, Tuple, Any
from dataclasses import dataclass, field
from enum import Enum
from contextlib import contextmanager

import numpy as np

# PyTorch
try:
    import torch
    import torch.nn.functional as F
    HAS_TORCH = True
    TORCH_CUDA = torch.cuda.is_available()
    if TORCH_CUDA:
        DEVICE = torch.device("cuda")
        GPU_NAME = torch.cuda.get_device_name(0)
        GPU_MEMORY = torch.cuda.get_device_properties(0).total_memory / (1024**3)
        GPU_SM_COUNT = torch.cuda.get_device_properties(0).multi_processor_count
        # Detectar capacidades
        GPU_CAPABILITY = torch.cuda.get_device_capability(0)
        HAS_TENSOR_CORES = GPU_CAPABILITY[0] >= 7  # Volta+
        HAS_BF16 = GPU_CAPABILITY[0] >= 8  # Ampere+
    else:
        DEVICE = torch.device("cpu")
        GPU_NAME = None
        GPU_MEMORY = 0
        GPU_SM_COUNT = 0
        GPU_CAPABILITY = (0, 0)
        HAS_TENSOR_CORES = False
        HAS_BF16 = False
except ImportError:
    HAS_TORCH = False
    TORCH_CUDA = False


class PrecisionMode(Enum):
    """Modos de precisión."""
    FP32 = "fp32"
    FP16 = "fp16"
    BF16 = "bf16"
    AUTO = "auto"


@dataclass
class GPUMetrics:
    """Métricas de rendimiento GPU."""
    operation: str
    time_ms: float
    memory_mb: float
    gflops: float
    throughput: float  # tokens/s o elementos/s
    
    def __str__(self):
        return f"{self.operation}: {self.time_ms:.2f}ms, {self.gflops:.1f} GFLOPS"


@dataclass
class GPUConfig:
    """Configuración avanzada de GPU."""
    # Precisión
    precision: PrecisionMode = PrecisionMode.AUTO
    use_tensor_cores: bool = True
    
    # Memoria
    persistent_weights: bool = True
    max_memory_fraction: float = 0.9
    
    # Optimizaciones
    use_flash_attention: bool = True
    use_fused_ops: bool = True
    use_channels_last: bool = False
    
    # Batch
    max_batch_size: int = 32
    
    # KV Cache
    use_kv_cache: bool = True
    max_cache_length: int = 2048
    
    # Auto-tuning
    auto_tune: bool = True
    benchmark_cudnn: bool = True


class PerformanceMonitor:
    """Monitor de rendimiento en tiempo real."""
    
    def __init__(self):
        self.metrics: List[GPUMetrics] = []
        self.start_time = time.time()
        self.lock = threading.Lock()
        
        # Estadísticas acumuladas
        self.total_ops = 0
        self.total_time_ms = 0
        self.total_gflops = 0
        self.peak_memory_mb = 0
    
    def record(self, metric: GPUMetrics):
        """Registra una métrica."""
        with self.lock:
            self.metrics.append(metric)
            self.total_ops += 1
            self.total_time_ms += metric.time_ms
            self.total_gflops += metric.gflops * (metric.time_ms / 1000)
            self.peak_memory_mb = max(self.peak_memory_mb, metric.memory_mb)
    
    def get_summary(self) -> Dict:
        """Obtiene resumen de rendimiento."""
        if not self.metrics:
            return {}
        
        elapsed = time.time() - self.start_time
        avg_time = self.total_time_ms / self.total_ops if self.total_ops > 0 else 0
        avg_gflops = self.total_gflops / elapsed if elapsed > 0 else 0
        
        return {
            "total_operations": self.total_ops,
            "total_time_ms": self.total_time_ms,
            "avg_time_ms": avg_time,
            "avg_gflops": avg_gflops,
            "peak_memory_mb": self.peak_memory_mb,
            "ops_per_second": self.total_ops / elapsed if elapsed > 0 else 0,
        }
    
    def reset(self):
        """Reinicia métricas."""
        with self.lock:
            self.metrics.clear()
            self.start_time = time.time()
            self.total_ops = 0
            self.total_time_ms = 0
            self.total_gflops = 0


class GPUAdvanced:
    """
    Sistema GPU avanzado con todas las optimizaciones.
    Diseñado para exprimir al máximo tu RTX 3060.
    """
    
    def __init__(self, config: GPUConfig = None):
        if not HAS_TORCH or not TORCH_CUDA:
            raise RuntimeError("CUDA no disponible. Instala PyTorch con CUDA.")
        
        self.config = config or GPUConfig()
        self.monitor = PerformanceMonitor()
        
        # Configurar precisión
        self._setup_precision()
        
        # Configurar memoria
        self._setup_memory()
        
        # Configurar optimizaciones
        self._setup_optimizations()
        
        # Cache de pesos en GPU
        self.weight_cache: Dict[str, torch.Tensor] = {}
        
        # KV Cache
        self.kv_cache: Dict[int, Tuple[torch.Tensor, torch.Tensor]] = {}
        
        self._print_config()
    
    def _setup_precision(self):
        """Configura precisión según GPU."""
        if self.config.precision == PrecisionMode.AUTO:
            if HAS_BF16:
                self.dtype = torch.bfloat16
                self.precision_name = "BF16"
            elif HAS_TENSOR_CORES:
                self.dtype = torch.float16
                self.precision_name = "FP16"
            else:
                self.dtype = torch.float32
                self.precision_name = "FP32"
        elif self.config.precision == PrecisionMode.BF16:
            self.dtype = torch.bfloat16
            self.precision_name = "BF16"
        elif self.config.precision == PrecisionMode.FP16:
            self.dtype = torch.float16
            self.precision_name = "FP16"
        else:
            self.dtype = torch.float32
            self.precision_name = "FP32"
    
    def _setup_memory(self):
        """Configura gestión de memoria."""
        if self.config.max_memory_fraction < 1.0:
            torch.cuda.set_per_process_memory_fraction(
                self.config.max_memory_fraction, 0
            )
        
        # Habilitar memory efficient attention si está disponible
        if hasattr(torch.backends.cuda, 'enable_flash_sdp'):
            torch.backends.cuda.enable_flash_sdp(self.config.use_flash_attention)
    
    def _setup_optimizations(self):
        """Configura optimizaciones."""
        if self.config.benchmark_cudnn:
            torch.backends.cudnn.benchmark = True
        
        torch.backends.cudnn.enabled = True
        
        # Habilitar TF32 en Ampere+
        if GPU_CAPABILITY[0] >= 8:
            torch.backends.cuda.matmul.allow_tf32 = True
            torch.backends.cudnn.allow_tf32 = True
    
    def _print_config(self):
        """Imprime configuración."""
        print("\n" + "=" * 70)
        print("   🚀 GPU Advanced - Optimizaciones Máximas")
        print("=" * 70)
        print(f"\n🎮 GPU: {GPU_NAME}")
        print(f"   VRAM: {GPU_MEMORY:.1f} GB")
        print(f"   SMs: {GPU_SM_COUNT}")
        print(f"   Compute: {GPU_CAPABILITY[0]}.{GPU_CAPABILITY[1]}")
        print(f"\n⚡ Optimizaciones:")
        print(f"   Precisión: {self.precision_name}")
        print(f"   Tensor Cores: {'✅' if HAS_TENSOR_CORES else '❌'}")
        print(f"   Flash Attention: {'✅' if self.config.use_flash_attention else '❌'}")
        print(f"   Fused Ops: {'✅' if self.config.use_fused_ops else '❌'}")
        print(f"   KV Cache: {'✅' if self.config.use_kv_cache else '❌'}")
        print(f"   Persistent Weights: {'✅' if self.config.persistent_weights else '❌'}")
        print("=" * 70)
    
    @contextmanager
    def timer(self, operation: str, elements: int = 0, flops: int = 0):
        """Context manager para medir tiempo."""
        torch.cuda.synchronize()
        start = time.perf_counter()
        mem_before = torch.cuda.memory_allocated() / (1024**2)
        
        yield
        
        torch.cuda.synchronize()
        elapsed = (time.perf_counter() - start) * 1000
        mem_after = torch.cuda.memory_allocated() / (1024**2)
        
        gflops = (flops / (elapsed / 1000) / 1e9) if flops > 0 and elapsed > 0 else 0
        throughput = (elements / (elapsed / 1000)) if elements > 0 and elapsed > 0 else 0
        
        metric = GPUMetrics(
            operation=operation,
            time_ms=elapsed,
            memory_mb=mem_after - mem_before,
            gflops=gflops,
            throughput=throughput
        )
        self.monitor.record(metric)
    
    # =========================================================================
    # CACHE DE PESOS PERSISTENTE
    # =========================================================================
    
    def cache_weights(self, name: str, weights: np.ndarray) -> torch.Tensor:
        """Cachea pesos en GPU."""
        if name not in self.weight_cache:
            self.weight_cache[name] = torch.from_numpy(
                weights.astype(np.float32)
            ).to(DEVICE, dtype=self.dtype)
        return self.weight_cache[name]
    
    def get_cached_weight(self, name: str) -> Optional[torch.Tensor]:
        """Obtiene peso cacheado."""
        return self.weight_cache.get(name)
    
    def clear_weight_cache(self):
        """Limpia cache de pesos."""
        self.weight_cache.clear()
        torch.cuda.empty_cache()
    
    # =========================================================================
    # OPERACIONES OPTIMIZADAS
    # =========================================================================
    
    def matmul(self, a: np.ndarray, b: np.ndarray, 
               a_cached: str = None, b_cached: str = None) -> np.ndarray:
        """Multiplicación de matrices optimizada."""
        m, k = a.shape
        k2, n = b.shape
        flops = 2 * m * k * n
        
        with self.timer("matmul", elements=m*n, flops=flops):
            # Usar cache si está disponible
            if a_cached and a_cached in self.weight_cache:
                a_gpu = self.weight_cache[a_cached]
            else:
                a_gpu = torch.from_numpy(a.astype(np.float32)).to(DEVICE, dtype=self.dtype)
            
            if b_cached and b_cached in self.weight_cache:
                b_gpu = self.weight_cache[b_cached]
            else:
                b_gpu = torch.from_numpy(b.astype(np.float32)).to(DEVICE, dtype=self.dtype)
            
            # MatMul con Tensor Cores
            c_gpu = torch.matmul(a_gpu, b_gpu)
            
            return c_gpu.float().cpu().numpy()
    
    def batch_matmul(self, a: np.ndarray, b: np.ndarray) -> np.ndarray:
        """Batch matrix multiplication."""
        batch, m, k = a.shape
        batch2, k2, n = b.shape
        flops = 2 * batch * m * k * n
        
        with self.timer("batch_matmul", elements=batch*m*n, flops=flops):
            a_gpu = torch.from_numpy(a.astype(np.float32)).to(DEVICE, dtype=self.dtype)
            b_gpu = torch.from_numpy(b.astype(np.float32)).to(DEVICE, dtype=self.dtype)
            c_gpu = torch.bmm(a_gpu, b_gpu)
            return c_gpu.float().cpu().numpy()
    
    def flash_attention(self, q: np.ndarray, k: np.ndarray, v: np.ndarray,
                        causal: bool = True) -> np.ndarray:
        """
        Flash Attention - Memoria eficiente y rápida.
        Usa scaled_dot_product_attention de PyTorch 2.0+
        """
        seq_len, dim = q.shape
        flops = 4 * seq_len * seq_len * dim  # Aproximado
        
        with self.timer("flash_attention", elements=seq_len*dim, flops=flops):
            q_gpu = torch.from_numpy(q.astype(np.float32)).to(DEVICE, dtype=self.dtype)
            k_gpu = torch.from_numpy(k.astype(np.float32)).to(DEVICE, dtype=self.dtype)
            v_gpu = torch.from_numpy(v.astype(np.float32)).to(DEVICE, dtype=self.dtype)
            
            # Reshape para scaled_dot_product_attention: (batch, heads, seq, dim)
            q_gpu = q_gpu.unsqueeze(0).unsqueeze(0)
            k_gpu = k_gpu.unsqueeze(0).unsqueeze(0)
            v_gpu = v_gpu.unsqueeze(0).unsqueeze(0)
            
            # Flash Attention (PyTorch 2.0+)
            if hasattr(F, 'scaled_dot_product_attention'):
                output = F.scaled_dot_product_attention(
                    q_gpu, k_gpu, v_gpu,
                    is_causal=causal,
                    dropout_p=0.0
                )
            else:
                # Fallback manual
                scale = 1.0 / (dim ** 0.5)
                scores = torch.matmul(q_gpu, k_gpu.transpose(-2, -1)) * scale
                if causal:
                    mask = torch.triu(torch.ones(seq_len, seq_len, device=DEVICE) * -1e9, diagonal=1)
                    scores = scores + mask
                weights = F.softmax(scores, dim=-1)
                output = torch.matmul(weights, v_gpu)
            
            return output.squeeze(0).squeeze(0).float().cpu().numpy()
    
    def multihead_attention(self, q: np.ndarray, k: np.ndarray, v: np.ndarray,
                            num_heads: int, causal: bool = True) -> np.ndarray:
        """Multi-head attention optimizada."""
        if len(q.shape) == 2:
            seq_len, dim = q.shape
            batch = 1
        else:
            batch, seq_len, dim = q.shape
        
        head_dim = dim // num_heads
        flops = 4 * batch * num_heads * seq_len * seq_len * head_dim
        
        with self.timer("multihead_attention", elements=batch*seq_len*dim, flops=flops):
            q_gpu = torch.from_numpy(q.astype(np.float32)).to(DEVICE, dtype=self.dtype)
            k_gpu = torch.from_numpy(k.astype(np.float32)).to(DEVICE, dtype=self.dtype)
            v_gpu = torch.from_numpy(v.astype(np.float32)).to(DEVICE, dtype=self.dtype)
            
            if len(q_gpu.shape) == 2:
                q_gpu = q_gpu.unsqueeze(0)
                k_gpu = k_gpu.unsqueeze(0)
                v_gpu = v_gpu.unsqueeze(0)
            
            # Reshape: (batch, seq, heads, head_dim) -> (batch, heads, seq, head_dim)
            q_gpu = q_gpu.view(batch, seq_len, num_heads, head_dim).transpose(1, 2)
            k_gpu = k_gpu.view(batch, seq_len, num_heads, head_dim).transpose(1, 2)
            v_gpu = v_gpu.view(batch, seq_len, num_heads, head_dim).transpose(1, 2)
            
            # Flash Attention
            if hasattr(F, 'scaled_dot_product_attention'):
                output = F.scaled_dot_product_attention(
                    q_gpu, k_gpu, v_gpu,
                    is_causal=causal,
                    dropout_p=0.0
                )
            else:
                scale = 1.0 / (head_dim ** 0.5)
                scores = torch.matmul(q_gpu, k_gpu.transpose(-2, -1)) * scale
                if causal:
                    mask = torch.triu(torch.ones(seq_len, seq_len, device=DEVICE) * -1e9, diagonal=1)
                    scores = scores + mask
                weights = F.softmax(scores, dim=-1)
                output = torch.matmul(weights, v_gpu)
            
            # Reshape back
            output = output.transpose(1, 2).contiguous().view(batch, seq_len, dim)
            
            if batch == 1:
                output = output.squeeze(0)
            
            return output.float().cpu().numpy()
    
    def fused_ffn(self, x: np.ndarray, w1: np.ndarray, w2: np.ndarray,
                  w1_cached: str = None, w2_cached: str = None) -> np.ndarray:
        """Feed-forward network fusionada (matmul + gelu + matmul)."""
        seq_len, dim = x.shape
        hidden_dim = w1.shape[1]
        flops = 2 * seq_len * dim * hidden_dim + 2 * seq_len * hidden_dim * dim
        
        with self.timer("fused_ffn", elements=seq_len*dim, flops=flops):
            x_gpu = torch.from_numpy(x.astype(np.float32)).to(DEVICE, dtype=self.dtype)
            
            if w1_cached and w1_cached in self.weight_cache:
                w1_gpu = self.weight_cache[w1_cached]
            else:
                w1_gpu = torch.from_numpy(w1.astype(np.float32)).to(DEVICE, dtype=self.dtype)
            
            if w2_cached and w2_cached in self.weight_cache:
                w2_gpu = self.weight_cache[w2_cached]
            else:
                w2_gpu = torch.from_numpy(w2.astype(np.float32)).to(DEVICE, dtype=self.dtype)
            
            # Fused: GELU(x @ W1) @ W2
            hidden = F.gelu(torch.matmul(x_gpu, w1_gpu))
            output = torch.matmul(hidden, w2_gpu)
            
            return output.float().cpu().numpy()
    
    def layer_norm(self, x: np.ndarray, eps: float = 1e-5) -> np.ndarray:
        """Layer normalization optimizada."""
        with self.timer("layer_norm", elements=x.size):
            x_gpu = torch.from_numpy(x.astype(np.float32)).to(DEVICE, dtype=self.dtype)
            output = F.layer_norm(x_gpu, x_gpu.shape[-1:], eps=eps)
            return output.float().cpu().numpy()
    
    def softmax(self, x: np.ndarray, dim: int = -1) -> np.ndarray:
        """Softmax optimizado."""
        with self.timer("softmax", elements=x.size):
            x_gpu = torch.from_numpy(x.astype(np.float32)).to(DEVICE, dtype=self.dtype)
            output = F.softmax(x_gpu, dim=dim)
            return output.float().cpu().numpy()
    
    # =========================================================================
    # TRANSFORMER LAYER COMPLETO
    # =========================================================================
    
    def transformer_layer_fused(self, x: np.ndarray,
                                 w_q: np.ndarray, w_k: np.ndarray, 
                                 w_v: np.ndarray, w_o: np.ndarray,
                                 w1: np.ndarray, w2: np.ndarray,
                                 num_heads: int = 8,
                                 layer_id: int = 0) -> np.ndarray:
        """
        Transformer layer completamente fusionado en GPU.
        Minimiza transferencias CPU<->GPU.
        """
        seq_len, dim = x.shape
        hidden_dim = w1.shape[1]
        
        # Calcular FLOPS totales
        flops = (
            4 * seq_len * dim * dim +  # Q, K, V, O projections
            4 * num_heads * seq_len * seq_len * (dim // num_heads) +  # Attention
            2 * seq_len * dim * hidden_dim +  # FFN up
            2 * seq_len * hidden_dim * dim  # FFN down
        )
        
        with self.timer(f"transformer_layer_{layer_id}", elements=seq_len*dim, flops=flops):
            # Transferir todo a GPU de una vez
            x_gpu = torch.from_numpy(x.astype(np.float32)).to(DEVICE, dtype=self.dtype)
            
            # Cachear pesos si es la primera vez
            cache_prefix = f"layer_{layer_id}_"
            if self.config.persistent_weights:
                w_q_gpu = self.cache_weights(f"{cache_prefix}w_q", w_q)
                w_k_gpu = self.cache_weights(f"{cache_prefix}w_k", w_k)
                w_v_gpu = self.cache_weights(f"{cache_prefix}w_v", w_v)
                w_o_gpu = self.cache_weights(f"{cache_prefix}w_o", w_o)
                w1_gpu = self.cache_weights(f"{cache_prefix}w1", w1)
                w2_gpu = self.cache_weights(f"{cache_prefix}w2", w2)
            else:
                w_q_gpu = torch.from_numpy(w_q.astype(np.float32)).to(DEVICE, dtype=self.dtype)
                w_k_gpu = torch.from_numpy(w_k.astype(np.float32)).to(DEVICE, dtype=self.dtype)
                w_v_gpu = torch.from_numpy(w_v.astype(np.float32)).to(DEVICE, dtype=self.dtype)
                w_o_gpu = torch.from_numpy(w_o.astype(np.float32)).to(DEVICE, dtype=self.dtype)
                w1_gpu = torch.from_numpy(w1.astype(np.float32)).to(DEVICE, dtype=self.dtype)
                w2_gpu = torch.from_numpy(w2.astype(np.float32)).to(DEVICE, dtype=self.dtype)
            
            # === ATTENTION ===
            q = torch.matmul(x_gpu, w_q_gpu)
            k = torch.matmul(x_gpu, w_k_gpu)
            v = torch.matmul(x_gpu, w_v_gpu)
            
            head_dim = dim // num_heads
            
            # Reshape para multi-head
            q = q.view(seq_len, num_heads, head_dim).transpose(0, 1)
            k = k.view(seq_len, num_heads, head_dim).transpose(0, 1)
            v = v.view(seq_len, num_heads, head_dim).transpose(0, 1)
            
            # Flash Attention
            if hasattr(F, 'scaled_dot_product_attention'):
                attn_out = F.scaled_dot_product_attention(
                    q.unsqueeze(0), k.unsqueeze(0), v.unsqueeze(0),
                    is_causal=True
                ).squeeze(0)
            else:
                scale = 1.0 / (head_dim ** 0.5)
                scores = torch.matmul(q, k.transpose(-2, -1)) * scale
                mask = torch.triu(torch.ones(seq_len, seq_len, device=DEVICE) * -1e9, diagonal=1)
                scores = scores + mask
                weights = F.softmax(scores, dim=-1)
                attn_out = torch.matmul(weights, v)
            
            # Reshape back
            attn_out = attn_out.transpose(0, 1).contiguous().view(seq_len, dim)
            attn_out = torch.matmul(attn_out, w_o_gpu)
            
            # Residual
            x_gpu = x_gpu + attn_out
            
            # === FFN ===
            hidden = F.gelu(torch.matmul(x_gpu, w1_gpu))
            ffn_out = torch.matmul(hidden, w2_gpu)
            
            # Residual
            x_gpu = x_gpu + ffn_out
            
            return x_gpu.float().cpu().numpy()
    
    # =========================================================================
    # BENCHMARK
    # =========================================================================
    
    def benchmark_comprehensive(self) -> Dict:
        """Benchmark completo de todas las operaciones."""
        print("\n" + "=" * 70)
        print("   🔥 BENCHMARK COMPLETO - GPU Advanced")
        print("=" * 70)
        
        results = {}
        
        # 1. MatMul
        print("\n📊 MatMul (con Tensor Cores):")
        print("-" * 60)
        for size in [512, 1024, 2048, 4096]:
            a = np.random.randn(size, size).astype(np.float32)
            b = np.random.randn(size, size).astype(np.float32)
            
            # Warmup
            _ = self.matmul(a, b)
            self.monitor.reset()
            
            # Benchmark
            times = []
            for _ in range(5):
                start = time.perf_counter()
                _ = self.matmul(a, b)
                torch.cuda.synchronize()
                times.append((time.perf_counter() - start) * 1000)
            
            avg_time = np.mean(times)
            gflops = (2 * size * size * size) / (avg_time / 1000) / 1e9
            print(f"   {size}x{size}: {avg_time:.2f} ms, {gflops:.1f} GFLOPS")
            results[f"matmul_{size}"] = {"time_ms": avg_time, "gflops": gflops}
        
        # 2. Flash Attention
        print("\n📊 Flash Attention:")
        print("-" * 60)
        for seq_len, dim in [(256, 128), (512, 256), (1024, 512), (2048, 512)]:
            q = np.random.randn(seq_len, dim).astype(np.float32)
            k = np.random.randn(seq_len, dim).astype(np.float32)
            v = np.random.randn(seq_len, dim).astype(np.float32)
            
            # Warmup
            _ = self.flash_attention(q, k, v)
            
            # Benchmark
            times = []
            for _ in range(5):
                start = time.perf_counter()
                _ = self.flash_attention(q, k, v)
                torch.cuda.synchronize()
                times.append((time.perf_counter() - start) * 1000)
            
            avg_time = np.mean(times)
            print(f"   seq={seq_len}, dim={dim}: {avg_time:.2f} ms")
            results[f"attention_{seq_len}x{dim}"] = {"time_ms": avg_time}
        
        # 3. Transformer Layer Fusionado
        print("\n📊 Transformer Layer (Fusionado):")
        print("-" * 60)
        for seq_len in [128, 256, 512]:
            dim = 256
            hidden = 1024
            heads = 8
            
            x = np.random.randn(seq_len, dim).astype(np.float32)
            w_q = np.random.randn(dim, dim).astype(np.float32) * 0.02
            w_k = np.random.randn(dim, dim).astype(np.float32) * 0.02
            w_v = np.random.randn(dim, dim).astype(np.float32) * 0.02
            w_o = np.random.randn(dim, dim).astype(np.float32) * 0.02
            w1 = np.random.randn(dim, hidden).astype(np.float32) * 0.02
            w2 = np.random.randn(hidden, dim).astype(np.float32) * 0.02
            
            # Warmup
            _ = self.transformer_layer_fused(x, w_q, w_k, w_v, w_o, w1, w2, heads)
            
            # Benchmark
            times = []
            for _ in range(5):
                start = time.perf_counter()
                _ = self.transformer_layer_fused(x, w_q, w_k, w_v, w_o, w1, w2, heads)
                torch.cuda.synchronize()
                times.append((time.perf_counter() - start) * 1000)
            
            avg_time = np.mean(times)
            tokens_per_sec = seq_len / (avg_time / 1000)
            print(f"   seq={seq_len}: {avg_time:.2f} ms, {tokens_per_sec:.0f} tok/s")
            results[f"transformer_{seq_len}"] = {"time_ms": avg_time, "tokens_per_sec": tokens_per_sec}
        
        # Resumen
        print("\n" + "=" * 70)
        print("   📈 RESUMEN")
        print("=" * 70)
        summary = self.monitor.get_summary()
        print(f"   Total operaciones: {summary.get('total_operations', 0)}")
        print(f"   Tiempo total: {summary.get('total_time_ms', 0):.1f} ms")
        print(f"   GFLOPS promedio: {summary.get('avg_gflops', 0):.1f}")
        print(f"   Memoria pico: {summary.get('peak_memory_mb', 0):.1f} MB")
        
        # Memoria GPU
        mem_allocated = torch.cuda.memory_allocated() / (1024**2)
        mem_reserved = torch.cuda.memory_reserved() / (1024**2)
        print(f"\n   GPU Memory:")
        print(f"   - Allocated: {mem_allocated:.1f} MB")
        print(f"   - Reserved: {mem_reserved:.1f} MB")
        
        return results
    
    def get_metrics(self) -> Dict:
        """Obtiene métricas actuales."""
        return self.monitor.get_summary()
    
    def clear_cache(self):
        """Limpia todos los caches."""
        self.weight_cache.clear()
        self.kv_cache.clear()
        torch.cuda.empty_cache()


# =============================================================================
# TRANSFORMER AVANZADO
# =============================================================================

class AdvancedGPUTransformer:
    """
    Transformer con todas las optimizaciones GPU.
    Diseñado para máximo rendimiento en RTX 3060.
    """
    
    def __init__(self, vocab_size: int, embed_dim: int, num_heads: int,
                 hidden_dim: int, num_layers: int, config: GPUConfig = None):
        self.vocab_size = vocab_size
        self.embed_dim = embed_dim
        self.num_heads = num_heads
        self.hidden_dim = hidden_dim
        self.num_layers = num_layers
        
        # GPU Advanced
        self.gpu = GPUAdvanced(config)
        
        # Inicializar pesos
        self._init_weights()
        
        # Pre-cachear pesos en GPU
        self._cache_all_weights()
    
    def _init_weights(self):
        """Inicializa pesos."""
        scale = 0.02
        
        self.embeddings = np.random.randn(self.vocab_size, self.embed_dim).astype(np.float32) * scale
        
        self.layers = []
        for _ in range(self.num_layers):
            layer = {
                "W_q": np.random.randn(self.embed_dim, self.embed_dim).astype(np.float32) * scale,
                "W_k": np.random.randn(self.embed_dim, self.embed_dim).astype(np.float32) * scale,
                "W_v": np.random.randn(self.embed_dim, self.embed_dim).astype(np.float32) * scale,
                "W_o": np.random.randn(self.embed_dim, self.embed_dim).astype(np.float32) * scale,
                "W1": np.random.randn(self.embed_dim, self.hidden_dim).astype(np.float32) * scale,
                "W2": np.random.randn(self.hidden_dim, self.embed_dim).astype(np.float32) * scale,
            }
            self.layers.append(layer)
        
        self.output_proj = np.random.randn(self.embed_dim, self.vocab_size).astype(np.float32) * scale
    
    def _cache_all_weights(self):
        """Pre-cachea todos los pesos en GPU."""
        print("📦 Cacheando pesos en GPU...")
        
        self.gpu.cache_weights("embeddings", self.embeddings)
        self.gpu.cache_weights("output_proj", self.output_proj)
        
        for i, layer in enumerate(self.layers):
            for name, weight in layer.items():
                self.gpu.cache_weights(f"layer_{i}_{name}", weight)
        
        mem = torch.cuda.memory_allocated() / (1024**2)
        print(f"   Memoria GPU usada: {mem:.1f} MB")
    
    def forward(self, token_ids: List[int]) -> np.ndarray:
        """Forward pass optimizado."""
        # Embeddings
        safe_ids = [min(max(0, t), self.vocab_size - 1) for t in token_ids]
        x = self.embeddings[safe_ids]
        
        # Capas transformer
        for i, layer in enumerate(self.layers):
            x = self.gpu.transformer_layer_fused(
                x,
                layer["W_q"], layer["W_k"], layer["W_v"], layer["W_o"],
                layer["W1"], layer["W2"],
                num_heads=self.num_heads,
                layer_id=i
            )
        
        # Output projection
        logits = self.gpu.matmul(x[-1:], self.output_proj)
        return logits[0]
    
    def generate_token(self, token_ids: List[int], temperature: float = 0.7,
                       top_k: int = 50) -> int:
        """Genera siguiente token."""
        logits = self.forward(token_ids)
        
        # Temperatura
        logits = logits / max(temperature, 0.1)
        
        # Top-k
        if top_k > 0:
            indices = np.argsort(logits)[-top_k:]
            mask = np.ones_like(logits) * -1e9
            mask[indices] = 0
            logits = logits + mask
        
        # Softmax
        probs = self.gpu.softmax(logits)
        
        return int(np.random.choice(len(probs), p=probs.astype(np.float64)))
    
    def get_metrics(self) -> Dict:
        """Obtiene métricas de rendimiento."""
        return self.gpu.get_metrics()


# =============================================================================
# DEMO
# =============================================================================

def demo():
    """Demo de GPU Advanced."""
    print("\n" + "=" * 70)
    print("   🔥 GPU Advanced Demo")
    print("   Optimizaciones Máximas para RTX 3060")
    print("=" * 70)
    
    # Crear GPU Advanced
    config = GPUConfig(
        precision=PrecisionMode.AUTO,
        use_flash_attention=True,
        persistent_weights=True,
    )
    
    gpu = GPUAdvanced(config)
    
    # Benchmark completo
    gpu.benchmark_comprehensive()
    
    print("\n" + "=" * 70)
    print("   ✅ Demo completada")
    print("=" * 70)


if __name__ == "__main__":
    demo()
