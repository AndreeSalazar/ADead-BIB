"""
Integración con Ollama para IA-Personal
========================================
Author: Eddi Andreé Salazar Matos
Made with ❤️ in Peru 🇵🇪

Integración con Ollama para respuestas avanzadas usando LLMs locales.
"""

import json
import time
import subprocess
from typing import Optional, Dict, List, Generator
from pathlib import Path

try:
    import requests
    HAS_REQUESTS = True
except ImportError:
    HAS_REQUESTS = False

import sys
sys.path.insert(0, str(Path(__file__).parent.parent.parent))

from IA_Personal.core.ia_personal import IAPersonal, IAPersonalConfig


class OllamaChat:
    """
    Cliente para Ollama - LLMs locales.
    Permite usar modelos como TinyLlama, Llama2, Mistral, etc.
    """
    
    DEFAULT_URL = "http://localhost:11434"
    DEFAULT_MODEL = "tinyllama"
    
    def __init__(self, base_url: str = None, model: str = None):
        self.base_url = base_url or self.DEFAULT_URL
        self.model = model or self.DEFAULT_MODEL
        self.available = False
        self.models: List[str] = []
        
        self._check_availability()
    
    def _check_availability(self):
        """Verifica si Ollama está disponible."""
        if not HAS_REQUESTS:
            print("⚠️ Ollama: requests no instalado (pip install requests)")
            return
        
        try:
            response = requests.get(f"{self.base_url}/api/tags", timeout=2)
            if response.status_code == 200:
                data = response.json()
                self.models = [m["name"] for m in data.get("models", [])]
                self.available = True
                print(f"✅ Ollama disponible: {len(self.models)} modelos")
                
                # Verificar modelo seleccionado
                if self.model not in self.models and self.models:
                    self.model = self.models[0]
                    print(f"   Usando modelo: {self.model}")
            else:
                print(f"⚠️ Ollama: Error {response.status_code}")
        except requests.exceptions.ConnectionError:
            print("⚠️ Ollama no está corriendo. Ejecuta: ollama serve")
        except Exception as e:
            print(f"⚠️ Ollama: {e}")
    
    def is_available(self) -> bool:
        """Verifica si Ollama está disponible."""
        return self.available and HAS_REQUESTS
    
    def list_models(self) -> List[str]:
        """Lista modelos disponibles."""
        return self.models
    
    def set_model(self, model: str):
        """Cambia el modelo activo."""
        if model in self.models:
            self.model = model
            print(f"Modelo cambiado a: {model}")
        else:
            print(f"Modelo '{model}' no disponible. Modelos: {self.models}")
    
    def chat(self, message: str, context: str = "", system: str = None) -> str:
        """
        Envía un mensaje a Ollama y obtiene respuesta.
        
        Args:
            message: Mensaje del usuario
            context: Contexto adicional (memorias, perfil, etc.)
            system: Prompt del sistema
        
        Returns:
            Respuesta del modelo
        """
        if not self.is_available():
            return "Ollama no está disponible. Ejecuta: ollama serve"
        
        # Construir prompt
        if system is None:
            system = """Eres una IA personal amigable y útil. 
Responde de forma concisa y natural en español.
Si te dan contexto sobre el usuario, úsalo para personalizar tus respuestas."""
        
        full_prompt = message
        if context:
            full_prompt = f"Contexto: {context}\n\nUsuario: {message}"
        
        try:
            response = requests.post(
                f"{self.base_url}/api/generate",
                json={
                    "model": self.model,
                    "prompt": full_prompt,
                    "system": system,
                    "stream": False,
                    "options": {
                        "temperature": 0.7,
                        "top_p": 0.9,
                        "num_predict": 150,
                    }
                },
                timeout=60
            )
            
            if response.status_code == 200:
                data = response.json()
                return data.get("response", "").strip()
            else:
                return f"Error: {response.status_code}"
                
        except requests.exceptions.Timeout:
            return "Timeout: El modelo tardó demasiado en responder."
        except Exception as e:
            return f"Error: {e}"
    
    def chat_stream(self, message: str, context: str = "") -> Generator[str, None, None]:
        """Chat con streaming de respuesta."""
        if not self.is_available():
            yield "Ollama no está disponible."
            return
        
        full_prompt = message
        if context:
            full_prompt = f"Contexto: {context}\n\nUsuario: {message}"
        
        try:
            response = requests.post(
                f"{self.base_url}/api/generate",
                json={
                    "model": self.model,
                    "prompt": full_prompt,
                    "stream": True,
                },
                stream=True,
                timeout=60
            )
            
            for line in response.iter_lines():
                if line:
                    data = json.loads(line)
                    if "response" in data:
                        yield data["response"]
                    if data.get("done", False):
                        break
                        
        except Exception as e:
            yield f"Error: {e}"
    
    def pull_model(self, model: str) -> bool:
        """Descarga un modelo."""
        print(f"Descargando modelo: {model}...")
        try:
            result = subprocess.run(
                ["ollama", "pull", model],
                capture_output=True,
                text=True,
                timeout=600
            )
            if result.returncode == 0:
                print(f"✅ Modelo {model} descargado")
                self._check_availability()
                return True
            else:
                print(f"❌ Error: {result.stderr}")
                return False
        except Exception as e:
            print(f"❌ Error: {e}")
            return False


