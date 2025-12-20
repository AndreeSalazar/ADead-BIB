# game_advanced.adB - Juego avanzado
# Demuestra: variables, operaciones, simulación de combate

def main():
    print("================================")
    print("   ADead-BIB Game Engine Demo   ")
    print("================================")
    print("")
    
    # Estadísticas del jugador
    player_x = 10
    player_y = 5
    player_hp = 100
    player_dmg = 25
    
    # Estadísticas del enemigo
    enemy_x = 50
    enemy_y = 30
    enemy_hp = 50
    enemy_dmg = 10
    
    # Estadísticas del boss
    boss_x = 100
    boss_y = 50
    boss_hp = 200
    boss_dmg = 50
    
    print("=== Player Stats ===")
    print("Position X:")
    print(player_x)
    print("Position Y:")
    print(player_y)
    print("Health:")
    print(player_hp)
    print("Damage:")
    print(player_dmg)
    print("")
    
    print("=== Enemy Stats ===")
    print("Position X:")
    print(enemy_x)
    print("Position Y:")
    print(enemy_y)
    print("Health:")
    print(enemy_hp)
    print("")
    
    print("=== Boss Stats ===")
    print("Position X:")
    print(boss_x)
    print("Position Y:")
    print(boss_y)
    print("Health:")
    print(boss_hp)
    print("Damage:")
    print(boss_dmg)
    print("")
    
    # Simulación de combate
    print("=== Combat Simulation ===")
    
    # Player ataca a Enemy
    enemy_hp = enemy_hp - player_dmg
    print("Player attacks Enemy!")
    print("Enemy HP remaining:")
    print(enemy_hp)
    
    # Enemy ataca a Player
    player_hp = player_hp - enemy_dmg
    print("Enemy attacks Player!")
    print("Player HP remaining:")
    print(player_hp)
    
    # Player ataca a Enemy (kill)
    enemy_hp = enemy_hp - player_dmg
    print("Player attacks Enemy!")
    print("Enemy HP remaining:")
    print(enemy_hp)
    print("Enemy defeated!")
    print("")
    
    # Calcular distancia al boss
    dx = boss_x - player_x
    dy = boss_y - player_y
    print("Distance to Boss:")
    print("X distance:")
    print(dx)
    print("Y distance:")
    print(dy)
    print("")
    
    print("================================")
    print("      Game Demo Complete!       ")
    print("================================")
