// ADead-BIB Standard Library - IO Module
// std::io - Input/Output functions

// Print with newline
fn puts(msg) {
    println(msg)
}

// Print without newline
fn write(msg) {
    print(msg)
}

// Read integer from stdin
fn read_int() {
    return input()
}

// Print a number with label
fn print_num(label, value) {
    print(label)
    println(value)
}

// Print separator line
fn separator() {
    println("----------------------------------------")
}

// Print header
fn header(title) {
    println("+========================================+")
    print("|   ")
    print(title)
    println("   |")
    println("+========================================+")
}
