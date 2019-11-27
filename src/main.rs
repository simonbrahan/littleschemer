use std::io::{self, Write};

mod lexer;

fn main() {
    println!("Little Scheme In Rust");

    loop {
        let input = get_input();

        println!("{:?}", lexer::lex_input(&input));
    }
}

fn get_input() -> String {
    let mut input = String::new();

    print!("user> ");
    let _ = io::stdout().flush();

    io::stdin()
        .read_line(&mut input)
        .expect("Could not read line from STDIN");

    input.trim().to_string()
}
