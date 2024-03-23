use std::{fs, io};

use tsr_lexer::Lexer;

#[test]
fn main() -> io::Result<()> {
    let input = fs::read_to_string("main.tsx")?;
    let code = input.as_bytes();

    let (_, tokens) = Lexer::lex_tokens(code.into()).unwrap();

    println!(
        "[\n{}\n]",
        tokens
            .iter()
            .map(|token| format!("  {token:?}"))
            .collect::<Vec<_>>()
            .join(",\n")
    );

    Ok(())
}
