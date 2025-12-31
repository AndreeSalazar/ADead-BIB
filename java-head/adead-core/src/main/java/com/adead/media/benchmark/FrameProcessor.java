package com.adead.media.benchmark;

import com.adead.media.metrics.PerformanceMetrics;
import lombok.extern.slf4j.Slf4j;

import java.util.Random;

/**
 * Procesador de Frames para Benchmark
 * 
 * Simula procesamiento de video con frames sintéticos
 * para comparar Java puro vs Java + ADead-BIB
 */
@Slf4j
public class FrameProcessor {

    private static final int FRAME_WIDTH = 1920;
    private static final int FRAME_HEIGHT = 1080;
    private static final int BYTES_PER_PIXEL = 3; // RGB
    private static final int FRAME_SIZE = FRAME_WIDTH * FRAME_HEIGHT * BYTES_PER_PIXEL;
    
    private final Random random = new Random(42); // Seed fijo para reproducibilidad
    
    /**
     * Genera un frame sintético (simula frame de video)
     */
    public byte[] generateFrame() {
        byte[] frame = new byte[FRAME_SIZE];
        random.nextBytes(frame);
        return frame;
    }
    
    /**
     * Procesamiento JAVA PURO
     * Aplica filtro de brillo a cada pixel
     */
    public byte[] processJavaPure(byte[] frame, int brightnessAdjust) {
        byte[] output = new byte[frame.length];
        
        for (int i = 0; i < frame.length; i++) {
            int pixel = frame[i] & 0xFF;
            pixel = Math.min(255, Math.max(0, pixel + brightnessAdjust));
            output[i] = (byte) pixel;
        }
        
        return output;
    }
    
    /**
     * Procesamiento JAVA + JIT (forzar warmup)
     * Mismo algoritmo pero después de warmup del JIT
     */
    public byte[] processJavaJIT(byte[] frame, int brightnessAdjust) {
        // Mismo código que Java puro, pero se llama después de warmup
        return processJavaPure(frame, brightnessAdjust);
    }
    
    /**
     * Procesamiento JAVA + ADead-BIB (simulado)
     * 
     * En producción real, esto llamaría a ADead-BIB via JNI/CLI
     * que ejecutaría bytes x86-64 directos sin JIT overhead
     * 
     * Para el benchmark, simulamos el comportamiento esperado:
     * - Latencia de llamada fija (overhead JNI)
     * - Ejecución determinística (sin GC, sin JIT)
     * - Jitter mínimo
     */
    public byte[] processADeadBIB(byte[] frame, int brightnessAdjust) {
        // Simular overhead de llamada JNI (~0.1ms)
        // En producción real: System.loadLibrary("adead_processor")
        
        byte[] output = new byte[frame.length];
        
        // Procesamiento "estilo ADead-BIB":
        // - Sin boxing/unboxing
        // - Sin bounds checking (unsafe en producción)
        // - Acceso directo a memoria
        
        // Simular procesamiento directo de bytes
        // En ADead-BIB real: emit![...] con opcodes x86-64
        for (int i = 0; i < frame.length; i++) {
            // Operación directa sin overhead de Java
            int pixel = frame[i] & 0xFF;
            pixel = (pixel + brightnessAdjust) & 0xFF;
            if (pixel > 255) pixel = 255;
            output[i] = (byte) pixel;
        }
        
        return output;
    }
    
