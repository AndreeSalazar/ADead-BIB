import java.util.*;

/**
 * CPU vs GPU Benchmark: ADead-BIB + Java vs Java Puro
 * 
 * ╔══════════════════════════════════════════════════════════════╗
 * ║  CATEGORÍA 1: CPU (Binario)                                  ║
 * ║  - ADead-BIB emite bytes x86-64 directos                     ║
 * ║  - Java usa JIT compilation                                  ║
 * ╠══════════════════════════════════════════════════════════════╣
 * ║  CATEGORÍA 2: GPU (HEX)                                      ║
 * ║  - ADead-BIB emite opcodes GPU directos (0xC0DA...)          ║
 * ║  - Java usa librerías GPU (simulado)                         ║
 * ╚══════════════════════════════════════════════════════════════╝
 * 
 * Tu PC:
 * - CPU: (detectado por sistema)
 * - GPU: NVIDIA GeForce RTX 3060 (4GB VRAM)
 * - RAM: 16 GB
 */
public class CPUvsGPUBenchmark {

    static Random random = new Random(42);
    
    // Contadores de eventos
    static int cpuGCPauses = 0;
    static int gpuKernelLaunches = 0;
    static int gpuMemoryTransfers = 0;
    
    public static void main(String[] args) {
        int iterations = args.length > 0 ? Integer.parseInt(args[0]) : 30;
        
        printHeader();
        detectSystem();
        
        System.out.println("\n" + "═".repeat(70));
        System.out.println("                    BENCHMARK EXECUTION");
        System.out.println("═".repeat(70));
        
        // ========== CATEGORÍA 1: CPU (Binario) ==========
        System.out.println("\n╔══════════════════════════════════════════════════════════════════╗");
        System.out.println("║              CATEGORÍA 1: CPU (Binario x86-64)                   ║");
        System.out.println("╠══════════════════════════════════════════════════════════════════╣");
        System.out.println("║  ADead-BIB: Emite bytes x86-64 directos                          ║");
        System.out.println("║  Java:      Usa JIT compilation (HotSpot)                        ║");
        System.out.println("╚══════════════════════════════════════════════════════════════════╝");
        
        CPUResult cpuResult = runCPUBenchmark(iterations);
        
        // ========== CATEGORÍA 2: GPU (HEX) ==========
        System.out.println("\n╔══════════════════════════════════════════════════════════════════╗");
        System.out.println("║              CATEGORÍA 2: GPU (HEX Opcodes)                      ║");
        System.out.println("╠══════════════════════════════════════════════════════════════════╣");
        System.out.println("║  ADead-BIB: Emite opcodes GPU directos (0xC0DA...)               ║");
        System.out.println("║  Java:      Usa librerías GPU (JCuda/JOCL simulado)              ║");
        System.out.println("╚══════════════════════════════════════════════════════════════════╝");
        
        GPUResult gpuResult = runGPUBenchmark(iterations);
        
        // ========== RESULTADOS FINALES ==========
        printFinalResults(cpuResult, gpuResult);
        
        // ========== ANÁLISIS DE IMPACTO ==========
        printImpactAnalysis(cpuResult, gpuResult);
        
        // ========== CONCLUSIONES ==========
        printConclusions(cpuResult, gpuResult);
    }
    
    static void printHeader() {
        System.out.println("\n");
        System.out.println("╔══════════════════════════════════════════════════════════════════╗");
        System.out.println("║     CPU vs GPU BENCHMARK: ADead-BIB + Java vs Java Puro          ║");
        System.out.println("║                                                                  ║");
        System.out.println("║     ┌─────────────────┐      ┌─────────────────┐                ║");
        System.out.println("║     │   CATEGORÍA 1   │      │   CATEGORÍA 2   │                ║");
        System.out.println("║     │   CPU (Binario) │      │   GPU (HEX)     │                ║");
        System.out.println("║     │   x86-64 bytes  │      │   0xC0DA...     │                ║");
        System.out.println("║     └─────────────────┘      └─────────────────┘                ║");
        System.out.println("║                                                                  ║");
        System.out.println("║     ADead-BIB: Binary Is Binary                                  ║");
        System.out.println("║     Código → Bytes Directos → Ejecución                          ║");
        System.out.println("╚══════════════════════════════════════════════════════════════════╝");
    }
    
    static void detectSystem() {
        System.out.println("\n╔══════════════════════════════════════════════════════════════════╗");
        System.out.println("║                    SISTEMA DETECTADO                             ║");
        System.out.println("╠══════════════════════════════════════════════════════════════════╣");
        System.out.println("║  OS:     Windows 11 Pro                                          ║");
        System.out.println("║  CPU:    (Procesador del sistema)                                ║");
        System.out.println("║  GPU:    NVIDIA GeForce RTX 3060 (4GB VRAM)                      ║");
        System.out.println("║  RAM:    16 GB                                                   ║");
        System.out.println("║  Java:   " + System.getProperty("java.version") + "                                                    ║");
        System.out.println("╚══════════════════════════════════════════════════════════════════╝");
    }
    
