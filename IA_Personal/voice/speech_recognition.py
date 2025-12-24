"""
Reconocimiento de Voz para IA-Personal
=======================================
Author: Eddi Andreé Salazar Matos
Email: eddi.salazar.dev@gmail.com
Made with ❤️ in Peru 🇵🇪

Sistema de reconocimiento de voz:
- Escucha continua del micrófono
- Reconocimiento de voz en español/inglés
- Detección de palabra clave (wake word)
- Integración con IA-Personal

Requisitos:
    pip install SpeechRecognition pyaudio
    
    # En Windows también:
    pip install pipwin
    pipwin install pyaudio
"""

import os
import sys
import time
import threading
import queue
from pathlib import Path
from typing import Optional, Callable, List
from dataclasses import dataclass
from enum import Enum

# Intentar importar speech_recognition
try:
    import speech_recognition as sr
    HAS_SPEECH = True
except ImportError:
    HAS_SPEECH = False
    print("⚠️ speech_recognition no instalado: pip install SpeechRecognition pyaudio")


class RecognitionEngine(Enum):
    """Motores de reconocimiento disponibles."""
    GOOGLE = "google"           # Google Speech Recognition (gratis, requiere internet)
    GOOGLE_CLOUD = "google_cloud"  # Google Cloud (de pago)
    SPHINX = "sphinx"           # CMU Sphinx (offline, menos preciso)
    WHISPER = "whisper"         # OpenAI Whisper (offline, muy preciso)


@dataclass
class VoiceConfig:
    """Configuración del reconocimiento de voz."""
    # Motor
    engine: RecognitionEngine = RecognitionEngine.GOOGLE
    language: str = "es-ES"  # Español de España
    
    # Wake word (palabra para activar)
    wake_word: str = "asistente"
    wake_word_enabled: bool = True
    
    # Micrófono
    device_index: Optional[int] = None  # None = micrófono por defecto
    sample_rate: int = 16000
    chunk_size: int = 1024
    
    # Detección
    energy_threshold: int = 300  # Umbral de energía para detectar voz
    dynamic_energy: bool = True  # Ajustar umbral automáticamente
    pause_threshold: float = 0.8  # Segundos de silencio para terminar frase
    phrase_time_limit: float = 10.0  # Máximo segundos por frase
    
    # Callbacks
    timeout: float = 5.0  # Timeout para escuchar


