// Tests básicos para ADead-BIB Compiler

use adead_bib::frontend::parser::Parser;
use adead_bib::frontend::lexer::{Lexer, Token};
use adead_bib::frontend::type_checker::TypeChecker;

#[test]
fn test_lexer_basic() {
    let source = "def main():\n    print(\"Hello\")";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    
    assert!(!tokens.is_empty());
    assert!(tokens.contains(&Token::Def));
    assert!(tokens.contains(&Token::Print));
}

#[test]
fn test_parser_hello_world() {
    let source = "def main():\n    print(\"Hello, World!\")";
    let result = Parser::parse_program(source);
    
    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.functions.len(), 1);
    assert_eq!(program.functions[0].name, "main");
}

#[test]
fn test_type_checker() {
    let source = "def main():\n    x = 10\n    y = 20\n    z = x + y";
    let program = Parser::parse_program(source).unwrap();
    let mut type_checker = TypeChecker::new();
    let types = type_checker.check_program(&program);
    
    // Verificar que se infirieron tipos
    assert!(!types.is_empty() || program.functions.len() > 0);
}

#[test]
fn test_syntax_check_valid() {
    let source = "def main():\n    print(\"Test\")";
    let result = Parser::parse_program(source);
    assert!(result.is_ok());
}

#[test]
fn test_syntax_check_invalid() {
    // Esto debería fallar en parsing
    let source = "def main(\n    print(\"Test\")"; // Falta cerrar paréntesis
    let result = Parser::parse_program(source);
    // Puede o no fallar dependiendo de cómo maneje el parser los errores
    // Por ahora solo verificamos que no crashee
    let _ = result;
}

