// Test Python-Style Class OOP v1.6.0

class Player:
    def attack(self, damage):
        return damage * 2
    
    def heal(self, amount):
        return amount + 10

class Enemy:
    def strike(self, power):
        return power * 3

fn main() {
    println("+========================================+")
    println("|   ADead-BIB v1.6.0 - Python Class     |")
    println("+========================================+")
    println("")
    
    println("[1] Player class methods")
    print("    Player::attack(25) = ")
    println(Player::attack(25))
    
    print("    Player::heal(15) = ")
    println(Player::heal(15))
    
    println("")
    println("[2] Enemy class methods")
    print("    Enemy::strike(10) = ")
    println(Enemy::strike(10))
    
    println("")
    println("+========================================+")
    println("|     OK - Python Class OOP works!      |")
    println("+========================================+")
}
