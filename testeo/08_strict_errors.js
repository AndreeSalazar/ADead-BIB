// 08_strict_errors.js — JsDead-BIB Test #8
// Este archivo DEBE FALLAR en compilación
// == está bloqueado — solo === es válido
// "Respeta los bits"

let x: int = 5;
if (x == 5) {
    console.log("esto no debería compilar");
}