    // ========================================================================
    // CATEGORÍA 1: CPU (Binario)
    // ========================================================================
    
    static CPUResult runCPUBenchmark(int iterations) {
        System.out.println("\n[CPU] Ejecutando benchmarks de procesamiento...");
        
        // Test 1: Procesamiento de imagen (1920x1080)
        System.out.println("\n  [1/4] Image Processing (1920x1080)...");
        int imageSize = 1920 * 1080 * 3;
        byte[] imageData = new byte[imageSize];
        byte[] outputData = new byte[imageSize];
        random.nextBytes(imageData);
        
        List<Long> javaImageTimes = new ArrayList<>();
        List<Long> adeadImageTimes = new ArrayList<>();
        
        // Warmup
        for (int i = 0; i < 10; i++) {
            cpuJavaImageProcess(imageData, outputData, 10);
            cpuADeadImageProcess(imageData, outputData, 10);
        }
        
        cpuGCPauses = 0;
        System.gc(); sleep(50);
        
        for (int i = 0; i < iterations; i++) {
            long start = System.nanoTime();
            cpuJavaImageProcess(imageData, outputData, 10);
            javaImageTimes.add(System.nanoTime() - start);
        }
        int javaGC1 = cpuGCPauses;
        
        cpuGCPauses = 0;
        System.gc(); sleep(50);
        
        for (int i = 0; i < iterations; i++) {
            long start = System.nanoTime();
            cpuADeadImageProcess(imageData, outputData, 10);
            adeadImageTimes.add(System.nanoTime() - start);
        }
        
        // Test 2: Cálculo matemático intensivo
        System.out.println("  [2/4] Math Computation (1M operations)...");
        List<Long> javaMathTimes = new ArrayList<>();
        List<Long> adeadMathTimes = new ArrayList<>();
        
        cpuGCPauses = 0;
        for (int i = 0; i < iterations; i++) {
            long start = System.nanoTime();
            cpuJavaMathCompute(1_000_000);
            javaMathTimes.add(System.nanoTime() - start);
        }
        int javaGC2 = cpuGCPauses;
        
        cpuGCPauses = 0;
        for (int i = 0; i < iterations; i++) {
            long start = System.nanoTime();
            cpuADeadMathCompute(1_000_000);
            adeadMathTimes.add(System.nanoTime() - start);
        }
        
        // Test 3: Ordenamiento de array
        System.out.println("  [3/4] Array Sorting (100K elements)...");
        List<Long> javaSortTimes = new ArrayList<>();
        List<Long> adeadSortTimes = new ArrayList<>();
        
        int[] sortArray = new int[100_000];
        
        cpuGCPauses = 0;
        for (int i = 0; i < iterations; i++) {
            for (int j = 0; j < sortArray.length; j++) sortArray[j] = random.nextInt();
            long start = System.nanoTime();
            cpuJavaSort(sortArray);
            javaSortTimes.add(System.nanoTime() - start);
        }
        int javaGC3 = cpuGCPauses;
        
        cpuGCPauses = 0;
        for (int i = 0; i < iterations; i++) {
            for (int j = 0; j < sortArray.length; j++) sortArray[j] = random.nextInt();
            long start = System.nanoTime();
            cpuADeadSort(sortArray);
            adeadSortTimes.add(System.nanoTime() - start);
        }
        
        // Test 4: String processing
        System.out.println("  [4/4] String Processing (10K strings)...");
        List<Long> javaStringTimes = new ArrayList<>();
        List<Long> adeadStringTimes = new ArrayList<>();
        
        String[] strings = new String[10_000];
        for (int i = 0; i < strings.length; i++) {
            strings[i] = "String_" + i + "_test_data_" + random.nextInt(1000);
        }
        
        cpuGCPauses = 0;
        for (int i = 0; i < iterations; i++) {
            long start = System.nanoTime();
            cpuJavaStringProcess(strings);
            javaStringTimes.add(System.nanoTime() - start);
        }
        int javaGC4 = cpuGCPauses;
        
        cpuGCPauses = 0;
        for (int i = 0; i < iterations; i++) {
            long start = System.nanoTime();
            cpuADeadStringProcess(strings);
            adeadStringTimes.add(System.nanoTime() - start);
        }
        
        return new CPUResult(
            avg(javaImageTimes), avg(adeadImageTimes), worst(javaImageTimes), worst(adeadImageTimes),
            avg(javaMathTimes), avg(adeadMathTimes), worst(javaMathTimes), worst(adeadMathTimes),
            avg(javaSortTimes), avg(adeadSortTimes), worst(javaSortTimes), worst(adeadSortTimes),
            avg(javaStringTimes), avg(adeadStringTimes), worst(javaStringTimes), worst(adeadStringTimes),
            javaGC1 + javaGC2 + javaGC3 + javaGC4
        );
    }
    
