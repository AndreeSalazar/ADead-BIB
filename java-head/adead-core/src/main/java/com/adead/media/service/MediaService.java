package com.adead.media.service;

import com.adead.media.model.Media;
import com.adead.media.repository.MediaRepository;
import com.adead.media.native_.ADeadBridge;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Service;

import java.util.List;
import java.util.Optional;

/**
 * Servicio de Media - Lógica de negocio
 */
@Service
@RequiredArgsConstructor
@Slf4j
public class MediaService {

    private final MediaRepository mediaRepository;
    private final ADeadBridge adeadBridge;

    public List<Media> findAll() {
        return mediaRepository.findAll();
    }

    public Optional<Media> findById(Long id) {
        return mediaRepository.findById(id);
    }

    public Media save(Media media) {
        return mediaRepository.save(media);
    }

    public void deleteById(Long id) {
        mediaRepository.deleteById(id);
    }

    /**
     * Inicia el proceso de transcoding usando ADead-BIB
     */
    public Optional<String> transcode(Long id) {
        return findById(id).map(media -> {
            log.info("Starting transcoding for media: {}", media.getTitle());
            
            // Actualizar estado
            media.setStatus("PROCESSING");
            mediaRepository.save(media);
            
            // Llamar a ADead-BIB para transcoding
            // Por ahora es un placeholder - se implementará con JNI/CLI
            String result = adeadBridge.transcodeVideo(
                media.getOriginalPath(),
                "720p"
            );
            
            if (result != null && result.startsWith("SUCCESS")) {
                media.setStatus("READY");
                media.setProcessedPath(result.split(":")[1]);
            } else {
                media.setStatus("ERROR");
            }
            
            mediaRepository.save(media);
            return result;
        });
    }

    public List<Media> findByStatus(String status) {
        return mediaRepository.findByStatus(status);
    }

    public List<Media> findByType(String type) {
        return mediaRepository.findByType(type);
    }
}
