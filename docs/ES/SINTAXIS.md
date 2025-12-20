# 📝 Referencia de Sintaxis ADead-BIB

> **Autor:** Eddi Andreé Salazar Matos | **Hecho con ❤️ en Perú** 🇵🇪

---

## Sintaxis Básica

ADead-BIB usa sintaxis estilo Python con extensiones para OOP.

### Hola Mundo

```python
def main():
    print("Hola, Mundo!")
```

### Variables

```python
def main():
    x = 10
    y = 20
    z = x + y
    print(z)
```

### Operaciones

```python
# Aritméticas
a = 10 + 5    # Suma
b = 10 - 5    # Resta
c = 10 * 5    # Multiplicación
d = 10 / 5    # División
e = 10 % 3    # Módulo

# Comparaciones
x == y        # Igual
x != y        # Diferente
x < y         # Menor que
x <= y        # Menor o igual
x > y         # Mayor que
x >= y        # Mayor o igual
```

---

## Control de Flujo

### If/Elif/Else

```python
def main():
    x = 10
    
    if x > 5:
        print("Mayor que 5")
    elif x == 5:
        print("Igual a 5")
    else:
        print("Menor que 5")
```

### Bucle While

```python
def main():
    i = 0
    while i < 5:
        print(i)
        i = i + 1
```

### Bucle For

```python
def main():
    for i in range(5):
        print(i)
```

---

## Funciones

```python
def sumar(a, b):
    return a + b

def main():
    resultado = sumar(10, 20)
    print(resultado)
```

---

## Programación Orientada a Objetos

### Clases

```python
class Entidad:
    x = 0
    y = 0
    
    def mover(self, dx, dy):
        self.x = self.x + dx
        self.y = self.y + dy
```

### Herencia

```python
class Jugador extends Entidad:
    salud = 100
    
    def recibir_dano(self, cantidad):
        self.salud = self.salud - cantidad
```

### Métodos Virtuales

```python
class Entidad:
    virtual def actualizar(self):
        pass

class Jugador extends Entidad:
    override def actualizar(self):
        print("Jugador actualizando")
```

---

## Funciones Incorporadas (70+)

### Funciones Matemáticas

| Función | Descripción | Ejemplo |
|---------|-------------|---------|
| `abs(x)` | Valor absoluto | `abs(-5)` → 5 |
| `min(a, b)` | Mínimo | `min(3, 7)` → 3 |
| `max(a, b)` | Máximo | `max(3, 7)` → 7 |
| `pow(x, n)` | Potencia | `pow(2, 3)` → 8 |
| `sqrt(x)` | Raíz cuadrada | `sqrt(16)` → 4 |
| `sqr(x)` | Cuadrado | `sqr(5)` → 25 |
| `cube(x)` | Cubo | `cube(3)` → 27 |
| `factorial(n)` | Factorial | `factorial(5)` → 120 |
| `fib(n)` | Fibonacci | `fib(10)` → 55 |
| `gcd(a, b)` | MCD | `gcd(12, 8)` → 4 |

### Funciones de IA/Matrices

| Función | Descripción | Ejemplo |
|---------|-------------|---------|
| `dot(a,b,c,d)` | Producto punto | `dot(2,3,4,5)` → 26 |
| `sum_sq(a,b,...)` | Suma de cuadrados | `sum_sq(3,4)` → 25 |
| `norm_sq(a,b,...)` | Norma al cuadrado | `norm_sq(3,4)` → 25 |
| `weighted_sum(v,w,...)` | Suma ponderada | `weighted_sum(10,2,20,3)` → 80 |
| `relu(x)` | Activación ReLU | `relu(-3)` → 0 |
| `scale(x, f)` | Escalar x*f/100 | `scale(200,50)` → 100 |
| `lerp(a, b, t)` | Interpolación lineal | `lerp(0,100,50)` → 50 |

### Funciones de Utilidad

| Función | Descripción | Ejemplo |
|---------|-------------|---------|
| `inc(x)` | Incrementar | `inc(5)` → 6 |
| `dec(x)` | Decrementar | `dec(5)` → 4 |
| `double(x)` | Duplicar | `double(5)` → 10 |
| `half(x)` | Mitad | `half(10)` → 5 |
| `sign(x)` | Signo (-1, 0, 1) | `sign(-5)` → -1 |
| `clamp(x, min, max)` | Limitar valor | `clamp(15, 0, 10)` → 10 |
| `between(x, a, b)` | Verificar rango | `between(5, 0, 10)` → 1 |

---

## Comentarios

```python
# Este es un comentario de una línea

def main():
    # Los comentarios pueden estar en cualquier lugar
    print("Hola")  # Incluso aquí
```

---

## Ejemplo: Programa Completo

```python
# Un sistema simple de entidades de juego
# Autor: Eddi Andreé Salazar Matos

class Entidad:
    x = 0
    y = 0
    
    virtual def actualizar(self):
        pass
    
    def mover(self, dx, dy):
        self.x = self.x + dx
        self.y = self.y + dy

class Jugador extends Entidad:
    salud = 100
    velocidad = 5
    
    override def actualizar(self):
        print("Jugador en:")
        print(self.x)
        print(self.y)

def main():
    print("Juego Iniciando!")
    
    # Usando funciones incorporadas
    dano = max(10, 5)
    print("Daño:")
    print(dano)
    
    # Funciones de IA
    similitud = dot(1, 2, 3, 4)
    print("Similitud:")
    print(similitud)
    
    print("Juego Terminado!")
```

---

**Creado por Eddi Andreé Salazar Matos** | eddi.salazar.dev@gmail.com | 🇵🇪
