use wzn::Lexer;

fn main() {
    // let lexer = Lexer::new("let x = 10; let mut y = x + 1;");

    // let lexer = Lexer::new("   \"hello\"   ;");
    // let lexer = Lexer::new("let s = \"hello\";");

    // let mut lexer = Lexer::new("   let id   = ;");
    // let lexer = Lexer::new("let id =  14.04 ;");
    let input = "let  id 43 23.23;";
    let mut lexer = Lexer::new(input);

    for i in 0..10 {
        println!("{:?}", lexer.next());
    }

    // let tokens: Vec<_> = lexer.collect();
    // println!("{:?}", tokens);
}
