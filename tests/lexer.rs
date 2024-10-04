use wzn::{Lexer, Token, Token::*};

fn lex<'a>(input: &'a str) -> Vec<Token> {
    Lexer::new(input).collect()
}

#[test]
fn keywords() {
    assert_eq!(lex("let"), vec![Let]);
    assert_eq!(lex("mut"), vec![Mut]);
    assert_eq!(lex("if"), vec![If]);
    assert_eq!(lex("else"), vec![Else]);
    assert_eq!(lex("while"), vec![While]);
    assert_eq!(lex("for"), vec![For]);
    assert_eq!(lex("loop"), vec![Loop]);
    assert_eq!(lex("match"), vec![Match]);
    assert_eq!(lex("break"), vec![Break]);
    assert_eq!(lex("continue"), vec![Continue]);
    assert_eq!(lex("type"), vec![Type]);
    assert_eq!(lex("struct"), vec![Struct]);
    assert_eq!(lex("enum"), vec![Enum]);
    assert_eq!(lex("impl"), vec![Impl]);
    assert_eq!(lex("trait"), vec![Trait]);
    assert_eq!(lex("where"), vec![Where]);
    assert_eq!(lex("as"), vec![As]);
    assert_eq!(lex("pub"), vec![Pub]);
    assert_eq!(lex("const"), vec![Const]);
    assert_eq!(lex("static"), vec![Static]);
    assert_eq!(lex("fn"), vec![Function]);
    assert_eq!(lex("return"), vec![Return]);
}

#[test]
fn identifiers() {
    assert_eq!(lex("foo"), vec![Identifier("foo")]);
    assert_eq!(lex("bar123"), vec![Identifier("bar123")]);
    assert_eq!(lex("_underscore"), vec![Identifier("_underscore")]);
}

#[test]
fn primitives() {
    assert_eq!(lex("42"), vec![Int("42")]);
    assert_eq!(lex("3.14"), vec![Float("3.14")]);
    assert_eq!(lex("true"), vec![Bool(true)]);
    assert_eq!(lex("false"), vec![Bool(false)]);
    assert_eq!(lex("'a'"), vec![Char('a')]);
    assert_eq!(lex("\"hello\""), vec![Str("hello")]);

    assert_eq!(
        lex("let  id 43 23.23;"),
        vec![Let, Identifier("id"), Int("43"), Float("23.23"), Semicolon,]
    );
}

#[test]
fn operators() {
    assert_eq!(lex("&&"), vec![And]);
    assert_eq!(lex("||"), vec![Or]);
    assert_eq!(lex("!"), vec![Not]);
    assert_eq!(lex("=="), vec![Equal]);
    assert_eq!(lex("!="), vec![NotEqual]);
    assert_eq!(lex("<"), vec![LessThan]);
    assert_eq!(lex(">"), vec![GreaterThan]);
    assert_eq!(lex("<="), vec![LessEqual]);
    assert_eq!(lex(">="), vec![GreaterEqual]);
}

#[test]
fn math_operators() {
    assert_eq!(lex("+"), vec![Plus]);
    assert_eq!(lex("-"), vec![Minus]);
    assert_eq!(lex("*"), vec![Multiply]);
    assert_eq!(lex("/"), vec![Divide]);
    assert_eq!(lex("%"), vec![Modulo]);
    assert_eq!(lex("**"), vec![Power]);
}

#[test]
fn assignment_operators() {
    assert_eq!(lex("="), vec![Assign]);
    assert_eq!(lex("+="), vec![PlusAssign]);
    assert_eq!(lex("-="), vec![MinusAssign]);
    assert_eq!(lex("*="), vec![MultiplyAssign]);
    assert_eq!(lex("/="), vec![DivideAssign]);
    assert_eq!(lex("%="), vec![ModuloAssign]);
    assert_eq!(lex("&="), vec![BitAndAssign]);
    assert_eq!(lex("|="), vec![BitOrAssign]);
    assert_eq!(lex("^="), vec![BitXorAssign]);
    assert_eq!(lex("<<="), vec![LeftShiftAssign]);
    assert_eq!(lex(">>="), vec![RightShiftAssign]);
}

#[test]
fn bitwise_operators() {
    assert_eq!(lex("&"), vec![BitAnd]);
    assert_eq!(lex("|"), vec![BitOr]);
    assert_eq!(lex("^"), vec![BitXor]);
    assert_eq!(lex("~"), vec![BitNot]);
    assert_eq!(lex("<<"), vec![LeftShift]);
    assert_eq!(lex(">>"), vec![RightShift]);
}

#[test]
fn punctuation() {
    assert_eq!(lex(":"), vec![Colon]);
    assert_eq!(lex("::"), vec![DoubleColon]);
    assert_eq!(lex("."), vec![Dot]);
    // assert_eq!(lex("=>"), vec![ArrowRight]);
    assert_eq!(lex("@"), vec![At]);
    assert_eq!(lex("?"), vec![Question]);
    assert_eq!(lex(";"), vec![Semicolon]);
    assert_eq!(lex("("), vec![OpenParen]);
    assert_eq!(lex(")"), vec![CloseParen]);
    assert_eq!(lex("{"), vec![OpenBrace]);
    assert_eq!(lex("}"), vec![CloseBrace]);
    assert_eq!(lex(","), vec![Comma]);
}

// #[test]
// fn pattern_matching() {
//     assert_eq!(lex("|"), vec![Pipe]);
//     assert_eq!(lex("_"), vec![Underscore]);
// }

// #[test]
// fn comments() {
//     assert_eq!(
//         lex("// This is a comment"),
//         vec![Comment(" This is a comment")]
//     );
//     assert_eq!(
//         lex("/* This is a\nmultiline comment */"),
//         vec![Comment(" This is a\nmultiline comment ")]
//     );
// }

#[test]
fn complex_expressions() {
    assert_eq!(
        lex("let x /= 5.0 + 3 * 2.203"),
        vec![
            Let,
            Identifier("x"),
            DivideAssign,
            Float("5.0"),
            Plus,
            Int("3"),
            Multiply,
            Float("2.203"),
        ]
    );

    assert_eq!(
        lex("fn add(a, b) { a + b }"),
        vec![
            Function,
            Identifier("add"),
            OpenParen,
            Identifier("a"),
            Comma,
            Identifier("b"),
            CloseParen,
            OpenBrace,
            Identifier("a"),
            Plus,
            Identifier("b"),
            CloseBrace,
        ]
    );
}
