#!/usr/bin/env python3
"""Corrige cli/main.rs para API correcta de Optimizer y UbDetector"""

from pathlib import Path

PATH = Path(r"C:\Users\andre\OneDrive\Documentos\ADead-BIB\C_Real_Optimo\compiler\cli\main.rs")

content = PATH.read_text(encoding='utf-8')

# Fix: Optimizer::run toma &mut self, &mut IrModule
content = content.replace(
    '''    // [5] OPTIMIZER
    print!("  [5/7] Optimizer... ");
    let ir = Optimizer::run(ir);
    println!("optimized");''',
    '''    // [5] OPTIMIZER
    print!("  [5/7] Optimizer... ");
    let mut ir = ir;
    let mut opt = Optimizer::new();
    opt.run(&mut ir);
    println!("optimized");'''
)

# Fix: UbDetector::check -> detect()
content = content.replace(
    '''    // [6] UB DETECTOR
    print!("  [6/7] UB Check... ");
    let issues = UbDetector::check(&ir);
    if issues.is_empty() {
        println!("clean");
    } else {
        println!("{} warnings", issues.len());
        for issue in &issues {
            eprintln!("    ⚠ {:?}", issue);
        }
    }''',
    '''    // [6] UB DETECTOR
    print!("  [6/7] UB Check... ");
    let mut ub = UbDetector::new();
    let issues = ub.detect(&ir);
    if issues.is_empty() {
        println!("clean");
    } else {
        println!("{} warnings", issues.len());
        for issue in &issues {
            eprintln!("    ⚠ {:?}", issue);
        }
    }'''
)

PATH.write_text(content, encoding='utf-8')
print("✅ cli/main.rs: corregido")
