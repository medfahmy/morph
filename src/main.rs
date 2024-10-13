use std::io::{stdin, stdout, Write};
use morph::{Parser, Env};


fn main() {
    let stdin = stdin();
    let mut stdout = stdout();

    loop {
        print!(">> ");
        stdout.flush().unwrap();
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();

        let program = Parser::parse(&buf);

        for result in program.results() {
            match result {
            }
        }
    }
}
