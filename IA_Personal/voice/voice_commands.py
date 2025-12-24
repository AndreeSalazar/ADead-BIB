"""
Comandos de Voz Inteligentes para IA-Personal
===============================================
Author: Eddi Andreé Salazar Matos
Email: eddi.salazar.dev@gmail.com
Made with ❤️ in Peru 🇵🇪

Sistema de comandos de voz inteligentes:
- Parseo de comandos naturales
- Ejecución de acciones del sistema
- Integración con IA-Personal
- Comandos personalizables
"""

import re
import time
from typing import Optional, Callable, Dict, List, Tuple, Any
from dataclasses import dataclass
from enum import Enum

from .system_control import SystemControl, Direction, MouseButton


class CommandType(Enum):
    """Tipos de comandos."""
    MOUSE = "mouse"
    KEYBOARD = "keyboard"
    APP = "app"
    SYSTEM = "system"
    CHAT = "chat"
    UNKNOWN = "unknown"


@dataclass
class ParsedCommand:
    """Comando parseado."""
    type: CommandType
    action: str
    params: Dict[str, Any]
    original_text: str
    confidence: float


class VoiceCommands:
    """
    Parser de comandos de voz naturales.
    Convierte texto hablado en acciones del sistema.
    """
    
    def __init__(self):
        # Patrones de comandos (español)
        self.patterns = {
            # === MOUSE ===
            # Movimiento
            r"(mueve|mover|lleva|llevar)\s*(el\s*)?(mouse|ratón|cursor)\s*(hacia\s*)?(arriba|up)": 
                ("mouse", "move", {"direction": Direction.UP}),
            r"(mueve|mover|lleva|llevar)\s*(el\s*)?(mouse|ratón|cursor)\s*(hacia\s*)?(abajo|down)": 
                ("mouse", "move", {"direction": Direction.DOWN}),
            r"(mueve|mover|lleva|llevar)\s*(el\s*)?(mouse|ratón|cursor)\s*(hacia\s*)?(la\s*)?(izquierda|left)": 
                ("mouse", "move", {"direction": Direction.LEFT}),
            r"(mueve|mover|lleva|llevar)\s*(el\s*)?(mouse|ratón|cursor)\s*(hacia\s*)?(la\s*)?(derecha|right)": 
                ("mouse", "move", {"direction": Direction.RIGHT}),
            r"(mueve|mover|lleva|llevar)\s*(el\s*)?(mouse|ratón|cursor)\s*(al\s*)?(centro)": 
                ("mouse", "move", {"direction": Direction.CENTER}),
            
            # Direcciones simples
            r"^(arriba|sube|subir)$": ("mouse", "move", {"direction": Direction.UP}),
            r"^(abajo|baja|bajar)$": ("mouse", "move", {"direction": Direction.DOWN}),
            r"^(izquierda|left)$": ("mouse", "move", {"direction": Direction.LEFT}),
            r"^(derecha|right)$": ("mouse", "move", {"direction": Direction.RIGHT}),
            r"^(centro|centrar)$": ("mouse", "move", {"direction": Direction.CENTER}),
            
            # Clicks
            r"(haz\s*)?(un\s*)?(click|clic)(\s*izquierdo)?": 
                ("mouse", "click", {"button": MouseButton.LEFT}),
            r"(haz\s*)?(un\s*)?(click|clic)\s*(derecho|secundario)": 
                ("mouse", "click", {"button": MouseButton.RIGHT}),
            r"(doble\s*click|doble\s*clic)": 
                ("mouse", "double_click", {}),
            
            # Scroll
            r"(scroll|desplaza|desplazar)\s*(hacia\s*)?(arriba|up)": 
                ("mouse", "scroll", {"amount": 3}),
            r"(scroll|desplaza|desplazar)\s*(hacia\s*)?(abajo|down)": 
                ("mouse", "scroll", {"amount": -3}),
            
            # === TECLADO ===
            r"(escribe|escribir|teclea|teclear)\s+(.+)": 
                ("keyboard", "type", {"text": "$2"}),
            r"(presiona|pulsa|press)\s+(enter|intro)": 
                ("keyboard", "press", {"key": "enter"}),
            r"(presiona|pulsa|press)\s+(escape|esc)": 
                ("keyboard", "press", {"key": "escape"}),
            r"(presiona|pulsa|press)\s+(espacio|space)": 
                ("keyboard", "press", {"key": "space"}),
            r"(presiona|pulsa|press)\s+(tab|tabulador)": 
                ("keyboard", "press", {"key": "tab"}),
            r"(presiona|pulsa|press)\s+(borrar|delete|suprimir)": 
                ("keyboard", "press", {"key": "delete"}),
            r"(presiona|pulsa|press)\s+(retroceso|backspace)": 
                ("keyboard", "press", {"key": "backspace"}),
            
            # Atajos
            r"(copiar|copy)": ("keyboard", "hotkey", {"keys": ["ctrl", "c"]}),
            r"(pegar|paste)": ("keyboard", "hotkey", {"keys": ["ctrl", "v"]}),
            r"(cortar|cut)": ("keyboard", "hotkey", {"keys": ["ctrl", "x"]}),
            r"(deshacer|undo)": ("keyboard", "hotkey", {"keys": ["ctrl", "z"]}),
            r"(rehacer|redo)": ("keyboard", "hotkey", {"keys": ["ctrl", "y"]}),
            r"(seleccionar\s*todo|select\s*all)": ("keyboard", "hotkey", {"keys": ["ctrl", "a"]}),
            r"(guardar|save)": ("keyboard", "hotkey", {"keys": ["ctrl", "s"]}),
            r"(nueva\s*pestaña|new\s*tab)": ("keyboard", "hotkey", {"keys": ["ctrl", "t"]}),
            r"(cerrar\s*pestaña|close\s*tab)": ("keyboard", "hotkey", {"keys": ["ctrl", "w"]}),
            r"(cambiar\s*ventana|switch\s*window|alt\s*tab)": ("keyboard", "hotkey", {"keys": ["alt", "tab"]}),
            r"(cerrar\s*ventana|close\s*window)": ("keyboard", "hotkey", {"keys": ["alt", "F4"]}),
            r"(mostrar\s*escritorio|show\s*desktop|minimizar\s*todo)": ("keyboard", "hotkey", {"keys": ["win", "d"]}),
            
            # === APLICACIONES ===
            r"(abre|abrir|open|ejecuta|ejecutar)\s+(el\s*)?(navegador|chrome|firefox|edge)": 
                ("app", "open", {"app": "chrome"}),
            r"(abre|abrir|open|ejecuta|ejecutar)\s+(el\s*)?(explorador|explorer)": 
                ("app", "open", {"app": "explorer"}),
            r"(abre|abrir|open|ejecuta|ejecutar)\s+(el\s*)?(bloc\s*de\s*notas|notepad)": 
                ("app", "open", {"app": "notepad"}),
            r"(abre|abrir|open|ejecuta|ejecutar)\s+(la\s*)?(calculadora|calc)": 
                ("app", "open", {"app": "calc"}),
            r"(abre|abrir|open|ejecuta|ejecutar)\s+(la\s*)?(terminal|cmd|consola)": 
                ("app", "open", {"app": "cmd"}),
            r"(abre|abrir|open|ejecuta|ejecutar)\s+(el\s*)?(vscode|visual\s*studio\s*code|code)": 
                ("app", "open", {"app": "code"}),
            r"(abre|abrir|open|ejecuta|ejecutar)\s+(el\s*)?(spotify)": 
                ("app", "open", {"app": "spotify"}),
            r"(abre|abrir|open|ejecuta|ejecutar)\s+(.+)": 
                ("app", "open", {"app": "$2"}),
            
            # URLs
            r"(busca|buscar|search)\s+en\s+google\s+(.+)": 
                ("app", "search_google", {"query": "$2"}),
            r"(busca|buscar|search)\s+en\s+youtube\s+(.+)": 
                ("app", "search_youtube", {"query": "$2"}),
            r"(abre|abrir|open)\s+(la\s*)?(página|url|sitio)\s+(.+)": 
                ("app", "open_url", {"url": "$4"}),
            
            # === SISTEMA ===
            # Volumen
            r"(sube|subir|aumenta|aumentar)\s*(el\s*)?(volumen|volume)": 
                ("system", "volume_up", {}),
            r"(baja|bajar|reduce|reducir)\s*(el\s*)?(volumen|volume)": 
                ("system", "volume_down", {}),
            r"(silencia|silenciar|mute|mutea|mutear)": 
                ("system", "volume_mute", {}),
            
            # Multimedia
            r"(play|pause|pausa|reproducir|pausar)": 
                ("system", "play_pause", {}),
            r"(siguiente|next|skip)(\s*canción|\s*track)?": 
                ("system", "next_track", {}),
            r"(anterior|previous|prev)(\s*canción|\s*track)?": 
                ("system", "prev_track", {}),
            
            # Screenshot
            r"(captura|screenshot|pantallazo|toma\s*captura)": 
                ("system", "screenshot", {}),
            
            # === CHAT/IA ===
            r"(desactivar|desactiva|dormir|duerme|sleep)": 
                ("chat", "deactivate", {}),
            r"(ayuda|help|comandos|commands)": 
                ("chat", "help", {}),
            r"(para|stop|detener|detente)": 
                ("chat", "stop", {}),
        }
        
        # Compilar patrones
        self.compiled_patterns = [
            (re.compile(pattern, re.IGNORECASE), action)
            for pattern, action in self.patterns.items()
        ]
    
    def parse(self, text: str) -> ParsedCommand:
        """Parsea texto y retorna comando."""
        text = text.strip().lower()
        
        for pattern, (cmd_type, action, params) in self.compiled_patterns:
            match = pattern.search(text)
            if match:
                # Reemplazar grupos capturados en params
                final_params = {}
                for key, value in params.items():
                    if isinstance(value, str) and value.startswith("$"):
                        group_num = int(value[1:])
                        if group_num <= len(match.groups()):
                            final_params[key] = match.group(group_num)
                        else:
                            final_params[key] = value
                    else:
                        final_params[key] = value
                
                return ParsedCommand(
                    type=CommandType(cmd_type),
                    action=action,
                    params=final_params,
                    original_text=text,
                    confidence=0.9
                )
        
        # No se reconoció como comando, es chat normal
        return ParsedCommand(
            type=CommandType.CHAT,
            action="chat",
            params={"message": text},
            original_text=text,
            confidence=0.5
        )
    
    def get_help(self) -> str:
        """Retorna ayuda de comandos."""
        return """🎤 **Comandos de Voz Disponibles:**

**🖱️ Mouse:**
• "mueve el mouse arriba/abajo/izquierda/derecha"
• "mueve el mouse al centro"
• "click" / "doble click" / "click derecho"
• "scroll arriba" / "scroll abajo"

**⌨️ Teclado:**
• "escribe [texto]"
• "presiona enter/escape/espacio/tab"
• "copiar" / "pegar" / "cortar"
• "deshacer" / "rehacer"
• "guardar" / "seleccionar todo"
• "nueva pestaña" / "cerrar pestaña"
• "cambiar ventana" / "cerrar ventana"

**📱 Aplicaciones:**
• "abre chrome/notepad/calculadora/terminal/vscode"
• "busca en google [texto]"
• "busca en youtube [texto]"

**🔊 Sistema:**
• "sube/baja el volumen"
• "silenciar"
• "play/pause" / "siguiente" / "anterior"
• "captura" (screenshot)

**💤 Control:**
• "desactivar" / "dormir" - Desactiva el asistente
• "ayuda" - Muestra esta ayuda
• "para" / "stop" - Detiene la acción actual
"""


