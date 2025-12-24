"""
IA-Personal con Control por Voz
================================
Author: Eddi Andreé Salazar Matos
Email: eddi.salazar.dev@gmail.com
Made with ❤️ in Peru 🇵🇪

Sistema completo de IA Personal con control por voz:
- Reconocimiento de voz continuo
- Comandos de voz inteligentes
- Control de mouse y teclado
- Control de aplicaciones
- Integración con GPU

Uso:
    python -m IA_Personal --voice
    
    # O directamente:
    from IA_Personal.voice import IAPersonalVoice
    ia = IAPersonalVoice()
    ia.start()
"""

import sys
import time
import threading
from pathlib import Path
from typing import Optional, Callable

# Agregar path
sys.path.insert(0, str(Path(__file__).parent.parent.parent))

# Importar componentes
try:
    from .speech_recognition import VoiceRecognizer, VoiceConfig, HAS_SPEECH
    from .voice_commands import VoiceCommandHandler, VoiceCommands
    from .system_control import SystemControl, HAS_PYAUTOGUI
except ImportError:
    from speech_recognition import VoiceRecognizer, VoiceConfig, HAS_SPEECH
    from voice_commands import VoiceCommandHandler, VoiceCommands
    from system_control import SystemControl, HAS_PYAUTOGUI

# Importar IA-Personal
from IA_Personal.core.ia_personal import IAPersonal, IAPersonalConfig

# Intentar importar versión GPU
try:
    from IA_Personal.integrations.ia_personal_gpu_max import IAPersonalGPUMax
    HAS_GPU = True
except:
    HAS_GPU = False

# Text-to-Speech
try:
    import pyttsx3
    HAS_TTS = True
except ImportError:
    HAS_TTS = False


class TextToSpeech:
    """Sistema de síntesis de voz."""
    
    def __init__(self, rate: int = 150, volume: float = 1.0, voice_id: int = 0):
        if not HAS_TTS:
            print("⚠️ pyttsx3 no instalado: pip install pyttsx3")
            self.engine = None
            return
        
        self.engine = pyttsx3.init()
        self.engine.setProperty('rate', rate)
        self.engine.setProperty('volume', volume)
        
        # Seleccionar voz (español si está disponible)
        voices = self.engine.getProperty('voices')
        for i, voice in enumerate(voices):
            if 'spanish' in voice.name.lower() or 'español' in voice.name.lower():
                self.engine.setProperty('voice', voice.id)
                print(f"🔊 Voz seleccionada: {voice.name}")
                break
        else:
            if voice_id < len(voices):
                self.engine.setProperty('voice', voices[voice_id].id)
    
    def speak(self, text: str):
        """Habla el texto."""
        if self.engine:
            self.engine.say(text)
            self.engine.runAndWait()
    
    def speak_async(self, text: str):
        """Habla el texto en background."""
        if self.engine:
            threading.Thread(target=self.speak, args=(text,), daemon=True).start()


