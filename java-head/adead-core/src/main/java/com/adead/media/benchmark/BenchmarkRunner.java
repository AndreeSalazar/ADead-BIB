package com.adead.media.benchmark;

/**
 * Runner para ejecutar el benchmark desde línea de comandos
 * 
 * Uso: java -jar adead-core.jar --benchmark
 *      java -jar adead-core.jar --benchmark --frames 1000
 */
public class BenchmarkRunner {

    public static void main(String[] args) {
        System.out.println("\n");
        System.out.println("╔══════════════════════════════════════════════════════════════╗");
        System.out.println("║        ADead-Core Media Platform - Benchmark Suite           ║");
        System.out.println("║        Java vs Java+JIT vs Java+ADead-BIB                    ║");
        System.out.println("╚══════════════════════════════════════════════════════════════╝");
        
        int numFrames = 100;      // Frames a medir
        int warmupFrames = 50;    // Frames de warmup para JIT
        
        // Parsear argumentos
        for (int i = 0; i < args.length; i++) {
            if ("--frames".equals(args[i]) && i + 1 < args.length) {
                numFrames = Integer.parseInt(args[i + 1]);
            }
            if ("--warmup".equals(args[i]) && i + 1 < args.length) {
                warmupFrames = Integer.parseInt(args[i + 1]);
            }
        }
        
        FrameProcessor processor = new FrameProcessor();
        processor.runBenchmark(numFrames, warmupFrames);
        
        System.out.println("\n");
        System.out.println("╔══════════════════════════════════════════════════════════════╗");
        System.out.println("║                    BENCHMARK COMPLETE                        ║");
        System.out.println("╚══════════════════════════════════════════════════════════════╝");
    }
}
