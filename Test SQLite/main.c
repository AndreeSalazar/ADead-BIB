// SQLite test with ADead-BIB
int printf(const char *format, ...);

// Forward declarations from sqlite3.h
typedef struct sqlite3 sqlite3;
const char *sqlite3_libversion(void);
int sqlite3_open(const char *filename, sqlite3 **ppDb);
int sqlite3_close(sqlite3 *db);

int main() {
    sqlite3 *db;
    int rc = sqlite3_open(":memory:", &db);
    if (rc == 0) {
        printf("SQLite OK\n");
        printf("Version: %s\n", sqlite3_libversion());
        sqlite3_close(db);
        return 0;
    }
    printf("FAIL: rc=%d\n", rc);
    return 1;
}
