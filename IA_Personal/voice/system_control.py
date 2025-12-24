"""
Control de Sistema para IA-Personal
=====================================
Author: Eddi Andreé Salazar Matos
Email: eddi.salazar.dev@gmail.com
Made with ❤️ in Peru 🇵🇪

Control del sistema por voz:
- Mover el mouse
- Clicks
- Escribir texto
- Abrir aplicaciones
- Control de volumen
- Y más...

Requisitos:
    pip install pyautogui pynput
"""

import os
import sys
import time
import subprocess
from pathlib import Path
from typing import Optional, Tuple, List
from dataclasses import dataclass
from enum import Enum

# Intentar importar pyautogui para control de mouse/teclado
try:
    import pyautogui
    pyautogui.FAILSAFE = True  # Mover mouse a esquina para detener
    pyautogui.PAUSE = 0.1  # Pausa entre acciones
    HAS_PYAUTOGUI = True
except ImportError:
    HAS_PYAUTOGUI = False
    print("⚠️ pyautogui no instalado: pip install pyautogui")

# Intentar importar pynput para escuchar teclas
try:
    from pynput import mouse, keyboard
    HAS_PYNPUT = True
except ImportError:
    HAS_PYNPUT = False


class MouseButton(Enum):
    """Botones del mouse."""
    LEFT = "left"
    RIGHT = "right"
    MIDDLE = "middle"


class Direction(Enum):
    """Direcciones de movimiento."""
    UP = "up"
    DOWN = "down"
    LEFT = "left"
    RIGHT = "right"
    CENTER = "center"


@dataclass
class ScreenInfo:
    """Información de la pantalla."""
    width: int
    height: int
    center_x: int
    center_y: int


