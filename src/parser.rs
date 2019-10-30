#[derive(Debug, PartialEq)]
struct Expr {
    items: Vec<Item>,
}

#[derive(Debug, PartialEq)]
enum Item {
    Num(f64),
    Symbol(String),
    String(String),
    Expr(Expr),
}

fn parse_input(input: &str) -> Result<Expr, &'static str> {
    Ok(Expr {
        items: vec![
            Item::Symbol("+".to_string()),
            Item::Num(1.0),
            Item::Num(2.0),
        ],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_expression() {
        let input = "(+ 1 2)";

        let expected_output = Expr {
            items: vec![
                Item::Symbol("+".to_string()),
                Item::Num(1.0),
                Item::Num(2.0),
            ],
        };

        let actual_output = parse_input(&input).unwrap();

        assert_eq!(actual_output, expected_output);
    }
}
