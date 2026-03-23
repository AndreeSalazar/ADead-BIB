// 05_arrays_strict.js — JsDead-BIB Test #5
// Arrays homogéneos + tipado estricto

function sum_array(arr: int[], size: int): int {
    let total: int = 0;
    for (let i: int = 0; i < size; i++) {
        total = total + arr[i];
    }
    return total;
}

function max_array(arr: int[], size: int): int {
    let max: int = arr[0];
    for (let i: int = 1; i < size; i++) {
        if (arr[i] > max) {
            max = arr[i];
        }
    }
    return max;
}

function main(): void {
    // Arrays homogéneos — JsDead-BIB los requiere
    let nums: int[] = [10, 20, 30, 40, 50];
    let total: int = sum_array(nums, 5);
    console.log(total);

    let biggest: int = max_array(nums, 5);
    console.log(biggest);

    // Bitwise operations — respeta los bits
    let a: int = 0xFF;
    let b: int = 0x0F;
    let and_result: int = a & b;
    console.log(and_result);

    let or_result: int = a | b;
    console.log(or_result);

    let xor_result: int = a ^ b;
    console.log(xor_result);

    let shift_left: int = 1 << 8;
    console.log(shift_left);
}

main();
