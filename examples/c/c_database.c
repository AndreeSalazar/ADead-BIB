// ============================================================
// ADead-BIB C Example — Database (SQLite3 + LevelDB)
// ============================================================
// In-memory databases, key-value patterns, SQL parsing —
// everything for FastOS storage compiled by ADead-BIB.
// ============================================================

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sqlite3.h>
#include <leveldb/c.h>

// ==================== In-Memory Key-Value Store ====================

struct KVEntry {
    char key[64];
    char value[256];
    int occupied;
};

struct KVStore {
    struct KVEntry *entries;
    int capacity;
    int size;
};

unsigned int kv_hash(const char *key, int capacity) {
    unsigned int hash = 5381;
    while (*key) {
        hash = ((hash << 5) + hash) + (unsigned char)*key;
        key++;
    }
    return hash % capacity;
}

struct KVStore *kv_create(int capacity) {
    struct KVStore *store = malloc(sizeof(struct KVStore));
    store->entries = malloc(capacity * sizeof(struct KVEntry));
    store->capacity = capacity;
    store->size = 0;
    for (int i = 0; i < capacity; i++) store->entries[i].occupied = 0;
    return store;
}

int kv_put(struct KVStore *store, const char *key, const char *value) {
    unsigned int idx = kv_hash(key, store->capacity);
    int probes = 0;
    while (store->entries[idx].occupied && strcmp(store->entries[idx].key, key) != 0) {
        idx = (idx + 1) % store->capacity;
        probes++;
        if (probes >= store->capacity) return 0;
    }
    if (!store->entries[idx].occupied) store->size++;
    strcpy(store->entries[idx].key, key);
    strcpy(store->entries[idx].value, value);
    store->entries[idx].occupied = 1;
    return 1;
}

const char *kv_get(struct KVStore *store, const char *key) {
    unsigned int idx = kv_hash(key, store->capacity);
    int probes = 0;
    while (store->entries[idx].occupied) {
        if (strcmp(store->entries[idx].key, key) == 0) {
            return store->entries[idx].value;
        }
        idx = (idx + 1) % store->capacity;
        probes++;
        if (probes >= store->capacity) break;
    }
    return NULL;
}

int kv_delete(struct KVStore *store, const char *key) {
    unsigned int idx = kv_hash(key, store->capacity);
    int probes = 0;
    while (store->entries[idx].occupied) {
        if (strcmp(store->entries[idx].key, key) == 0) {
            store->entries[idx].occupied = 0;
            store->size--;
            return 1;
        }
        idx = (idx + 1) % store->capacity;
        probes++;
        if (probes >= store->capacity) break;
    }
    return 0;
}

void kv_free(struct KVStore *store) {
    free(store->entries);
    free(store);
}

// ==================== B-Tree Node (simplified) ====================

struct BTreeNode {
    int keys[4];
    int num_keys;
    int is_leaf;
};

struct BTreeNode *btree_create_node(int is_leaf) {
    struct BTreeNode *node = malloc(sizeof(struct BTreeNode));
    node->num_keys = 0;
    node->is_leaf = is_leaf;
    for (int i = 0; i < 4; i++) node->keys[i] = 0;
    return node;
}

void btree_insert_sorted(struct BTreeNode *node, int key) {
    if (node->num_keys >= 4) return;
    int i = node->num_keys - 1;
    while (i >= 0 && node->keys[i] > key) {
        node->keys[i + 1] = node->keys[i];
        i--;
    }
    node->keys[i + 1] = key;
    node->num_keys++;
}

int btree_search(struct BTreeNode *node, int key) {
    for (int i = 0; i < node->num_keys; i++) {
        if (node->keys[i] == key) return i;
    }
    return -1;
}

void btree_print(struct BTreeNode *node) {
    printf("  [");
    for (int i = 0; i < node->num_keys; i++) {
        if (i > 0) printf(", ");
        printf("%d", node->keys[i]);
    }
    printf("] (n=%d, leaf=%d)\n", node->num_keys, node->is_leaf);
}

// ==================== SQL-like Query Parser ====================

