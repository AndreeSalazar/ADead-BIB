"""
IA-Personal GPU MAX - Máximo Rendimiento
=========================================
Author: Eddi Andreé Salazar Matos
Email: eddi.salazar.dev@gmail.com
Made with ❤️ in Peru 🇵🇪

La versión más potente de IA-Personal:
- GPU Advanced con todas las optimizaciones
- Flash Attention
- BF16/FP16 con Tensor Cores
- Pesos persistentes en GPU
- Métricas en tiempo real

Uso:
    python -m IA_Personal --gpu-max
    
    # O directamente:
    from IA_Personal.integrations.ia_personal_gpu_max import IAPersonalGPUMax
    ia = IAPersonalGPUMax()
    ia.chat("Hola")
"""

import sys
import time
from pathlib import Path
from typing import Dict, List, Optional

import numpy as np

sys.path.insert(0, str(Path(__file__).parent.parent.parent))

from IA_Personal.core.ia_personal import IAPersonal, IAPersonalConfig
from IA_Personal.core.tokenizer import SmartTokenizer
from IA_Personal.integrations.gpu_advanced import (
    GPUAdvanced, GPUConfig, PrecisionMode, AdvancedGPUTransformer
)


class IAPersonalGPUMax(IAPersonal):
    """
    IA Personal con GPU al máximo rendimiento.
    Usa todas las optimizaciones disponibles.
    """
    
    def __init__(self, config: IAPersonalConfig = None):
        # Configuración GPU optimizada
        self.gpu_config = GPUConfig(
            precision=PrecisionMode.AUTO,  # BF16 en Ampere
            use_tensor_cores=True,
            persistent_weights=True,
            use_flash_attention=True,
            use_fused_ops=True,
            use_kv_cache=True,
            auto_tune=True,
            benchmark_cudnn=True,
        )
        
        # Inicializar GPU Advanced
        self.gpu_advanced = GPUAdvanced(self.gpu_config)
        
        # Llamar constructor padre
        super().__init__(config)
        
        # Reemplazar modelo con versión GPU Max
        self.model = AdvancedGPUTransformer(
            vocab_size=len(self.tokenizer),
            embed_dim=self.config.embed_dim,
            num_heads=self.config.num_heads,
            hidden_dim=self.config.hidden_dim,
            num_layers=self.config.num_layers,
            config=self.gpu_config
        )
        
        # Estadísticas de rendimiento
        self.perf_stats = {
            "total_chats": 0,
            "total_time_ms": 0,
            "avg_time_ms": 0,
            "tokens_generated": 0,
        }
        
        print(f"\n🔥 IA-Personal GPU MAX inicializado")
        print(f"   Precisión: {self.gpu_advanced.precision_name}")
        print(f"   Flash Attention: ✅")
        print(f"   Tensor Cores: ✅")
    
    def chat(self, message: str) -> str:
        """Chat con métricas de rendimiento."""
        start = time.perf_counter()
        
        response = super().chat(message)
        
        elapsed = (time.perf_counter() - start) * 1000
        self.perf_stats["total_chats"] += 1
        self.perf_stats["total_time_ms"] += elapsed
        self.perf_stats["avg_time_ms"] = (
            self.perf_stats["total_time_ms"] / self.perf_stats["total_chats"]
        )
        
        return response
    
    def benchmark_full(self) -> Dict:
        """Benchmark completo del sistema."""
        print("\n" + "=" * 70)
        print("   🔥 BENCHMARK COMPLETO - IA-Personal GPU MAX")
        print("=" * 70)
        
        # Benchmark GPU
        gpu_results = self.gpu_advanced.benchmark_comprehensive()
        
        # Benchmark de chat
        print("\n📊 Benchmark de Chat:")
        print("-" * 60)
        
        prompts = ["Hola", "¿Cómo estás?", "¿Qué puedes hacer?", "Cuéntame algo"]
        times = []
        
        for _ in range(20):
            prompt = prompts[_ % len(prompts)]
            start = time.perf_counter()
            _ = self.chat(prompt)
            times.append((time.perf_counter() - start) * 1000)
        
        print(f"   Tiempo promedio: {np.mean(times):.2f} ms")
        print(f"   Tiempo mínimo: {np.min(times):.2f} ms")
        print(f"   Tiempo máximo: {np.max(times):.2f} ms")
        print(f"   Chats/segundo: {1000 / np.mean(times):.1f}")
        
        return {
            "gpu_results": gpu_results,
            "chat_avg_ms": np.mean(times),
            "chat_min_ms": np.min(times),
            "chat_max_ms": np.max(times),
        }
    
    def get_gpu_metrics(self) -> Dict:
        """Obtiene métricas de GPU."""
        return {
            "gpu_metrics": self.gpu_advanced.get_metrics(),
            "perf_stats": self.perf_stats,
            "precision": self.gpu_advanced.precision_name,
        }
    
    def print_status(self):
        """Imprime estado del sistema."""
        metrics = self.get_gpu_metrics()
        gpu = metrics["gpu_metrics"]
        perf = metrics["perf_stats"]
        
        print("\n" + "=" * 60)
        print("   📊 Estado IA-Personal GPU MAX")
        print("=" * 60)
        print(f"\n🎮 GPU:")
        print(f"   Precisión: {metrics['precision']}")
        print(f"   Operaciones: {gpu.get('total_operations', 0)}")
        print(f"   GFLOPS promedio: {gpu.get('avg_gflops', 0):.1f}")
        print(f"   Memoria pico: {gpu.get('peak_memory_mb', 0):.1f} MB")
        print(f"\n💬 Chat:")
        print(f"   Total chats: {perf['total_chats']}")
        print(f"   Tiempo promedio: {perf['avg_time_ms']:.2f} ms")
        print("=" * 60)


# =============================================================================
# CLI INTEGRATION
# =============================================================================

def add_gpu_max_mode():
    """Agrega modo --gpu-max al CLI."""
    pass  # Se integra en cli.py


# =============================================================================
# DEMO
# =============================================================================

def demo():
    """Demo de IA-Personal GPU MAX."""
    print("\n" + "=" * 70)
    print("   🔥 IA-Personal GPU MAX Demo")
    print("   Máximo Rendimiento en RTX 3060")
    print("=" * 70)
    
    # Crear IA con GPU MAX
    config = IAPersonalConfig(
        vocab_size=10000,
        embed_dim=256,
        num_heads=8,
        hidden_dim=1024,
        num_layers=4,
        temperature=0.7,
    )
    
    ia = IAPersonalGPUMax(config)
    
    # Conversación de prueba
    print("\n📝 Conversación de Prueba:")
    print("-" * 60)
    
    messages = [
        "Hola",
        "Me llamo GPU Master",
        "Me gusta el deep learning",
        "¿Qué sabes de mí?",
        "perfil",
    ]
    
    for msg in messages:
        print(f"\n👤: {msg}")
        start = time.perf_counter()
        response = ia.chat(msg)
        elapsed = (time.perf_counter() - start) * 1000
        print(f"🤖: {response}")
        print(f"   ⏱️ {elapsed:.2f} ms")
    
    # Benchmark
    print("\n")
    ia.benchmark_full()
    
    # Estado final
    ia.print_status()
    
    print("\n" + "=" * 70)
    print("   ✅ Demo completada")
    print("=" * 70)


if __name__ == "__main__":
    demo()
