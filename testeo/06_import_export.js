// 06_import_export.js — JsDead-BIB Test #6
// import/export — compilador resuelve internamente
// Sin Node.js modules. Sin bundler. Sin webpack.

import { tcp, tls, http } from "net";

// En JsDead-BIB, imports se resuelven a las stdlib internas
// "net" → funciones nativas de red compiladas a ASM

export function create_request(host: string, port: int): int {
    console.log("Connecting to:");
    console.log(host);
    console.log(port);
    return 0;
}

export function parse_response(status: int): void {
    if (status === 200) {
        console.log("OK");
    } else if (status === 404) {
        console.log("Not Found");
    } else {
        console.log("Error");
    }
}

function main(): void {
    let status: int = create_request("api.example.com", 443);
    parse_response(200);
    parse_response(404);
    parse_response(500);
}

main();
