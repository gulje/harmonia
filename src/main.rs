mod build;
mod lexer;
mod parser;

fn main() {
    let input_code = r#"define renaming implementation my_implementation {
        if p contains "tap" {
            return "UwU"
        } else {
            return "rust and roll"
        }
    }

    hello1
"#;

    let mut lexer = lexer::Lexer::new(input_code);
    let tokens = lexer.lex();

    for token in tokens {
        println!("{token:?}");
    }
}
