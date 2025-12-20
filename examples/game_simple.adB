# game_simple.adB - Juego simple con variables
# Demuestra el potencial para juegos

def main():
    print("=== ADead-BIB Game ===")
    
    # Posición del jugador
    player_x = 10
    player_y = 5
    
    # Posición del enemigo
    enemy_x = 50
    enemy_y = 30
    
    # Salud
    player_hp = 100
    enemy_hp = 50
    
    print("Player Position:")
    print(player_x)
    print(player_y)
    
    print("Player HP:")
    print(player_hp)
    
    print("Enemy Position:")
    print(enemy_x)
    print(enemy_y)
    
    print("Enemy HP:")
    print(enemy_hp)
    
    # Calcular distancia (simplificado)
    dx = enemy_x - player_x
    dy = enemy_y - player_y
    
    print("Distance X:")
    print(dx)
    
    print("Distance Y:")
    print(dy)
    
    print("Game Ready!")
