# ADead-BIB Project Template

> Estructura de proyecto siguiendo la arquitectura binaria dual.

## Estructura

```
Project/
├── main.adB          # Binario base (entrypoint)
├── call.adB          # Lógica OOP pura
├── core/             # Intrínsecos del sistema
│   ├── init.adB      # Inicialización CPU/GPU
│   └── shutdown.adB  # Limpieza y shutdown
├── cpu/              # Módulos CPU específicos
├── gpu/              # Módulos GPU específicos
├── build.adB         # Configuración de build
└── README.md         # Este archivo
```

## Arquitectura Binaria Dual

### `main.adB` — Binario Base (ROOT)

- Define el **punto de entrada real**
- Controla el **flujo global**
- Inicializa CPU / GPU
- **NO** tiene lógica compleja

### `call.adB` — Binario de Comportamiento (OOP)

- Define **objetos** y **métodos**
- Contiene **lógica de alto nivel**
- Exporta funciones públicas
- **NO** conoce el entrypoint

## Flujo de Ejecución

```
main.adB::_start()
    ↓
core::init()
    ↓
call::run()  ──→  [OOP puro]
    ↓
core::shutdown()
    ↓
exit
```

## Compilar

```bash
adeadc build build.adB
```

## Ejecutar

```bash
adeadc run main.adB
```

---

**ADead-BIB: Código → Bytes → Binario**
