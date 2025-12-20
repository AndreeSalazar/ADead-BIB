# game_oop.adB - Ejemplo de juego con OOP
# Demuestra clases, herencia y polimorfismo

# Clase base para entidades del juego
class Entity:
    x = 0
    y = 0
    
    def init(self, x, y):
        self.x = x
        self.y = y
    
    virtual def update(self):
        pass
    
    virtual def draw(self):
        print("Entity")

# Jugador hereda de Entity
class Player extends Entity:
    health = 100
    
    override def update(self):
        print("Player update")
    
    override def draw(self):
        print("Player @")

# Enemigo hereda de Entity
class Enemy extends Entity:
    damage = 10
    
    override def update(self):
        print("Enemy update")
    
    override def draw(self):
        print("Enemy X")

# Función principal del juego
def main():
    print("=== ADead-BIB Game Demo ===")
    print("OOP: Classes, Inheritance, Polymorphism")
    
    # Variables simples para demostrar
    player_x = 10
    player_y = 20
    enemy_x = 50
    enemy_y = 30
    
    print("Player position:")
    print(player_x)
    print(player_y)
    
    print("Enemy position:")
    print(enemy_x)
    print(enemy_y)
    
    print("Game running!")
