use std::io::{stdin, stdout, Write};
use wzn::{Lexer, Token};

fn main() {
    let input = "\"Hello World\"";
    let lexer = Lexer::new(input);

    let tokens: Vec<_> = lexer.collect();
    println!("{:?}", tokens);

    // let stdin = stdin();
    // let mut stdout = stdout();
    //
    // loop {
    //     print!(">> ");
    //     stdout.flush().unwrap();
    //     let mut buf = String::new();
    //     stdin.read_line(&mut buf).unwrap();
    //
    //     if errors.is_empty() {
    //         println!("{}", program.eval(&mut env));
    //     } else {
    //         println!("parser errors: ");
    //
    //         for error in errors {
    //             println!("- {}", error);
    //         }
    //     }
    // }
}
