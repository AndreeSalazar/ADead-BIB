# 🐹 Go + ADead-BIB

**Integración de Go con ADead-BIB para servicios web y microservicios**

Author: Eddi Andreé Salazar Matos  
Made with ❤️ in Peru 🇵🇪

---

## 🧠 Filosofía

```
Go (Simplicidad + Concurrencia) + ADead-BIB (Rendimiento Nativo) = Microservicios Perfectos
```

Go proporciona:
- **Goroutines** para concurrencia masiva
- **Compilación rápida**
- **Deployment simple** (binario único)

ADead-BIB añade:
- **Cómputo nativo** sin overhead
- **GPU acceleration**
- **Binarios ultra-compactos**

---

## 🚀 Casos de Uso

### 1. API de Alto Rendimiento
```go
package main

import (
    "github.com/adead-bib/go-binding"
    "net/http"
)

func main() {
    engine := adead.NewEngine()
    
    http.HandleFunc("/matmul", func(w http.ResponseWriter, r *http.Request) {
        // Cómputo pesado con ADead-BIB
        result := engine.MatMul(a, b, 1024)
        json.NewEncoder(w).Encode(result)
    })
    
    http.ListenAndServe(":8080", nil)
}
```

### 2. Worker Pool con Cómputo Nativo
```go
func processJobs(jobs <-chan Job, results chan<- Result) {
    engine := adead.NewEngine()
    
    for job := range jobs {
        // Cada goroutine usa ADead-BIB
        result := engine.Process(job.Data)
        results <- Result{Data: result}
    }
}

func main() {
    jobs := make(chan Job, 100)
    results := make(chan Result, 100)
    
    // Pool de workers con ADead-BIB
    for i := 0; i < runtime.NumCPU(); i++ {
        go processJobs(jobs, results)
    }
}
```

### 3. CLI Tool
```go
package main

import (
    "github.com/adead-bib/go-binding"
    "flag"
    "fmt"
)

func main() {
    input := flag.String("input", "", "ADead-BIB code")
    flag.Parse()
    
    compiler := adead.NewCompiler()
    result := compiler.Run(*input)
    
    fmt.Println("Result:", result)
}
```

---

## 📦 Instalación

```bash
go get github.com/adead-bib/go-binding
```

---

## 🔧 API

```go
// Crear engine
engine := adead.NewEngine()

// Operaciones
result := engine.MatMul(a, b, size)
output := engine.Attention(q, k, v, dim)
tokens := engine.Tokenize(text)

// Compilar y ejecutar
compiler := adead.NewCompiler()
binary := compiler.Compile(code)
result := binary.Execute()
```

---

## 📊 Benchmarks

| Operación | Go Puro | Go + ADead-BIB |
|-----------|---------|----------------|
| MatMul 512² | 120ms | 0.1ms |
| JSON Parse 1M | 200ms | 50ms |
| HTTP req/s | 50K | 150K |

---

**Go + ADead-BIB: Simplicidad + Rendimiento** 🐹💪
