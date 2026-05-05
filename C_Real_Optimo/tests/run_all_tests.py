"""
run_all_tests.py - ADead-BIB v12.0 Test Harness (Python)

Compila cada .c, ejecuta el .exe con timeout de 5 segundos,
y reporta PASS/FAIL/HANG/COMPILE-FAIL.

Usage:
    python run_all_tests.py
"""

import subprocess
import sys
import os
from pathlib import Path

THIS_DIR = Path(__file__).resolve().parent
ROOT = THIS_DIR.parent
COMPILER = ROOT / "target" / "release" / "adB.exe"

TESTS = [
    # Categoría 1 — Tests originales (01-10)
    ("c99/01_variables.c",  0),
    ("c99/02_arithmetic.c", 0),
    ("c99/03_if_else.c",    0),
    ("c99/04_while_loop.c", 0),
    ("c99/05_for_loop.c",   0),
    ("c99/06_functions.c",  0),
    ("c99/07_recursion.c",  0),
    ("c99/08_pointers.c",   0),
    ("c99/09_arrays.c",     0),
    ("c99/10_structs.c",    0),
    # Categoría 2 — Operadores (11-15)
    ("c99/11_bitwise.c",          0),
    ("c99/12_comparisons.c",      0),
    ("c99/13_logical.c",          0),
    ("c99/14_compound_assign.c",  0),
    ("c99/15_inc_dec.c",          0),
    # Categoría 3 — Control flow (16-18, 24-27)
    ("c99/16_nested_if.c",        0),
    ("c99/17_nested_loops.c",     0),
    ("c99/18_for_nested.c",       0),
    ("c99/24_ternary.c",          0),
    ("c99/25_break_continue.c",   0),
    ("c99/26_do_while.c",         0),
    ("c99/27_switch.c",           0),
    # Categoría 4 — Funciones (19-23, 31)
    ("c99/19_multi_func.c",       0),
    ("c99/20_factorial.c",        0),
    ("c99/21_fibonacci.c",        0),
    ("c99/22_4args.c",            0),
    ("c99/23_void_func.c",        0),
    ("c99/31_global_var.c",       0),
    # Categoría 5 — Tipos y declaraciones (28-30)
    ("c99/28_sizeof.c",           0),
    ("c99/29_typedef.c",          0),
    ("c99/30_enum.c",             0),
    # Categoría 6 — Aritmética avanzada (32-36)
    ("c99/32_long_chain.c",       0),
    ("c99/33_neg.c",              0),
    ("c99/34_paren.c",            0),
    ("c99/35_precedence.c",       0),
    ("c99/36_neg_div.c",          0),
    # Categoría 7 — Algoritmos (37-40)
    ("c99/37_loop_factorial.c",   0),
    ("c99/38_gcd.c",              0),
    ("c99/39_power.c",            0),
    ("c99/40_complex.c",          0),
    # Misc
    ("hello.c",                   0),
]

TIMEOUT = 5  # seconds


def main() -> int:
    if not COMPILER.exists():
        print(f"[ERROR] Compiler not found: {COMPILER}")
        print("Run 'cargo build --release' inside C_Real_Optimo/ first.")
        return 1

    print()
    print("=== ADead-BIB Test Harness v12.0 ===")
    print(f"Compiler: {COMPILER}")
    print(f"Timeout:  {TIMEOUT}s per test")
    print()

    pass_count = fail_count = hang_count = compile_fail = 0
    skipped = 0

    for rel_path, expected in TESTS:
        src = THIS_DIR / rel_path
        if not src.exists():
            print(f"[skip]         {rel_path}")
            skipped += 1
            continue

        exe = src.with_suffix(".test.exe")
        if exe.exists():
            try:
                exe.unlink()
            except OSError:
                pass

        # Compile
        try:
            r = subprocess.run(
                [str(COMPILER), "cc", str(src), "-o", str(exe)],
                capture_output=True, text=True, timeout=10,
            )
        except subprocess.TimeoutExpired:
            print(f"[FAIL-COMPILE] {rel_path} (compiler hang)")
            compile_fail += 1
            continue

        if r.returncode != 0 or not exe.exists():
            print(f"[FAIL-COMPILE] {rel_path}")
            compile_fail += 1
            continue

        # Run with timeout
        try:
            run = subprocess.run(
                [str(exe)], capture_output=True, timeout=TIMEOUT
            )
            code = run.returncode
            if code == expected:
                print(f"[PASS]         {rel_path} exit={code}")
                pass_count += 1
            else:
                print(f"[FAIL]         {rel_path} exit={code} expected={expected}")
                fail_count += 1
        except subprocess.TimeoutExpired:
            print(f"[HANG]         {rel_path} (>{TIMEOUT}s)")
            hang_count += 1
        finally:
            try:
                exe.unlink()
            except OSError:
                pass

    total = pass_count + fail_count + hang_count + compile_fail
    print()
    print("=== Summary ===")
    print(f"  PASS:         {pass_count} / {total}")
    print(f"  FAIL:         {fail_count} / {total}")
    print(f"  HANG:         {hang_count} / {total}")
    print(f"  COMPILE-FAIL: {compile_fail} / {total}")
    if skipped:
        print(f"  SKIPPED:      {skipped}")
    print()

    if total > 0 and pass_count == total:
        print("[OK] ALL TESTS PASSED")
        return 0
    return 1


if __name__ == "__main__":
    sys.exit(main())
