"""
fix_runtime_callbacks.py
=========================
Reemplaza `: ()` por `: *mut core::ffi::c_void` en todas las firmas
`extern "C"` del runtime, eliminando los warnings FFI-safe.

P-03 de Documentos_Soluciones.md
"""

from pathlib import Path
import re

ROOT = Path(__file__).resolve().parent.parent / "C_Real_Optimo" / "runtime"

# Reemplaza `name: ()` o `name: ())` (parámetros) por puntero opaco
PATS = [
    (re.compile(r'(\b\w+):\s*\(\)(\s*[,\)])'), r'\1: *mut core::ffi::c_void\2'),
    # Tipo de retorno `-> ()` no se toca, es válido
]

if not ROOT.exists():
    raise SystemExit(f"No existe {ROOT}")

total_files = 0
total_replacements = 0
for p in sorted(ROOT.rglob("*.rs")):
    txt = p.read_text(encoding="utf-8")
    new = txt
    file_count = 0
    for pat, repl in PATS:
        new, n = pat.subn(repl, new)
        file_count += n
    if new != txt:
        p.write_text(new, encoding="utf-8")
        total_files += 1
        total_replacements += file_count
        print(f"  fixed {p.relative_to(ROOT)}: {file_count} replacements")

print(f"\nDONE: {total_replacements} replacements across {total_files} files.")
