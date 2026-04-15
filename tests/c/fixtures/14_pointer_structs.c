// ============================================================
// Test 14: Punteros a Structs — ->, malloc struct, linked structures
// ============================================================
// ADead-BIB Test Canon — C99 §6.5.2.3
// Verifica: operador ->, struct en heap, struct anidados via ptr
// ============================================================

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct Point {
    int x;
    int y;
};

struct Player {
    char name[32];
    int hp;
    int attack;
    struct Point position;
};

struct Node {
    int value;
    struct Node *next;
};

struct Point *create_point(int x, int y) {
    struct Point *p = (struct Point *)malloc(sizeof(struct Point));
    p->x = x;
    p->y = y;
    return p;
}

void move_point(struct Point *p, int dx, int dy) {
    p->x += dx;
    p->y += dy;
}

int distance_sq(const struct Point *a, const struct Point *b) {
    int dx = a->x - b->x;
    int dy = a->y - b->y;
    return dx * dx + dy * dy;
}

struct Player *create_player(const char *name, int hp, int attack) {
    struct Player *p = (struct Player *)malloc(sizeof(struct Player));
    strcpy(p->name, name);
    p->hp = hp;
    p->attack = attack;
    p->position.x = 0;
    p->position.y = 0;
    return p;
}

void player_move(struct Player *p, int dx, int dy) {
    p->position.x += dx;
    p->position.y += dy;
}

int player_is_alive(const struct Player *p) {
    return p->hp > 0;
}

void player_take_damage(struct Player *p, int dmg) {
    p->hp -= dmg;
    if (p->hp < 0) p->hp = 0;
}

// --- Linked list ---
struct Node *node_create(int value) {
    struct Node *n = (struct Node *)malloc(sizeof(struct Node));
    n->value = value;
    n->next = (struct Node *)0;
    return n;
}

void list_push_front(struct Node **head, int value) {
    struct Node *n = node_create(value);
    n->next = *head;
    *head = n;
}

int list_length(const struct Node *head) {
    int len = 0;
    while (head) {
        len++;
        head = head->next;
    }
    return len;
}

int list_sum(const struct Node *head) {
    int sum = 0;
    while (head) {
        sum += head->value;
        head = head->next;
    }
    return sum;
}

void list_free(struct Node *head) {
    while (head) {
        struct Node *next = head->next;
        free(head);
        head = next;
    }
}

int main() {
    // --- Point en heap ---
    struct Point *p1 = create_point(10, 20);
    struct Point *p2 = create_point(40, 60);
    printf("p1=(%d,%d)\n", p1->x, p1->y);
    printf("p2=(%d,%d)\n", p2->x, p2->y);

    move_point(p1, 5, -3);
    printf("moved p1=(%d,%d)\n", p1->x, p1->y);

    int dist = distance_sq(p1, p2);
    printf("dist_sq=%d\n", dist);

    free(p1);
    free(p2);

    // --- Player con struct anidado ---
    struct Player *hero = create_player("Warrior", 100, 25);
    printf("player=%s hp=%d atk=%d pos=(%d,%d)\n",
           hero->name, hero->hp, hero->attack,
           hero->position.x, hero->position.y);

    player_move(hero, 10, 5);
    printf("moved: pos=(%d,%d)\n", hero->position.x, hero->position.y);

    player_take_damage(hero, 30);
    printf("after damage: hp=%d alive=%d\n", hero->hp, player_is_alive(hero));

    player_take_damage(hero, 200);
    printf("after lethal: hp=%d alive=%d\n", hero->hp, player_is_alive(hero));

    free(hero);

    // --- Linked list ---
    struct Node *list = (struct Node *)0;
    list_push_front(&list, 10);
    list_push_front(&list, 20);
    list_push_front(&list, 30);
    list_push_front(&list, 40);
    list_push_front(&list, 50);

    printf("list len=%d sum=%d\n", list_length(list), list_sum(list));

    // --- Imprimir lista ---
    struct Node *cur = list;
    printf("list: ");
    while (cur) {
        printf("%d ", cur->value);
        cur = cur->next;
    }
    printf("\n");

    list_free(list);

    // --- Array de punteros a struct ---
    struct Point *pts[3];
    pts[0] = create_point(1, 2);
    pts[1] = create_point(3, 4);
    pts[2] = create_point(5, 6);

    int i;
    int total_x = 0;
    for (i = 0; i < 3; i++) {
        total_x += pts[i]->x;
        free(pts[i]);
    }
    printf("total_x=%d\n", total_x);

    return 0;
}
// Expected:
// p1=(10,20)
// p2=(40,60)
// moved p1=(15,17)
// player=Warrior hp=100 atk=25 pos=(0,0)
// moved: pos=(10,5)
// after damage: hp=70 alive=1
// after lethal: hp=0 alive=0
// list len=5 sum=150
// list: 50 40 30 20 10
// total_x=9
