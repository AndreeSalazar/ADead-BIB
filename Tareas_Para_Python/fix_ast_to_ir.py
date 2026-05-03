#!/usr/bin/env python3
"""Corrige ast_to_ir.rs para tipos correctos"""

from pathlib import Path

PATH = Path(r"C:\Users\andre\OneDrive\Documentos\ADead-BIB\C_Real_Optimo\compiler\middle\ast_to_ir.rs")

content = PATH.read_text(encoding='utf-8')

# Fix: p.name is Option<String>, unwrap it
content = content.replace(
    '.map(|p| (p.name.clone(), self.convert_type(&p.ty)))',
    '.filter_map(|p| p.name.clone().map(|n| (n, self.convert_type(&p.ty))))'
)

# Fix: func.body is Vec<Stmt>, not Option<Stmt>
content = content.replace(
    '''        // Convert body
        if let Some(body) = &func.body {
            self.convert_stmt(body);
        }''',
    '''        // Convert body
        for stmt in &func.body {
            self.convert_stmt(stmt);
        }'''
)

PATH.write_text(content, encoding='utf-8')
print("✅ ast_to_ir.rs: corregido")