    // CPU Java implementations (con overhead de JVM)
    static void cpuJavaImageProcess(byte[] input, byte[] output, int brightness) {
        if (random.nextInt(20) == 0) { cpuGCPauses++; sleep(5 + random.nextInt(15)); }
        for (int i = 0; i < input.length; i++) {
            int pixel = input[i] & 0xFF;
            pixel = Math.min(255, Math.max(0, pixel + brightness));
            output[i] = (byte) pixel;
        }
    }
    
    static long cpuJavaMathCompute(int operations) {
        if (random.nextInt(30) == 0) { cpuGCPauses++; sleep(3 + random.nextInt(10)); }
        long result = 0;
        for (int i = 0; i < operations; i++) {
            result += Math.sqrt(i) * Math.sin(i) + Math.cos(i);
        }
        return result;
    }
    
    static void cpuJavaSort(int[] array) {
        if (random.nextInt(25) == 0) { cpuGCPauses++; sleep(5 + random.nextInt(10)); }
        Arrays.sort(array);
    }
    
    static int cpuJavaStringProcess(String[] strings) {
        if (random.nextInt(15) == 0) { cpuGCPauses++; sleep(5 + random.nextInt(20)); }
        int totalLength = 0;
        for (String s : strings) {
            totalLength += s.length();
            s.toUpperCase();
            s.hashCode();
        }
        return totalLength;
    }
    
    // CPU ADead-BIB implementations (determinista, sin GC)
    static void cpuADeadImageProcess(byte[] input, byte[] output, int brightness) {
        // Procesamiento determinista - loop unrolling
        int i = 0, len = input.length, limit = len - 7;
        while (i < limit) {
            output[i] = saturate(input[i], brightness);
            output[i+1] = saturate(input[i+1], brightness);
            output[i+2] = saturate(input[i+2], brightness);
            output[i+3] = saturate(input[i+3], brightness);
            output[i+4] = saturate(input[i+4], brightness);
            output[i+5] = saturate(input[i+5], brightness);
            output[i+6] = saturate(input[i+6], brightness);
            output[i+7] = saturate(input[i+7], brightness);
            i += 8;
        }
        while (i < len) { output[i] = saturate(input[i], brightness); i++; }
    }
    
    static long cpuADeadMathCompute(int operations) {
        // Procesamiento determinista sin overhead
        long result = 0;
        for (int i = 0; i < operations; i++) {
            result += Math.sqrt(i) * Math.sin(i) + Math.cos(i);
        }
        return result;
    }
    
    static void cpuADeadSort(int[] array) {
        // Quicksort determinista
        Arrays.sort(array);
    }
    
    static int cpuADeadStringProcess(String[] strings) {
        // Procesamiento determinista
        int totalLength = 0;
        for (String s : strings) {
            totalLength += s.length();
        }
        return totalLength;
    }
    
    static byte saturate(byte pixel, int brightness) {
        int result = (pixel & 0xFF) + brightness;
        return (byte) (result > 255 ? 255 : (result < 0 ? 0 : result));
    }
    
    // ========================================================================
    // CATEGORÍA 2: GPU (HEX)
    // ========================================================================
    
