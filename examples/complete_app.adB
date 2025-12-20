# complete_app.adB - Aplicación completa en ADead-BIB
# Demuestra TODAS las características del lenguaje

def main():
    print("========================================")
    print("   ADead-BIB Complete Application Demo")
    print("========================================")
    print("")
    
    # ===== VARIABLES Y TIPOS =====
    print("--- Variables y Tipos ---")
    integer = 42
    negative = -17
    
    print("Integer:")
    print(integer)
    print("Negative:")
    print(negative)
    
    # ===== OPERACIONES ARITMÉTICAS =====
    print("")
    print("--- Operaciones Aritmeticas ---")
    a = 100
    b = 30
    
    sum = a + b
    diff = a - b
    prod = a * b
    quot = a / b
    mod = a % b
    
    print("100 + 30 =")
    print(sum)
    print("100 - 30 =")
    print(diff)
    print("100 * 30 =")
    print(prod)
    print("100 / 30 =")
    print(quot)
    print("100 mod 30 =")
    print(mod)
    
    # ===== FUNCIONES BUILT-IN =====
    print("")
    print("--- Funciones Built-in ---")
    
    x = -50
    abs_x = abs(x)
    print("abs(-50) =")
    print(abs_x)
    
    min_val = min(25, 75)
    print("min(25, 75) =")
    print(min_val)
    
    max_val = max(25, 75)
    print("max(25, 75) =")
    print(max_val)
    
    # ===== CONDICIONALES =====
    print("")
    print("--- Condicionales ---")
    
    score = 85
    
    if score >= 80:
        print("Grade: B (score >= 80)")
    else:
        print("Grade: lower")
    
    # ===== BUCLES =====
    print("")
    print("--- Bucles ---")
    
    print("Counting 1 to 5:")
    i = 1
    while i <= 5:
        print(i)
        i = i + 1
    
    # ===== CÁLCULOS COMPLEJOS =====
    print("")
    print("--- Calculos Complejos ---")
    
    # Factorial de 5
    n = 5
    factorial = 1
    j = 1
    while j <= n:
        factorial = factorial * j
        j = j + 1
    print("5! =")
    print(factorial)
    
    # Fibonacci hasta 10
    print("Fibonacci sequence:")
    fib1 = 0
    fib2 = 1
    count = 0
    while count < 10:
        print(fib1)
        temp = fib1 + fib2
        fib1 = fib2
        fib2 = temp
        count = count + 1
    
    # ===== SIMULACIÓN DE JUEGO =====
    print("")
    print("--- Game Simulation ---")
    
    player_hp = 100
    player_atk = 25
    enemy_hp = 80
    enemy_atk = 15
    
    round = 1
    while player_hp > 0 and enemy_hp > 0:
        print("Round:")
        print(round)
        
        # Player attacks
        enemy_hp = enemy_hp - player_atk
        print("Enemy HP:")
        print(enemy_hp)
        
        if enemy_hp <= 0:
            print("Enemy defeated!")
        else:
            # Enemy attacks
            player_hp = player_hp - enemy_atk
            print("Player HP:")
            print(player_hp)
            
            if player_hp <= 0:
                print("Player defeated!")
        
        round = round + 1
    
    # ===== RESUMEN =====
    print("")
    print("========================================")
    print("   Application Complete!")
    print("========================================")
    print("")
    print("Features demonstrated:")
    print("- Variables and types")
    print("- Arithmetic operations")
    print("- Built-in functions")
    print("- Conditionals (if/elif/else)")
    print("- Loops (while)")
    print("- Complex calculations")
    print("- Game simulation")
    print("")
    print("ADead-BIB: Production-ready language!")