class SystemControl:
    """
    Control del sistema (mouse, teclado, aplicaciones).
    Permite controlar el PC por comandos de voz.
    """
    
    def __init__(self):
        if not HAS_PYAUTOGUI:
            raise ImportError("Instala: pip install pyautogui")
        
        # Información de pantalla
        self.screen = self._get_screen_info()
        
        # Posición actual del mouse
        self.mouse_x, self.mouse_y = pyautogui.position()
        
        # Velocidad de movimiento
        self.move_speed = 100  # pixels por comando
        self.move_duration = 0.2  # segundos para animación
        
        # Aplicaciones comunes
        self.apps = {
            "navegador": "chrome",
            "chrome": "chrome",
            "firefox": "firefox",
            "edge": "msedge",
            "explorador": "explorer",
            "bloc de notas": "notepad",
            "notepad": "notepad",
            "calculadora": "calc",
            "terminal": "cmd",
            "cmd": "cmd",
            "powershell": "powershell",
            "vscode": "code",
            "visual studio code": "code",
            "spotify": "spotify",
        }
        
        print(f"🖥️ SystemControl inicializado")
        print(f"   Pantalla: {self.screen.width}x{self.screen.height}")
    
    def _get_screen_info(self) -> ScreenInfo:
        """Obtiene información de la pantalla."""
        width, height = pyautogui.size()
        return ScreenInfo(
            width=width,
            height=height,
            center_x=width // 2,
            center_y=height // 2
        )
    
    # =========================================================================
    # CONTROL DE MOUSE
    # =========================================================================
    
    def move_mouse(self, direction: Direction, distance: int = None):
        """Mueve el mouse en una dirección."""
        distance = distance or self.move_speed
        x, y = pyautogui.position()
        
        if direction == Direction.UP:
            y = max(0, y - distance)
        elif direction == Direction.DOWN:
            y = min(self.screen.height, y + distance)
        elif direction == Direction.LEFT:
            x = max(0, x - distance)
        elif direction == Direction.RIGHT:
            x = min(self.screen.width, x + distance)
        elif direction == Direction.CENTER:
            x, y = self.screen.center_x, self.screen.center_y
        
        pyautogui.moveTo(x, y, duration=self.move_duration)
        self.mouse_x, self.mouse_y = x, y
        print(f"🖱️ Mouse movido a ({x}, {y})")
    
    def move_mouse_to(self, x: int, y: int):
        """Mueve el mouse a una posición específica."""
        x = max(0, min(self.screen.width, x))
        y = max(0, min(self.screen.height, y))
        
        pyautogui.moveTo(x, y, duration=self.move_duration)
        self.mouse_x, self.mouse_y = x, y
        print(f"🖱️ Mouse movido a ({x}, {y})")
    
    def click(self, button: MouseButton = MouseButton.LEFT, clicks: int = 1):
        """Hace click con el mouse."""
        pyautogui.click(button=button.value, clicks=clicks)
        print(f"🖱️ Click {button.value} x{clicks}")
    
    def double_click(self):
        """Doble click."""
        pyautogui.doubleClick()
        print("🖱️ Doble click")
    
    def right_click(self):
        """Click derecho."""
        pyautogui.rightClick()
        print("🖱️ Click derecho")
    
    def scroll(self, amount: int):
        """Scroll del mouse."""
        pyautogui.scroll(amount)
        direction = "arriba" if amount > 0 else "abajo"
        print(f"🖱️ Scroll {direction}")
    
    def drag(self, direction: Direction, distance: int = None):
        """Arrastra el mouse."""
        distance = distance or self.move_speed
        x, y = pyautogui.position()
        
        if direction == Direction.UP:
            end_y = max(0, y - distance)
            pyautogui.drag(0, -distance, duration=self.move_duration)
        elif direction == Direction.DOWN:
            pyautogui.drag(0, distance, duration=self.move_duration)
        elif direction == Direction.LEFT:
            pyautogui.drag(-distance, 0, duration=self.move_duration)
        elif direction == Direction.RIGHT:
            pyautogui.drag(distance, 0, duration=self.move_duration)
        
        print(f"🖱️ Arrastrado hacia {direction.value}")
    
    def get_mouse_position(self) -> Tuple[int, int]:
        """Obtiene posición actual del mouse."""
        return pyautogui.position()
    
    # =========================================================================
    # CONTROL DE TECLADO
    # =========================================================================
    
    def type_text(self, text: str, interval: float = 0.05):
        """Escribe texto."""
        pyautogui.typewrite(text, interval=interval)
        print(f"⌨️ Escrito: {text[:50]}...")
    
    def type_text_unicode(self, text: str):
        """Escribe texto con caracteres especiales (español)."""
        pyautogui.write(text)
        print(f"⌨️ Escrito: {text[:50]}...")
    
    def press_key(self, key: str):
        """Presiona una tecla."""
        pyautogui.press(key)
        print(f"⌨️ Tecla: {key}")
    
    def hotkey(self, *keys):
        """Ejecuta combinación de teclas."""
        pyautogui.hotkey(*keys)
        print(f"⌨️ Hotkey: {'+'.join(keys)}")
    
    def copy(self):
        """Ctrl+C"""
        self.hotkey('ctrl', 'c')
    
    def paste(self):
        """Ctrl+V"""
        self.hotkey('ctrl', 'v')
    
    def cut(self):
        """Ctrl+X"""
        self.hotkey('ctrl', 'x')
    
    def undo(self):
        """Ctrl+Z"""
        self.hotkey('ctrl', 'z')
    
    def redo(self):
        """Ctrl+Y"""
        self.hotkey('ctrl', 'y')
    
    def select_all(self):
        """Ctrl+A"""
        self.hotkey('ctrl', 'a')
    
    def save(self):
        """Ctrl+S"""
        self.hotkey('ctrl', 's')
    
    def new_tab(self):
        """Ctrl+T"""
        self.hotkey('ctrl', 't')
    
    def close_tab(self):
        """Ctrl+W"""
        self.hotkey('ctrl', 'w')
    
    def switch_window(self):
        """Alt+Tab"""
        self.hotkey('alt', 'tab')
    
    def close_window(self):
        """Alt+F4"""
        self.hotkey('alt', 'F4')
    
    def minimize_all(self):
        """Win+D (mostrar escritorio)"""
        self.hotkey('win', 'd')
    
    def screenshot(self, filename: str = None) -> str:
        """Toma screenshot."""
        if filename is None:
            filename = f"screenshot_{int(time.time())}.png"
        
        screenshot = pyautogui.screenshot()
        screenshot.save(filename)
        print(f"📸 Screenshot guardado: {filename}")
        return filename
    
    # =========================================================================
    # CONTROL DE APLICACIONES
    # =========================================================================
    
    def open_app(self, app_name: str) -> bool:
        """Abre una aplicación."""
        app_name_lower = app_name.lower()
        
        # Buscar en diccionario de apps
        if app_name_lower in self.apps:
            app_cmd = self.apps[app_name_lower]
        else:
            app_cmd = app_name
        
        try:
            if sys.platform == "win32":
                os.startfile(app_cmd)
            else:
                subprocess.Popen([app_cmd])
            
            print(f"🚀 Abriendo: {app_name}")
            return True
        except Exception as e:
            print(f"❌ Error abriendo {app_name}: {e}")
            return False
    
    def open_url(self, url: str):
        """Abre una URL en el navegador."""
        import webbrowser
        webbrowser.open(url)
        print(f"🌐 Abriendo URL: {url}")
    
    def search_google(self, query: str):
        """Busca en Google."""
        url = f"https://www.google.com/search?q={query.replace(' ', '+')}"
        self.open_url(url)
    
    def search_youtube(self, query: str):
        """Busca en YouTube."""
        url = f"https://www.youtube.com/results?search_query={query.replace(' ', '+')}"
        self.open_url(url)
    
    # =========================================================================
    # CONTROL DE VOLUMEN (Windows)
    # =========================================================================
    
    def volume_up(self, steps: int = 2):
        """Sube el volumen."""
        for _ in range(steps):
            pyautogui.press('volumeup')
        print(f"🔊 Volumen +{steps}")
    
    def volume_down(self, steps: int = 2):
        """Baja el volumen."""
        for _ in range(steps):
            pyautogui.press('volumedown')
        print(f"🔉 Volumen -{steps}")
    
    def volume_mute(self):
        """Silencia/activa sonido."""
        pyautogui.press('volumemute')
        print("🔇 Mute toggle")
    
    def play_pause(self):
        """Play/Pause multimedia."""
        pyautogui.press('playpause')
        print("⏯️ Play/Pause")
    
    def next_track(self):
        """Siguiente pista."""
        pyautogui.press('nexttrack')
        print("⏭️ Siguiente")
    
    def prev_track(self):
        """Pista anterior."""
        pyautogui.press('prevtrack')
        print("⏮️ Anterior")
    
    # =========================================================================
    # UTILIDADES
    # =========================================================================
    
    def wait(self, seconds: float):
        """Espera un tiempo."""
        time.sleep(seconds)
    
    def alert(self, message: str, title: str = "IA-Personal"):
        """Muestra alerta."""
        pyautogui.alert(message, title)
    
    def confirm(self, message: str, title: str = "IA-Personal") -> bool:
        """Muestra confirmación."""
        return pyautogui.confirm(message, title) == "OK"
    
    def prompt(self, message: str, title: str = "IA-Personal") -> Optional[str]:
        """Muestra prompt para entrada."""
        return pyautogui.prompt(message, title)


