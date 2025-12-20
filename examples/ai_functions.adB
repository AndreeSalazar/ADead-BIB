# Ejemplo: Funciones Matriciales para IA
# ADead-BIB ahora tiene funciones optimizadas para IA

def main():
    print("=== Funciones Matriciales para IA ===")
    
    # dot product: a*b + c*d
    print("dot(2, 3, 4, 5) =")
    print(dot(2, 3, 4, 5))
    
    # sum of squares: a² + b² + c²
    print("sum_sq(3, 4) =")
    print(sum_sq(3, 4))
    
    # norm squared
    print("norm_sq(3, 4) =")
    print(norm_sq(3, 4))
    
    # ReLU activation
    print("relu(5) =")
    print(relu(5))
    
    print("relu(-3) =")
    print(relu(-3))
    
    # weighted sum: v1*w1 + v2*w2
    print("weighted_sum(10, 2, 20, 3) =")
    print(weighted_sum(10, 2, 20, 3))
    
    # scale: x * factor / 100
    print("scale(200, 50) =")
    print(scale(200, 50))
    
    # lerp: a + (b-a) * t / 100
    print("lerp(0, 100, 50) =")
    print(lerp(0, 100, 50))
    
    print("=== Fin ===")
