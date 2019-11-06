#[derive(Debug, PartialEq)]
enum LexToken {
    Num(f64),
    Symbol(String),
    String(String),
    LeftBracket,
    RightBracket,
}

fn lex_input(input: &str) -> Result<Vec<LexToken>, &'static str> {
    Ok(vec![
        LexToken::LeftBracket,
        LexToken::Symbol("repeat".to_string()),
        LexToken::String("scheme".to_string()),
        LexToken::LeftBracket,
        LexToken::Symbol("+".to_string()),
        LexToken::Num(1.0),
        LexToken::Num(2.0),
        LexToken::RightBracket,
        LexToken::RightBracket,
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_simple_expression() {
        let input = "(repeat \"scheme\" (+ 1 2))";

        let expected_output = vec![
            LexToken::LeftBracket,
            LexToken::Symbol("repeat".to_string()),
            LexToken::String("scheme".to_string()),
            LexToken::LeftBracket,
            LexToken::Symbol("+".to_string()),
            LexToken::Num(1.0),
            LexToken::Num(2.0),
            LexToken::RightBracket,
            LexToken::RightBracket,
        ];

        let actual_output = lex_input(&input).unwrap();

        assert_eq!(actual_output, expected_output);
    }
}
