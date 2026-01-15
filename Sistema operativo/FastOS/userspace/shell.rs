// ============================================================================
// FastOS - Shell
// ============================================================================
// Shell interactiva del sistema
//
// Author: Eddi Andreé Salazar Matos 🇵🇪
// ============================================================================

#![allow(dead_code)]

/// Comandos disponibles
pub enum Command {
    Help,
    Clear,
    Info,
    Mem,
    Ls,
    Cd(String),
    Cat(String),
    Echo(String),
    Reboot,
    Shutdown,
    Unknown(String),
}

/// Parsear comando
pub fn parse_command(input: &str) -> Command {
    let input = input.trim();
    let parts: Vec<&str> = input.split_whitespace().collect();
    
    if parts.is_empty() {
        return Command::Unknown(String::new());
    }
    
    match parts[0] {
        "help" | "?" => Command::Help,
        "clear" | "cls" => Command::Clear,
        "info" | "about" => Command::Info,
        "mem" | "memory" => Command::Mem,
        "ls" | "dir" => Command::Ls,
        "cd" => {
            if parts.len() > 1 {
                Command::Cd(parts[1].to_string())
            } else {
                Command::Cd("/".to_string())
            }
        }
        "cat" | "type" => {
            if parts.len() > 1 {
                Command::Cat(parts[1].to_string())
            } else {
                Command::Unknown("cat: missing filename".to_string())
            }
        }
        "echo" => {
            let text = parts[1..].join(" ");
            Command::Echo(text)
        }
        "reboot" | "restart" => Command::Reboot,
        "shutdown" | "halt" | "poweroff" => Command::Shutdown,
        _ => Command::Unknown(parts[0].to_string()),
    }
}

/// Texto de ayuda
pub const HELP_TEXT: &str = r#"
FastOS Shell - Comandos disponibles:

  help, ?        - Mostrar esta ayuda
  clear, cls     - Limpiar pantalla
  info, about    - Informacion del sistema
  mem, memory    - Estado de memoria
  ls, dir        - Listar archivos
  cd <dir>       - Cambiar directorio
  cat <file>     - Mostrar contenido de archivo
  echo <text>    - Imprimir texto
  reboot         - Reiniciar sistema
  shutdown       - Apagar sistema

Atajos de teclado:
  Ctrl+C         - Cancelar comando
  Ctrl+L         - Limpiar pantalla
"#;

/// Información del sistema
pub const INFO_TEXT: &str = r#"
╔════════════════════════════════════════════════════════════╗
║                    FastOS v0.1.0                           ║
╠════════════════════════════════════════════════════════════╣
║  Stack: ADead-BIB + Rust + wgpu                            ║
║  Arquitectura: x86_64                                      ║
║  Kernel: Monolítico                                        ║
║  Licencia: GPLv2                                           ║
╠════════════════════════════════════════════════════════════╣
║  Author: Eddi Andreé Salazar Matos                         ║
║  Email: eddi.salazar.dev@gmail.com                         ║
║  País: Perú 🇵🇪                                             ║
╚════════════════════════════════════════════════════════════╝
"#;

// String simple para no_std
pub struct String {
    data: [u8; 256],
    len: usize,
}

impl String {
    pub fn new() -> Self {
        String {
            data: [0; 256],
            len: 0,
        }
    }

    pub fn from(s: &str) -> Self {
        let mut string = String::new();
        let bytes = s.as_bytes();
        let copy_len = bytes.len().min(255);
        string.data[..copy_len].copy_from_slice(&bytes[..copy_len]);
        string.len = copy_len;
        string
    }

    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.data[..self.len]).unwrap_or("")
    }

    pub fn to_string(&self) -> Self {
        let mut new = String::new();
        new.data = self.data;
        new.len = self.len;
        new
    }
}

impl Default for String {
    fn default() -> Self {
        String::new()
    }
}
