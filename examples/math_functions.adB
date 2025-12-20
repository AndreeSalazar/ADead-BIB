# math_functions.adB - Funciones matemáticas
# Demuestra: pow, sqrt, sum

def main():
    print("=== Math Functions ===")
    
    # pow(base, exp)
    result = pow(2, 8)
    print("pow(2, 8) = 256:")
    print(result)
    
    result2 = pow(3, 4)
    print("pow(3, 4) = 81:")
    print(result2)
    
    # sqrt(x)
    root = sqrt(100)
    print("sqrt(100) = 10:")
    print(root)
    
    root2 = sqrt(81)
    print("sqrt(81) = 9:")
    print(root2)
    
    # sum(a, b, c, ...)
    total = sum(1, 2, 3, 4, 5)
    print("sum(1,2,3,4,5) = 15:")
    print(total)
    
    # Combinaciones
    x = pow(2, 4)
    y = sqrt(x)
    print("sqrt(pow(2,4)) = 4:")
    print(y)
    
    print("=== Done ===")
