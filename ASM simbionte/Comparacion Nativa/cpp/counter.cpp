// C++ Nativo - Benchmark de Loop
// Compila con: cl /O2 counter.cpp (MSVC)
//          o: g++ -O3 counter.cpp -o counter.exe (MinGW)
//          o: clang++ -O3 counter.cpp -o counter.exe

#include <iostream>
#include <chrono>
#include <cstdint>

// Prevenir optimización de dead code
volatile int64_t sink;

int64_t count_to(int64_t limit) {
    int64_t counter = 0;
    while (counter < limit) {
        counter++;
    }
    sink = counter;  // Efecto observable
    return counter;
}

int main() {
    const int64_t ITERATIONS = 1000000000;  // 1 billón
    
    std::cout << "C++ Nativo - Loop de " << ITERATIONS << " iteraciones" << std::endl;
    
    auto start = std::chrono::high_resolution_clock::now();
    int64_t result = count_to(ITERATIONS);
    auto end = std::chrono::high_resolution_clock::now();
    
    auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(end - start);
    double seconds = duration.count() / 1000.0;
    
    std::cout << "Resultado: " << result << std::endl;
    std::cout << "Tiempo: " << seconds << "s" << std::endl;
    
    return 0;
}
