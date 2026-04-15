// ============================================================
// Test 35: Production Completo — Integración total de todo C
// ============================================================
// ADead-BIB Test Canon — Test final de validación completa
// Combina: tipos, punteros, structs, fn ptrs, malloc, strings,
//          arrays, control flow, bitwise, recursión, algorithms
// ============================================================

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// ==================== TIPOS ====================
typedef unsigned char u8;
typedef unsigned int u32;
typedef int i32;

// ==================== STRUCTS ====================
struct Vec2 { int x, y; };
struct Entity {
    u32 id;
    char name[32];
    struct Vec2 pos;
    int hp;
    int (*update)(struct Entity *self);
};

// ==================== FUNCTION POINTERS ====================
typedef int (*Comparator)(const void *, const void *);
typedef void (*Visitor)(int value, void *user_data);

// ==================== ENTITY SYSTEM ====================
static u32 next_id = 0;

int entity_update_default(struct Entity *self) {
    self->pos.x++;
    self->pos.y++;
    return 0;
}

struct Entity *entity_create(const char *name, int hp) {
    struct Entity *e = (struct Entity *)malloc(sizeof(struct Entity));
    e->id = next_id++;
    strcpy(e->name, name);
    e->pos.x = 0;
    e->pos.y = 0;
    e->hp = hp;
    e->update = entity_update_default;
    return e;
}

// ==================== DYNAMIC ARRAY ====================
struct EntityArray {
    struct Entity **items;
    int count;
    int capacity;
};

struct EntityArray *earray_create() {
    struct EntityArray *ea = (struct EntityArray *)malloc(sizeof(struct EntityArray));
    ea->items = (struct Entity **)malloc(sizeof(struct Entity *) * 8);
    ea->count = 0;
    ea->capacity = 8;
    return ea;
}

void earray_push(struct EntityArray *ea, struct Entity *e) {
    if (ea->count >= ea->capacity) {
        ea->capacity *= 2;
        ea->items = (struct Entity **)realloc(ea->items,
                     sizeof(struct Entity *) * ea->capacity);
    }
    ea->items[ea->count++] = e;
}

void earray_free(struct EntityArray *ea) {
    int i;
    for (i = 0; i < ea->count; i++) {
        free(ea->items[i]);
    }
    free(ea->items);
    free(ea);
}

// ==================== STRING BUILDER ====================
struct StringBuilder {
    char *buffer;
    int length;
    int capacity;
};

struct StringBuilder *sb_create(int initial) {
    struct StringBuilder *sb = (struct StringBuilder *)malloc(sizeof(struct StringBuilder));
    sb->buffer = (char *)malloc(initial);
    sb->buffer[0] = '\0';
    sb->length = 0;
    sb->capacity = initial;
    return sb;
}

void sb_append(struct StringBuilder *sb, const char *str) {
    int slen = (int)strlen(str);
    while (sb->length + slen + 1 > sb->capacity) {
        sb->capacity *= 2;
        sb->buffer = (char *)realloc(sb->buffer, sb->capacity);
    }
    strcpy(sb->buffer + sb->length, str);
    sb->length += slen;
}

const char *sb_str(struct StringBuilder *sb) {
    return sb->buffer;
}

void sb_free(struct StringBuilder *sb) {
    free(sb->buffer);
    free(sb);
}

// ==================== TOKEN PARSER ====================
enum TokenType { TOK_NUMBER, TOK_PLUS, TOK_MINUS, TOK_STAR, TOK_END };

struct Token {
    enum TokenType type;
    int value;
};

struct Token tokenize_next(const char **input) {
    struct Token tok;
    while (**input == ' ') (*input)++;
    if (**input == '\0') { tok.type = TOK_END; tok.value = 0; return tok; }
    if (**input >= '0' && **input <= '9') {
        tok.type = TOK_NUMBER;
        tok.value = 0;
        while (**input >= '0' && **input <= '9') {
            tok.value = tok.value * 10 + (**input - '0');
            (*input)++;
        }
        return tok;
    }
    switch (**input) {
        case '+': tok.type = TOK_PLUS; break;
        case '-': tok.type = TOK_MINUS; break;
        case '*': tok.type = TOK_STAR; break;
        default:  tok.type = TOK_END; break;
    }
    tok.value = 0;
    (*input)++;
    return tok;
}

// ==================== HASH MAP (simple) ====================
#define MAP_SIZE 32

struct MapEntry {
    char key[32];
    int value;
    int occupied;
};

struct HashMap {
    struct MapEntry entries[MAP_SIZE];
};

