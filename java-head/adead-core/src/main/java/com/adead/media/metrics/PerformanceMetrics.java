package com.adead.media.metrics;

import lombok.Getter;
import lombok.extern.slf4j.Slf4j;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.concurrent.CopyOnWriteArrayList;

/**
 * Sistema de Métricas de Rendimiento
 * 
 * Mide:
 * - Tiempo de llamada (call latency)
 * - Tiempo de ejecución (execution time)
 * - Jitter entre frames
 * - Percentiles: p50, p95, p99, worst
 */
@Slf4j
public class PerformanceMetrics {

    private final String name;
    
    @Getter
    private final List<Long> callLatencies = new CopyOnWriteArrayList<>();
    
    @Getter
    private final List<Long> executionTimes = new CopyOnWriteArrayList<>();
    
    @Getter
    private final List<Long> jitterValues = new CopyOnWriteArrayList<>();
    
    private long lastFrameTime = 0;
    private long totalFrames = 0;
    
    public PerformanceMetrics(String name) {
        this.name = name;
    }
    
    /**
     * Registra una medición completa de un frame
     */
    public void recordFrame(long callLatencyNs, long executionTimeNs) {
        callLatencies.add(callLatencyNs);
        executionTimes.add(executionTimeNs);
        
        // Calcular jitter (variación respecto al frame anterior)
        long currentTime = System.nanoTime();
        if (lastFrameTime > 0) {
            long frameInterval = currentTime - lastFrameTime;
            long expectedInterval = executionTimeNs; // Idealmente igual
            long jitter = Math.abs(frameInterval - expectedInterval);
            jitterValues.add(jitter);
        }
        lastFrameTime = currentTime;
        totalFrames++;
    }
    
    /**
     * Calcula percentil de una lista de valores
     */
    private long percentile(List<Long> values, double percentile) {
        if (values.isEmpty()) return 0;
        
        List<Long> sorted = new ArrayList<>(values);
        Collections.sort(sorted);
        
        int index = (int) Math.ceil(percentile / 100.0 * sorted.size()) - 1;
        index = Math.max(0, Math.min(index, sorted.size() - 1));
        return sorted.get(index);
    }
    
    /**
     * Calcula promedio de una lista
     */
    private double average(List<Long> values) {
        if (values.isEmpty()) return 0;
        return values.stream().mapToLong(Long::longValue).average().orElse(0);
    }
    
    /**
     * Obtiene el peor caso (máximo)
     */
    private long worst(List<Long> values) {
        if (values.isEmpty()) return 0;
        return Collections.max(values);
    }
    
    /**
     * Obtiene el mejor caso (mínimo)
     */
    private long best(List<Long> values) {
        if (values.isEmpty()) return 0;
        return Collections.min(values);
    }
    
    /**
     * Genera reporte completo de métricas
     */
    public MetricsReport generateReport() {
        return new MetricsReport(
            name,
            totalFrames,
            
            // Call Latency
            nsToMs(average(callLatencies)),
            nsToMs(percentile(callLatencies, 50)),
            nsToMs(percentile(callLatencies, 95)),
            nsToMs(percentile(callLatencies, 99)),
            nsToMs(worst(callLatencies)),
            
            // Execution Time
            nsToMs(average(executionTimes)),
            nsToMs(percentile(executionTimes, 50)),
            nsToMs(percentile(executionTimes, 95)),
            nsToMs(percentile(executionTimes, 99)),
            nsToMs(worst(executionTimes)),
            
            // Jitter
            nsToMs(average(jitterValues)),
            nsToMs(percentile(jitterValues, 95)),
            nsToMs(worst(jitterValues))
        );
    }
    
    private double nsToMs(double ns) {
        return ns / 1_000_000.0;
    }
    
    private double nsToMs(long ns) {
        return ns / 1_000_000.0;
    }
    
    /**
     * Limpia las métricas
     */
    public void reset() {
        callLatencies.clear();
        executionTimes.clear();
        jitterValues.clear();
        lastFrameTime = 0;
        totalFrames = 0;
    }
    
    /**
     * Imprime reporte en consola
     */
    public void printReport() {
        MetricsReport report = generateReport();
        System.out.println(report.toFormattedString());
    }
    
    /**
     * Clase interna para el reporte
     */
    public record MetricsReport(
        String name,
        long totalFrames,
        
        // Call Latency (ms)
        double callLatencyAvg,
        double callLatencyP50,
        double callLatencyP95,
        double callLatencyP99,
        double callLatencyWorst,
        
        // Execution Time (ms)
        double execTimeAvg,
        double execTimeP50,
        double execTimeP95,
        double execTimeP99,
        double execTimeWorst,
        
        // Jitter (ms)
        double jitterAvg,
        double jitterP95,
        double jitterWorst
    ) {
        public String toFormattedString() {
            StringBuilder sb = new StringBuilder();
            sb.append("\n");
            sb.append("╔══════════════════════════════════════════════════════════════╗\n");
            sb.append(String.format("║  METRICS: %-50s ║\n", name));
            sb.append("╠══════════════════════════════════════════════════════════════╣\n");
            sb.append(String.format("║  Total Frames: %-46d ║\n", totalFrames));
            sb.append("╠══════════════════════════════════════════════════════════════╣\n");
            sb.append("║  CALL LATENCY (ms)                                           ║\n");
            sb.append(String.format("║    avg: %8.3f | p50: %8.3f | p95: %8.3f           ║\n", 
                callLatencyAvg, callLatencyP50, callLatencyP95));
            sb.append(String.format("║    p99: %8.3f | worst: %8.3f                       ║\n", 
                callLatencyP99, callLatencyWorst));
            sb.append("╠══════════════════════════════════════════════════════════════╣\n");
            sb.append("║  EXECUTION TIME (ms)                                         ║\n");
            sb.append(String.format("║    avg: %8.3f | p50: %8.3f | p95: %8.3f           ║\n", 
                execTimeAvg, execTimeP50, execTimeP95));
            sb.append(String.format("║    p99: %8.3f | worst: %8.3f                       ║\n", 
                execTimeP99, execTimeWorst));
            sb.append("╠══════════════════════════════════════════════════════════════╣\n");
            sb.append("║  JITTER (ms)                                                 ║\n");
            sb.append(String.format("║    avg: %8.3f | p95: %8.3f | worst: %8.3f        ║\n", 
                jitterAvg, jitterP95, jitterWorst));
            sb.append("╚══════════════════════════════════════════════════════════════╝\n");
            return sb.toString();
        }
    }
}
