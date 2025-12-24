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
                if val_type == Type::Unknown {
                    eprintln!("⚠️  Warning: Cannot infer type for variable '{}'", name);
                }
                self.symbol_table.insert(name.clone(), val_type);
            }
            Stmt::If { condition, then_body, else_body } => {
                let cond_type = self.infer_expr(condition);
                if cond_type != Type::Bool && cond_type != Type::Unknown {
                    eprintln!("⚠️  Warning: Condition in if statement should be bool, found {:?}", cond_type);
                }
                for s in then_body { self.check_stmt(s); }
                if let Some(else_stmts) = else_body {
                    for s in else_stmts { self.check_stmt(s); }
                }
            }
            Stmt::Return(Some(expr)) => {
                let ret_type = self.infer_expr(expr);
                if self.current_return_type != Type::Void && ret_type != self.current_return_type {
                    if ret_type != Type::Unknown && self.current_return_type != Type::Unknown {
                        eprintln!("⚠️  Warning: Return type mismatch. Expected {:?}, found {:?}", 
                                 self.current_return_type, ret_type);
                    }
                }
            }
            Stmt::Return(None) => {
                if self.current_return_type != Type::Void {
                    eprintln!("⚠️  Warning: Function should return {:?}, but found void return", 
                             self.current_return_type);
                }
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
            Expr::Null => Type::Unknown,
            Expr::Variable(name) => {
                self.symbol_table.get(name).cloned().unwrap_or_else(|| {
                    eprintln!("⚠️  Warning: Variable '{}' used before assignment", name);
                    Type::Unknown
                })
            }
            Expr::BinaryOp { left, right, op } => {
                let l = self.infer_expr(left);
                let r = self.infer_expr(right);
                
                // Validar tipos compatibles
                match op {
                    BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div => {
                        if l == Type::String || r == Type::String {
                            eprintln!("⚠️  Warning: Cannot perform arithmetic on string types");
                            return Type::Unknown;
                        }
                        if l == Type::Float || r == Type::Float {
                            Type::Float
                        } else if l == Type::Int && r == Type::Int {
                            Type::Int
                        } else {
                            Type::Unknown
                        }
                    }
                    BinOp::Mod => {
                        if l != Type::Int || r != Type::Int {
                            eprintln!("⚠️  Warning: Modulo operator requires integer operands");
                            return Type::Unknown;
                        }
                        Type::Int
                    }
                    BinOp::And | BinOp::Or => {
                        if l != Type::Bool || r != Type::Bool {
                            eprintln!("⚠️  Warning: Logical operators require boolean operands");
                            return Type::Unknown;
                        }
                        Type::Bool
                    }
                }
            }
            Expr::Comparison { left, right, op: _op } => {
                let _l = self.infer_expr(left);
                let _r = self.infer_expr(right);
                Type::Bool
            }
            Expr::Call { name, args: _args } => {
                // Por ahora, asumimos que las funciones built-in retornan tipos conocidos
                // En el futuro, esto debería consultar la tabla de símbolos de funciones
                eprintln!("⚠️  Warning: Cannot infer return type for function call '{}'", name);
                Type::Unknown
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
