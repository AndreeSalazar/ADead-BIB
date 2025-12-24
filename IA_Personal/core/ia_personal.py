"""
IA-Personal - Sistema Principal
================================
Author: Eddi Andreé Salazar Matos
Email: eddi.salazar.dev@gmail.com
Made with ❤️ in Peru 🇵🇪

Sistema de IA Personal con memoria persistente y contexto personal.
"""

import re
import time
from pathlib import Path
from typing import List, Dict, Optional, Tuple
from dataclasses import dataclass

from .memory import PersistentMemory
from .context import PersonalContext
from .tokenizer import SmartTokenizer
from .model import LightTransformer, ModelConfig


@dataclass
class IAPersonalConfig:
    """Configuración de IA-Personal."""
    # Modelo
    vocab_size: int = 15000
    embed_dim: int = 128
    num_heads: int = 8
    hidden_dim: int = 256
    num_layers: int = 2
    max_seq_len: int = 256
    
    # Generación
    temperature: float = 0.7
    top_k: int = 50
    top_p: float = 0.9
    repetition_penalty: float = 1.1
    
    # Memoria
    max_memory_items: int = 1000
    context_window: int = 10
    
    # Optimización
    use_float16: bool = True
    
    # Rutas
    data_dir: str = ""
    
    def __post_init__(self):
        if not self.data_dir:
            self.data_dir = str(Path(__file__).parent.parent / "data")


