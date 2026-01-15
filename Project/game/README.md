# ADead-BIB - Juego Simple Funcional

> Este juego **COMPILA DE VERDAD** con el backend actual de ADead-BIB.

## Ejecutar

```bash
cd Project/game
adeadc run main.adB
```

O desde la raíz del proyecto:

```bash
cargo run --bin adeadc -- run Project/game/main.adB
```

## Qué hace el juego

1. **Inicializa** el estado del jugador (HP, nivel, oro)
2. **Mueve** al jugador en el mapa
3. **Calcula** distancia al enemigo
4. **Combate** con el enemigo (2 ataques)
5. **Victoria** y recompensa de oro
6. **Sube de nivel**
7. **Muestra** estado final

## Salida esperada

```
========================================
     ADead-BIB - Aventura Simple
     Binary Is Binary
========================================

=== Estado Inicial ===
Jugador HP: 100
Jugador Nivel: 1
Oro: 0

=== Moviendo al jugador ===
Nueva posicion: (5, 3)

=== Calculando distancia al enemigo ===
Distancia^2: 29

=== Combate! ===
Atacas al enemigo por 25 de dano!
Enemigo HP restante: 25

Atacas de nuevo por 25 de dano!
Enemigo HP restante: 0

=== Victoria! ===
Ganaste 50 de oro!
Oro total: 50

Subiste al nivel 2

=== Estado Final ===
Jugador HP: 100
Jugador Nivel: 2
Oro: 50

========================================
     Gracias por jugar!
     ADead-BIB v2.0
========================================
```

## Filosofía

Este juego demuestra que ADead-BIB puede:
- Compilar código real a binario nativo
- Ejecutar lógica de juego
- Manejar variables y operaciones
- Producir salida visible

**Sin ASM. Sin LLVM. Sin mentiras.**
