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

        if let Some((lexed_number, new_idx)) = lex_number(&input, current_idx) {
            output.push(lexed_number);
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

        if let Some(new_idx) = lex_whitespace(&input, current_idx) {
            current_idx = new_idx;
            continue;
        }

        if let Some((lexed_symbol, new_idx)) = lex_symbol(&input, current_idx) {
            output.push(lexed_symbol);
            current_idx = new_idx;
            continue;
        }
    }

    Ok(output)
}

fn lex_string(input: &str, from_idx: usize) -> Option<(LexToken, usize)> {
    if input
        .chars()
        .nth(from_idx)
        .expect("Lexxer skipped past the end of the input")
        != '"'
    {
        return None;
    }

    let output = input
        .chars()
        .skip(from_idx + 1)
        .take_while(|&char| char != '"')
        .collect::<String>();

    Some((
        LexToken::String(output.to_string()),
        from_idx + output.len() + 2,
    ))
}

fn lex_left_bracket(input: &str, from_idx: usize) -> Option<(LexToken, usize)> {
    if input
        .chars()
        .nth(from_idx)
        .expect("Lexxer skipped past the end of the input")
        != '('
    {
        return None;
    }

    Some((LexToken::LeftBracket, from_idx + 1))
}

fn lex_right_bracket(input: &str, from_idx: usize) -> Option<(LexToken, usize)> {
    if input
        .chars()
        .nth(from_idx)
        .expect("Lexxer skipped past the end of the input")
        != ')'
    {
        return None;
    }

    Some((LexToken::RightBracket, from_idx + 1))
}

fn lex_whitespace(input: &str, from_idx: usize) -> Option<usize> {
    if input
        .chars()
        .nth(from_idx)
        .expect("Lexxer skipped past the end of the input")
        .is_whitespace()
    {
        return Some(from_idx + 1);
    }

    None
}

fn lex_number(input: &str, from_idx: usize) -> Option<(LexToken, usize)> {
    let next_char = input
        .chars()
        .nth(from_idx)
        .expect("Lexxer skipped past the end of the input");
    if !next_char.is_numeric() && next_char != '-' && next_char != '.' {
        return None;
    }

    let num_as_string = input
        .chars()
        .skip(from_idx)
        .take_while(|&char| char.is_numeric() || char == '.' || char == 'e' || char == '-')
        .collect::<String>();

    match num_as_string.parse::<f64>() {
        Ok(num) => Some((LexToken::Num(num), from_idx + num_as_string.len())),
        Err(_) => None,
    }
}

