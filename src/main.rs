use morph::{Lexer, Parser};
use std::io::{stdin, stdout, Write};

fn main() {
    let stdin = stdin();
    let mut stdout = stdout();

    loop {
        print!(">> ");
        stdout.flush().unwrap();
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();

        let lexer = Lexer::new(&buf);

        // for token in lexer {
        //     println!("{:?}", token);
        // }

        let program = Parser::parse(&buf);

        println!("{:?}", program);
    }
}
