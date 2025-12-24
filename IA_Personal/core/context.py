"""
Contexto Personal para IA-Personal
===================================
Author: Eddi Andreé Salazar Matos
Made with ❤️ in Peru 🇵🇪
"""

import json
import time
from pathlib import Path
from typing import List, Dict, Any
from dataclasses import dataclass, field, asdict
from datetime import datetime


@dataclass
class UserProfile:
    """Perfil del usuario para personalización."""
    name: str = "Usuario"
    language: str = "es"
    interests: List[str] = field(default_factory=list)
    preferences: Dict[str, Any] = field(default_factory=dict)
    interaction_count: int = 0
    first_interaction: float = 0.0
    last_interaction: float = 0.0
    learned_facts: Dict[str, str] = field(default_factory=dict)
    
    def to_dict(self) -> Dict:
        return asdict(self)
    
    @classmethod
    def from_dict(cls, data: Dict) -> "UserProfile":
        return cls(**data)


class PersonalContext:
    """Gestiona el contexto personal del usuario."""
    
    def __init__(self, data_dir: str):
        self.data_path = Path(data_dir)
        self.data_path.mkdir(parents=True, exist_ok=True)
        self.profile_file = self.data_path / "profile.json"
        
        self.profile = UserProfile()
        self._load()
    
    def update_interaction(self):
        """Actualiza estadísticas de interacción."""
        now = time.time()
        if self.profile.first_interaction == 0:
            self.profile.first_interaction = now
        self.profile.last_interaction = now
        self.profile.interaction_count += 1
        self._save()
    
    def set_name(self, name: str):
        """Establece el nombre del usuario."""
        self.profile.name = name.strip().capitalize()
        self._save()
    
    def learn_fact(self, key: str, value: str):
        """Aprende un hecho sobre el usuario."""
        self.profile.learned_facts[key] = value
        self._save()
    
    def add_interest(self, interest: str):
        """Agrega un interés."""
        interest = interest.strip().lower()
        if interest and interest not in [i.lower() for i in self.profile.interests]:
            self.profile.interests.append(interest)
            self._save()
    
    def remove_interest(self, interest: str):
        """Elimina un interés."""
        interest = interest.strip().lower()
        self.profile.interests = [i for i in self.profile.interests if i.lower() != interest]
        self._save()
    
    def set_preference(self, key: str, value: Any):
        """Establece una preferencia."""
        self.profile.preferences[key] = value
        self._save()
    
    def get_preference(self, key: str, default: Any = None) -> Any:
        """Obtiene una preferencia."""
        return self.profile.preferences.get(key, default)
    
    def get_greeting(self) -> str:
        """Genera un saludo personalizado."""
        hour = datetime.now().hour
        if hour < 12:
            greeting = "Buenos días"
        elif hour < 18:
            greeting = "Buenas tardes"
        else:
            greeting = "Buenas noches"
        
        name = self.profile.name
        count = self.profile.interaction_count
        
        if count == 0:
            return f"¡Hola! Soy tu IA Personal. ¿Cómo te llamas?"
        elif count < 3:
            return f"{greeting}, {name}. ¿En qué puedo ayudarte hoy?"
        elif count < 10:
            return f"{greeting}, {name}. Me alegra verte de nuevo."
        else:
            # Personalizado según intereses
            if self.profile.interests:
                interest = self.profile.interests[0]
                return f"{greeting}, {name}. ¿Qué tal va todo con {interest}?"
            return f"{greeting}, {name}. ¡Qué bueno verte otra vez!"
    
    def get_summary(self) -> str:
        """Obtiene un resumen del perfil."""
        p = self.profile
        lines = [
            f"👤 **Perfil de {p.name}**",
            f"• Interacciones: {p.interaction_count}",
        ]
        
        if p.interests:
            lines.append(f"• Intereses: {', '.join(p.interests[:5])}")
        
        if p.learned_facts:
            lines.append("• Hechos conocidos:")
            for k, v in list(p.learned_facts.items())[:5]:
                lines.append(f"  - {k}: {v}")
        
        if p.first_interaction > 0:
            days = (time.time() - p.first_interaction) / 86400
            lines.append(f"• Te conozco hace: {days:.0f} días")
        
        return "\n".join(lines)
    
    def reset(self):
        """Reinicia el perfil."""
        self.profile = UserProfile()
        self._save()
    
    def _save(self):
        """Guarda perfil a disco."""
        with open(self.profile_file, 'w', encoding='utf-8') as f:
            json.dump(self.profile.to_dict(), f, ensure_ascii=False, indent=2)
    
    def _load(self):
        """Carga perfil desde disco."""
        if self.profile_file.exists():
            try:
                with open(self.profile_file, 'r', encoding='utf-8') as f:
                    data = json.load(f)
                self.profile = UserProfile.from_dict(data)
                print(f"👤 Perfil cargado: {self.profile.name}")
            except Exception as e:
                print(f"⚠️ Error cargando perfil: {e}")
