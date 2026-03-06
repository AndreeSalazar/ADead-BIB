use adead_bib::frontend::c::{compile_c_to_program, c_lexer::CLexer, c_parser::CParser};

fn main() {
    let source = r#"
void uaf_inmediato() {
    int *ptr = (int*)malloc(sizeof(int));
    *ptr = 100;
    free(ptr);
    printf("%d\n", *ptr);
}
    "#;
    let mut lexer = CLexer::new(source);
    let mut tokens = Vec::new();
    let mut lines = Vec::new();
    loop {
        let tok = lexer.next_token();
        lines.push(lexer.line);
        tokens.push(tok.clone());
        if tok == adead_bib::frontend::c::c_lexer::CToken::Eof { break; }
    }
    for (t, l) in tokens.iter().zip(lines.iter()) {
        println!("{:?} at line {}", t, l);
    }
}
