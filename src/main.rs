use morph::parser::Parser;
use std::io::{stdin, stdout, Write};

fn main() {
    let stdin = stdin();
    let mut stdout = stdout();

    println!("Morph v0.1.0");
    println!();

    loop {
        print!(">> ");
        stdout.flush().unwrap();
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();

        // for token in lexer {
        //     println!("{:?}", token);
        // }

        let mut parser = Parser::new(&buf);
        let ast = parser.parse();

        match ast {
            Ok(ast) => {
                for stmt in ast.stmts() {
                    println!("{:?}", stmt);
                }
            }
            Err(err) => println!("{:?}", err),
        }

        println!();
    }
}
