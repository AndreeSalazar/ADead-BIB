# builtins.adB - Funciones built-in
# Demuestra: abs, min, max

def main():
    print("=== Built-in Functions ===")
    
    # abs()
    x = -42
    y = abs(x)
    print("abs(-42):")
    print(y)
    
    # min()
    a = 10
    b = 25
    m = min(a, b)
    print("min(10, 25):")
    print(m)
    
    # max()
    n = max(a, b)
    print("max(10, 25):")
    print(n)
    
    # Combinaciones
    c = -15
    d = 30
    result = max(abs(c), d)
    print("max(abs(-15), 30):")
    print(result)
    
    print("=== Done ===")
