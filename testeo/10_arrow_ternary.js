// 10_arrow_ternary.js — JsDead-BIB Test #10
// Arrow functions + ternarios + operadores compuestos

function apply(fn: int, x: int): int {
    // Arrow functions en JsDead-BIB se compilan inline
    return fn + x;
}

function main(): void {
    // Operadores compuestos
    let counter: int = 0;
    counter += 10;
    counter -= 3;
    counter *= 2;
    console.log(counter);

    // Ternarios anidados
    let score: int = 85;
    let grade: string = score >= 90 ? "A" : score >= 80 ? "B" : score >= 70 ? "C" : "F";
    console.log(grade);

    // Pre/Post increment
    let i: int = 5;
    let pre: int = ++i;
    console.log(pre);
    console.log(i);

    let j: int = 10;
    let post: int = j++;
    console.log(post);
    console.log(j);

    // Nullish coalescing
    let val: int = 0 ?? 42;
    console.log(val);

    // Strict equality chains
    let a: int = 5;
    let b: int = 5;
    if (a === b) {
        console.log("equal");
    }
    if (a !== 10) {
        console.log("not ten");
    }
}

main();
