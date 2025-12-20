# 📝 ADead-BIB Syntax Reference

> **Author:** Eddi Andreé Salazar Matos | **Made with ❤️ in Peru** 🇵🇪

---

## Basic Syntax

ADead-BIB uses Python-style syntax with some extensions for OOP.

### Hello World

```python
def main():
    print("Hello, World!")
```

### Variables

```python
def main():
    x = 10
    y = 20
    z = x + y
    print(z)
```

### Operations

```python
# Arithmetic
a = 10 + 5    # Addition
b = 10 - 5    # Subtraction
c = 10 * 5    # Multiplication
d = 10 / 5    # Division
e = 10 % 3    # Modulo

# Comparisons
x == y        # Equal
x != y        # Not equal
x < y         # Less than
x <= y        # Less or equal
x > y         # Greater than
x >= y        # Greater or equal
```

---

## Control Flow

### If/Elif/Else

```python
def main():
    x = 10
    
    if x > 5:
        print("Greater than 5")
    elif x == 5:
        print("Equal to 5")
    else:
        print("Less than 5")
```

### While Loop

```python
def main():
    i = 0
    while i < 5:
        print(i)
        i = i + 1
```

### For Loop

```python
def main():
    for i in range(5):
        print(i)
```

---

## Functions

```python
def add(a, b):
    return a + b

def main():
    result = add(10, 20)
    print(result)
```

---

## Object-Oriented Programming

### Classes

```python
class Entity:
    x = 0
    y = 0
    
    def move(self, dx, dy):
        self.x = self.x + dx
        self.y = self.y + dy
```

### Inheritance

```python
class Player extends Entity:
    health = 100
    
    def take_damage(self, amount):
        self.health = self.health - amount
```

### Virtual Methods

```python
class Entity:
    virtual def update(self):
        pass

class Player extends Entity:
    override def update(self):
        print("Player updating")
```

---

## Built-in Functions (70+)

### Math Functions

| Function | Description | Example |
|----------|-------------|---------|
| `abs(x)` | Absolute value | `abs(-5)` → 5 |
| `min(a, b)` | Minimum | `min(3, 7)` → 3 |
| `max(a, b)` | Maximum | `max(3, 7)` → 7 |
| `pow(x, n)` | Power | `pow(2, 3)` → 8 |
| `sqrt(x)` | Square root | `sqrt(16)` → 4 |
| `sqr(x)` | Square | `sqr(5)` → 25 |
| `cube(x)` | Cube | `cube(3)` → 27 |
| `factorial(n)` | Factorial | `factorial(5)` → 120 |
| `fib(n)` | Fibonacci | `fib(10)` → 55 |
| `gcd(a, b)` | GCD | `gcd(12, 8)` → 4 |

### AI/Matrix Functions

| Function | Description | Example |
|----------|-------------|---------|
| `dot(a,b,c,d)` | Dot product | `dot(2,3,4,5)` → 26 |
| `sum_sq(a,b,...)` | Sum of squares | `sum_sq(3,4)` → 25 |
| `norm_sq(a,b,...)` | Norm squared | `norm_sq(3,4)` → 25 |
| `weighted_sum(v,w,...)` | Weighted sum | `weighted_sum(10,2,20,3)` → 80 |
| `relu(x)` | ReLU activation | `relu(-3)` → 0 |
| `scale(x, f)` | Scale x*f/100 | `scale(200,50)` → 100 |
| `lerp(a, b, t)` | Linear interpolation | `lerp(0,100,50)` → 50 |

### Utility Functions

| Function | Description | Example |
|----------|-------------|---------|
| `inc(x)` | Increment | `inc(5)` → 6 |
| `dec(x)` | Decrement | `dec(5)` → 4 |
| `double(x)` | Double | `double(5)` → 10 |
| `half(x)` | Half | `half(10)` → 5 |
| `sign(x)` | Sign (-1, 0, 1) | `sign(-5)` → -1 |
| `clamp(x, min, max)` | Clamp value | `clamp(15, 0, 10)` → 10 |
| `between(x, a, b)` | Check range | `between(5, 0, 10)` → 1 |

---

## Comments

```python
# This is a single-line comment

def main():
    # Comments can be anywhere
    print("Hello")  # Even here
```

---

## Example: Complete Program

```python
# A simple game entity system
# Author: Eddi Andreé Salazar Matos

class Entity:
    x = 0
    y = 0
    
    virtual def update(self):
        pass
    
    def move(self, dx, dy):
        self.x = self.x + dx
        self.y = self.y + dy

class Player extends Entity:
    health = 100
    speed = 5
    
    override def update(self):
        print("Player at:")
        print(self.x)
        print(self.y)

def main():
    print("Game Starting!")
    
    # Using built-in functions
    damage = max(10, 5)
    print("Damage:")
    print(damage)
    
    # AI functions
    similarity = dot(1, 2, 3, 4)
    print("Similarity:")
    print(similarity)
    
    print("Game Over!")
```

---

**Created by Eddi Andreé Salazar Matos** | eddi.salazar.dev@gmail.com | 🇵🇪