class VoiceRecognizer:
    """
    Sistema de reconocimiento de voz para IA-Personal.
    Escucha el micrófono y convierte voz a texto.
    """
    
    def __init__(self, config: VoiceConfig = None):
        if not HAS_SPEECH:
            print("⚠️ Reconocimiento de voz no disponible")
            self.config = config or VoiceConfig()
            self.recognizer = None
            self.microphone = None
            self.is_listening = False
            self.is_active = False
            self.message_queue = queue.Queue()
            self.listen_thread = None
            self.stop_event = threading.Event()
            self.on_speech_detected = None
            self.on_wake_word = None
            self.on_error = None
            return
        
        self.config = config or VoiceConfig()
        self.recognizer = sr.Recognizer()
        self.microphone = None
        
        # Estado
        self.is_listening = False
        self.is_active = False  # True después de wake word
        
        # Cola de mensajes reconocidos
        self.message_queue = queue.Queue()
        
        # Thread de escucha
        self.listen_thread: Optional[threading.Thread] = None
        self.stop_event = threading.Event()
        
        # Callbacks
        self.on_speech_detected: Optional[Callable[[str], None]] = None
        self.on_wake_word: Optional[Callable[[], None]] = None
        self.on_error: Optional[Callable[[str], None]] = None
        
        self._setup()
    
    def _setup(self):
        """Configura el reconocedor."""
        # Configurar umbrales
        self.recognizer.energy_threshold = self.config.energy_threshold
        self.recognizer.dynamic_energy_threshold = self.config.dynamic_energy
        self.recognizer.pause_threshold = self.config.pause_threshold
        
        # Listar micrófonos disponibles
        self._list_microphones()
    
    def _list_microphones(self):
        """Lista micrófonos disponibles."""
        if not HAS_SPEECH:
            print("⚠️ No se puede listar micrófonos sin speech_recognition")
            return
        print("\n🎤 Micrófonos disponibles:")
        for i, name in enumerate(sr.Microphone.list_microphone_names()):
            print(f"   [{i}] {name}")
    
    def _get_microphone(self):
        """Obtiene el micrófono configurado."""
        if self.config.device_index is not None:
            return sr.Microphone(
                device_index=self.config.device_index,
                sample_rate=self.config.sample_rate,
                chunk_size=self.config.chunk_size
            )
        return sr.Microphone(
            sample_rate=self.config.sample_rate,
            chunk_size=self.config.chunk_size
        )
    
    def calibrate(self, duration: float = 2.0):
        """Calibra el micrófono para el ruido ambiente."""
        print(f"🔧 Calibrando micrófono ({duration}s de silencio)...")
        
        with self._get_microphone() as source:
            self.recognizer.adjust_for_ambient_noise(source, duration=duration)
        
        print(f"   Umbral de energía: {self.recognizer.energy_threshold:.0f}")
        print("   ✅ Calibración completada")
    
    def recognize_once(self, timeout: float = None) -> Optional[str]:
        """
        Escucha una vez y retorna el texto reconocido.
        Bloquea hasta que se detecte voz o timeout.
        """
        timeout = timeout or self.config.timeout
        
        try:
            with self._get_microphone() as source:
                print("🎤 Escuchando...")
                audio = self.recognizer.listen(
                    source,
                    timeout=timeout,
                    phrase_time_limit=self.config.phrase_time_limit
                )
            
            return self._recognize_audio(audio)
            
        except sr.WaitTimeoutError:
            return None
        except Exception as e:
            if self.on_error:
                self.on_error(str(e))
            return None
    
    def _recognize_audio(self, audio) -> Optional[str]:
        """Reconoce audio usando el motor configurado."""
        try:
            if self.config.engine == RecognitionEngine.GOOGLE:
                text = self.recognizer.recognize_google(
                    audio,
                    language=self.config.language
                )
            elif self.config.engine == RecognitionEngine.SPHINX:
                text = self.recognizer.recognize_sphinx(audio)
            elif self.config.engine == RecognitionEngine.WHISPER:
                text = self.recognizer.recognize_whisper(
                    audio,
                    language=self.config.language[:2]  # "es" de "es-ES"
                )
            else:
                text = self.recognizer.recognize_google(
                    audio,
                    language=self.config.language
                )
            
            return text.strip()
            
        except sr.UnknownValueError:
            # No se entendió el audio
            return None
        except sr.RequestError as e:
            if self.on_error:
                self.on_error(f"Error de API: {e}")
            return None
    
    def _check_wake_word(self, text: str) -> bool:
        """Verifica si el texto contiene la palabra de activación."""
        if not self.config.wake_word_enabled:
            return True
        
        wake_words = [
            self.config.wake_word.lower(),
            "hey " + self.config.wake_word.lower(),
            "oye " + self.config.wake_word.lower(),
            "hola " + self.config.wake_word.lower(),
        ]
        
        text_lower = text.lower()
        for wake in wake_words:
            if wake in text_lower:
                return True
        
        return False
    
    def _listen_loop(self):
        """Loop de escucha continua (ejecuta en thread separado)."""
        print("\n🎧 Escucha continua iniciada")
        print(f"   Wake word: '{self.config.wake_word}'")
        print("   Di la palabra clave para activar...")
        
        with self._get_microphone() as source:
            # Calibrar al inicio
            self.recognizer.adjust_for_ambient_noise(source, duration=1)
            
            while not self.stop_event.is_set():
                try:
                    # Escuchar
                    audio = self.recognizer.listen(
                        source,
                        timeout=1,
                        phrase_time_limit=self.config.phrase_time_limit
                    )
                    
                    # Reconocer
                    text = self._recognize_audio(audio)
                    
                    if text:
                        # Verificar wake word si está habilitado
                        if self.config.wake_word_enabled and not self.is_active:
                            if self._check_wake_word(text):
                                self.is_active = True
                                print(f"\n🔔 Wake word detectado!")
                                if self.on_wake_word:
                                    self.on_wake_word()
                                
                                # Remover wake word del texto
                                for wake in [self.config.wake_word, "hey", "oye", "hola"]:
                                    text = text.lower().replace(wake.lower(), "").strip()
                        
                        if self.is_active or not self.config.wake_word_enabled:
                            if text:
                                print(f"\n🗣️ Reconocido: {text}")
                                self.message_queue.put(text)
                                
                                if self.on_speech_detected:
                                    self.on_speech_detected(text)
                
                except sr.WaitTimeoutError:
                    continue
                except Exception as e:
                    if self.on_error:
                        self.on_error(str(e))
        
        print("\n🛑 Escucha continua detenida")
    
    def start_listening(self):
        """Inicia escucha continua en background."""
        if self.is_listening:
            return
        
        self.is_listening = True
        self.stop_event.clear()
        
        self.listen_thread = threading.Thread(target=self._listen_loop, daemon=True)
        self.listen_thread.start()
    
    def stop_listening(self):
        """Detiene la escucha continua."""
        if not self.is_listening:
            return
        
        self.stop_event.set()
        self.is_listening = False
        
        if self.listen_thread:
            self.listen_thread.join(timeout=2)
    
    def get_message(self, timeout: float = None) -> Optional[str]:
        """Obtiene el siguiente mensaje de la cola."""
        try:
            return self.message_queue.get(timeout=timeout)
        except queue.Empty:
            return None
    
    def deactivate(self):
        """Desactiva el asistente (requiere wake word de nuevo)."""
        self.is_active = False
        print("💤 Asistente desactivado. Di la palabra clave para activar.")
    
    def set_wake_word(self, word: str):
        """Cambia la palabra de activación."""
        self.config.wake_word = word
        print(f"🔔 Nueva wake word: '{word}'")


# =============================================================================
# DEMO
# =============================================================================

def demo():
    """Demo de reconocimiento de voz."""
    print("\n" + "=" * 60)
    print("   🎤 Demo de Reconocimiento de Voz")
    print("   IA-Personal Voice")
    print("=" * 60)
    
    if not HAS_SPEECH:
        print("\n❌ Instala las dependencias:")
        print("   pip install SpeechRecognition pyaudio")
        return
    
    # Crear reconocedor
    config = VoiceConfig(
        language="es-ES",
        wake_word="asistente",
        wake_word_enabled=False,  # Desactivar para demo simple
    )
    
    recognizer = VoiceRecognizer(config)
    
    # Calibrar
    recognizer.calibrate(duration=2)
    
    # Escuchar 3 veces
    print("\n📝 Voy a escuchar 3 frases:")
    
    for i in range(3):
        print(f"\n[{i+1}/3] Habla ahora...")
        text = recognizer.recognize_once(timeout=10)
        
        if text:
            print(f"   ✅ Reconocido: {text}")
        else:
            print(f"   ❌ No se reconoció nada")
    
    print("\n" + "=" * 60)
    print("   Demo completada")
    print("=" * 60)


if __name__ == "__main__":
    demo()
