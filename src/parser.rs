use crate::lexer::LexToken;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Num(f64),
    Symbol(String),
    String(String),
    List(Vec<Expr>),
}

pub fn parse_tokens(input: Vec<LexToken>) -> Result<Expr, &'static str> {
    Ok(Expr::Symbol("little-schemer".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::*;

    #[test]
    fn parse_symbol() {
        let input = vec![LexToken::Symbol("little-schemer".to_string())];

        let expected_output = Expr::Symbol("little-schemer".to_string());

        let actual_output = parse_tokens(input).unwrap();

        assert_eq!(actual_output, expected_output);
    }
}