class IAPersonalOllama(IAPersonal):
    """
    IA Personal con integración Ollama para respuestas avanzadas.
    Combina memoria local + LLM para respuestas inteligentes.
    """
    
    def __init__(self, config: IAPersonalConfig = None, ollama_model: str = "tinyllama"):
        # Inicializar Ollama
        self.ollama = OllamaChat(model=ollama_model)
        
        # Llamar al constructor padre
        super().__init__(config)
        
        # Configuración de Ollama
        self.use_ollama = self.ollama.is_available()
        self.ollama_for_complex = True  # Usar Ollama solo para preguntas complejas
        
        if self.use_ollama:
            print(f"🦙 Ollama integrado: {self.ollama.model}")
    
    def _is_complex_query(self, message: str) -> bool:
        """Determina si una pregunta requiere Ollama."""
        message_lower = message.lower()
        
        # Preguntas simples que no necesitan Ollama
        simple_patterns = [
            "hola", "hi", "hello", "ayuda", "help",
            "memoria", "perfil", "busca", "search",
            "me llamo", "mi nombre", "me gusta", "recuerda",
            "gracias", "adiós", "bye", "chao",
        ]
        
        for pattern in simple_patterns:
            if pattern in message_lower:
                return False
        
        # Preguntas complejas que sí necesitan Ollama
        complex_indicators = [
            "explica", "explain", "cómo funciona", "how does",
            "por qué", "why", "qué es", "what is",
            "cuál es la diferencia", "difference between",
            "dame un ejemplo", "give me an example",
            "escribe", "write", "genera", "generate",
            "traduce", "translate", "resume", "summarize",
        ]
        
        for indicator in complex_indicators:
            if indicator in message_lower:
                return True
        
        # Si el mensaje es largo, probablemente es complejo
        if len(message.split()) > 10:
            return True
        
        return False
    
    def _build_ollama_context(self, message: str) -> str:
        """Construye contexto para Ollama."""
        parts = []
        
        # Información del usuario
        name = self.context.profile.name
        if name != "Usuario":
            parts.append(f"El usuario se llama {name}.")
        
        if self.context.profile.interests:
            interests = ", ".join(self.context.profile.interests[:3])
            parts.append(f"Sus intereses incluyen: {interests}.")
        
        # Memorias relevantes
        relevant = self.memory.search(message, top_k=3)
        if relevant:
            parts.append("Información relevante:")
            for mem in relevant:
                parts.append(f"- {mem.content[:100]}")
        
        # Conversación reciente
        if self.conversation_history:
            parts.append("Conversación reciente:")
            for user_msg, ai_msg in self.conversation_history[-2:]:
                parts.append(f"Usuario: {user_msg}")
                parts.append(f"IA: {ai_msg[:100]}")
        
        return "\n".join(parts)
    
    def chat(self, message: str) -> str:
        """Procesa un mensaje con posible uso de Ollama."""
        self.context.update_interaction()
        
        # Verificar aprendizaje
        learning_response = self._check_learning(message)
        if learning_response:
            self.conversation_history.append((message, learning_response))
            return learning_response
        
        # Comandos especiales (siempre locales)
        message_lower = message.lower().strip()
        
        if message_lower in ["hola", "hi", "hello"]:
            response = self.context.get_greeting()
        elif message_lower in ["ayuda", "help", "?"]:
            response = self._get_help()
        elif message_lower in ["memoria", "memorias", "memory"]:
            response = self._get_memory_stats()
        elif message_lower in ["perfil", "profile"]:
            response = self.context.get_summary()
        elif message_lower.startswith("busca ") or message_lower.startswith("search "):
            query = message[6:].strip()
            response = self._search_memory(query)
        elif message_lower in ["ollama", "modelos", "models"]:
            response = self._get_ollama_info()
        elif self.use_ollama and self.ollama_for_complex and self._is_complex_query(message):
            # Usar Ollama para preguntas complejas
            context = self._build_ollama_context(message)
            response = self.ollama.chat(message, context=context)
            response = f"🦙 {response}"
        else:
            # Respuesta local
            response = self._get_smart_response(message)
        
        # Guardar en historial y memoria
        self.conversation_history.append((message, response))
        self.memory.add(f"Usuario: {message}", category="conversations")
        
        return response
    
    def _get_ollama_info(self) -> str:
        """Información sobre Ollama."""
        if not self.ollama.is_available():
            return """🦙 **Ollama no está disponible**

Para activar respuestas avanzadas:
1. Instala Ollama: winget install Ollama.Ollama
2. Ejecuta: ollama serve
3. Descarga un modelo: ollama pull tinyllama"""
        
        models = self.ollama.list_models()
        return f"""🦙 **Ollama Activo**
• Modelo actual: {self.ollama.model}
• Modelos disponibles: {', '.join(models)}

Las preguntas complejas se responden con Ollama automáticamente."""
    
    def _get_help(self) -> str:
        """Ayuda extendida con info de Ollama."""
        base_help = super()._get_help()
        
        if self.use_ollama:
            base_help += """

🦙 **Ollama Integrado:**
• Las preguntas complejas usan el LLM automáticamente
• Escribe "ollama" para ver info del modelo
• Preguntas como "explica", "qué es", "escribe" usan Ollama"""
        
        return base_help
    
    def set_ollama_model(self, model: str):
        """Cambia el modelo de Ollama."""
        self.ollama.set_model(model)
    
    def toggle_ollama(self, enabled: bool = None):
        """Activa/desactiva el uso de Ollama."""
        if enabled is None:
            self.ollama_for_complex = not self.ollama_for_complex
        else:
            self.ollama_for_complex = enabled
        
        status = "activado" if self.ollama_for_complex else "desactivado"
        print(f"Ollama {status} para preguntas complejas")