fn lex_symbol(input: &str, from_idx: usize) -> Option<(LexToken, usize)> {
    let output = input
        .chars()
        .skip(from_idx)
        .take_while(|&char| !char.is_whitespace() && char != '(' && char != ')')
        .collect::<String>();

    Some((
        LexToken::Symbol(output.to_string()),
        from_idx + output.len(),
    ))
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

    #[test]
    fn lex_list_of_strings_with_whitespace() {
        let input = "  (  \"little\"   \"scheme\"  )  ";

        let expected_output = vec![
            LexToken::LeftBracket,
            LexToken::String("little".to_string()),
            LexToken::String("scheme".to_string()),
            LexToken::RightBracket,
        ];

        let actual_output = lex_input(&input).unwrap();

        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn lex_number() {
        let tests = vec![
            ("123", LexToken::Num(123f64)),
            ("0.123", LexToken::Num(0.123f64)),
            ("-0.1e-5", LexToken::Num(-0.1e-5f64)),
        ];

        for (input, expect) in tests {
            let expected_output = vec![expect];
            let actual_output = lex_input(&input).unwrap();
            assert_eq!(actual_output, expected_output);
        }
    }

    #[test]
    fn lex_list_of_numbers() {
        let input = "(123 0.123 -0.1e-5)";

        let expected_output = vec![
            LexToken::LeftBracket,
            LexToken::Num(123f64),
            LexToken::Num(0.123f64),
            LexToken::Num(-0.1e-5f64),
            LexToken::RightBracket,
        ];

        let actual_output = lex_input(&input).unwrap();

        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn lex_symbol() {
        let tests = vec![
            ("some_func", LexToken::Symbol("some_func".to_string())),
            ("-", LexToken::Symbol("-".to_string())),
            ("e", LexToken::Symbol("e".to_string())),
        ];

        for (input, expect) in tests {
            let expected_output = vec![expect];
            let actual_output = lex_input(&input).unwrap();
            assert_eq!(actual_output, expected_output);
        }
    }

    #[test]
    fn lex_list_of_symbols() {
        let input = "(somefunc #some_symbol -)";

        let expected_output = vec![
            LexToken::LeftBracket,
            LexToken::Symbol("somefunc".to_string()),
            LexToken::Symbol("#some_symbol".to_string()),
            LexToken::Symbol("-".to_string()),
            LexToken::RightBracket,
        ];

        let actual_output = lex_input(&input).unwrap();

        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn lex_fizzbuzz() {
        let input = "
        (define (fizzable num) (= 0 (modulo num 3)))
        (define (buzzable num) (= 0 (modulo num 5)))

        (define (fizzbuzz num)
          (let ((isFizzable (fizzable num))
                (isBuzzable (buzzable num)))
            (cond
              ((and isFizzable isBuzzable) \"fizzbuzz\")
              (isFizzable \"fizz\")
              (isBuzzable \"buzz\")
              (#t (number->string num)))))

        (define (fizzbuzzrange fromnum tonum)
          (display (fizzbuzz fromnum))
          (newline)

          (if (< fromnum tonum)
            (fizzbuzzrange (+ fromnum 1) tonum)))

        (fizzbuzzrange 1 100)
        ";

        let expected_output = vec![
            // fizzable
            LexToken::LeftBracket,
            LexToken::Symbol("define".to_string()),
            LexToken::LeftBracket,
            LexToken::Symbol("fizzable".to_string()),
            LexToken::Symbol("num".to_string()),
            LexToken::RightBracket,
            LexToken::LeftBracket,
            LexToken::Symbol("=".to_string()),
            LexToken::Num(0.0),
            LexToken::LeftBracket,
            LexToken::Symbol("modulo".to_string()),
            LexToken::Symbol("num".to_string()),
            LexToken::Num(3.0),
            LexToken::RightBracket,
            LexToken::RightBracket,
            LexToken::RightBracket,
            // buzzable
            LexToken::LeftBracket,
            LexToken::Symbol("define".to_string()),
            LexToken::LeftBracket,
            LexToken::Symbol("buzzable".to_string()),
            LexToken::Symbol("num".to_string()),
            LexToken::RightBracket,
            LexToken::LeftBracket,
            LexToken::Symbol("=".to_string()),
            LexToken::Num(0.0),
            LexToken::LeftBracket,
            LexToken::Symbol("modulo".to_string()),
            LexToken::Symbol("num".to_string()),
            LexToken::Num(5.0),
            LexToken::RightBracket,
            LexToken::RightBracket,
            LexToken::RightBracket,
            // fizzbuzz
            LexToken::LeftBracket,
            LexToken::Symbol("define".to_string()),
            LexToken::LeftBracket,
            LexToken::Symbol("fizzbuzz".to_string()),
            LexToken::Symbol("num".to_string()),
            LexToken::RightBracket,
            LexToken::LeftBracket,
            LexToken::Symbol("let".to_string()),
            LexToken::LeftBracket,
            LexToken::LeftBracket,
            LexToken::Symbol("isFizzable".to_string()),
            LexToken::LeftBracket,
            LexToken::Symbol("fizzable".to_string()),
            LexToken::Symbol("num".to_string()),
            LexToken::RightBracket,
            LexToken::RightBracket,
            LexToken::LeftBracket,
            LexToken::Symbol("isBuzzable".to_string()),
            LexToken::LeftBracket,
            LexToken::Symbol("buzzable".to_string()),
            LexToken::Symbol("num".to_string()),
            LexToken::RightBracket,
            LexToken::RightBracket,
            LexToken::RightBracket,
            LexToken::LeftBracket,
            LexToken::Symbol("cond".to_string()),
            LexToken::LeftBracket,
            LexToken::LeftBracket,
            LexToken::Symbol("and".to_string()),
            LexToken::Symbol("isFizzable".to_string()),
            LexToken::Symbol("isBuzzable".to_string()),
            LexToken::RightBracket,
            LexToken::String("fizzbuzz".to_string()),
            LexToken::RightBracket,
            LexToken::LeftBracket,
            LexToken::Symbol("isFizzable".to_string()),
            LexToken::String("fizz".to_string()),
            LexToken::RightBracket,
            LexToken::LeftBracket,
            LexToken::Symbol("isBuzzable".to_string()),
            LexToken::String("buzz".to_string()),
            LexToken::RightBracket,
            LexToken::LeftBracket,
            LexToken::Symbol("#t".to_string()),
            LexToken::LeftBracket,
            LexToken::Symbol("number->string".to_string()),
            LexToken::Symbol("num".to_string()),
            LexToken::RightBracket,
            LexToken::RightBracket,
            LexToken::RightBracket,
            LexToken::RightBracket,
            LexToken::RightBracket,
            // fizzbuzzrange
            LexToken::LeftBracket,
            LexToken::Symbol("define".to_string()),
            LexToken::LeftBracket,
            LexToken::Symbol("fizzbuzzrange".to_string()),
            LexToken::Symbol("fromnum".to_string()),
            LexToken::Symbol("tonum".to_string()),
            LexToken::RightBracket,
            LexToken::LeftBracket,
            LexToken::Symbol("display".to_string()),
            LexToken::LeftBracket,
            LexToken::Symbol("fizzbuzz".to_string()),
            LexToken::Symbol("fromnum".to_string()),
            LexToken::RightBracket,
            LexToken::RightBracket,
            LexToken::LeftBracket,
            LexToken::Symbol("newline".to_string()),
            LexToken::RightBracket,
            LexToken::LeftBracket,
            LexToken::Symbol("if".to_string()),
            LexToken::LeftBracket,
            LexToken::Symbol("<".to_string()),
            LexToken::Symbol("fromnum".to_string()),
            LexToken::Symbol("tonum".to_string()),
            LexToken::RightBracket,
            LexToken::LeftBracket,
            LexToken::Symbol("fizzbuzzrange".to_string()),
            LexToken::LeftBracket,
            LexToken::Symbol("+".to_string()),
            LexToken::Symbol("fromnum".to_string()),
            LexToken::Num(1.0),
            LexToken::RightBracket,
            LexToken::Symbol("tonum".to_string()),
            LexToken::RightBracket,
            LexToken::RightBracket,
            LexToken::RightBracket,
            // call to fizzbuzzrange
            LexToken::LeftBracket,
            LexToken::Symbol("fizzbuzzrange".to_string()),
            LexToken::Num(1.0),
            LexToken::Num(100.0),
            LexToken::RightBracket,
        ];

        let actual_output = lex_input(&input).unwrap();

        assert_eq!(actual_output, expected_output);
    }
}
