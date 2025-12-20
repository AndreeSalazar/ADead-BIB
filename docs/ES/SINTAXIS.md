# 📝 Sintaxis ADead-BIB (Estilo Python)

## 🎯 Principios de Diseño

- ✅ **Estilo Python**: Similar pero no igual
- ✅ **Simple y legible**: Fácil de entender
- ✅ **Expresivo**: Permite código claro
- ✅ **Sin complejidad innecesaria**

---

## 📋 Elementos Básicos

### Comentarios
```adB
# Comentario de una línea
# Los comentarios empiezan con #
```

### Función Principal
```adB
def main():
    print("Hello, World!")
```

### Variables
```adB
# Asignación simple
x = 10
y = 20
result = x + y
```

### Tipos de Datos

#### Números
```adB
# Enteros
x = 42
y = -10

# Flotantes (futuro)
# x = 3.14
```

#### Strings
```adB
message = "Hello, World!"
name = "ADead-BIB"
```

#### Booleanos (futuro)
```adB
# flag = True
# flag = False
```

---

## 🔤 Operaciones

### Aritméticas
```adB
a = 10
b = 20
suma = a + b      # 30
resta = a - b     # -10
multiplicacion = a * b  # 200
division = b / a  # 2
```

### Comparaciones (futuro)
```adB
# igual = a == b
# diferente = a != b
# mayor = a > b
# menor = a < b
```

---

## 🏗️ Estructuras de Control

### Condicionales (futuro)
```adB
# if x > 10:
#     print("Mayor que 10")
# else:
#     print("Menor o igual")
```

### Bucles (futuro)
```adB
# while x > 0:
#     print(x)
#     x = x - 1
```

---

## 📞 Funciones

### Definición
```adB
def nombre_funcion():
    print("Hola")

def suma(a, b):
    return a + b
```

### Llamadas
```adB
print("Hello")
resultado = suma(10, 20)
```

---

## 🎯 Sintaxis para hello_world.adB

### Versión Simple (MVP)
```adB
def main():
    print("Hello, World!")
```

### Versión con Variables
```adB
def main():
    message = "Hello, World!"
    print(message)
```

---

## 📊 Comparación con Python

| Característica | Python | ADead-BIB | Notas |
|---------------|--------|-----------|-------|
| Indentación | ✅ Obligatoria | ✅ Obligatoria | Igual |
| `def` funciones | ✅ | ✅ | Igual |
| `print()` | ✅ | ✅ | Similar |
| Tipos explícitos | ❌ | ❌ (por ahora) | Igual |
| `:` después de `def` | ✅ | ✅ | Igual |
| Strings | `"..."` o `'...'` | `"..."` | Solo dobles por ahora |
| Comentarios | `#` | `#` | Igual |

---

## 🚀 Ejemplos

### Ejemplo 1: Hola Mundo
```adB
def main():
    print("Hello, World!")
```

### Ejemplo 2: Variables y Operaciones
```adB
def main():
    x = 10
    y = 20
    result = x + y
    print(result)
```

### Ejemplo 3: Múltiples Prints
```adB
def main():
    print("First line")
    print("Second line")
    print("Third line")
```

---

## ⚠️ Limitaciones Iniciales (MVP)

Para el primer paso (hello_world), solo soportamos:

✅ **Soportado:**
- `def main():`
- `print("string")`
- Variables simples: `x = value`
- Strings literales: `"..."`

❌ **No soportado (futuro):**
- Parámetros de función
- `return`
- Condicionales (`if`, `else`)
- Bucles (`while`, `for`)
- Operadores complejos
- Múltiples tipos

---

## 🎯 Gramática BNF (Simplificada para MVP)

```
program      ::= function_def
function_def ::= "def" identifier "()" ":" statement_list
statement_list ::= (statement NEWLINE)+
statement    ::= print_stmt | assign_stmt
print_stmt   ::= "print" "(" string_literal ")"
assign_stmt  ::= identifier "=" (number | string_literal)
identifier   ::= [a-zA-Z_][a-zA-Z0-9_]*
string_literal ::= '"' [^"]* '"'
number       ::= [0-9]+
```

---

**Nota**: Esta sintaxis evolucionará. Empezamos simple para hacer funcionar `hello_world.adB`.