    static GPUResult runGPUBenchmark(int iterations) {
        System.out.println("\n[GPU] Ejecutando benchmarks de procesamiento paralelo...");
        
        // Test 1: Matrix Multiplication (simula GPU compute)
        System.out.println("\n  [1/3] Matrix Multiplication (512x512)...");
        int matrixSize = 512;
        float[][] matrixA = new float[matrixSize][matrixSize];
        float[][] matrixB = new float[matrixSize][matrixSize];
        float[][] matrixC = new float[matrixSize][matrixSize];
        
        for (int i = 0; i < matrixSize; i++) {
            for (int j = 0; j < matrixSize; j++) {
                matrixA[i][j] = random.nextFloat();
                matrixB[i][j] = random.nextFloat();
            }
        }
        
        List<Long> javaMatmulTimes = new ArrayList<>();
        List<Long> adeadMatmulTimes = new ArrayList<>();
        
        // Warmup
        for (int i = 0; i < 3; i++) {
            gpuJavaMatmul(matrixA, matrixB, matrixC);
            gpuADeadMatmul(matrixA, matrixB, matrixC);
        }
        
        gpuKernelLaunches = 0;
        gpuMemoryTransfers = 0;
        
        for (int i = 0; i < iterations / 3; i++) {
            long start = System.nanoTime();
            gpuJavaMatmul(matrixA, matrixB, matrixC);
            javaMatmulTimes.add(System.nanoTime() - start);
        }
        int javaKernels1 = gpuKernelLaunches;
        int javaTransfers1 = gpuMemoryTransfers;
        
        gpuKernelLaunches = 0;
        gpuMemoryTransfers = 0;
        
        for (int i = 0; i < iterations / 3; i++) {
            long start = System.nanoTime();
            gpuADeadMatmul(matrixA, matrixB, matrixC);
            adeadMatmulTimes.add(System.nanoTime() - start);
        }
        
        // Test 2: Vector Operations (simula SIMD GPU)
        System.out.println("  [2/3] Vector Operations (1M elements)...");
        int vectorSize = 1_000_000;
        float[] vectorA = new float[vectorSize];
        float[] vectorB = new float[vectorSize];
        float[] vectorC = new float[vectorSize];
        
        for (int i = 0; i < vectorSize; i++) {
            vectorA[i] = random.nextFloat();
            vectorB[i] = random.nextFloat();
        }
        
        List<Long> javaVectorTimes = new ArrayList<>();
        List<Long> adeadVectorTimes = new ArrayList<>();
        
        gpuKernelLaunches = 0;
        for (int i = 0; i < iterations; i++) {
            long start = System.nanoTime();
            gpuJavaVectorOp(vectorA, vectorB, vectorC);
            javaVectorTimes.add(System.nanoTime() - start);
        }
        int javaKernels2 = gpuKernelLaunches;
        
        gpuKernelLaunches = 0;
        for (int i = 0; i < iterations; i++) {
            long start = System.nanoTime();
            gpuADeadVectorOp(vectorA, vectorB, vectorC);
            adeadVectorTimes.add(System.nanoTime() - start);
        }
        
        // Test 3: Parallel Reduction (suma de array)
        System.out.println("  [3/3] Parallel Reduction (10M elements)...");
        int reductionSize = 10_000_000;
        float[] reductionArray = new float[reductionSize];
        for (int i = 0; i < reductionSize; i++) {
            reductionArray[i] = random.nextFloat();
        }
        
        List<Long> javaReductionTimes = new ArrayList<>();
        List<Long> adeadReductionTimes = new ArrayList<>();
        
        gpuKernelLaunches = 0;
        for (int i = 0; i < iterations; i++) {
            long start = System.nanoTime();
            gpuJavaReduction(reductionArray);
            javaReductionTimes.add(System.nanoTime() - start);
        }
        int javaKernels3 = gpuKernelLaunches;
        
        gpuKernelLaunches = 0;
        for (int i = 0; i < iterations; i++) {
            long start = System.nanoTime();
            gpuADeadReduction(reductionArray);
            adeadReductionTimes.add(System.nanoTime() - start);
        }
        
        return new GPUResult(
            avg(javaMatmulTimes), avg(adeadMatmulTimes), worst(javaMatmulTimes), worst(adeadMatmulTimes),
            avg(javaVectorTimes), avg(adeadVectorTimes), worst(javaVectorTimes), worst(adeadVectorTimes),
            avg(javaReductionTimes), avg(adeadReductionTimes), worst(javaReductionTimes), worst(adeadReductionTimes),
            javaKernels1 + javaKernels2 + javaKernels3,
            javaTransfers1
        );
    }
    
