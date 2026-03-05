// ============================================================
// Canon C++98 — Punteros a Objetos
// ============================================================
// Intención: Un puntero a objeto es lo mismo que un puntero
// a struct en C. El operador -> accede a miembros a través
// del puntero. Es exactamente [base + offset].
//
// obj.method()   → call method con &obj como this
// ptr->method()  → call method con ptr como this
//
// ADead-BIB: -> es lo mismo que (*ptr).member
// ============================================================

int printf(const char *format, ...);

// --- Clase simple ---
class Vec2 {
public:
    int x;
    int y;

    Vec2() : x(0), y(0) {}
    Vec2(int x, int y) : x(x), y(y) {}

    int length_sq() { return x * x + y * y; }

    Vec2 add(Vec2 other) {
        return Vec2(x + other.x, y + other.y);
    }
};

void vec2_scale(Vec2 *v, int factor) {
    v->x = v->x * factor;
    v->y = v->y * factor;
}

void vec2_translate(Vec2 *v, int dx, int dy) {
    v->x = v->x + dx;
    v->y = v->y + dy;
}

void vec2_reset(Vec2 *v) {
    v->x = 0;
    v->y = 0;
}

// --- Stack-based container ---
class IntStack {
public:
    int data[16];
    int top;

    IntStack() : top(0) {}

    void push(int val) {
        if (top < 16) {
            data[top] = val;
            top = top + 1;
        }
    }

    int pop() {
        if (top > 0) {
            top = top - 1;
            return data[top];
        }
        return -1;
    }

    int peek() {
        if (top > 0) return data[top - 1];
        return -1;
    }

    int size() { return top; }
    int empty() { return top == 0; }
};

// --- Función que recibe puntero a objeto ---
void stack_push_range(IntStack *s, int start, int count) {
    int i;
    for (i = 0; i < count; i++) {
        s->push(start + i);
    }
}

int stack_sum(IntStack *s) {
    int total = 0;
    int i;
    for (i = 0; i < s->top; i++) {
        total = total + s->data[i];
    }
    return total;
}

int main() {
    printf("=== Canon C++98: Punteros a Objetos ===\n\n");

    // --- Pointer to stack object ---
    printf("Vec2 via pointer:\n");
    Vec2 v(3, 4);
    Vec2 *ptr = &v;
    printf("  via ptr: (%d, %d)\n", ptr->x, ptr->y);
    printf("  length_sq = %d\n", ptr->length_sq());

    vec2_scale(ptr, 2);
    printf("  after scale(2): (%d, %d)\n", v.x, v.y);
    printf("  length_sq = %d\n", v.length_sq());

    vec2_translate(&v, 10, 20);
    printf("  after translate(+10,+20): (%d, %d)\n", v.x, v.y);

    vec2_reset(&v);
    printf("  after reset: (%d, %d)\n", v.x, v.y);

    // --- IntStack via pointer ---
    printf("\nIntStack via pointer:\n");
    IntStack s;
    IntStack *sp = &s;

    stack_push_range(sp, 10, 5);
    printf("  pushed 10..14\n");
    printf("  size: %d\n", sp->size());
    printf("  peek: %d\n", sp->peek());
    printf("  sum: %d\n", stack_sum(sp));

    int popped = sp->pop();
    printf("  pop: %d\n", popped);
    printf("  size after pop: %d\n", sp->size());

    // --- Array of objects ---
    printf("\nArray of Vec2:\n");
    Vec2 points[3];
    points[0] = Vec2(1, 0);
    points[1] = Vec2(0, 1);
    points[2] = Vec2(1, 1);
    int i;
    for (i = 0; i < 3; i++) {
        printf("  point[%d] = (%d,%d) len_sq=%d\n",
            i, points[i].x, points[i].y, points[i].length_sq());
    }

    // --- Vec2 add ---
    Vec2 a(10, 20);
    Vec2 b(30, 40);
    Vec2 sum = a.add(b);
    printf("\n(%d,%d) + (%d,%d) = (%d,%d)\n", a.x, a.y, b.x, b.y, sum.x, sum.y);

    // --- Verificación ---
    int pass = 0;
    int total = 0;

    total++; if (v.x == 0 && v.y == 0)   { pass++; } else { printf("FAIL: reset\n"); }
    total++; if (popped == 14)           { pass++; } else { printf("FAIL: pop\n"); }
    total++; if (sp->size() == 4)        { pass++; } else { printf("FAIL: size\n"); }
    total++; if (stack_sum(sp) == 46)    { pass++; } else { printf("FAIL: sum\n"); }
    total++; if (points[2].length_sq()==2){ pass++; } else { printf("FAIL: point\n"); }
    total++; if (sum.x == 40 && sum.y == 60) { pass++; } else { printf("FAIL: add\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}
