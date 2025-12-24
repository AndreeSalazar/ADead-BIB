"""
IA_Personal UI - Interfaces de usuario
"""

from .chat import IAPersonalChat
from .cli import main as cli_main

__all__ = [
    "IAPersonalChat",
    "cli_main",
]
