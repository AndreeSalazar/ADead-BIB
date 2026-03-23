// 09_try_catch.js — JsDead-BIB Test #9
// try/catch/finally — manejo de errores básico

function divide(a: int, b: int): int {
    if (b === 0) {
        throw "Division by zero";
    }
    return a / b;
}

function safe_divide(a: int, b: int): int {
    try {
        let result: int = divide(a, b);
        console.log(result);
        return result;
    } catch (e) {
        console.log("Error caught");
        return -1;
    } finally {
        console.log("Operation complete");
    }
}

function main(): void {
    safe_divide(10, 2);
    safe_divide(20, 0);
    safe_divide(100, 4);
}

main();
