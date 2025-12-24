"""
IA_Personal Integrations - Integraciones externas
"""

from .adead_accelerator import ADeadAccelerator, IAPersonalADead
from .ollama_chat import OllamaChat, IAPersonalOllama
from .gpu_compute import GPUCompute, GPUTransformer, IAPersonalGPU, ComputeBackend
from .gpu_advanced import GPUAdvanced, GPUConfig, PrecisionMode, AdvancedGPUTransformer
from .ia_personal_gpu_max import IAPersonalGPUMax

__all__ = [
    "ADeadAccelerator",
    "IAPersonalADead",
    "OllamaChat",
    "IAPersonalOllama",
    "GPUCompute",
    "GPUTransformer",
    "IAPersonalGPU",
    "ComputeBackend",
    "GPUAdvanced",
    "GPUConfig",
    "PrecisionMode",
    "AdvancedGPUTransformer",
    "IAPersonalGPUMax",
]
