// ============================================
// ADead-BIB - Ejemplo 06: Herencia y Polimorfismo
// OOP avanzado: extends, override, virtual
// ============================================

// --- CLASE BASE ---
class Animal:
    nombre = ""
    edad = 0
    
    def __init__(self, nombre, edad):
        self.nombre = nombre
        self.edad = edad
    
    virtual def hablar(self):
        print("...")
    
    virtual def mover(self):
        print("El animal se mueve")
    
    def info(self):
        print("Animal:")
        print(self.nombre)

// --- CLASES DERIVADAS ---
class Perro extends Animal:
    raza = ""
    
    def __init__(self, nombre, edad, raza):
        super().__init__(nombre, edad)
        self.raza = raza
    
    override def hablar(self):
        print("Guau guau!")
    
    override def mover(self):
        print("El perro corre")
    
    def ladrar(self):
        print("GUAU GUAU GUAU!")

class Gato extends Animal:
    color = ""
    
    override def hablar(self):
        print("Miau!")
    
    override def mover(self):
        print("El gato camina sigilosamente")
    
    def ronronear(self):
        print("Prrrrr...")

class Pajaro extends Animal:
    puede_volar = true
    
    override def hablar(self):
        print("Pio pio!")
    
    override def mover(self):
        if self.puede_volar:
            print("El pajaro vuela")
        else:
            print("El pajaro camina")

// --- TRAIT/INTERFACE ESTILO RUST ---
trait Mascota {
    fn alimentar(&self);
    fn jugar(&self);
    fn dormir(&self);
}

// Implementación de trait
impl Mascota for Perro {
    fn alimentar(&self) {
        print("Dando comida al perro")
    }
    
    fn jugar(&self) {
        print("Jugando con la pelota")
    }
    
    fn dormir(&self) {
        print("El perro duerme en su cama")
    }
}

// --- POLIMORFISMO EN ACCIÓN ---
fn hacer_hablar(animal: Animal) {
    // Llama al método override correspondiente
    animal.hablar()
}

fn main() {
    print("=== Herencia y Polimorfismo ===")
    
    // Crear instancias
    let perro_nombre = "Rex"
    let gato_nombre = "Michi"
    
    print("Perro:")
    print(perro_nombre)
    
    print("Gato:")
    print(gato_nombre)
    
    print("Polimorfismo demostrado!")
}
