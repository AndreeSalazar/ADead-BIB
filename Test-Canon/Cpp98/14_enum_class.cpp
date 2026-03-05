// ============================================================
// Canon C++11 — Enum Class (Scoped Enumerations)
// ============================================================
// Intención: enum class es un enum con scope y tipado fuerte.
// Los valores no se mezclan con otros tipos — el compilador
// verifica en compilación.
//
// C++11 §7.2: "An enumeration declared with enum class
// has a fixed underlying type."
//
// ADead-BIB: enum class → int con scope prefix. Sin overhead.
// ============================================================

int printf(const char *format, ...);

// --- Enum class básico ---
enum class Color : int {
    Red = 0,
    Green = 1,
    Blue = 2,
    Alpha = 3
};

enum class Direction : int {
    North = 0,
    East = 1,
    South = 2,
    West = 3
};

enum class LogLevel : int {
    Error = 0,
    Warning = 1,
    Info = 2,
    Debug = 3,
    Trace = 4
};

// --- Funciones ---
const char *color_name(Color c) {
    switch (c) {
        case Color::Red:   return "Red";
        case Color::Green: return "Green";
        case Color::Blue:  return "Blue";
        case Color::Alpha: return "Alpha";
        default:           return "Unknown";
    }
}

const char *direction_name(Direction d) {
    switch (d) {
        case Direction::North: return "North";
        case Direction::East:  return "East";
        case Direction::South: return "South";
        case Direction::West:  return "West";
        default:               return "Unknown";
    }
}

Direction turn_right(Direction d) {
    switch (d) {
        case Direction::North: return Direction::East;
        case Direction::East:  return Direction::South;
        case Direction::South: return Direction::West;
        case Direction::West:  return Direction::North;
        default:               return d;
    }
}

int should_log(LogLevel current, LogLevel message) {
    return (int)message <= (int)current;
}

int main() {
    printf("=== Canon C++11: Enum Class ===\n\n");

    // --- Color ---
    printf("Colors:\n");
    Color c = Color::Red;
    printf("  %s = %d\n", color_name(c), (int)c);
    printf("  %s = %d\n", color_name(Color::Green), (int)Color::Green);
    printf("  %s = %d\n", color_name(Color::Blue), (int)Color::Blue);

    // --- Direction ---
    printf("\nDirection:\n");
    Direction d = Direction::North;
    printf("  Start: %s\n", direction_name(d));
    d = turn_right(d);
    printf("  Turn right: %s\n", direction_name(d));
    d = turn_right(d);
    printf("  Turn right: %s\n", direction_name(d));
    d = turn_right(d);
    printf("  Turn right: %s\n", direction_name(d));
    d = turn_right(d);
    printf("  Turn right: %s (full circle)\n", direction_name(d));

    // --- LogLevel ---
    printf("\nLogLevel:\n");
    LogLevel current = LogLevel::Warning;
    printf("  Current level: %d\n", (int)current);
    printf("  Show Error?   %d\n", should_log(current, LogLevel::Error));
    printf("  Show Warning? %d\n", should_log(current, LogLevel::Warning));
    printf("  Show Info?    %d\n", should_log(current, LogLevel::Info));
    printf("  Show Debug?   %d\n", should_log(current, LogLevel::Debug));

    // --- Verificación ---
    int pass = 0;
    int total = 0;

    total++; if ((int)Color::Red == 0)                     { pass++; } else { printf("FAIL: red\n"); }
    total++; if ((int)Color::Blue == 2)                    { pass++; } else { printf("FAIL: blue\n"); }
    total++; if ((int)turn_right(Direction::North) == 1)   { pass++; } else { printf("FAIL: turn right\n"); }
    total++; if ((int)turn_right(Direction::West) == 0)    { pass++; } else { printf("FAIL: full circle\n"); }
    total++; if (should_log(current, LogLevel::Error))     { pass++; } else { printf("FAIL: log error\n"); }
    total++; if (!should_log(current, LogLevel::Debug))    { pass++; } else { printf("FAIL: log debug\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}
