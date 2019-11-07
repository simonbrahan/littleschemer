#[derive(Debug, PartialEq)]
enum LexToken {
    Num(f64),
    Symbol(String),
    String(String),
    LeftBracket,
    RightBracket,
}

fn lex_input(input: &str) -> Result<Vec<LexToken>, &'static str> {
    let mut output = Vec::new();

    let input_length = input.len();
    let mut current_idx = 0;

    while current_idx < input_length {
        if let Some((lexed_string, new_idx)) = lex_string(&input, current_idx) {
            output.push(LexToken::String(lexed_string));
            current_idx = new_idx;
            continue;
        }

        current_idx += 1;
    }

    Ok(output)
}

fn lex_string(input: &str, from_idx: usize) -> Option<(String, usize)> {
    if input.chars().nth(from_idx).unwrap() != '"' {
        return None;
    }

    let output = input
        .chars()
        .skip(from_idx + 1)
        .take_while(|&char| char != '"')
        .collect::<String>();

    Some((output.to_string(), from_idx + output.len() + 2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_string() {
        let input = "\"scheme\"";

        let expected_output = vec![LexToken::String("scheme".to_string())];

        let actual_output = lex_input(&input).unwrap();

        assert_eq!(actual_output, expected_output);
    }
}