class VoiceCommandHandler:
    """
    Manejador de comandos de voz.
    Ejecuta los comandos parseados.
    """
    
    def __init__(self, ia_personal=None):
        self.commands = VoiceCommands()
        self.system = SystemControl()
        self.ia = ia_personal
        
        # Callbacks
        self.on_command_executed: Optional[Callable[[ParsedCommand, str], None]] = None
        self.on_chat_message: Optional[Callable[[str], str]] = None
    
    def execute(self, text: str) -> Tuple[bool, str]:
        """
        Ejecuta un comando de voz.
        
        Returns:
            (success, response_message)
        """
        command = self.commands.parse(text)
        
        try:
            if command.type == CommandType.MOUSE:
                return self._execute_mouse(command)
            elif command.type == CommandType.KEYBOARD:
                return self._execute_keyboard(command)
            elif command.type == CommandType.APP:
                return self._execute_app(command)
            elif command.type == CommandType.SYSTEM:
                return self._execute_system(command)
            elif command.type == CommandType.CHAT:
                return self._execute_chat(command)
            else:
                return False, "Comando no reconocido"
        
        except Exception as e:
            return False, f"Error ejecutando comando: {e}"
    
    def _execute_mouse(self, cmd: ParsedCommand) -> Tuple[bool, str]:
        """Ejecuta comando de mouse."""
        action = cmd.action
        params = cmd.params
        
        if action == "move":
            self.system.move_mouse(params["direction"])
            return True, f"Mouse movido hacia {params['direction'].value}"
        
        elif action == "click":
            button = params.get("button", MouseButton.LEFT)
            self.system.click(button)
            return True, f"Click {button.value}"
        
        elif action == "double_click":
            self.system.double_click()
            return True, "Doble click"
        
        elif action == "scroll":
            self.system.scroll(params["amount"])
            direction = "arriba" if params["amount"] > 0 else "abajo"
            return True, f"Scroll {direction}"
        
        return False, "Acción de mouse no reconocida"
    
    def _execute_keyboard(self, cmd: ParsedCommand) -> Tuple[bool, str]:
        """Ejecuta comando de teclado."""
        action = cmd.action
        params = cmd.params
        
        if action == "type":
            text = params.get("text", "")
            if text:
                self.system.type_text_unicode(text)
                return True, f"Escrito: {text[:30]}..."
        
        elif action == "press":
            key = params.get("key", "")
            if key:
                self.system.press_key(key)
                return True, f"Tecla: {key}"
        
        elif action == "hotkey":
            keys = params.get("keys", [])
            if keys:
                self.system.hotkey(*keys)
                return True, f"Hotkey: {'+'.join(keys)}"
        
        return False, "Acción de teclado no reconocida"
    
    def _execute_app(self, cmd: ParsedCommand) -> Tuple[bool, str]:
        """Ejecuta comando de aplicación."""
        action = cmd.action
        params = cmd.params
        
        if action == "open":
            app = params.get("app", "")
            if app:
                success = self.system.open_app(app)
                if success:
                    return True, f"Abriendo {app}"
                return False, f"No se pudo abrir {app}"
        
        elif action == "search_google":
            query = params.get("query", "")
            if query:
                self.system.search_google(query)
                return True, f"Buscando en Google: {query}"
        
        elif action == "search_youtube":
            query = params.get("query", "")
            if query:
                self.system.search_youtube(query)
                return True, f"Buscando en YouTube: {query}"
        
        elif action == "open_url":
            url = params.get("url", "")
            if url:
                if not url.startswith("http"):
                    url = "https://" + url
                self.system.open_url(url)
                return True, f"Abriendo: {url}"
        
        return False, "Acción de aplicación no reconocida"
    
    def _execute_system(self, cmd: ParsedCommand) -> Tuple[bool, str]:
        """Ejecuta comando de sistema."""
        action = cmd.action
        
        if action == "volume_up":
            self.system.volume_up()
            return True, "Volumen aumentado"
        
        elif action == "volume_down":
            self.system.volume_down()
            return True, "Volumen reducido"
        
        elif action == "volume_mute":
            self.system.volume_mute()
            return True, "Mute toggle"
        
        elif action == "play_pause":
            self.system.play_pause()
            return True, "Play/Pause"
        
        elif action == "next_track":
            self.system.next_track()
            return True, "Siguiente pista"
        
        elif action == "prev_track":
            self.system.prev_track()
            return True, "Pista anterior"
        
        elif action == "screenshot":
            filename = self.system.screenshot()
            return True, f"Screenshot guardado: {filename}"
        
        return False, "Acción de sistema no reconocida"
    
    def _execute_chat(self, cmd: ParsedCommand) -> Tuple[bool, str]:
        """Ejecuta comando de chat."""
        action = cmd.action
        params = cmd.params
        
        if action == "help":
            return True, self.commands.get_help()
        
        elif action == "deactivate":
            return True, "DEACTIVATE"
        
        elif action == "stop":
            return True, "STOP"
        
        elif action == "chat":
            message = params.get("message", "")
            if self.on_chat_message:
                response = self.on_chat_message(message)
                return True, response
            elif self.ia:
                response = self.ia.chat(message)
                return True, response
            return True, message
        
        return False, "Acción de chat no reconocida"


# =============================================================================
# DEMO
# =============================================================================

def demo():
    """Demo de comandos de voz."""
    print("\n" + "=" * 60)
    print("   🎤 Demo de Comandos de Voz")
    print("   IA-Personal Voice")
    print("=" * 60)
    
    handler = VoiceCommandHandler()
    
    # Mostrar ayuda
    print(handler.commands.get_help())
    
    # Probar algunos comandos
    test_commands = [
        "mueve el mouse arriba",
        "click",
        "abre chrome",
        "busca en google inteligencia artificial",
        "sube el volumen",
        "hola como estas",
    ]
    
    print("\n📝 Probando comandos:")
    print("-" * 40)
    
    for cmd_text in test_commands:
        print(f"\n🗣️ '{cmd_text}'")
        command = handler.commands.parse(cmd_text)
        print(f"   Tipo: {command.type.value}")
        print(f"   Acción: {command.action}")
        print(f"   Params: {command.params}")
    
    print("\n" + "=" * 60)
    print("   Demo completada")
    print("=" * 60)


if __name__ == "__main__":
    demo()
