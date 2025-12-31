package com.adead.media.model;

import jakarta.persistence.*;
import lombok.Data;
import lombok.NoArgsConstructor;
import lombok.AllArgsConstructor;

import java.time.LocalDateTime;

/**
 * Entidad Media - Representa un contenido de video/audio
 */
@Entity
@Table(name = "media")
@Data
@NoArgsConstructor
@AllArgsConstructor
public class Media {

    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Long id;

    @Column(nullable = false)
    private String title;

    @Column(length = 2000)
    private String description;

    @Column(nullable = false)
    private String type; // VIDEO, AUDIO, SERIES, MOVIE

    @Column(nullable = false)
    private String status; // PENDING, PROCESSING, READY, ERROR

    private String originalPath;
    
    private String processedPath;

    private Long durationSeconds;

    private Integer width;
    
    private Integer height;

    private String codec;

    private Long fileSizeBytes;

    private String thumbnailPath;

    @Column(nullable = false)
    private LocalDateTime createdAt;

    private LocalDateTime processedAt;

    @PrePersist
    protected void onCreate() {
        createdAt = LocalDateTime.now();
        if (status == null) {
            status = "PENDING";
        }
    }
}
