// ============================================================
// Canon C99 — §6.7.2.2 Enums y Constantes Enteras
// ============================================================
// Intención: Un enum es un conjunto de constantes enteras
// con nombre. No es un tipo fuerte — son ints.
//
// C99 §6.7.2.2: "An enumeration comprises a set of named
// integer constant values."
// ============================================================

#include <stdio.h>

// --- Enum con valores automáticos (0, 1, 2...) ---
enum Direction { NORTH, EAST, SOUTH, WEST };

// --- Enum con valores explícitos ---
enum HttpStatus {
    HTTP_OK           = 200,
    HTTP_NOT_FOUND    = 404,
    HTTP_SERVER_ERROR = 500
};

// --- Enum para flags (bitwise) ---
enum Permission {
    PERM_NONE    = 0,
    PERM_READ    = 1,
    PERM_WRITE   = 2,
    PERM_EXEC    = 4
};

// --- Funciones ---

const char *direction_name(enum Direction d) {
    switch (d) {
        case NORTH: return "North";
        case EAST:  return "East";
        case SOUTH: return "South";
        case WEST:  return "West";
        default:    return "Unknown";
    }
}

const char *http_message(enum HttpStatus s) {
    switch (s) {
        case HTTP_OK:           return "OK";
        case HTTP_NOT_FOUND:    return "Not Found";
        case HTTP_SERVER_ERROR: return "Internal Server Error";
        default:                return "Unknown";
    }
}

int has_permission(int flags, enum Permission p) {
    return (flags & p) != 0;
}

int main() {
    printf("=== Canon C99: Enums y Constantes ===\n\n");

    // --- Enum automático ---
    printf("Directions:\n");
    printf("  NORTH = %d\n", NORTH);
    printf("  EAST  = %d\n", EAST);
    printf("  SOUTH = %d\n", SOUTH);
    printf("  WEST  = %d\n", WEST);

    enum Direction d = EAST;
    printf("  Current: %s (%d)\n", direction_name(d), d);

    // --- Enum con valores explícitos ---
    printf("\nHTTP Status:\n");
    printf("  200 = %s\n", http_message(HTTP_OK));
    printf("  404 = %s\n", http_message(HTTP_NOT_FOUND));
    printf("  500 = %s\n", http_message(HTTP_SERVER_ERROR));

    // --- Enum como flags (bitwise OR) ---
    printf("\nPermissions (bitwise):\n");
    int user_perms = PERM_READ | PERM_WRITE;
    int admin_perms = PERM_READ | PERM_WRITE | PERM_EXEC;

    printf("  user  read=%d write=%d exec=%d\n",
        has_permission(user_perms, PERM_READ),
        has_permission(user_perms, PERM_WRITE),
        has_permission(user_perms, PERM_EXEC));

    printf("  admin read=%d write=%d exec=%d\n",
        has_permission(admin_perms, PERM_READ),
        has_permission(admin_perms, PERM_WRITE),
        has_permission(admin_perms, PERM_EXEC));

    // --- Enum es int (aritmética) ---
    int val = SOUTH + 10;
    printf("\nSOUTH + 10 = %d\n", val);

    // --- Verificación ---
    int pass = 0;
    int total = 0;

    total++; if (NORTH == 0)                     { pass++; } else { printf("FAIL: NORTH\n"); }
    total++; if (EAST == 1)                      { pass++; } else { printf("FAIL: EAST\n"); }
    total++; if (HTTP_OK == 200)                 { pass++; } else { printf("FAIL: HTTP_OK\n"); }
    total++; if (HTTP_NOT_FOUND == 404)           { pass++; } else { printf("FAIL: HTTP_404\n"); }
    total++; if (has_permission(user_perms, PERM_READ))  { pass++; } else { printf("FAIL: perm read\n"); }
    total++; if (!has_permission(user_perms, PERM_EXEC)) { pass++; } else { printf("FAIL: perm exec\n"); }
    total++; if (val == 12)                      { pass++; } else { printf("FAIL: enum arith\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}