# =============================================================================
# DEMO
# =============================================================================

def demo():
    """Demo de control de sistema."""
    print("\n" + "=" * 60)
    print("   🖥️ Demo de Control de Sistema")
    print("   IA-Personal Voice")
    print("=" * 60)
    
    if not HAS_PYAUTOGUI:
        print("\n❌ Instala las dependencias:")
        print("   pip install pyautogui")
        return
    
    control = SystemControl()
    
    print("\n📝 Demostración de comandos:")
    
    # Mostrar posición actual
    x, y = control.get_mouse_position()
    print(f"\n1. Posición actual del mouse: ({x}, {y})")
    
    # Mover al centro
    print("\n2. Moviendo mouse al centro...")
    control.move_mouse(Direction.CENTER)
    time.sleep(0.5)
    
    # Mover en direcciones
    print("\n3. Moviendo en direcciones...")
    control.move_mouse(Direction.UP, 100)
    time.sleep(0.3)
    control.move_mouse(Direction.RIGHT, 100)
    time.sleep(0.3)
    control.move_mouse(Direction.DOWN, 100)
    time.sleep(0.3)
    control.move_mouse(Direction.LEFT, 100)
    time.sleep(0.3)
    
    # Volver al centro
    control.move_mouse(Direction.CENTER)
    
    print("\n" + "=" * 60)
    print("   Demo completada")
    print("=" * 60)


if __name__ == "__main__":
    demo()
