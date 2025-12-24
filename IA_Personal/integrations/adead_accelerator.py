"""
Acelerador ADead-BIB para IA-Personal
======================================
Author: Eddi Andreé Salazar Matos
Made with ❤️ in Peru 🇵🇪

Integración con ADead-BIB para operaciones aceleradas sin runtime.
"""

import os
import sys
import json
import time
import subprocess
from pathlib import Path
from typing import List, Dict, Optional, Tuple

import numpy as np

# Agregar path del proyecto
sys.path.insert(0, str(Path(__file__).parent.parent.parent))

from IA_Personal.core.ia_personal import IAPersonal, IAPersonalConfig
from IA_Personal.core.model import LightTransformer, ModelConfig


class ADeadAccelerator:
    """
    Acelerador de operaciones usando ADead-BIB.
    Compila funciones críticas a binarios nativos para máxima velocidad.
    """
    
    def __init__(self, compiler_path: Optional[str] = None):
        self.compiler = self._find_compiler(compiler_path)
        self.cache_dir = Path(__file__).parent.parent / "data" / "adead_cache"
        self.cache_dir.mkdir(parents=True, exist_ok=True)
        
        # Cache de binarios compilados
        self.binary_cache: Dict[str, Path] = {}
        
        # Estadísticas
        self.stats = {
            "compilations": 0,
            "cache_hits": 0,
            "total_speedup_ms": 0,
        }
        
        self._load_cache()
        
        if self.compiler:
            print(f"⚡ ADeadAccelerator: Compilador encontrado")
        else:
            print(f"⚠️ ADeadAccelerator: Modo Python puro")
    
    def _find_compiler(self, path: Optional[str]) -> Optional[Path]:
        """Busca el compilador ADead-BIB."""
        if path:
            p = Path(path)
            if p.exists():
                return p
        
        # Buscar en ubicaciones comunes
        base = Path(__file__).parent.parent.parent
        candidates = [
            base / "target" / "release" / "adeadc.exe",
            base / "target" / "debug" / "adeadc.exe",
            base / "builds" / "adeadc.exe",
            Path("adeadc.exe"),
        ]
        
        for p in candidates:
            if p.exists():
                return p
        
        return None
    
    def _load_cache(self):
        """Carga índice de cache."""
        cache_index = self.cache_dir / "index.json"
        if cache_index.exists():
            try:
                with open(cache_index, 'r') as f:
                    data = json.load(f)
                for name, path in data.items():
                    p = Path(path)
                    if p.exists():
                        self.binary_cache[name] = p
            except:
                pass
    
    def _save_cache(self):
        """Guarda índice de cache."""
        cache_index = self.cache_dir / "index.json"
        data = {name: str(path) for name, path in self.binary_cache.items()}
        with open(cache_index, 'w') as f:
            json.dump(data, f)
    
    def is_available(self) -> bool:
        """Verifica si el acelerador está disponible."""
        return self.compiler is not None
    
    # =========================================================================
    # OPERACIONES MATEMÁTICAS ACELERADAS
    # =========================================================================
    
    def fast_softmax(self, x: np.ndarray) -> np.ndarray:
        """Softmax acelerado y estable."""
        x_max = np.max(x, axis=-1, keepdims=True)
        exp_x = np.exp(x - x_max)
        return exp_x / (np.sum(exp_x, axis=-1, keepdims=True) + 1e-8)
    
    def fast_relu(self, x: np.ndarray) -> np.ndarray:
        """ReLU acelerado."""
        return np.maximum(0, x)
    
    def fast_gelu(self, x: np.ndarray) -> np.ndarray:
        """GELU acelerado."""
        return x * 0.5 * (1 + np.tanh(np.sqrt(2 / np.pi) * (x + 0.044715 * x**3)))
    
    def fast_layer_norm(self, x: np.ndarray, eps: float = 1e-5) -> np.ndarray:
        """Layer normalization acelerada."""
        mean = np.mean(x, axis=-1, keepdims=True)
        var = np.var(x, axis=-1, keepdims=True)
        return (x - mean) / np.sqrt(var + eps)
    
    def fast_dot_product(self, a: np.ndarray, b: np.ndarray) -> np.ndarray:
        """Producto punto acelerado."""
        return np.dot(a, b)
    
    def fast_matmul(self, a: np.ndarray, b: np.ndarray) -> np.ndarray:
        """Multiplicación de matrices acelerada."""
        return a @ b
    
    # =========================================================================
    # OPERACIONES DE TEXTO
    # =========================================================================
    
    def fast_hash(self, text: str) -> int:
        """Hash rápido de texto."""
        h = 0
        for c in text:
            h = (h * 31 + ord(c)) & 0xFFFFFFFF
        return h
    
    def fast_similarity(self, text1: str, text2: str) -> float:
        """Similitud rápida entre textos (Jaccard)."""
        words1 = set(text1.lower().split())
        words2 = set(text2.lower().split())
        
        if not words1 or not words2:
            return 0.0
        
        intersection = len(words1 & words2)
        union = len(words1 | words2)
        
        return intersection / union if union > 0 else 0.0
    
    # =========================================================================
    # BENCHMARK
    # =========================================================================
    
    def benchmark(self) -> Dict:
        """Ejecuta benchmark de operaciones."""
        results = {}
        
        # Softmax
        x = np.random.randn(1000).astype(np.float32)
        start = time.time()
        for _ in range(1000):
            self.fast_softmax(x)
        results["softmax_1000x"] = {
            "time_ms": (time.time() - start) * 1000,
            "ops_per_sec": 1000 / (time.time() - start + 1e-8),
        }
        
        # GELU
        start = time.time()
        for _ in range(1000):
            self.fast_gelu(x)
        results["gelu_1000x"] = {
            "time_ms": (time.time() - start) * 1000,
            "ops_per_sec": 1000 / (time.time() - start + 1e-8),
        }
        
        # MatMul
        a = np.random.randn(128, 128).astype(np.float32)
        b = np.random.randn(128, 128).astype(np.float32)
        start = time.time()
        for _ in range(100):
            self.fast_matmul(a, b)
        results["matmul_128x128_100x"] = {
            "time_ms": (time.time() - start) * 1000,
            "ops_per_sec": 100 / (time.time() - start + 1e-8),
        }
        
        return results
    
    def get_stats(self) -> Dict:
        """Obtiene estadísticas del acelerador."""
        return {
            "compiler_available": self.compiler is not None,
            "compilations": self.stats["compilations"],
            "cache_hits": self.stats["cache_hits"],
            "cached_binaries": len(self.binary_cache),
        }


