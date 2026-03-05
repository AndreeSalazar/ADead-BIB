// ============================================================
// Canon C++98 — §9.4 Static Members
// ============================================================
// Intención: Un static member pertenece a la CLASE, no a
// una instancia. Existe una sola copia para todos los
// objetos. Static methods no reciben this*.
//
// ADead-BIB: static data → variable global con prefijo.
// Static method → función normal (sin this).
//
// NOTA: ADead-BIB no requiere definición separada de
// static members — los inicializa inline.
// ============================================================

int printf(const char *format, ...);

// --- Contador global (simula static member) ---
int entity_count = 0;

class Entity {
public:
    int id;

    Entity() {
        entity_count = entity_count + 1;
        id = entity_count;
    }

    int getId() { return id; }
};

// --- Config con globals (simula static-only class) ---
int config_debug = 0;
int config_log_level = 1;
int config_max_conn = 100;

void config_set_debug(int on) { config_debug = on; }
int config_is_debug() { return config_debug; }
void config_set_log(int level) { config_log_level = level; }
int config_get_log() { return config_log_level; }

// --- ID Generator ---
int next_gen_id = 0;

int id_generate() {
    next_gen_id = next_gen_id + 1;
    return next_gen_id;
}

void id_reset() {
    next_gen_id = 0;
}

int main() {
    printf("=== Canon C++98: Static Members ===\n\n");

    // --- Entity count ---
    printf("Entity count:\n");
    printf("  before: %d\n", entity_count);

    Entity e1;
    Entity e2;
    Entity e3;

    printf("  after 3 entities: %d\n", entity_count);
    printf("  e1.id = %d\n", e1.getId());
    printf("  e2.id = %d\n", e2.getId());
    printf("  e3.id = %d\n", e3.getId());

    // --- Config ---
    printf("\nConfig (static-only pattern):\n");
    printf("  debug: %d\n", config_is_debug());
    config_set_debug(1);
    printf("  debug after set: %d\n", config_is_debug());

    printf("  log level: %d\n", config_get_log());
    config_set_log(3);
    printf("  log level after set: %d\n", config_get_log());

    printf("  max connections: %d\n", config_max_conn);

    // --- IDGenerator ---
    printf("\nIDGenerator:\n");
    printf("  id: %d\n", id_generate());
    printf("  id: %d\n", id_generate());
    printf("  id: %d\n", id_generate());
    id_reset();
    printf("  after reset: %d\n", id_generate());

    // --- Verificación ---
    int pass = 0;
    int total = 0;

    total++; if (entity_count == 3)      { pass++; } else { printf("FAIL: count\n"); }
    total++; if (e1.getId() == 1)        { pass++; } else { printf("FAIL: e1.id\n"); }
    total++; if (e3.getId() == 3)        { pass++; } else { printf("FAIL: e3.id\n"); }
    total++; if (config_is_debug() == 1) { pass++; } else { printf("FAIL: debug\n"); }
    total++; if (config_get_log() == 3)  { pass++; } else { printf("FAIL: log level\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}