void map_init(struct HashMap *m) {
    int i;
    for (i = 0; i < MAP_SIZE; i++) {
        m->entries[i].occupied = 0;
    }
}

unsigned int map_hash(const char *key) {
    unsigned int h = 5381;
    while (*key) {
        h = ((h << 5) + h) + (unsigned char)*key;
        key++;
    }
    return h % MAP_SIZE;
}

void map_set(struct HashMap *m, const char *key, int value) {
    unsigned int idx = map_hash(key);
    int tries = 0;
    while (m->entries[idx].occupied && strcmp(m->entries[idx].key, key) != 0) {
        idx = (idx + 1) % MAP_SIZE;
        if (++tries >= MAP_SIZE) return;
    }
    strcpy(m->entries[idx].key, key);
    m->entries[idx].value = value;
    m->entries[idx].occupied = 1;
}

int map_get(struct HashMap *m, const char *key, int *found) {
    unsigned int idx = map_hash(key);
    int tries = 0;
    while (m->entries[idx].occupied) {
        if (strcmp(m->entries[idx].key, key) == 0) {
            *found = 1;
            return m->entries[idx].value;
        }
        idx = (idx + 1) % MAP_SIZE;
        if (++tries >= MAP_SIZE) break;
    }
    *found = 0;
    return 0;
}

// ==================== MAIN ====================
int main() {
    printf("=== ADead-BIB Production Test ===\n\n");

    // --- Entity system ---
    struct EntityArray *entities = earray_create();
    earray_push(entities, entity_create("Warrior", 100));
    earray_push(entities, entity_create("Mage", 80));
    earray_push(entities, entity_create("Archer", 90));

    int i;
    for (i = 0; i < 3; i++) {
        entities->items[i]->update(entities->items[i]);
    }
    printf("Entities:\n");
    for (i = 0; i < entities->count; i++) {
        struct Entity *e = entities->items[i];
        printf("  [%d] %s hp=%d pos=(%d,%d)\n",
               e->id, e->name, e->hp, e->pos.x, e->pos.y);
    }

    // --- StringBuilder ---
    struct StringBuilder *sb = sb_create(16);
    sb_append(sb, "ADead");
    sb_append(sb, "-");
    sb_append(sb, "BIB");
    sb_append(sb, " v8.0");
    printf("\nStringBuilder: '%s' (len=%d)\n", sb_str(sb), sb->length);
    sb_free(sb);

    // --- Tokenizer ---
    const char *expr = "42 + 10 - 3 * 5";
    const char *pos = expr;
    printf("\nTokens from '%s':\n", expr);
    struct Token tok;
    do {
        tok = tokenize_next(&pos);
        switch (tok.type) {
            case TOK_NUMBER: printf("  NUM(%d)\n", tok.value); break;
            case TOK_PLUS:   printf("  PLUS\n"); break;
            case TOK_MINUS:  printf("  MINUS\n"); break;
            case TOK_STAR:   printf("  STAR\n"); break;
            case TOK_END:    printf("  END\n"); break;
        }
    } while (tok.type != TOK_END);

    // --- HashMap ---
    struct HashMap map;
    map_init(&map);
    map_set(&map, "health", 100);
    map_set(&map, "mana", 50);
    map_set(&map, "stamina", 75);
    map_set(&map, "gold", 1000);

    int found;
    int val = map_get(&map, "mana", &found);
    printf("\nHashMap: mana=%d found=%d\n", val, found);
    val = map_get(&map, "gold", &found);
    printf("HashMap: gold=%d found=%d\n", val, found);
    val = map_get(&map, "xp", &found);
    printf("HashMap: xp found=%d\n", found);

    // --- Bitwise flags ---
    u32 flags = 0;
    flags |= (1 << 0);
    flags |= (1 << 3);
    flags |= (1 << 7);
    printf("\nFlags: 0x%08X\n", flags);
    printf("Bit 0=%d Bit 1=%d Bit 3=%d Bit 7=%d\n",
           (flags >> 0) & 1, (flags >> 1) & 1, (flags >> 3) & 1, (flags >> 7) & 1);

    // --- Pointer chain verification ---
    int x = 42;
    int *p1 = &x;
    int **p2 = &p1;
    int ***p3 = &p2;
    printf("\nPointer chain: x=%d *p1=%d **p2=%d ***p3=%d\n",
           x, *p1, **p2, ***p3);
    ***p3 = 999;
    printf("After ***p3=999: x=%d\n", x);

    // --- Cleanup ---
    earray_free(entities);

    printf("\n=== ALL TESTS PASSED ===\n");
    return 0;
}
