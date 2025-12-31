package com.adead.media.api;

import com.adead.media.model.Media;
import com.adead.media.service.MediaService;
import lombok.RequiredArgsConstructor;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.util.List;

/**
 * REST Controller para gestión de Media
 */
@RestController
@RequestMapping("/api/media")
@RequiredArgsConstructor
public class MediaController {

    private final MediaService mediaService;

    @GetMapping
    public ResponseEntity<List<Media>> getAllMedia() {
        return ResponseEntity.ok(mediaService.findAll());
    }

    @GetMapping("/{id}")
    public ResponseEntity<Media> getMedia(@PathVariable Long id) {
        return mediaService.findById(id)
                .map(ResponseEntity::ok)
                .orElse(ResponseEntity.notFound().build());
    }

    @PostMapping
    public ResponseEntity<Media> createMedia(@RequestBody Media media) {
        return ResponseEntity.ok(mediaService.save(media));
    }

    @PostMapping("/{id}/transcode")
    public ResponseEntity<String> transcodeMedia(@PathVariable Long id) {
        // Aquí se llamará a ADead-BIB para transcoding
        return mediaService.transcode(id)
                .map(result -> ResponseEntity.ok("Transcoding started: " + result))
                .orElse(ResponseEntity.notFound().build());
    }

    @DeleteMapping("/{id}")
    public ResponseEntity<Void> deleteMedia(@PathVariable Long id) {
        mediaService.deleteById(id);
        return ResponseEntity.noContent().build();
    }
}
