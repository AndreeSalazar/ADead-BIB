import java.util.*;

/**
 * Benchmark Simple: Java vs Java+JIT vs Java+ADead-BIB
 * 
 * Ejecutar:
 *   cd java-head/adead-core/benchmark
 *   javac SimpleBenchmark.java
 *   java SimpleBenchmark
 *   java SimpleBenchmark 200 50   # 200 frames, 50 warmup
 */
public class SimpleBenchmark {

    static final int FRAME_WIDTH = 1920;
    static final int FRAME_HEIGHT = 1080;
    static final int BYTES_PER_PIXEL = 3;
    static final int FRAME_SIZE = FRAME_WIDTH * FRAME_HEIGHT * BYTES_PER_PIXEL;
    
    static Random random = new Random(42);
    
    public static void main(String[] args) {
        int numFrames = args.length > 0 ? Integer.parseInt(args[0]) : 100;
        int warmupFrames = args.length > 1 ? Integer.parseInt(args[1]) : 50;
        
        System.out.println("\n");
        System.out.println("╔══════════════════════════════════════════════════════════════╗");
        System.out.println("║        BENCHMARK: Java vs Java+JIT vs Java+ADead-BIB         ║");
        System.out.println("║        Frame Processing Comparison                           ║");
        System.out.println("╠══════════════════════════════════════════════════════════════╣");
        System.out.println("║  Frame Size: 1920x1080 RGB (6,220,800 bytes)                 ║");
        System.out.printf("║  Frames: %d (+ %d warmup)                                    ║%n", numFrames, warmupFrames);
        System.out.println("╚══════════════════════════════════════════════════════════════╝");
        
        // Generar frames
        System.out.println("\n[1] Generating test frames...");
        byte[][] frames = new byte[numFrames + warmupFrames][];
        for (int i = 0; i < frames.length; i++) {
            frames[i] = generateFrame();
        }
        System.out.println("    Done. Total data: " + (frames.length * FRAME_SIZE / 1024 / 1024) + " MB");
        
        // Warmup
        System.out.println("\n[2] Warming up JIT compiler...");
        for (int i = 0; i < warmupFrames; i++) {
            processJavaPure(frames[i], 10);
            processJavaJIT(frames[i], 10);
            processADeadBIB(frames[i], 10);
        }
        System.out.println("    Warmup complete.");
        
        // Métricas
        List<Long> javaPureTimes = new ArrayList<>();
        List<Long> javaJITTimes = new ArrayList<>();
        List<Long> adeadTimes = new ArrayList<>();
        
        // Java Pure
        System.out.println("\n[3] Running Java Pure benchmark...");
        System.gc();
        sleep(100);
        for (int i = warmupFrames; i < frames.length; i++) {
            long start = System.nanoTime();
            processJavaPure(frames[i], 10);
            javaPureTimes.add(System.nanoTime() - start);
        }
        
        // Java JIT
        System.out.println("[4] Running Java + JIT benchmark...");
        System.gc();
        sleep(100);
        for (int i = warmupFrames; i < frames.length; i++) {
            long start = System.nanoTime();
            processJavaJIT(frames[i], 10);
            javaJITTimes.add(System.nanoTime() - start);
        }
        
        // ADead-BIB
        System.out.println("[5] Running Java + ADead-BIB benchmark...");
        System.gc();
        sleep(100);
        for (int i = warmupFrames; i < frames.length; i++) {
            long start = System.nanoTime();
            processADeadBIB(frames[i], 10);
            adeadTimes.add(System.nanoTime() - start);
        }
        
        // Resultados
        System.out.println("\n" + "=".repeat(66));
        System.out.println("                         RESULTS");
        System.out.println("=".repeat(66));
        
        printMetrics("Java Pure", javaPureTimes);
        printMetrics("Java + JIT (warmed)", javaJITTimes);
        printMetrics("Java + ADead-BIB", adeadTimes);
        
        // Comparación
        System.out.println("\n╔══════════════════════════════════════════════════════════════╗");
        System.out.println("║                      COMPARISON                              ║");
        System.out.println("╠══════════════════════════════════════════════════════════════╣");
        System.out.println("║  Metric              Java Pure   Java+JIT   Java+ADead-BIB   ║");
        System.out.println("╠══════════════════════════════════════════════════════════════╣");
        System.out.printf("║  Exec p95 (ms)       %8.3f   %8.3f   %8.3f         ║%n",
            percentile(javaPureTimes, 95), percentile(javaJITTimes, 95), percentile(adeadTimes, 95));
        System.out.printf("║  Exec p99 (ms)       %8.3f   %8.3f   %8.3f         ║%n",
            percentile(javaPureTimes, 99), percentile(javaJITTimes, 99), percentile(adeadTimes, 99));
        System.out.printf("║  Exec worst (ms)     %8.3f   %8.3f   %8.3f         ║%n",
            worst(javaPureTimes), worst(javaJITTimes), worst(adeadTimes));
        System.out.println("╚══════════════════════════════════════════════════════════════╝");
        
        // Análisis de estabilidad
        double jitterPure = worst(javaPureTimes) / avg(javaPureTimes);
        double jitterJIT = worst(javaJITTimes) / avg(javaJITTimes);
        double jitterADead = worst(adeadTimes) / avg(adeadTimes);
        
        System.out.println("\n╔══════════════════════════════════════════════════════════════╗");
        System.out.println("║                 STABILITY ANALYSIS                           ║");
        System.out.println("╠══════════════════════════════════════════════════════════════╣");
        System.out.printf("║  Java Pure jitter ratio (worst/avg):     %.2fx              ║%n", jitterPure);
        System.out.printf("║  Java+JIT jitter ratio (worst/avg):      %.2fx              ║%n", jitterJIT);
        System.out.printf("║  Java+ADead-BIB jitter ratio:            %.2fx              ║%n", jitterADead);
        System.out.println("║                                                              ║");
        System.out.println("║  NOTE: Lower ratio = more predictable/stable                 ║");
        System.out.println("║        ADead-BIB goal: deterministic execution               ║");
        System.out.println("╚══════════════════════════════════════════════════════════════╝");
        
        System.out.println("\n╔══════════════════════════════════════════════════════════════╗");
        System.out.println("║                    BENCHMARK COMPLETE                        ║");
        System.out.println("╚══════════════════════════════════════════════════════════════╝\n");
    }
    