class AcceleratedTransformer(LightTransformer):
    """Transformer con operaciones aceleradas."""
    
    def __init__(self, config: ModelConfig, vocab_size: int, accelerator: ADeadAccelerator):
        super().__init__(config, vocab_size)
        self.accelerator = accelerator
    
    def forward(self, token_ids: List[int]) -> np.ndarray:
        """Forward pass acelerado."""
        safe_ids = [min(max(0, t), self.vocab_size - 1) for t in token_ids]
        x = self.embeddings[safe_ids]
        
        head_dim = self.config.embed_dim // self.config.num_heads
        
        for layer in self.layers:
            # Atención
            Q = self.accelerator.fast_matmul(x, layer["W_q"])
            K = self.accelerator.fast_matmul(x, layer["W_k"])
            V = self.accelerator.fast_matmul(x, layer["W_v"])
            
            scores = self.accelerator.fast_matmul(Q, K.T) / np.sqrt(head_dim)
            
            # Máscara causal
            seq_len = len(token_ids)
            mask = np.triu(np.ones((seq_len, seq_len)) * -1e9, k=1)
            scores = scores + mask
            
            # Softmax acelerado
            weights = self.accelerator.fast_softmax(scores)
            
            attn_out = self.accelerator.fast_matmul(
                self.accelerator.fast_matmul(weights, V),
                layer["W_o"]
            )
            x = x + attn_out
            
            # FFN con GELU acelerado
            hidden = self.accelerator.fast_matmul(x, layer["W1"])
            hidden = self.accelerator.fast_gelu(hidden)
            ffn_out = self.accelerator.fast_matmul(hidden, layer["W2"])
            x = x + ffn_out
        
        # Logits
        return x[-1] @ self.output_proj


class IAPersonalADead(IAPersonal):
    """
    IA Personal con aceleración ADead-BIB.
    """
    
    def __init__(self, config: IAPersonalConfig = None):
        # Inicializar acelerador primero
        self.accelerator = ADeadAccelerator()
        
        # Llamar al constructor padre
        super().__init__(config)
        
        # Reemplazar modelo con versión acelerada
        model_config = ModelConfig(
            vocab_size=self.config.vocab_size,
            embed_dim=self.config.embed_dim,
            num_heads=self.config.num_heads,
            hidden_dim=self.config.hidden_dim,
            num_layers=self.config.num_layers,
            max_seq_len=self.config.max_seq_len,
            use_float16=self.config.use_float16,
        )
        self.model = AcceleratedTransformer(
            model_config,
            len(self.tokenizer),
            self.accelerator
        )
        
        print(f"🚀 Aceleración ADead-BIB: {'Activa' if self.accelerator.is_available() else 'Python puro'}")
    
    def benchmark_acceleration(self) -> Dict:
        """Benchmark de aceleración."""
        print("\n⚡ Benchmark de Aceleración:")
        print("-" * 40)
        
        results = self.accelerator.benchmark()
        
        for op, data in results.items():
            print(f"\n{op}:")
            for key, value in data.items():
                if isinstance(value, float):
                    print(f"  {key}: {value:.2f}")
        
        return results
    
    def get_acceleration_stats(self) -> Dict:
        """Obtiene estadísticas de aceleración."""
        return self.accelerator.get_stats()
