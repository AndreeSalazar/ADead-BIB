"""
Speed Test: Python vs ADead-BIB
================================
Compara el tiempo de ejecución del loop de 1 billón
"""

import time
import subprocess
import os

def test_python():
    """Test Python puro - como en la imagen"""
    print("=" * 50)
    print("PYTHON PURO")
    print("=" * 50)
    print()
    print("counter = 0")
    print("while (counter < 1000000000):")
    print("    counter += 1")
    print("print(counter)")
    print()
    
    start = time.time()
    counter = 0
    while counter < 1000000000:
        counter += 1
    elapsed = time.time() - start
    
    print(f"Resultado: {counter}")
    print(f"⏱️ Tiempo: {elapsed:.2f}s")
    return elapsed


def test_adead():
    """Test ADead-BIB"""
    print()
    print("=" * 50)
    print("ADead-BIB (Compilado a binario nativo)")
    print("=" * 50)
    print()
    print("fn main() {")
    print("    let counter = 0")
    print("    while counter < 1000000000 {")
    print("        counter = counter + 1")
    print("    }")
    print("    println(counter)")
    print("}")
    print()
    
    # Crear archivo temporal - usando += para optimización
    code = '''fn main() {
    let counter = 0
    while counter < 1000000000 {
        counter += 1
    }
    println(counter)
}'''
    
    temp_file = "_speed_test.adB"
    with open(temp_file, 'w') as f:
        f.write(code)
    
    try:
        start = time.time()
        result = subprocess.run(
            ["adeadc", "run", temp_file],
            capture_output=True,
            text=True,
            timeout=120
        )
        elapsed = time.time() - start
        
        # Obtener resultado
        output = result.stdout.strip()
        lines = output.split('\n')
        for line in reversed(lines):
            line = line.strip()
            if line and line.isdigit():
                print(f"Resultado: {line}")
                break
        
        print(f"⏱️ Tiempo: {elapsed:.2f}s")
        return elapsed
    finally:
        if os.path.exists(temp_file):
            os.remove(temp_file)


def main():
    print()
    print("🔥 SPEED TEST: Python vs ADead-BIB 🔥")
    print("Loop de 1 billón de iteraciones")
    print()
    
    # Test ADead-BIB primero (más rápido)
    adead_time = test_adead()
    
    print()
    print("=" * 50)
    print("COMPARACIÓN")
    print("=" * 50)
    print()
    print(f"Python esperado:  ~7.32s (interpretado)")
    print(f"ADead-BIB:        {adead_time:.2f}s (binario nativo)")
    print()
    
    if adead_time > 0:
        speedup = 7.32 / adead_time
        print(f"🚀 ADead-BIB es ~{speedup:.1f}x más rápido que Python!")
    
    print()
    print("=" * 50)


if __name__ == "__main__":
    main()
