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
    ("hello.c",             0),
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
