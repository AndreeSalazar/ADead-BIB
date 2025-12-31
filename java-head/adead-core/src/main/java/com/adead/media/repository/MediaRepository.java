package com.adead.media.repository;

import com.adead.media.model.Media;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

import java.util.List;

/**
 * Repositorio de Media - Acceso a datos
 */
@Repository
public interface MediaRepository extends JpaRepository<Media, Long> {

    List<Media> findByStatus(String status);
    
    List<Media> findByType(String type);
    
    List<Media> findByTitleContainingIgnoreCase(String title);
}
