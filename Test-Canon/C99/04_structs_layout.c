// ============================================================
// Canon C99 — §6.7.2.1 Structs y Layout en Memoria
// ============================================================
// Intención: Un struct es un bloque de memoria con campos
// en orden secuencial. El compilador los posiciona
// contiguamente (con posible padding para alignment).
//
// C99 §6.7.2.1: "The members of a structure are allocated
// sequentially in the order in which they are declared."
//
// El programador accede con s.campo o p->campo (puntero).
// ============================================================

#include <stdio.h>

// --- Struct básico ---
struct Point {
    int x;
    int y;
};

// --- Struct con varios tipos ---
struct Player {
    int id;
    int health;
    int armor;
    int score;
};

// --- Struct nested ---
struct Rect {
    struct Point origin;
    struct Point size;
};

// --- Struct para linked list ---
struct Node {
    int value;
    struct Node *next;
};

// --- Funciones con structs ---

struct Point point_add(struct Point a, struct Point b) {
    struct Point result;
    result.x = a.x + b.x;
    result.y = a.y + b.y;
    return result;
}

int point_distance_sq(struct Point a, struct Point b) {
    int dx = a.x - b.x;
    int dy = a.y - b.y;
    return dx * dx + dy * dy;
}

int rect_area(struct Rect r) {
    return r.size.x * r.size.y;
}

void player_damage(struct Player *p, int dmg) {
    if (p->armor > 0) {
        int absorbed = dmg / 2;
        p->armor = p->armor - absorbed;
        if (p->armor < 0) p->armor = 0;
        dmg = dmg - absorbed;
    }
    p->health = p->health - dmg;
    if (p->health < 0) p->health = 0;
}

void player_heal(struct Player *p, int amount) {
    p->health = p->health + amount;
    if (p->health > 100) p->health = 100;
}

void player_print(struct Player *p) {
    printf("  Player #%d: HP=%d ARM=%d Score=%d\n",
        p->id, p->health, p->armor, p->score);
}

int main() {
    printf("=== Canon C99: Structs y Layout ===\n\n");

    // --- Point ---
    struct Point a;
    a.x = 3;
    a.y = 4;
    struct Point b;
    b.x = 7;
    b.y = 1;

    struct Point sum = point_add(a, b);
    printf("(%d,%d) + (%d,%d) = (%d,%d)\n", a.x, a.y, b.x, b.y, sum.x, sum.y);

    int dist = point_distance_sq(a, b);
    printf("distance² = %d\n", dist);

    // --- Rect (nested struct) ---
    struct Rect r;
    r.origin.x = 0;
    r.origin.y = 0;
    r.size.x = 10;
    r.size.y = 5;
    printf("\nRect: origin=(%d,%d) size=(%d,%d) area=%d\n",
        r.origin.x, r.origin.y, r.size.x, r.size.y, rect_area(r));

    // --- Player via puntero (-> operator) ---
    struct Player hero;
    hero.id = 1;
    hero.health = 100;
    hero.armor = 50;
    hero.score = 0;

    printf("\nPlayer:\n");
    player_print(&hero);

    player_damage(&hero, 40);
    printf("  Después de 40 dmg:\n");
    player_print(&hero);

    player_heal(&hero, 30);
    printf("  Después de heal 30:\n");
    player_print(&hero);

    hero.score = hero.score + 100;
    printf("  Score +100:\n");
    player_print(&hero);

    // --- Verificación ---
    int pass = 0;
    int total = 0;

    total++; if (sum.x == 10)       { pass++; } else { printf("FAIL: point add x\n"); }
    total++; if (sum.y == 5)        { pass++; } else { printf("FAIL: point add y\n"); }
    total++; if (dist == 25)        { pass++; } else { printf("FAIL: distance\n"); }
    total++; if (rect_area(r) == 50){ pass++; } else { printf("FAIL: rect area\n"); }
    total++; if (hero.health > 0)   { pass++; } else { printf("FAIL: player alive\n"); }
    total++; if (hero.score == 100) { pass++; } else { printf("FAIL: player score\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}
