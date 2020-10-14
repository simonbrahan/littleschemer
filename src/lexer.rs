#[derive(Debug, PartialEq)]
pub enum LexToken {
    Num(f64),
    Symbol(String),
    String(String),
    LeftBracket,
    RightBracket,
}

struct InputBuffer<'a> {
    input: &'a str,
    current_idx: usize,
}

impl InputBuffer<'_> {
    fn from_input(input: &str) -> InputBuffer {
        InputBuffer {
            input,
            current_idx: 0,
        }
    }

    fn has_chars_remaining(&self) -> bool {
        self.input.chars().count() > self.current_idx
    }

    fn next_char_is(&self, look_for: fn(char) -> bool) -> bool {
        let next_char = self
            .input
            .chars()
            .nth(self.current_idx)
            .expect("Lexxer skipped past the end of the input");

        look_for(next_char)
    }

    fn skip(&mut self, num_chars_to_skip: usize) {
        self.current_idx += num_chars_to_skip;
    }

    fn take_while(&mut self, look_for: for<'r> fn(&'r char) -> bool) -> String {
        let output = self
            .input
            .chars()
            .skip(self.current_idx)
            .take_while(look_for)
            .collect::<String>();

        self.current_idx += output.chars().count();

        output
    }
}

pub fn lex_input(input: &str) -> Result<Vec<LexToken>, &'static str> {
    let mut input_buffer = InputBuffer::from_input(input);
    let mut output = Vec::new();

    while input_buffer.has_chars_remaining() {
        if let Some(lexed_string) = lex_string(&mut input_buffer) {
            output.push(lexed_string);
            continue;
        }

        if let Some(lexed_number) = lex_number(&mut input_buffer) {
            output.push(lexed_number);
            continue;
        }

        if let Some(lexed_left_bracket) = lex_left_bracket(&mut input_buffer) {
            output.push(lexed_left_bracket);
            continue;
        }

        if let Some(lexed_right_bracket) = lex_right_bracket(&mut input_buffer) {
            output.push(lexed_right_bracket);
            continue;
        }

        if lex_whitespace(&mut input_buffer) {
            continue;
        }

        if let Some(lexed_symbol) = lex_symbol(&mut input_buffer) {
            output.push(lexed_symbol);
            continue;
        }
    }

    Ok(output)
}

fn lex_string(input: &mut InputBuffer) -> Option<LexToken> {
    if !input.next_char_is(|char| char == '"') {
        return None;
    }

    input.skip(1);
    let output = input.take_while(|char| *char != '"');
    input.skip(1);

    Some(LexToken::String(output))
}

fn lex_left_bracket(input: &mut InputBuffer) -> Option<LexToken> {
    if !input.next_char_is(|char| char == '(') {
        return None;
    }

    input.skip(1);

    Some(LexToken::LeftBracket)
}

fn lex_right_bracket(input: &mut InputBuffer) -> Option<LexToken> {
    if !input.next_char_is(|char| char == ')') {
        return None;
    }

    input.skip(1);

    Some(LexToken::RightBracket)
}

fn lex_whitespace(input: &mut InputBuffer) -> bool {
    if input.next_char_is(|char| char.is_whitespace()) {
        input.skip(1);
        return true;
    }

    false
}

fn lex_number(input: &mut InputBuffer) -> Option<LexToken> {
    if !input.next_char_is(|char| char.is_numeric() || char == '.' || char == 'e' || char == '-') {
        return None;
    }

    let num_as_string =
        input.take_while(|char| char.is_numeric() || *char == '.' || *char == 'e' || *char == '-');

    match num_as_string.parse::<f64>() {
        Ok(num) => Some(LexToken::Num(num)),
        Err(_) => None,
    }
}

fn lex_symbol(input: &mut InputBuffer) -> Option<LexToken> {
    let output = input.take_while(|char| !char.is_whitespace() && *char != '(' && *char != ')');

    dbg!(&output);

    Some(LexToken::Symbol(output))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_brackets() {
        let input = "()";

        let expected_output = vec![LexToken::LeftBracket, LexToken::RightBracket];

        compare(input, expected_output);
    }

    #[test]
    fn lex_string() {
        let input = r#""scheme""#;

        let expected_output = vec![LexToken::String("scheme".to_string())];

        compare(input, expected_output);
    }

    #[test]
    fn lex_list_of_strings() {
        let input = r#"("little" "scheme")"#;

        let expected_output = vec![
            LexToken::LeftBracket,
            LexToken::String("little".to_string()),
            LexToken::String("scheme".to_string()),
            LexToken::RightBracket,
        ];

        compare(input, expected_output);
    }

    #[test]
    fn lex_list_of_strings_with_whitespace() {
        let input = r#"  (  "little"   "scheme"  )  "#;

        let expected_output = vec![
            LexToken::LeftBracket,
            LexToken::String("little".to_string()),
            LexToken::String("scheme".to_string()),
            LexToken::RightBracket,
        ];

        compare(input, expected_output);
    }

    #[test]
    fn lex_number() {
        let tests = vec![
            ("123", LexToken::Num(123f64)),
            ("0.123", LexToken::Num(0.123f64)),
            ("-0.1e-5", LexToken::Num(-0.1e-5f64)),
        ];

        for (input, expect) in tests {
            compare(input, vec![expect]);
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

        compare(input, expected_output);
    }

    #[test]
    fn lex_symbol() {
        let tests = vec![
            ("some_func", LexToken::Symbol("some_func".to_string())),
            ("+", LexToken::Symbol("+".to_string())),
            (",", LexToken::Symbol(",".to_string())),
            ("#symbol", LexToken::Symbol("#symbol".to_string())),
        ];

        for (input, expect) in tests {
            compare(input, vec![expect]);
        }
    }

    #[test]
    fn lex_list_of_symbols() {
        let input = "(somefunc #some_symbol +)";

        let expected_output = vec![
            LexToken::LeftBracket,
            LexToken::Symbol("somefunc".to_string()),
            LexToken::Symbol("#some_symbol".to_string()),
            LexToken::Symbol("+".to_string()),
            LexToken::RightBracket,
        ];

        compare(input, expected_output);
    }

    #[test]
    fn lex_fizzbuzz() {
        let input = r#"
        (define (fizzable num) (= 0 (modulo num 3)))
        (define (buzzable num) (= 0 (modulo num 5)))

        (define (fizzbuzz num)
          (let ((isFizzable (fizzable num))
                (isBuzzable (buzzable num)))
            (cond
              ((and isFizzable isBuzzable) "fizzbuzz")
              (isFizzable "fizz")
              (isBuzzable "buzz")
              (#t (number->string num)))))

        (define (fizzbuzzrange fromnum tonum)
          (display (fizzbuzz fromnum))
          (newline)

          (if (< fromnum tonum)
            (fizzbuzzrange (+ fromnum 1) tonum)))

        (fizzbuzzrange 1 100)
        "#;

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

        compare(input, expected_output);
    }

    fn compare(input: &str, expected_output: Vec<LexToken>) {
        let actual_output = lex_input(input).unwrap();

        assert_eq!(actual_output, expected_output);
    }
}
