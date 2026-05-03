#!/usr/bin/env python3
"""Añade métodos faltantes a ir.rs: add_global, add_string"""

from pathlib import Path

IR_PATH = Path(r"C:\Users\andre\OneDrive\Documentos\ADead-BIB\C_Real_Optimo\compiler\middle\ir.rs")

# Leer archivo existente y añadir métodos antes del cierre de impl IrBuilder
content = IR_PATH.read_text(encoding='utf-8')

# Buscar donde añadir los métodos (antes del último } de impl IrBuilder)
methods = '''
    // Globals
    pub fn add_global(&mut self, name: &str, ty: IrType) {
        self.module.globals.push((name.to_string(), ty));
    }
    
    // Strings
    pub fn add_string(&mut self, s: &str) -> usize {
        let idx = self.module.strings.len();
        self.module.strings.push(s.to_string());
        idx
    }
'''

# Añadir campos a IrModule si no existen
if 'globals: Vec<(String, IrType)>' not in content:
    content = content.replace(
        'pub struct IrModule {',
        '''pub struct IrModule {
    pub globals: Vec<(String, IrType)>,
    pub strings: Vec<String>,'''
    )
    
    # También inicializar en new()
    content = content.replace(
        'functions: Vec::new(),',
        '''functions: Vec::new(),
            globals: Vec::new(),
            strings: Vec::new(),'''
    )

# Añadir métodos antes de finish()
if 'pub fn add_global' not in content:
    content = content.replace(
        '    pub fn finish(self) -> IrModule {',
        methods + '\n    pub fn finish(self) -> IrModule {'
    )

IR_PATH.write_text(content, encoding='utf-8')
print("✅ ir.rs: añadidos add_global, add_string")
