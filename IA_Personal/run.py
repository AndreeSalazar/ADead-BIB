"""
IA-Personal - Script de Inicio Rápido
======================================
Ejecuta directamente: python run.py

Author: Eddi Andreé Salazar Matos
Made with ❤️ in Peru 🇵🇪
"""

import sys
from pathlib import Path

# Agregar path del proyecto
sys.path.insert(0, str(Path(__file__).parent.parent))

from IA_Personal.ui.cli import main

if __name__ == "__main__":
    main()
