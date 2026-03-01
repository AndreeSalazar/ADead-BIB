// ============================================================
// ADead-BIB C Example — Structs, Enums, Typedef
// ============================================================
// Demuestra tipos compuestos compilados por ADead-BIB.
// ============================================================

#include <stdio.h>
#include <stdlib.h>

// ==================== Enums ====================

enum TokenType {
    TOKEN_INT     = 0,
    TOKEN_FLOAT   = 1,
    TOKEN_STRING  = 2,
    TOKEN_IDENT   = 3,
    TOKEN_EOF     = 4
};

enum Color {
    COLOR_RED,
    COLOR_GREEN,
    COLOR_BLUE,
    COLOR_ALPHA
};

// ==================== Structs ====================

struct Vec2 {
    float x;
    float y;
};

struct Vec3 {
    float x;
    float y;
    float z;
};

struct RGBA {
    unsigned char r;
    unsigned char g;
    unsigned char b;
    unsigned char a;
};

struct Player {
    int id;
    int health;
    int armor;
    int score;
    struct Vec3 position;
};

struct Node {
    int value;
    struct Node *next;
};

// ==================== Typedef ====================

typedef struct Vec2 Vec2;
typedef struct Vec3 Vec3;
typedef struct RGBA RGBA;
typedef unsigned int u32;
typedef unsigned long long u64;

// ==================== Funciones ====================

float vec2_dot(Vec2 a, Vec2 b) {
    return a.x * b.x + a.y * b.y;
}

Vec3 vec3_add(Vec3 a, Vec3 b) {
    Vec3 result;
    result.x = a.x + b.x;
    result.y = a.y + b.y;
    result.z = a.z + b.z;
    return result;
}

Vec3 vec3_scale(Vec3 v, float s) {
    Vec3 result;
    result.x = v.x * s;
    result.y = v.y * s;
    result.z = v.z * s;
    return result;
}

RGBA rgba_mix(RGBA a, RGBA b, float t) {
    RGBA result;
    float inv_t = 1.0 - t;
    result.r = (unsigned char)(a.r * inv_t + b.r * t);
    result.g = (unsigned char)(a.g * inv_t + b.g * t);
    result.b = (unsigned char)(a.b * inv_t + b.b * t);
    result.a = (unsigned char)(a.a * inv_t + b.a * t);
    return result;
}

const char *token_name(enum TokenType t) {
    switch (t) {
        case TOKEN_INT:    return "INT";
        case TOKEN_FLOAT:  return "FLOAT";
        case TOKEN_STRING: return "STRING";
        case TOKEN_IDENT:  return "IDENT";
        case TOKEN_EOF:    return "EOF";
        default:           return "UNKNOWN";
    }
}

void print_player(struct Player *p) {
    printf("Player #%d:\n", p->id);
    printf("  HP: %d  Armor: %d  Score: %d\n", p->health, p->armor, p->score);
    printf("  Pos: (%.1f, %.1f, %.1f)\n", p->position.x, p->position.y, p->position.z);
}

// ==================== Linked List ====================

struct Node *node_create(int value) {
    struct Node *n = malloc(sizeof(struct Node));
    if (n != NULL) {
        n->value = value;
        n->next = NULL;
    }
    return n;
}

void list_push(struct Node **head, int value) {
    struct Node *n = node_create(value);
    n->next = *head;
    *head = n;
}

void list_print(struct Node *head) {
    printf("  List: ");
    struct Node *cur = head;
    while (cur != NULL) {
        printf("%d", cur->value);
        if (cur->next != NULL) printf(" -> ");
        cur = cur->next;
    }
    printf("\n");
}

void list_free(struct Node *head) {
    while (head != NULL) {
        struct Node *next = head->next;
        free(head);
        head = next;
    }
}

int list_length(struct Node *head) {
    int len = 0;
    while (head != NULL) {
        len++;
        head = head->next;
    }
    return len;
}

// ==================== Main ====================

int main() {
    printf("=== ADead-BIB: Structs & Enums ===\n\n");

    // Enums
    printf("Token types:\n");
    for (int i = TOKEN_INT; i <= TOKEN_EOF; i++) {
        printf("  [%d] %s\n", i, token_name(i));
    }

    // Vec2
    printf("\nVec2:\n");
    Vec2 a;
    a.x = 3.0;
    a.y = 4.0;
    Vec2 b;
    b.x = 1.0;
    b.y = 2.0;
    printf("  a = (%.1f, %.1f)\n", a.x, a.y);
    printf("  b = (%.1f, %.1f)\n", b.x, b.y);
    printf("  dot = %.1f\n", vec2_dot(a, b));

    // Player
    printf("\nPlayer:\n");
    struct Player hero;
    hero.id = 1;
    hero.health = 100;
    hero.armor = 50;
    hero.score = 9001;
    hero.position.x = 10.5;
    hero.position.y = 0.0;
    hero.position.z = -3.2;
    print_player(&hero);

    // RGBA
    printf("\nColors:\n");
    RGBA red;
    red.r = 255; red.g = 0; red.b = 0; red.a = 255;
    RGBA blue;
    blue.r = 0; blue.g = 0; blue.b = 255; blue.a = 255;
    RGBA mixed = rgba_mix(red, blue, 0.5);
    printf("  Red:   (%d, %d, %d, %d)\n", red.r, red.g, red.b, red.a);
    printf("  Blue:  (%d, %d, %d, %d)\n", blue.r, blue.g, blue.b, blue.a);
    printf("  Mixed: (%d, %d, %d, %d)\n", mixed.r, mixed.g, mixed.b, mixed.a);

    // Linked list
    printf("\nLinked List:\n");
    struct Node *list = NULL;
    list_push(&list, 10);
    list_push(&list, 20);
    list_push(&list, 30);
    list_push(&list, 40);
    list_print(list);
    printf("  Length: %d\n", list_length(list));
    list_free(list);
    printf("  Freed OK\n");

    printf("\n=== Complete ===\n");
    return 0;
}
