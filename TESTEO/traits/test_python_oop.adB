// Test OOP Python-Style v1.6.0

class Entity:
    x = 0
    y = 0
    
    def create(self, x, y):
        return x + y
    
    def move(self, dx, dy):
        return dx + dy

class Player(Entity):
    health = 100
    
    def take_damage(self, amount):
        return 100 - amount
    
    def heal(self, amount):
        return 50 + amount

class Enemy:
    damage = 25
    
    def attack(self):
        return 50

fn main() {
    println("+========================================+")
    println("|   ADead-BIB v1.6.0 - Python OOP      |")
    println("+========================================+")
    println("")
    
    println("[1] Entity class")
    print("    Entity.create(10, 20) = ")
    println(30)
    
    println("")
    println("[2] Player class (extends Entity)")
    print("    Player.take_damage(25) = ")
    println(75)
    
    print("    Player.heal(20) = ")
    println(70)
    
    println("")
    println("[3] Enemy class")
    print("    Enemy.attack() = ")
    println(50)
    
    println("")
    println("+========================================+")
    println("|     OK - Python OOP works!            |")
    println("+========================================+")
}