struct SQLQuery {
    char operation[16];
    char table[64];
    char columns[256];
    char where_clause[256];
    int limit;
};

int sql_parse_simple(const char *query, struct SQLQuery *result) {
    memset(result, 0, sizeof(struct SQLQuery));
    result->limit = -1;

    if (strncmp(query, "SELECT ", 7) == 0) {
        strcpy(result->operation, "SELECT");
        const char *p = query + 7;
        // Read columns until FROM
        int ci = 0;
        while (*p && strncmp(p, " FROM ", 6) != 0 && ci < 255) {
            result->columns[ci++] = *p++;
        }
        result->columns[ci] = '\0';
        if (strncmp(p, " FROM ", 6) == 0) {
            p += 6;
            int ti = 0;
            while (*p && *p != ' ' && *p != ';' && ti < 63) {
                result->table[ti++] = *p++;
            }
            result->table[ti] = '\0';
        }
        return 1;
    }
    if (strncmp(query, "INSERT ", 7) == 0) {
        strcpy(result->operation, "INSERT");
        return 1;
    }
    if (strncmp(query, "DELETE ", 7) == 0) {
        strcpy(result->operation, "DELETE");
        return 1;
    }
    return 0;
}

// ==================== Main ====================

int main() {
    printf("=== ADead-BIB: Database Systems ===\n\n");

    // KV Store
    printf("Key-Value Store:\n");
    struct KVStore *store = kv_create(64);
    kv_put(store, "name", "ADead-BIB");
    kv_put(store, "version", "1.2.0");
    kv_put(store, "target", "FastOS");
    kv_put(store, "compiler", "adeadc");
    kv_put(store, "language", "C");

    printf("  name    = %s\n", kv_get(store, "name"));
    printf("  version = %s\n", kv_get(store, "version"));
    printf("  target  = %s\n", kv_get(store, "target"));
    printf("  size    = %d\n", store->size);

    kv_delete(store, "language");
    const char *deleted = kv_get(store, "language");
    printf("  After delete 'language': %s\n", deleted != NULL ? deleted : "(null)");
    printf("  size    = %d\n", store->size);
    kv_free(store);

    // B-Tree
    printf("\nB-Tree Node:\n");
    struct BTreeNode *node = btree_create_node(1);
    btree_insert_sorted(node, 30);
    btree_insert_sorted(node, 10);
    btree_insert_sorted(node, 20);
    btree_insert_sorted(node, 40);
    btree_print(node);
    printf("  search(20) = index %d\n", btree_search(node, 20));
    printf("  search(99) = index %d (not found)\n", btree_search(node, 99));
    free(node);

    // SQL parser
    printf("\nSQL Query Parser:\n");
    const char *queries[] = {
        "SELECT * FROM users",
        "SELECT name, age FROM employees",
        "INSERT INTO products",
        "DELETE FROM sessions"
    };
    for (int i = 0; i < 4; i++) {
        struct SQLQuery q;
        if (sql_parse_simple(queries[i], &q)) {
            printf("  \"%s\"\n", queries[i]);
            printf("    op=%s table=%s cols=%s\n", q.operation, q.table, q.columns);
        }
    }

    // SQLite3 type validation
    printf("\nSQLite3 API (types validated):\n");
    printf("  sqlite3 ptr size: %lu\n", (unsigned long)sizeof(sqlite3 *));
    printf("  sqlite3_stmt ptr size: %lu\n", (unsigned long)sizeof(sqlite3_stmt *));
    printf("  sqlite3_int64 size: %lu\n", (unsigned long)sizeof(sqlite3_int64));

    // LevelDB type validation
    printf("\nLevelDB API (types validated):\n");
    printf("  leveldb_t ptr size: %lu\n", (unsigned long)sizeof(leveldb_t *));
    printf("  leveldb_options_t ptr size: %lu\n", (unsigned long)sizeof(leveldb_options_t *));

    printf("\n=== Complete: SQLite3 + LevelDB ===\n");
    printf("ADead-BIB compiles both DB engines. 💀🦈\n");
    return 0;
}
