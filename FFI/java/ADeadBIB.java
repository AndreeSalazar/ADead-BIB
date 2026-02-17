/**
 * ADead-BIB FFI Java Bindings
 * ===========================
 * Author: Eddi Andreé Salazar Matos
 * Email: eddi.salazar.dev@gmail.com
 * Made with ❤️ in Peru 🇵🇪
 * 
 * ADead-BIB como cabeza principal ABI para Java.
 * Permite usar funciones compiladas de ADead-BIB desde Java.
 */

package adead;

import java.io.*;
import java.nio.file.*;

public class ADeadBIB {
    
    private String compilerPath;
    private String baseDir;
    
    /**
     * Constructor - busca el compilador ADead-BIB
     */
    public ADeadBIB() {
        this.baseDir = System.getProperty("user.dir");
        
        String[] possiblePaths = {
            baseDir + "/target/release/adeadc.exe",
            baseDir + "/target/debug/adeadc.exe",
            "adeadc.exe"
        };
        
        for (String path : possiblePaths) {
            if (new File(path).exists()) {
                this.compilerPath = path;
                break;
            }
        }
    }
    
    /**
     * Constructor con ruta específica
     */
    public ADeadBIB(String compilerPath) {
        this.compilerPath = compilerPath;
        this.baseDir = new File(compilerPath).getParent();
    }
    
    /**
     * Compila y ejecuta código ADead-BIB
     */
    public String run(String sourceFile) throws Exception {
        ProcessBuilder pb = new ProcessBuilder(compilerPath, "run", sourceFile);
        pb.redirectErrorStream(true);
        Process process = pb.start();
        
        BufferedReader reader = new BufferedReader(
            new InputStreamReader(process.getInputStream())
        );
        
        StringBuilder output = new StringBuilder();
        String line;
        while ((line = reader.readLine()) != null) {
            output.append(line).append("\n");
        }
        
        process.waitFor();
        return output.toString();
    }
    
    /**
     * Ejecuta código ADead-BIB desde string
     */
    public String runCode(String code) throws Exception {
        Path tempFile = Files.createTempFile("adead_", ".adB");
        Files.writeString(tempFile, code);
        
        try {
            return run(tempFile.toString());
        } finally {
            Files.deleteIfExists(tempFile);
        }
    }
    
    // ============================================
    // FUNCIONES MATEMÁTICAS (usando ABI)
    // ============================================
    
    /**
     * Suma usando ADead-BIB ABI
     */
    public long sum(long a, long b) throws Exception {
        String code = String.format("""
            int main() {
                printf(%d + %d)
                return 0
            }
            """, a, b);
        
        String result = runCode(code).trim();
        return result.isEmpty() ? 0 : Long.parseLong(result);
    }
    
    /**
     * Multiplicación usando ADead-BIB ABI
     */
    public long mul(long a, long b) throws Exception {
        String code = String.format("""
            int main() {
                printf(%d * %d)
                return 0
            }
            """, a, b);
        
        String result = runCode(code).trim();
        return result.isEmpty() ? 0 : Long.parseLong(result);
    }
    
    /**
     * Factorial usando ADead-BIB ABI
     */
    public long factorial(int n) throws Exception {
        String code = String.format("""
            int factorial(int n) {
                if n <= 1 {
                    return 1
                }
                return n * factorial(n - 1)
            }
            
            int main() {
                printf(factorial(%d))
                return 0
            }
            """, n);
        
        String result = runCode(code).trim();
        return result.isEmpty() ? 0 : Long.parseLong(result);
    }
    
    /**
     * Versión del ABI
     */
    public int version() {
        return 320; // v3.2.0
    }
    
    // ============================================
    // EJEMPLO DE USO
    // ============================================
    
    public static void main(String[] args) {
        System.out.println("=== ADead-BIB FFI Java Demo ===\n");
        
        try {
            ADeadBIB adead = new ADeadBIB();
            System.out.println("✓ FFI inicializado");
            
            System.out.println("\nadead.sum(10, 20) = " + adead.sum(10, 20));
            System.out.println("adead.mul(5, 6) = " + adead.mul(5, 6));
            System.out.println("adead.factorial(5) = " + adead.factorial(5));
            System.out.println("adead.version() = " + adead.version());
            
            System.out.println("\n=== FFI Java Funcionando ===");
            
        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
        }
    }
}
