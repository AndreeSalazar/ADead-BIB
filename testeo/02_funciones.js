// 02_funciones.js — JsDead-BIB Test #2
// Funciones con tipos estrictos

function add(a: int, b: int): int {
    return a + b;
}

function multiply(a: int, b: int): int {
    return a * b;
}

function factorial(n: int): int {
    if (n <= 1) {
        return 1;
    }
    return n * factorial(n - 1);
}

function main(): void {
    let sum: int = add(10, 20);
    console.log(sum);

    let product: int = multiply(6, 7);
    console.log(product);

    let fact: int = factorial(5);
    console.log(fact);
}

main();
