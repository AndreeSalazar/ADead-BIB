// ADead-BIB Standard Library - String Module
// std::string - String utility functions

// Get string length (wrapper for len)
fn strlen(s) {
    return len(s)
}

// Repeat a character n times (prints it)
fn repeat_char(ch, n) {
    let i = 0
    while i < n {
        print(ch)
        i = i + 1
    }
}

// Print n spaces
fn spaces(n) {
    let i = 0
    while i < n {
        print(" ")
        i = i + 1
    }
}

// Print n dashes
fn dashes(n) {
    let i = 0
    while i < n {
        print("-")
        i = i + 1
    }
    println("")
}
