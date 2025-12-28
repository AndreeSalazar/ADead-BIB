// ADead-BIB Standard Library - Math Module
// std::math - Mathematical functions

// Absolute value
fn abs(x) {
    if x < 0 {
        return 0 - x
    }
    return x
}

// Maximum of two numbers
fn max(a, b) {
    if a > b {
        return a
    }
    return b
}

// Minimum of two numbers
fn min(a, b) {
    if a < b {
        return a
    }
    return b
}

// Power function (integer exponent)
fn pow(base, exp) {
    let result = 1
    let i = 0
    while i < exp {
        result = result * base
        i = i + 1
    }
    return result
}

// Factorial
fn factorial(n) {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}

// Sum of range 1 to n
fn sum_range(n) {
    return n * (n + 1) / 2
}

// Clamp value between min and max
fn clamp(value, min_val, max_val) {
    if value < min_val {
        return min_val
    }
    if value > max_val {
        return max_val
    }
    return value
}
