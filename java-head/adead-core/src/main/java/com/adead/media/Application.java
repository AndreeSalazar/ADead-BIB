package com.adead.media;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

/**
 * ADead-Core Media Platform
 * 
 * Plataforma de media experimental que combina:
 * - Java (Spring Boot) para backend/API
 * - ADead-BIB para procesamiento de alto rendimiento
 * 
 * @author Eddi Andreé Salazar Matos
 */
@SpringBootApplication
public class Application {

    public static void main(String[] args) {
        System.out.println("╔══════════════════════════════════════════════════════════╗");
        System.out.println("║        ADead-Core Media Platform v0.1.0                  ║");
        System.out.println("║        Java + ADead-BIB = High Performance Media         ║");
        System.out.println("╚══════════════════════════════════════════════════════════╝");
        
        SpringApplication.run(Application.class, args);
    }
}