class IAPersonalVoice:
    """
    IA Personal con control completo por voz.
    Combina reconocimiento de voz, comandos y síntesis.
    """
    
    def __init__(self, use_gpu: bool = True, use_tts: bool = True):
        print("\n" + "=" * 60)
        print("   🎤 IA-Personal Voice")
        print("   Control por Voz Inteligente")
        print("=" * 60)
        
        # Verificar dependencias
        self._check_dependencies()
        
        # Crear IA (con GPU si está disponible)
        if use_gpu and HAS_GPU:
            print("\n🚀 Iniciando con GPU MAX...")
            self.ia = IAPersonalGPUMax()
        else:
            print("\n💻 Iniciando en modo CPU...")
            self.ia = IAPersonal()
        
        # Configuración de voz
        self.voice_config = VoiceConfig(
            language="es-ES",
            wake_word="asistente",
            wake_word_enabled=True,
            energy_threshold=300,
            pause_threshold=0.8,
        )
        
        # Componentes de voz
        self.recognizer = None
        self.command_handler = None
        self.tts = None
        
        if HAS_SPEECH:
            self.recognizer = VoiceRecognizer(self.voice_config)
            self.command_handler = VoiceCommandHandler(self.ia)
            
            # Configurar callbacks
            self.recognizer.on_speech_detected = self._on_speech
            self.recognizer.on_wake_word = self._on_wake_word
            self.recognizer.on_error = self._on_error
            
            self.command_handler.on_chat_message = self._on_chat
        
        if use_tts and HAS_TTS:
            self.tts = TextToSpeech(rate=180)
        
        # Estado
        self.is_running = False
        self.last_response = ""
        
        print("\n✅ IA-Personal Voice inicializado")
        print(f"   Wake word: '{self.voice_config.wake_word}'")
        print(f"   TTS: {'✅' if self.tts else '❌'}")
        print(f"   GPU: {'✅' if (use_gpu and HAS_GPU) else '❌'}")
    
    def _check_dependencies(self):
        """Verifica dependencias."""
        missing = []
        
        if not HAS_SPEECH:
            missing.append("SpeechRecognition pyaudio")
        if not HAS_PYAUTOGUI:
            missing.append("pyautogui")
        if not HAS_TTS:
            missing.append("pyttsx3")
        
        if missing:
            print("\n⚠️ Dependencias faltantes:")
            print(f"   pip install {' '.join(missing)}")
    
    def _on_wake_word(self):
        """Callback cuando se detecta wake word."""
        print("\n🔔 ¡Asistente activado!")
        if self.tts:
            self.tts.speak_async("¿Sí? ¿En qué puedo ayudarte?")
    
    def _on_speech(self, text: str):
        """Callback cuando se detecta voz."""
        print(f"\n🗣️ Tú: {text}")
        
        # Ejecutar comando
        success, response = self.command_handler.execute(text)
        
        # Manejar respuestas especiales
        if response == "DEACTIVATE":
            self.recognizer.deactivate()
            response = "Hasta luego. Di la palabra clave para activarme."
        elif response == "STOP":
            response = "Detenido."
        
        self.last_response = response
        print(f"🤖 IA: {response}")
        
        # Hablar respuesta
        if self.tts and response:
            # No hablar respuestas muy largas
            if len(response) < 200:
                self.tts.speak_async(response)
    
    def _on_chat(self, message: str) -> str:
        """Callback para mensajes de chat."""
        return self.ia.chat(message)
    
    def _on_error(self, error: str):
        """Callback de error."""
        print(f"⚠️ Error: {error}")
    
    def calibrate(self):
        """Calibra el micrófono."""
        if self.recognizer:
            self.recognizer.calibrate(duration=2)
    
    def start(self):
        """Inicia el asistente de voz."""
        if not self.recognizer:
            print("❌ Reconocimiento de voz no disponible")
            return
        
        self.is_running = True
        
        print("\n" + "=" * 60)
        print("   🎤 Asistente de Voz Iniciado")
        print("=" * 60)
        print(f"\n💡 Di '{self.voice_config.wake_word}' para activar")
        print("   Presiona Ctrl+C para salir\n")
        
        # Calibrar
        self.calibrate()
        
        # Iniciar escucha
        self.recognizer.start_listening()
        
        # Mantener vivo
        try:
            while self.is_running:
                time.sleep(0.1)
        except KeyboardInterrupt:
            print("\n\n🛑 Deteniendo asistente...")
            self.stop()
    
    def stop(self):
        """Detiene el asistente."""
        self.is_running = False
        if self.recognizer:
            self.recognizer.stop_listening()
        print("👋 ¡Hasta luego!")
    
    def process_text(self, text: str) -> str:
        """Procesa texto como si fuera voz (para testing)."""
        success, response = self.command_handler.execute(text)
        return response
    
    def interactive_text(self):
        """Modo interactivo por texto (para testing sin micrófono)."""
        print("\n" + "=" * 60)
        print("   🎤 Modo Interactivo (Texto)")
        print("   Escribe comandos como si hablaras")
        print("=" * 60)
        print("\n💡 Escribe 'ayuda' para ver comandos")
        print("   Escribe 'salir' para terminar\n")
        
        while True:
            try:
                text = input("🗣️ Tú: ").strip()
                
                if not text:
                    continue
                
                if text.lower() in ['salir', 'exit', 'quit']:
                    print("👋 ¡Hasta luego!")
                    break
                
                response = self.process_text(text)
                print(f"🤖 IA: {response}\n")
                
            except KeyboardInterrupt:
                print("\n👋 ¡Hasta luego!")
                break


# =============================================================================
# CLI
# =============================================================================

def main():
    """Punto de entrada principal."""
    import argparse
    
    parser = argparse.ArgumentParser(description="IA-Personal Voice")
    parser.add_argument("--no-gpu", action="store_true", help="Desactivar GPU")
    parser.add_argument("--no-tts", action="store_true", help="Desactivar síntesis de voz")
    parser.add_argument("--text", action="store_true", help="Modo texto (sin micrófono)")
    parser.add_argument("--wake-word", type=str, default="asistente", help="Palabra de activación")
    args = parser.parse_args()
    
    # Crear asistente
    ia_voice = IAPersonalVoice(
        use_gpu=not args.no_gpu,
        use_tts=not args.no_tts
    )
    
    # Configurar wake word
    if args.wake_word != "asistente":
        ia_voice.voice_config.wake_word = args.wake_word
        if ia_voice.recognizer:
            ia_voice.recognizer.config.wake_word = args.wake_word
    
    # Iniciar
    if args.text:
        ia_voice.interactive_text()
    else:
        ia_voice.start()


if __name__ == "__main__":
    main()
