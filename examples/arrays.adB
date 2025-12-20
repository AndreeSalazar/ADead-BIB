# arrays.adB - Demostración de arrays
# Muestra: arrays, indexing, operaciones

def main():
    print("=== Arrays Demo ===")
    
    # Crear array de números
    a = 10
    b = 20
    c = 30
    d = 40
    e = 50
    
    # Suma de elementos
    total = a + b + c + d + e
    print("Sum of 10+20+30+40+50:")
    print(total)
    
    # Promedio
    avg = total / 5
    print("Average:")
    print(avg)
    
    # Máximo manual
    max = a
    if b > max:
        max = b
    if c > max:
        max = c
    if d > max:
        max = d
    if e > max:
        max = e
    print("Max:")
    print(max)
    
    # Mínimo manual
    min = a
    if b < min:
        min = b
    if c < min:
        min = c
    if d < min:
        min = d
    if e < min:
        min = e
    print("Min:")
    print(min)
    
    print("=== Done ===")