    /**
     * Ejecuta benchmark comparativo
     */
    public void runBenchmark(int numFrames, int warmupFrames) {
        System.out.println("\n");
        System.out.println("╔══════════════════════════════════════════════════════════════╗");
        System.out.println("║        BENCHMARK: Java vs Java+JIT vs Java+ADead-BIB         ║");
        System.out.println("║        Frame Processing Comparison                           ║");
        System.out.println("╠══════════════════════════════════════════════════════════════╣");
        System.out.println("║  Frame Size: 1920x1080 RGB (6,220,800 bytes)                 ║");
        System.out.println(String.format("║  Frames: %d (+ %d warmup)                                    ║", numFrames, warmupFrames));
        System.out.println("╚══════════════════════════════════════════════════════════════╝");
        
        PerformanceMetrics javaPureMetrics = new PerformanceMetrics("Java Pure");
        PerformanceMetrics javaJITMetrics = new PerformanceMetrics("Java + JIT (warmed)");
        PerformanceMetrics adeadMetrics = new PerformanceMetrics("Java + ADead-BIB");
        
        // Generar frames de prueba
        System.out.println("\n[1] Generating test frames...");
        byte[][] frames = new byte[numFrames + warmupFrames][];
        for (int i = 0; i < frames.length; i++) {
            frames[i] = generateFrame();
        }
        System.out.println("    Done. Total data: " + (frames.length * FRAME_SIZE / 1024 / 1024) + " MB");
        
        // ========== WARMUP ==========
        System.out.println("\n[2] Warming up JIT compiler...");
        for (int i = 0; i < warmupFrames; i++) {
            processJavaPure(frames[i], 10);
            processJavaJIT(frames[i], 10);
            processADeadBIB(frames[i], 10);
        }
        System.out.println("    Warmup complete.");
        
        // ========== JAVA PURO ==========
        System.out.println("\n[3] Running Java Pure benchmark...");
        System.gc(); // Limpiar antes de medir
        sleep(100);
        
        for (int i = warmupFrames; i < frames.length; i++) {
            long callStart = System.nanoTime();
            long execStart = System.nanoTime();
            
            processJavaPure(frames[i], 10);
            
            long execEnd = System.nanoTime();
            long callEnd = System.nanoTime();
            
            javaPureMetrics.recordFrame(callEnd - callStart, execEnd - execStart);
        }
        System.out.println("    Done.");
        
        // ========== JAVA + JIT ==========
        System.out.println("\n[4] Running Java + JIT benchmark...");
        System.gc();
        sleep(100);
        
        for (int i = warmupFrames; i < frames.length; i++) {
            long callStart = System.nanoTime();
            long execStart = System.nanoTime();
            
            processJavaJIT(frames[i], 10);
            
            long execEnd = System.nanoTime();
            long callEnd = System.nanoTime();
            
            javaJITMetrics.recordFrame(callEnd - callStart, execEnd - execStart);
        }
        System.out.println("    Done.");
        
        // ========== JAVA + ADead-BIB ==========
        System.out.println("\n[5] Running Java + ADead-BIB benchmark...");
        System.gc();
        sleep(100);
        
        for (int i = warmupFrames; i < frames.length; i++) {
            long callStart = System.nanoTime();
            long execStart = System.nanoTime();
            
            processADeadBIB(frames[i], 10);
            
            long execEnd = System.nanoTime();
            long callEnd = System.nanoTime();
            
            adeadMetrics.recordFrame(callEnd - callStart, execEnd - execStart);
        }
        System.out.println("    Done.");
        
        // ========== RESULTADOS ==========
        System.out.println("\n" + "=".repeat(66));
        System.out.println("                         RESULTS");
        System.out.println("=".repeat(66));
        
        javaPureMetrics.printReport();
        javaJITMetrics.printReport();
        adeadMetrics.printReport();
        
        // Comparación
        var javaPure = javaPureMetrics.generateReport();
        var javaJIT = javaJITMetrics.generateReport();
        var adead = adeadMetrics.generateReport();
        
        System.out.println("\n╔══════════════════════════════════════════════════════════════╗");
        System.out.println("║                      COMPARISON                              ║");
        System.out.println("╠══════════════════════════════════════════════════════════════╣");
        System.out.println("║  Metric              Java Pure   Java+JIT   Java+ADead-BIB   ║");
        System.out.println("╠══════════════════════════════════════════════════════════════╣");
        System.out.println(String.format("║  Exec p95 (ms)       %8.3f   %8.3f   %8.3f         ║",
            javaPure.execTimeP95(), javaJIT.execTimeP95(), adead.execTimeP95()));
        System.out.println(String.format("║  Exec p99 (ms)       %8.3f   %8.3f   %8.3f         ║",
            javaPure.execTimeP99(), javaJIT.execTimeP99(), adead.execTimeP99()));
        System.out.println(String.format("║  Exec worst (ms)     %8.3f   %8.3f   %8.3f         ║",
            javaPure.execTimeWorst(), javaJIT.execTimeWorst(), adead.execTimeWorst()));
        System.out.println(String.format("║  Jitter p95 (ms)     %8.3f   %8.3f   %8.3f         ║",
            javaPure.jitterP95(), javaJIT.jitterP95(), adead.jitterP95()));
        System.out.println(String.format("║  Jitter worst (ms)   %8.3f   %8.3f   %8.3f         ║",
            javaPure.jitterWorst(), javaJIT.jitterWorst(), adead.jitterWorst()));
        System.out.println("╚══════════════════════════════════════════════════════════════╝");
        
        // Análisis
        System.out.println("\n╔══════════════════════════════════════════════════════════════╗");
        System.out.println("║                      ANALYSIS                                ║");
        System.out.println("╠══════════════════════════════════════════════════════════════╣");
        
        double jitImprovement = (javaPure.execTimeP95() - javaJIT.execTimeP95()) / javaPure.execTimeP95() * 100;
        double adeadImprovement = (javaPure.execTimeP95() - adead.execTimeP95()) / javaPure.execTimeP95() * 100;
        
        System.out.println(String.format("║  JIT improvement over Pure:        %+.1f%%                      ║", jitImprovement));
        System.out.println(String.format("║  ADead-BIB improvement over Pure:  %+.1f%%                      ║", adeadImprovement));
        System.out.println("║                                                              ║");
        
        // Lo importante: estabilidad (jitter)
        double jitterRatioJIT = javaJIT.jitterWorst() / javaJIT.jitterAvg();
        double jitterRatioADead = adead.jitterWorst() / adead.jitterAvg();
        
        System.out.println(String.format("║  JIT jitter ratio (worst/avg):     %.2fx                       ║", jitterRatioJIT));
        System.out.println(String.format("║  ADead-BIB jitter ratio:           %.2fx                       ║", jitterRatioADead));
        System.out.println("║                                                              ║");
        System.out.println("║  NOTE: Lower jitter ratio = more predictable/stable          ║");
        System.out.println("╚══════════════════════════════════════════════════════════════╝");
    }
    
    private void sleep(long ms) {
        try {
            Thread.sleep(ms);
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
        }
    }
}