    static byte[] generateFrame() {
        byte[] frame = new byte[FRAME_SIZE];
        random.nextBytes(frame);
        return frame;
    }
    
    // Java Pure - sin optimizaciones especiales
    static byte[] processJavaPure(byte[] frame, int brightness) {
        byte[] output = new byte[frame.length];
        for (int i = 0; i < frame.length; i++) {
            int pixel = frame[i] & 0xFF;
            pixel = Math.min(255, Math.max(0, pixel + brightness));
            output[i] = (byte) pixel;
        }
        return output;
    }
    
    // Java JIT - mismo código, pero después de warmup
    static byte[] processJavaJIT(byte[] frame, int brightness) {
        return processJavaPure(frame, brightness);
    }
    
    // ADead-BIB simulado - procesamiento directo estilo bytes
    static byte[] processADeadBIB(byte[] frame, int brightness) {
        byte[] output = new byte[frame.length];
        // Simula procesamiento directo sin overhead Java
        // En producción real: JNI a código ADead-BIB compilado
        for (int i = 0; i < frame.length; i++) {
            int pixel = (frame[i] & 0xFF) + brightness;
            output[i] = (byte) (pixel > 255 ? 255 : (pixel < 0 ? 0 : pixel));
        }
        return output;
    }
    
    static void printMetrics(String name, List<Long> times) {
        System.out.println("\n╔══════════════════════════════════════════════════════════════╗");
        System.out.printf("║  METRICS: %-50s ║%n", name);
        System.out.println("╠══════════════════════════════════════════════════════════════╣");
        System.out.printf("║  Total Frames: %-46d ║%n", times.size());
        System.out.println("╠══════════════════════════════════════════════════════════════╣");
        System.out.println("║  EXECUTION TIME (ms)                                         ║");
        System.out.printf("║    avg: %8.3f | p50: %8.3f | p95: %8.3f           ║%n",
            avg(times), percentile(times, 50), percentile(times, 95));
        System.out.printf("║    p99: %8.3f | worst: %8.3f                       ║%n",
            percentile(times, 99), worst(times));
        System.out.println("╚══════════════════════════════════════════════════════════════╝");
    }
    
    static double avg(List<Long> values) {
        return values.stream().mapToLong(Long::longValue).average().orElse(0) / 1_000_000.0;
    }
    
    static double percentile(List<Long> values, int p) {
        List<Long> sorted = new ArrayList<>(values);
        Collections.sort(sorted);
        int index = (int) Math.ceil(p / 100.0 * sorted.size()) - 1;
        index = Math.max(0, Math.min(index, sorted.size() - 1));
        return sorted.get(index) / 1_000_000.0;
    }
    
    static double worst(List<Long> values) {
        return Collections.max(values) / 1_000_000.0;
    }
    
    static void sleep(long ms) {
        try { Thread.sleep(ms); } catch (InterruptedException e) {}
    }
}
