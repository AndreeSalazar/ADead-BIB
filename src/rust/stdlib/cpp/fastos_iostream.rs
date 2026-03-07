// ============================================================
// fastos_iostream.rs — <iostream> implementation
// ============================================================
// std::cout, std::cin, std::cerr, std::endl
// Implementado sobre fastos_stdio internamente
// ============================================================

pub const IOSTREAM_OBJECTS: &[&str] = &[
    "cout", "cin", "cerr", "clog",
];

pub const IOSTREAM_MANIPULATORS: &[&str] = &[
    "endl", "flush", "ends",
    "dec", "hex", "oct",
    "fixed", "scientific",
    "left", "right", "internal",
    "boolalpha", "noboolalpha",
    "showbase", "noshowbase",
    "showpoint", "noshowpoint",
    "showpos", "noshowpos",
    "uppercase", "nouppercase",
    "setw", "setprecision", "setfill",
];

pub const IOSTREAM_CLASSES: &[&str] = &[
    "ostream", "istream", "iostream",
    "ofstream", "ifstream", "fstream",
    "ostringstream", "istringstream", "stringstream",
];

pub fn is_iostream_symbol(name: &str) -> bool {
    IOSTREAM_OBJECTS.contains(&name)
        || IOSTREAM_MANIPULATORS.contains(&name)
        || IOSTREAM_CLASSES.contains(&name)
}
