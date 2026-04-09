// ADead-BIB Bridge Test 20 — Enums & Exhaustive Switch
// Level: BASIC
// Tests: Large enum declarations, exhaustive switch/case over enums

#include <stdio.h>
#include <string.h>

typedef enum {
    TOKEN_INT = 0,
    TOKEN_FLOAT,
    TOKEN_STRING,
    TOKEN_CHAR,
    TOKEN_PLUS,
    TOKEN_MINUS,
    TOKEN_STAR,
    TOKEN_SLASH,
    TOKEN_PERCENT,
    TOKEN_LPAREN,
    TOKEN_RPAREN,
    TOKEN_LBRACE,
    TOKEN_RBRACE,
    TOKEN_SEMICOLON,
    TOKEN_COMMA,
    TOKEN_ASSIGN,
    TOKEN_EOF,
    TOKEN_COUNT
} TokenType;

const char* token_name(TokenType t) {
    switch (t) {
        case TOKEN_INT:       return "int";
        case TOKEN_FLOAT:     return "float";
        case TOKEN_STRING:    return "string";
        case TOKEN_CHAR:      return "char";
        case TOKEN_PLUS:      return "+";
        case TOKEN_MINUS:     return "-";
        case TOKEN_STAR:      return "*";
        case TOKEN_SLASH:     return "/";
        case TOKEN_PERCENT:   return "%";
        case TOKEN_LPAREN:    return "(";
        case TOKEN_RPAREN:    return ")";
        case TOKEN_LBRACE:    return "{";
        case TOKEN_RBRACE:    return "}";
        case TOKEN_SEMICOLON: return ";";
        case TOKEN_COMMA:     return ",";
        case TOKEN_ASSIGN:    return "=";
        case TOKEN_EOF:       return "EOF";
        case TOKEN_COUNT:     return "COUNT";
    }
    return "unknown";
}

typedef enum {
    SEVERITY_INFO = 10,
    SEVERITY_WARN = 20,
    SEVERITY_ERROR = 30,
    SEVERITY_FATAL = 100
} Severity;

const char* severity_name(Severity s) {
    switch (s) {
        case SEVERITY_INFO:  return "info";
        case SEVERITY_WARN:  return "warn";
        case SEVERITY_ERROR: return "error";
        case SEVERITY_FATAL: return "fatal";
    }
    return "unknown";
}

int main() {
    printf("=== ADead-BIB Bridge Test 20: Enum Switch ===\n");
    int pass = 0, fail = 0;

    // Enum auto-increment: TOKEN_INT=0, TOKEN_FLOAT=1, ...
    if (TOKEN_INT == 0) { pass++; } else { fail++; printf("FAIL: TOKEN_INT=%d\n", TOKEN_INT); }
    if (TOKEN_FLOAT == 1) { pass++; } else { fail++; printf("FAIL: TOKEN_FLOAT=%d\n", TOKEN_FLOAT); }
    if (TOKEN_PLUS == 4) { pass++; } else { fail++; printf("FAIL: TOKEN_PLUS=%d\n", TOKEN_PLUS); }

    // TOKEN_COUNT should be 17
    if (TOKEN_COUNT == 17) { pass++; } else { fail++; printf("FAIL: TOKEN_COUNT=%d\n", TOKEN_COUNT); }

    // Exhaustive switch — verify each case returns correct string
    if (strcmp(token_name(TOKEN_INT), "int") == 0) { pass++; } else { fail++; printf("FAIL: token_name(INT)\n"); }
    if (strcmp(token_name(TOKEN_STAR), "*") == 0) { pass++; } else { fail++; printf("FAIL: token_name(STAR)\n"); }
    if (strcmp(token_name(TOKEN_SEMICOLON), ";") == 0) { pass++; } else { fail++; printf("FAIL: token_name(SEMICOLON)\n"); }
    if (strcmp(token_name(TOKEN_EOF), "EOF") == 0) { pass++; } else { fail++; printf("FAIL: token_name(EOF)\n"); }

    // Explicit-value enum
    if (SEVERITY_INFO == 10 && SEVERITY_WARN == 20 && SEVERITY_ERROR == 30 && SEVERITY_FATAL == 100) { pass++; } else { fail++; printf("FAIL: severity values\n"); }

    // Nested enum switch
    if (strcmp(severity_name(SEVERITY_INFO), "info") == 0) { pass++; } else { fail++; printf("FAIL: severity_name(INFO)\n"); }
    if (strcmp(severity_name(SEVERITY_FATAL), "fatal") == 0) { pass++; } else { fail++; printf("FAIL: severity_name(FATAL)\n"); }

    // Loop through all token types, verify none return "unknown"
    int all_known = 1;
    for (int i = 0; i <= TOKEN_COUNT; i++) {
        if (strcmp(token_name((TokenType)i), "unknown") == 0) { all_known = 0; break; }
    }
    if (all_known) { pass++; } else { fail++; printf("FAIL: exhaustive switch coverage\n"); }

    printf("Results: %d passed, %d failed\n", pass, fail);
    printf("=== Test 20: %s ===\n", fail == 0 ? "PASS" : "FAIL");
    return fail;
}
