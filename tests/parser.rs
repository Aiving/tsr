use std::{fs, io};

use tsr_lexer::Lexer;
use tsr_parser::Parser;

#[test]
fn main() -> io::Result<()> {
    let input = fs::read_to_string("main.tsx")?;
    let code = input.as_bytes();

    let (_, tokens) = Lexer::lex_tokens(code.into()).unwrap();
    let (_, ast) = Parser::parse_tokens(&tokens).unwrap();

    println!(
        "[\n{}\n]",
        ast.value
            .iter()
            .map(|token| format!("  {token:#?}"))
            .collect::<Vec<_>>()
            .join(",\n")
    );

    Ok(())
}
