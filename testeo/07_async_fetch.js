// 07_async_fetch.js — JsDead-BIB Test #7
// async/await → funciones normales (sin event loop)
// JsDead-BIB: async = hint de compilador, no runtime
// La función se compila igual — sin Promise, sin callback

async function fetch_data(url: string): int {
    console.log("Fetching:");
    console.log(url);
    // En JsDead-BIB, await se compila como llamada síncrona
    // No hay event loop — es ASM directo
    return 200;
}

async function process(data: int): void {
    if (data === 200) {
        console.log("Success");
    } else {
        console.log("Failed");
    }
}

function main(): void {
    let status: int = fetch_data("https://api.example.com/data");
    process(status);

    let status2: int = fetch_data("https://api.example.com/users");
    process(status2);
}

main();
