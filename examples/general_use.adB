# general_use.adB - Ejemplo de uso general
# ADead-BIB: Listo para uso general

import math

def main():
    print("=== ADead-BIB ===")
    
    # Matemáticas
    print(pow(2, 8))
    print(sqrt(144))
    print(abs(-42))
    
    # Utilidades
    print(max(10, 25))
    print(clamp(150, 0, 100))
    print(min3(5, 2, 8))
    
    # Bits
    print(bit_and(15, 7))
    print(shl(1, 4))
    
    # Comparaciones
    print(equals(5, 5))
    print(between(5, 1, 10))
    
    # Loop
    i = 1
    t = 0
    while i <= 5:
        t = t + i
        i = i + 1
    print(t)
    
    print("=== OK ===")