class IAPersonal:
    """
    Sistema de IA Personal completo.
    Combina memoria, contexto personal y respuestas inteligentes.
    """
    
    def __init__(self, config: IAPersonalConfig = None):
        self.config = config or IAPersonalConfig()
        
        print("=" * 60)
        print("   🤖 IA-Personal para ADead-BIB")
        print("   Sistema de IA Personal Ultra-Ligero")
        print("=" * 60)
        
        # Componentes
        self.tokenizer = SmartTokenizer(self.config.vocab_size)
        self.memory = PersistentMemory(self.config.data_dir, self.config.max_memory_items)
        self.context = PersonalContext(self.config.data_dir)
        
        # Modelo
        model_config = ModelConfig(
            vocab_size=self.config.vocab_size,
            embed_dim=self.config.embed_dim,
            num_heads=self.config.num_heads,
            hidden_dim=self.config.hidden_dim,
            num_layers=self.config.num_layers,
            max_seq_len=self.config.max_seq_len,
            use_float16=self.config.use_float16,
        )
        self.model = LightTransformer(model_config, len(self.tokenizer))
        
        # Historial de conversación actual
        self.conversation_history: List[Tuple[str, str]] = []
        
        # Patrones de aprendizaje
        self.learning_patterns = {
            r"me llamo (\w+)": self._learn_name,
            r"mi nombre es (\w+)": self._learn_name,
            r"soy (\w+)": self._learn_name,
            r"me gusta (.+)": self._learn_interest,
            r"me interesa (.+)": self._learn_interest,
            r"recuerda que (.+)": self._learn_fact,
            r"no olvides que (.+)": self._learn_fact,
        }
        
        self._print_stats()
    
    def _print_stats(self):
        """Imprime estadísticas del sistema."""
        print(f"\n📊 Configuración:")
        print(f"  Vocabulario: {len(self.tokenizer)} tokens")
        print(f"  Embeddings:  {self.config.embed_dim} dim")
        print(f"  Capas:       {self.config.num_layers}")
        print(f"  Memorias:    {len(self.memory.memories)}")
        print(f"\n💾 RAM Total:  {self.model.ram_mb:.2f} MB")
        print("=" * 60)
    
    def _learn_name(self, match: re.Match) -> str:
        """Aprende el nombre del usuario."""
        name = match.group(1).capitalize()
        self.context.set_name(name)
        self.memory.add(f"El usuario se llama {name}", category="personal", importance=2.0)
        return f"¡Encantado de conocerte, {name}! Recordaré tu nombre."
    
    def _learn_interest(self, match: re.Match) -> str:
        """Aprende un interés del usuario."""
        interest = match.group(1).strip()
        self.context.add_interest(interest)
        self.memory.add(f"Al usuario le interesa: {interest}", category="preferences", importance=1.5)
        return f"¡Interesante! Recordaré que te gusta {interest}."
    
    def _learn_fact(self, match: re.Match) -> str:
        """Aprende un hecho."""
        fact = match.group(1).strip()
        self.memory.add(fact, category="facts", importance=1.5)
        return f"Entendido, lo recordaré: {fact}"
    
    def _check_learning(self, message: str) -> Optional[str]:
        """Verifica si el mensaje contiene información para aprender."""
        message_lower = message.lower()
        for pattern, handler in self.learning_patterns.items():
            match = re.search(pattern, message_lower)
            if match:
                return handler(match)
        return None
    
    def _get_smart_response(self, message: str) -> str:
        """Genera respuesta inteligente basada en el contexto."""
        message_lower = message.lower()
        name = self.context.profile.name
        
        # Respuestas basadas en palabras clave
        if any(w in message_lower for w in ["qué sabes", "que sabes", "conoces", "recuerdas"]):
            facts = []
            if name != "Usuario":
                facts.append(f"Te llamas {name}")
            if self.context.profile.interests:
                facts.append(f"Te interesa: {', '.join(self.context.profile.interests)}")
            
            relevant = self.memory.search(message, top_k=3, category="facts")
            for mem in relevant:
                facts.append(mem.content)
            
            if facts:
                return "Esto es lo que sé de ti:\n• " + "\n• ".join(facts)
            return "Aún estoy aprendiendo sobre ti. ¡Cuéntame más!"
        
        if any(w in message_lower for w in ["cómo estás", "como estas", "qué tal", "que tal"]):
            return "¡Estoy muy bien, gracias por preguntar! ¿Y tú cómo estás?"
        
        if any(w in message_lower for w in ["qué puedes", "que puedes", "qué haces", "que haces"]):
            return """Puedo ayudarte con varias cosas:
• Recordar información sobre ti
• Mantener conversaciones
• Buscar en mis memorias
• Aprender de nuestras interacciones

Escribe 'ayuda' para ver todos los comandos."""
        
        if any(w in message_lower for w in ["gracias", "thanks"]):
            return f"¡De nada, {name}! Estoy aquí para ayudarte."
        
        if any(w in message_lower for w in ["adiós", "adios", "bye", "chao"]):
            return f"¡Hasta pronto, {name}! Fue un gusto conversar contigo."
        
        # Buscar en memorias para contexto
        relevant = self.memory.search(message, top_k=2)
        if relevant:
            context_info = relevant[0].content
            return f"Hmm, recuerdo algo relacionado: {context_info[:100]}... ¿Quieres que te cuente más?"
        
        # Respuestas genéricas variadas
        responses = [
            "Interesante, cuéntame más sobre eso.",
            "Entiendo. ¿Qué más te gustaría compartir?",
            "Gracias por contarme. ¿Hay algo más en lo que pueda ayudarte?",
            f"Hmm, {message[:30]}... es un tema interesante.",
            "¿Puedes darme más detalles al respecto?",
            "Me parece muy interesante lo que dices.",
            "Sigo aprendiendo, pero me encanta conversar contigo.",
        ]
        return responses[hash(message) % len(responses)]
    
    def chat(self, message: str) -> str:
        """Procesa un mensaje y genera respuesta."""
        self.context.update_interaction()
        
        # Verificar aprendizaje
        learning_response = self._check_learning(message)
        if learning_response:
            self.conversation_history.append((message, learning_response))
            return learning_response
        
        # Comandos especiales
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
        else:
            response = self._get_smart_response(message)
        
        # Guardar en historial y memoria
        self.conversation_history.append((message, response))
        self.memory.add(f"Usuario: {message}", category="conversations")
        
        return response
    
    def _get_help(self) -> str:
        """Retorna mensaje de ayuda."""
        return """🤖 **IA-Personal - Comandos:**

• **Conversación normal** - Solo escribe y responderé
• **"me llamo [nombre]"** - Aprendo tu nombre
• **"me gusta [algo]"** - Aprendo tus intereses
• **"recuerda que [algo]"** - Guardo información
• **"busca [tema]"** - Busco en mis memorias
• **"memoria"** - Muestro estadísticas de memoria
• **"perfil"** - Muestro tu perfil
• **"ayuda"** - Este mensaje

💡 Soy tu IA personal, aprendo de ti y recuerdo nuestras conversaciones."""
    
    def _get_memory_stats(self) -> str:
        """Retorna estadísticas de memoria."""
        stats = self.memory.stats()
        lines = [
            "📚 **Estadísticas de Memoria:**",
            f"• Total: {stats['total_memories']} memorias",
            f"• Accesos totales: {stats['total_accesses']}",
            "• Por categoría:"
        ]
        for cat, count in stats['categories'].items():
            if count > 0:
                lines.append(f"  - {cat}: {count}")
        return "\n".join(lines)
    
    def _search_memory(self, query: str) -> str:
        """Busca en la memoria."""
        results = self.memory.search(query, top_k=5)
        if not results:
            return f"No encontré nada sobre '{query}' en mis memorias."
        
        lines = [f"🔍 **Resultados para '{query}':**"]
        for i, mem in enumerate(results, 1):
            lines.append(f"{i}. {mem.content[:100]}...")
        return "\n".join(lines)
    
    def interactive(self):
        """Modo interactivo de chat."""
        print("\n" + "=" * 60)
        print("   🤖 IA-Personal - Modo Interactivo")
        print("   Escribe 'salir' para terminar")
        print("=" * 60)
        
        print(f"\n{self.context.get_greeting()}\n")
        
        while True:
            try:
                user_input = input("Tú: ").strip()
                
                if not user_input:
                    continue
                
                if user_input.lower() in ["salir", "exit", "quit", "q"]:
                    print(f"\n¡Hasta luego, {self.context.profile.name}! Fue un placer conversar contigo. 👋")
                    break
                
                response = self.chat(user_input)
                print(f"\n🤖: {response}\n")
                
            except KeyboardInterrupt:
                print("\n\n¡Hasta luego! 👋")
                break
            except Exception as e:
                print(f"\n⚠️ Error: {e}\n")
    
    def get_stats(self) -> Dict:
        """Obtiene estadísticas del sistema."""
        # Obtener RAM del modelo (compatible con diferentes tipos)
        if hasattr(self.model, 'ram_mb'):
            ram = self.model.ram_mb
        elif hasattr(self.model, 'memory_mb'):
            ram = self.model.memory_mb
        else:
            ram = 0
        
        return {
            "vocab_size": len(self.tokenizer),
            "memory_count": len(self.memory.memories),
            "interaction_count": self.context.profile.interaction_count,
            "ram_mb": ram,
            "user_name": self.context.profile.name,
        }
