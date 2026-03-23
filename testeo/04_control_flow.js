// 04_control_flow.js — JsDead-BIB Test #4
// for, while, do-while, switch, if/else, ternario

function fizzbuzz(n: int): void {
    for (let i: int = 1; i <= n; i++) {
        if (i % 15 === 0) {
            console.log("FizzBuzz");
        } else if (i % 3 === 0) {
            console.log("Fizz");
        } else if (i % 5 === 0) {
            console.log("Buzz");
        } else {
            console.log(i);
        }
    }
}

function countdown(n: int): void {
    let i: int = n;
    while (i > 0) {
        console.log(i);
        i = i - 1;
    }
    console.log("Go!");
}

function sum_to(n: int): int {
    let total: int = 0;
    let i: int = 1;
    do {
        total = total + i;
        i = i + 1;
    } while (i <= n);
    return total;
}

function day_name(day: int): void {
    switch (day) {
        case 1:
            console.log("Monday");
            break;
        case 2:
            console.log("Tuesday");
            break;
        case 3:
            console.log("Wednesday");
            break;
        default:
            console.log("Other");
            break;
    }
}

function main(): void {
    fizzbuzz(15);
    countdown(3);
    let s: int = sum_to(10);
    console.log(s);
    day_name(1);
    day_name(3);

    // Ternario
    let x: int = 10;
    let result: int = x > 5 ? 1 : 0;
    console.log(result);
}

main();
