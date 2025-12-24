use crate::frontend::ast::*;
use crate::frontend::types::Type;
use std::collections::HashMap;

pub struct TypeChecker {
    symbol_table: HashMap<String, Type>,
    current_return_type: Type,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            symbol_table: HashMap::new(),
            current_return_type: Type::Void,
        }
    }
    
    pub fn check_program(&mut self, program: &Program) -> HashMap<String, Type> {
        let mut types = HashMap::new();
        
        // Primero registrar funciones
        for func in &program.functions {
            let ret_type = self.parse_type_name(func.return_type.as_deref().unwrap_or("void"));
            types.insert(func.name.clone(), ret_type);
        }
        
        // Verificar cuerpos
        for func in &program.functions {
            self.check_function(func);
        }
        
        // Devolver tabla de símbolos global (simplificado)
        self.symbol_table.clone()
    }
    
    fn check_function(&mut self, func: &Function) {
        self.symbol_table.clear();
        
        // Registrar parámetros
        for param in &func.params {
            let type_ = self.parse_type_name(param.type_name.as_deref().unwrap_or("var"));
            self.symbol_table.insert(param.name.clone(), type_);
        }
        
        self.current_return_type = self.parse_type_name(func.return_type.as_deref().unwrap_or("void"));
        
        // Verificar statements
        for stmt in &func.body {
            self.check_stmt(stmt);
        }
    }
    
    fn check_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Assign { name, value } => {
                let val_type = self.infer_expr(value);
                self.symbol_table.insert(name.clone(), val_type);
            }
            Stmt::If { condition, then_body, else_body } => {
                self.infer_expr(condition); // Debe ser Bool
                for s in then_body { self.check_stmt(s); }
                if let Some(else_stmts) = else_body {
                    for s in else_stmts { self.check_stmt(s); }
                }
            }
            Stmt::Return(Some(expr)) => {
                let ret = self.infer_expr(expr);
                // Aquí verificaríamos ret == self.current_return_type
            }
            _ => {} // Implementar resto
        }
    }
    
    fn infer_expr(&self, expr: &Expr) -> Type {
        match expr {
            Expr::Number(_) => Type::Int,
            Expr::Float(_) => Type::Float,
            Expr::String(_) => Type::String,
            Expr::Bool(_) => Type::Bool,
            Expr::Variable(name) => self.symbol_table.get(name).cloned().unwrap_or(Type::Unknown),
            Expr::BinaryOp { left, right, op } => {
                let l = self.infer_expr(left);
                let r = self.infer_expr(right);
                
                if l == Type::Float || r == Type::Float {
                    Type::Float
                } else {
                    l
                }
            }
            _ => Type::Unknown,
        }
    }
    
    fn parse_type_name(&self, name: &str) -> Type {
        match name {
            "int" => Type::Int,
            "float" => Type::Float,
            "bool" => Type::Bool,
            "string" => Type::String,
            "void" => Type::Void,
            "vec4" => Type::Vec4,
            "vec8" => Type::Vec8,
            _ => Type::Unknown, // O Class(name)
        }
    }
}