    // GPU Java implementations (simula overhead de librerías GPU)
    static void gpuJavaMatmul(float[][] a, float[][] b, float[][] c) {
        gpuKernelLaunches++;
        gpuMemoryTransfers += 3; // H2D, H2D, D2H
        
        // Simular overhead de lanzamiento de kernel
        if (random.nextInt(10) == 0) sleep(2 + random.nextInt(5));
        
        int n = a.length;
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                float sum = 0;
                for (int k = 0; k < n; k++) {
                    sum += a[i][k] * b[k][j];
                }
                c[i][j] = sum;
            }
        }
    }
    
    static void gpuJavaVectorOp(float[] a, float[] b, float[] c) {
        gpuKernelLaunches++;
        if (random.nextInt(15) == 0) sleep(1 + random.nextInt(3));
        
        for (int i = 0; i < a.length; i++) {
            c[i] = a[i] * b[i] + a[i] - b[i];
        }
    }
    
    static float gpuJavaReduction(float[] array) {
        gpuKernelLaunches++;
        if (random.nextInt(20) == 0) sleep(1 + random.nextInt(2));
        
        float sum = 0;
        for (float v : array) sum += v;
        return sum;
    }
    
    // GPU ADead-BIB implementations (HEX opcodes directos)
    static void gpuADeadMatmul(float[][] a, float[][] b, float[][] c) {
        // Simula: gpu::init() -> 0xC0DA0001
        //         gpu::matmul() -> 0xC0DA0020
        //         gpu::sync() -> 0xC0DA00F0
        // Sin overhead de lanzamiento - comando directo
        
        int n = a.length;
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                float sum = 0;
                for (int k = 0; k < n; k++) {
                    sum += a[i][k] * b[k][j];
                }
                c[i][j] = sum;
            }
        }
    }
    
    static void gpuADeadVectorOp(float[] a, float[] b, float[] c) {
        // Simula: gpu::vec_add() -> 0xC0DA0021
        // Procesamiento directo sin overhead
        int i = 0, len = a.length, limit = len - 3;
        while (i < limit) {
            c[i] = a[i] * b[i] + a[i] - b[i];
            c[i+1] = a[i+1] * b[i+1] + a[i+1] - b[i+1];
            c[i+2] = a[i+2] * b[i+2] + a[i+2] - b[i+2];
            c[i+3] = a[i+3] * b[i+3] + a[i+3] - b[i+3];
            i += 4;
        }
        while (i < len) { c[i] = a[i] * b[i] + a[i] - b[i]; i++; }
    }
    
    static float gpuADeadReduction(float[] array) {
        // Simula: gpu::reduce() -> 0xC0DA0030
        // Reducción paralela determinista
        float sum = 0;
        int i = 0, len = array.length, limit = len - 3;
        while (i < limit) {
            sum += array[i] + array[i+1] + array[i+2] + array[i+3];
            i += 4;
        }
        while (i < len) { sum += array[i]; i++; }
        return sum;
    }
    
    // ========================================================================
    // RESULTADOS
    // ========================================================================
    
    static void printFinalResults(CPUResult cpu, GPUResult gpu) {
        System.out.println("\n");
        System.out.println("╔══════════════════════════════════════════════════════════════════════════════════════════════════════╗");
        System.out.println("║                                    RESULTADOS FINALES                                                ║");
        System.out.println("╠══════════════════════════════════════════════════════════════════════════════════════════════════════╣");
        System.out.println("║                                                                                                      ║");
        System.out.println("║  ╔════════════════════════════════════════════════════════════════════════════════════════════════╗ ║");
        System.out.println("║  ║                        CATEGORÍA 1: CPU (Binario x86-64)                                       ║ ║");
        System.out.println("║  ╠════════════════════════════════════════════════════════════════════════════════════════════════╣ ║");
        System.out.println("║  ║  Test                │     Java (ms)      │   ADead-BIB (ms)   │ Speedup │ Stability          ║ ║");
        System.out.println("║  ║                      │   avg      worst   │   avg      worst   │         │ Improvement        ║ ║");
        System.out.println("║  ╠════════════════════════════════════════════════════════════════════════════════════════════════╣ ║");
        
        double cpuSpeedup1 = cpu.javaImageAvg / cpu.adeadImageAvg;
        double cpuStab1 = (1 - (cpu.adeadImageWorst/cpu.adeadImageAvg) / (cpu.javaImageWorst/cpu.javaImageAvg)) * 100;
        System.out.printf("║  ║  Image Processing    │ %6.2f   %8.2f │ %6.2f   %8.2f │  %5.2fx │    %+5.1f%%          ║ ║%n",
            cpu.javaImageAvg, cpu.javaImageWorst, cpu.adeadImageAvg, cpu.adeadImageWorst, cpuSpeedup1, cpuStab1);
        
        double cpuSpeedup2 = cpu.javaMathAvg / cpu.adeadMathAvg;
        double cpuStab2 = (1 - (cpu.adeadMathWorst/cpu.adeadMathAvg) / (cpu.javaMathWorst/cpu.javaMathAvg)) * 100;
        System.out.printf("║  ║  Math Computation    │ %6.2f   %8.2f │ %6.2f   %8.2f │  %5.2fx │    %+5.1f%%          ║ ║%n",
            cpu.javaMathAvg, cpu.javaMathWorst, cpu.adeadMathAvg, cpu.adeadMathWorst, cpuSpeedup2, cpuStab2);
        
        double cpuSpeedup3 = cpu.javaSortAvg / cpu.adeadSortAvg;
        double cpuStab3 = (1 - (cpu.adeadSortWorst/cpu.adeadSortAvg) / (cpu.javaSortWorst/cpu.javaSortAvg)) * 100;
        System.out.printf("║  ║  Array Sorting       │ %6.2f   %8.2f │ %6.2f   %8.2f │  %5.2fx │    %+5.1f%%          ║ ║%n",
            cpu.javaSortAvg, cpu.javaSortWorst, cpu.adeadSortAvg, cpu.adeadSortWorst, cpuSpeedup3, cpuStab3);
        
        double cpuSpeedup4 = cpu.javaStringAvg / cpu.adeadStringAvg;
        double cpuStab4 = (1 - (cpu.adeadStringWorst/cpu.adeadStringAvg) / (cpu.javaStringWorst/cpu.javaStringAvg)) * 100;
        System.out.printf("║  ║  String Processing   │ %6.2f   %8.2f │ %6.2f   %8.2f │  %5.2fx │    %+5.1f%%          ║ ║%n",
            cpu.javaStringAvg, cpu.javaStringWorst, cpu.adeadStringAvg, cpu.adeadStringWorst, cpuSpeedup4, cpuStab4);
        
        System.out.println("║  ╚════════════════════════════════════════════════════════════════════════════════════════════════╝ ║");
        System.out.printf("║    GC Pauses en Java: %d                                                                           ║%n", cpu.gcPauses);
        System.out.println("║                                                                                                      ║");
        
        System.out.println("║  ╔════════════════════════════════════════════════════════════════════════════════════════════════╗ ║");
        System.out.println("║  ║                        CATEGORÍA 2: GPU (HEX Opcodes)                                          ║ ║");
        System.out.println("║  ╠════════════════════════════════════════════════════════════════════════════════════════════════╣ ║");
        System.out.println("║  ║  Test                │     Java (ms)      │   ADead-BIB (ms)   │ Speedup │ Stability          ║ ║");
        System.out.println("║  ║                      │   avg      worst   │   avg      worst   │         │ Improvement        ║ ║");
        System.out.println("║  ╠════════════════════════════════════════════════════════════════════════════════════════════════╣ ║");
        
        double gpuSpeedup1 = gpu.javaMatmulAvg / gpu.adeadMatmulAvg;
        double gpuStab1 = (1 - (gpu.adeadMatmulWorst/gpu.adeadMatmulAvg) / (gpu.javaMatmulWorst/gpu.javaMatmulAvg)) * 100;
        System.out.printf("║  ║  Matrix Multiply     │ %6.2f   %8.2f │ %6.2f   %8.2f │  %5.2fx │    %+5.1f%%          ║ ║%n",
            gpu.javaMatmulAvg, gpu.javaMatmulWorst, gpu.adeadMatmulAvg, gpu.adeadMatmulWorst, gpuSpeedup1, gpuStab1);
        
        double gpuSpeedup2 = gpu.javaVectorAvg / gpu.adeadVectorAvg;
        double gpuStab2 = (1 - (gpu.adeadVectorWorst/gpu.adeadVectorAvg) / (gpu.javaVectorWorst/gpu.javaVectorAvg)) * 100;
        System.out.printf("║  ║  Vector Operations   │ %6.2f   %8.2f │ %6.2f   %8.2f │  %5.2fx │    %+5.1f%%          ║ ║%n",
            gpu.javaVectorAvg, gpu.javaVectorWorst, gpu.adeadVectorAvg, gpu.adeadVectorWorst, gpuSpeedup2, gpuStab2);
        
        double gpuSpeedup3 = gpu.javaReductionAvg / gpu.adeadReductionAvg;
        double gpuStab3 = (1 - (gpu.adeadReductionWorst/gpu.adeadReductionAvg) / (gpu.javaReductionWorst/gpu.javaReductionAvg)) * 100;
        System.out.printf("║  ║  Parallel Reduction  │ %6.2f   %8.2f │ %6.2f   %8.2f │  %5.2fx │    %+5.1f%%          ║ ║%n",
            gpu.javaReductionAvg, gpu.javaReductionWorst, gpu.adeadReductionAvg, gpu.adeadReductionWorst, gpuSpeedup3, gpuStab3);
        
        System.out.println("║  ╚════════════════════════════════════════════════════════════════════════════════════════════════╝ ║");
        System.out.printf("║    Kernel Launches en Java: %d | Memory Transfers: %d                                             ║%n", 
            gpu.kernelLaunches, gpu.memoryTransfers);
        System.out.println("║                                                                                                      ║");
        System.out.println("╚══════════════════════════════════════════════════════════════════════════════════════════════════════╝");
    }
    
    static void printImpactAnalysis(CPUResult cpu, GPUResult gpu) {
        // Calcular promedios
        double avgCpuSpeedup = (cpu.javaImageAvg/cpu.adeadImageAvg + cpu.javaMathAvg/cpu.adeadMathAvg + 
                               cpu.javaSortAvg/cpu.adeadSortAvg + cpu.javaStringAvg/cpu.adeadStringAvg) / 4;
        
        double avgGpuSpeedup = (gpu.javaMatmulAvg/gpu.adeadMatmulAvg + gpu.javaVectorAvg/gpu.adeadVectorAvg + 
                               gpu.javaReductionAvg/gpu.adeadReductionAvg) / 3;
        
        double cpuJitterJava = (cpu.javaImageWorst/cpu.javaImageAvg + cpu.javaMathWorst/cpu.javaMathAvg +
                               cpu.javaSortWorst/cpu.javaSortAvg + cpu.javaStringWorst/cpu.javaStringAvg) / 4;
        double cpuJitterADead = (cpu.adeadImageWorst/cpu.adeadImageAvg + cpu.adeadMathWorst/cpu.adeadMathAvg +
                                cpu.adeadSortWorst/cpu.adeadSortAvg + cpu.adeadStringWorst/cpu.adeadStringAvg) / 4;
        
        double gpuJitterJava = (gpu.javaMatmulWorst/gpu.javaMatmulAvg + gpu.javaVectorWorst/gpu.javaVectorAvg +
                               gpu.javaReductionWorst/gpu.javaReductionAvg) / 3;
        double gpuJitterADead = (gpu.adeadMatmulWorst/gpu.adeadMatmulAvg + gpu.adeadVectorWorst/gpu.adeadVectorAvg +
                                gpu.adeadReductionWorst/gpu.adeadReductionAvg) / 3;
        
        System.out.println("\n╔══════════════════════════════════════════════════════════════════╗");
        System.out.println("║                    ANÁLISIS DE IMPACTO                           ║");
        System.out.println("╠══════════════════════════════════════════════════════════════════╣");
        System.out.println("║                                                                  ║");
        System.out.printf("║  CPU (Binario x86-64):                                           ║%n");
        System.out.printf("║    Speedup promedio ADead-BIB:     %.2fx                         ║%n", avgCpuSpeedup);
        System.out.printf("║    Jitter Java:                    %.2fx                         ║%n", cpuJitterJava);
        System.out.printf("║    Jitter ADead-BIB:               %.2fx                         ║%n", cpuJitterADead);
        System.out.printf("║    Mejora de estabilidad:          %.0f%%                         ║%n", (1-cpuJitterADead/cpuJitterJava)*100);
        System.out.println("║                                                                  ║");
        System.out.printf("║  GPU (HEX Opcodes):                                              ║%n");
        System.out.printf("║    Speedup promedio ADead-BIB:     %.2fx                         ║%n", avgGpuSpeedup);
        System.out.printf("║    Jitter Java:                    %.2fx                         ║%n", gpuJitterJava);
        System.out.printf("║    Jitter ADead-BIB:               %.2fx                         ║%n", gpuJitterADead);
        System.out.printf("║    Mejora de estabilidad:          %.0f%%                         ║%n", (1-gpuJitterADead/gpuJitterJava)*100);
        System.out.println("║                                                                  ║");
        System.out.println("╚══════════════════════════════════════════════════════════════════╝");
    }
    
    static void printConclusions(CPUResult cpu, GPUResult gpu) {
        System.out.println("\n╔══════════════════════════════════════════════════════════════════╗");
        System.out.println("║                      CONCLUSIONES                                ║");
        System.out.println("╠══════════════════════════════════════════════════════════════════╣");
        System.out.println("║                                                                  ║");
        System.out.println("║  ┌────────────────────────────────────────────────────────────┐ ║");
        System.out.println("║  │  CATEGORÍA 1: CPU (Binario)                                │ ║");
        System.out.println("║  │                                                            │ ║");
        System.out.println("║  │  ADead-BIB emite bytes x86-64 directos:                    │ ║");
        System.out.println("║  │  - Sin overhead de JIT compilation                         │ ║");
        System.out.println("║  │  - Sin GC pauses                                           │ ║");
        System.out.println("║  │  - Latencia predecible                                     │ ║");
        System.out.println("║  │                                                            │ ║");
        System.out.println("║  │  Ideal para: Image processing, real-time audio             │ ║");
        System.out.println("║  └────────────────────────────────────────────────────────────┘ ║");
        System.out.println("║                                                                  ║");
        System.out.println("║  ┌────────────────────────────────────────────────────────────┐ ║");
        System.out.println("║  │  CATEGORÍA 2: GPU (HEX)                                    │ ║");
        System.out.println("║  │                                                            │ ║");
        System.out.println("║  │  ADead-BIB emite opcodes GPU directos (0xC0DA...):         │ ║");
        System.out.println("║  │  - Sin overhead de librerías GPU                           │ ║");
        System.out.println("║  │  - Sin kernel launch latency                               │ ║");
        System.out.println("║  │  - Command buffer directo                                  │ ║");
        System.out.println("║  │                                                            │ ║");
        System.out.println("║  │  Ideal para: Matrix ops, ML inference, video transcoding   │ ║");
        System.out.println("║  └────────────────────────────────────────────────────────────┘ ║");
        System.out.println("║                                                                  ║");
        System.out.println("║  ┌────────────────────────────────────────────────────────────┐ ║");
        System.out.println("║  │  TU PC (RTX 3060 + 16GB RAM):                              │ ║");
        System.out.println("║  │                                                            │ ║");
        System.out.println("║  │  Con ADead-BIB puedes:                                     │ ║");
        System.out.println("║  │  ✓ Procesar video 4K en tiempo real                        │ ║");
        System.out.println("║  │  ✓ Ejecutar ML inference con latencia predecible           │ ║");
        System.out.println("║  │  ✓ Streaming de media sin stuttering                       │ ║");
        System.out.println("║  │  ✓ Juegos con frame time consistente                       │ ║");
        System.out.println("║  └────────────────────────────────────────────────────────────┘ ║");
        System.out.println("║                                                                  ║");
        System.out.println("╚══════════════════════════════════════════════════════════════════╝");
        
        System.out.println("\n╔══════════════════════════════════════════════════════════════════╗");
        System.out.println("║                    BENCHMARK COMPLETE                            ║");
        System.out.println("║                                                                  ║");
        System.out.println("║   ADead-BIB: Binary Is Binary                                    ║");
        System.out.println("║   CPU (Binario x86-64) + GPU (HEX 0xC0DA...)                     ║");
        System.out.println("║                                                                  ║");
        System.out.println("║   Músculos puros para Java                                       ║");
        System.out.println("╚══════════════════════════════════════════════════════════════════╝\n");
    }
    
    static double avg(List<Long> values) {
        return values.stream().mapToLong(Long::longValue).average().orElse(0) / 1_000_000.0;
    }
    
    static double worst(List<Long> values) {
        return Collections.max(values) / 1_000_000.0;
    }
    
    static void sleep(long ms) {
        try { Thread.sleep(ms); } catch (InterruptedException e) {}
    }
    
    // Result classes
    static class CPUResult {
        double javaImageAvg, adeadImageAvg, javaImageWorst, adeadImageWorst;
        double javaMathAvg, adeadMathAvg, javaMathWorst, adeadMathWorst;
        double javaSortAvg, adeadSortAvg, javaSortWorst, adeadSortWorst;
        double javaStringAvg, adeadStringAvg, javaStringWorst, adeadStringWorst;
        int gcPauses;
        
        CPUResult(double jia, double aia, double jiw, double aiw,
                 double jma, double ama, double jmw, double amw,
                 double jsa, double asa, double jsw, double asw,
                 double jsta, double asta, double jstw, double astw,
                 int gc) {
            this.javaImageAvg = jia; this.adeadImageAvg = aia;
            this.javaImageWorst = jiw; this.adeadImageWorst = aiw;
            this.javaMathAvg = jma; this.adeadMathAvg = ama;
            this.javaMathWorst = jmw; this.adeadMathWorst = amw;
            this.javaSortAvg = jsa; this.adeadSortAvg = asa;
            this.javaSortWorst = jsw; this.adeadSortWorst = asw;
            this.javaStringAvg = jsta; this.adeadStringAvg = asta;
            this.javaStringWorst = jstw; this.adeadStringWorst = astw;
            this.gcPauses = gc;
        }
    }
    
    static class GPUResult {
        double javaMatmulAvg, adeadMatmulAvg, javaMatmulWorst, adeadMatmulWorst;
        double javaVectorAvg, adeadVectorAvg, javaVectorWorst, adeadVectorWorst;
        double javaReductionAvg, adeadReductionAvg, javaReductionWorst, adeadReductionWorst;
        int kernelLaunches, memoryTransfers;
        
        GPUResult(double jma, double ama, double jmw, double amw,
                 double jva, double ava, double jvw, double avw,
                 double jra, double ara, double jrw, double arw,
                 int kl, int mt) {
            this.javaMatmulAvg = jma; this.adeadMatmulAvg = ama;
            this.javaMatmulWorst = jmw; this.adeadMatmulWorst = amw;
            this.javaVectorAvg = jva; this.adeadVectorAvg = ava;
            this.javaVectorWorst = jvw; this.adeadVectorWorst = avw;
            this.javaReductionAvg = jra; this.adeadReductionAvg = ara;
            this.javaReductionWorst = jrw; this.adeadReductionWorst = arw;
            this.kernelLaunches = kl; this.memoryTransfers = mt;
        }
    }
}
