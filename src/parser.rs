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
            output.push(lexed_string);
            current_idx = new_idx;
            continue;
        }

        if let Some((lexed_left_bracket, new_idx)) = lex_left_bracket(&input, current_idx) {
            output.push(lexed_left_bracket);
            current_idx = new_idx;
            continue;
        }

        if let Some((lexed_right_bracket, new_idx)) = lex_right_bracket(&input, current_idx) {
            output.push(lexed_right_bracket);
            current_idx = new_idx;
            continue;
        }

        current_idx += 1;
    }

    Ok(output)
}

fn lex_string(input: &str, from_idx: usize) -> Option<(LexToken, usize)> {
    if input.chars().nth(from_idx).unwrap() != '"' {
        return None;
    }

    let output = input
        .chars()
        .skip(from_idx + 1)
        .take_while(|&char| char != '"')
        .collect::<String>();

    Some((LexToken::String(output.to_string()), from_idx + output.len() + 2))
}

fn lex_left_bracket(input: &str, from_idx: usize) -> Option<(LexToken, usize)> {
    if input.chars().nth(from_idx).unwrap() != '(' {
        return None;
    }

    Some((LexToken::LeftBracket, from_idx + 1))
}

fn lex_right_bracket(input: &str, from_idx: usize) -> Option<(LexToken, usize)> {
    if input.chars().nth(from_idx).unwrap() != ')' {
        return None;
    }

    Some((LexToken::RightBracket, from_idx + 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_brackets() {
        let input = "()";

        let expected_output = vec![LexToken::LeftBracket, LexToken::RightBracket];

        let actual_output = lex_input(&input).unwrap();

        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn lex_string() {
        let input = "\"scheme\"";

        let expected_output = vec![LexToken::String("scheme".to_string())];

        let actual_output = lex_input(&input).unwrap();

        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn lex_list_of_strings() {
        let input = "(\"little\" \"scheme\")";

        let expected_output = vec![
            LexToken::LeftBracket,
            LexToken::String("little".to_string()),
            LexToken::String("scheme".to_string()),
            LexToken::RightBracket,
        ];

        let actual_output = lex_input(&input).unwrap();

        assert_eq!(actual_output, expected_output);
    }
}
