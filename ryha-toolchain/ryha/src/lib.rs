// ryha-toolchain/ryha/src/lib.rs

pub mod ast;
pub mod lexer;
pub mod parser;

#[cfg(test)]
mod tests {
    use super::lexer::Lexer;
    use super::parser::Parser;
    use super::ast::Statement;

    #[test]
    fn test_let_statement() {
        let input = "let x = 5;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert_eq!(program.statements.len(), 1);

        let stmt = &program.statements[0];
        if let Statement::Let(name, _) = stmt {
            assert_eq!(name, "x");
        } else {
            panic!("Expected Let statement");
        }
    }
}
