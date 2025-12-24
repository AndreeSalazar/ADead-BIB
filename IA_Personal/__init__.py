"""
IA_Personal - Sistema de IA Personal para ADead-BIB
===================================================
Author: Eddi Andreé Salazar Matos
Email: eddi.salazar.dev@gmail.com
Made with ❤️ in Peru 🇵🇪

Tu asistente personal que aprende de ti y se integra con ADead-BIB.

Uso rápido:
    from IA_Personal import IAPersonal
    
    ia = IAPersonal()
    ia.chat("Hola")
    
    # O ejecutar directamente:
    # python -m IA_Personal
"""

from .core.ia_personal import IAPersonal, IAPersonalConfig
from .core.memory import PersistentMemory, MemoryItem
from .core.context import PersonalContext, UserProfile
from .core.tokenizer import SmartTokenizer
from .core.model import LightTransformer

from .integrations.adead_accelerator import ADeadAccelerator, IAPersonalADead
from .integrations.ollama_chat import OllamaChat, IAPersonalOllama

from .ui.chat import IAPersonalChat
from .ui.cli import main as cli_main

__version__ = "1.0.0"
__author__ = "Eddi Andreé Salazar Matos"
__email__ = "eddi.salazar.dev@gmail.com"

__all__ = [
    # Core
    "IAPersonal",
    "IAPersonalConfig", 
    "PersistentMemory",
    "MemoryItem",
    "PersonalContext",
    "UserProfile",
    "SmartTokenizer",
    "LightTransformer",
    # Integrations
    "ADeadAccelerator",
    "IAPersonalADead",
    "OllamaChat",
    "IAPersonalOllama",
    # UI
    "IAPersonalChat",
    "cli_main",
]


def quick_start():
    """Inicio rápido de IA-Personal."""
    ia = IAPersonal()
    ia.interactive()


def chat():
    """Inicia el chat con interfaz mejorada."""
    chat_ui = IAPersonalChat()
    chat_ui.run()
