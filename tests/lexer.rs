use morph::{Lexer, TokenKind, TokenKind::*};

fn lex<'a>(input: &'a str) -> Vec<TokenKind> {
    Lexer::new(input).map(|token| token.kind).collect()
}

#[test]
fn keywords() {
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
    assert_eq!(lex("impl"), vec![Impl]);
    assert_eq!(lex("trait"), vec![Trait]);
    assert_eq!(lex("where"), vec![Where]);
    assert_eq!(lex("as"), vec![As]);
    assert_eq!(lex("pub"), vec![Pub]);
    assert_eq!(lex("const"), vec![Const]);
    assert_eq!(lex("fn"), vec![Function]);
    assert_eq!(lex("return"), vec![Return]);
}

#[test]
fn identifiers() {
    assert_eq!(lex("foo"), vec![Identifier]);
    assert_eq!(lex("bar123"), vec![Identifier]);
    assert_eq!(lex("_underscore"), vec![Identifier]);
}

#[test]
fn primitives() {
    assert_eq!(lex("42"), vec![Int]);
    assert_eq!(lex("3.14"), vec![Float]);
    assert_eq!(lex("true"), vec![Bool]);
    assert_eq!(lex("false"), vec![Bool]);
    assert_eq!(lex("'a'"), vec![Char]);
    assert_eq!(lex("\"hello\""), vec![Str]);

    assert_eq!(lex("id 43 23.23;"), vec![Identifier, Int, Float, Semicolon],);
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
    assert_eq!(lex("&"), vec![Ampersand]);
    assert_eq!(lex("|"), vec![Pipe]);
    assert_eq!(lex("^"), vec![Caret]);
    assert_eq!(lex("~"), vec![Tilde]);
    assert_eq!(lex("<<"), vec![LeftShift]);
    assert_eq!(lex(">>"), vec![RightShift]);
}

#[test]
fn punctuation() {
    assert_eq!(lex(":"), vec![Colon]);
    assert_eq!(lex("::"), vec![DoubleColon]);
    assert_eq!(lex("."), vec![Dot]);
    assert_eq!(lex("->"), vec![Arrow]);
    assert_eq!(lex("=>"), vec![ArrowRight]);
    assert_eq!(lex("@"), vec![At]);
    assert_eq!(lex("?"), vec![Question]);
    assert_eq!(lex(";"), vec![Semicolon]);
    assert_eq!(lex("("), vec![OpenParen]);
    assert_eq!(lex(")"), vec![CloseParen]);
    assert_eq!(lex("{"), vec![OpenBrace]);
    assert_eq!(lex("}"), vec![CloseBrace]);
    assert_eq!(lex(","), vec![Comma]);
    assert_eq!(lex("_"), vec![Underscore]);
}

#[test]
fn complex_expressions() {
    assert_eq!(
        lex("x /= 5.0 + 3 * 2.203"),
        vec![Identifier, DivideAssign, Float, Plus, Int, Multiply, Float,]
    );

    assert_eq!(
        lex("fn add(a, b) { a + b }"),
        vec![
            Function, Identifier, OpenParen, Identifier, Comma, Identifier, CloseParen, OpenBrace,
            Identifier, Plus, Identifier, CloseBrace,
        ]
    );

    assert_eq!(
        lex("result = (10 - 5) * (3 + 2) / 2.5"),
        vec![
            Identifier, Assign, OpenParen, Int, Minus, Int, CloseParen, Multiply, OpenParen, Int,
            Plus, Int, CloseParen, Divide, Float,
        ]
    );

    assert_eq!(
        lex("if x > 0 { print(x) } else { print(-x) }"),
        vec![
            If,
            Identifier,
            GreaterThan,
            Int,
            OpenBrace,
            Identifier,
            OpenParen,
            Identifier,
            CloseParen,
            CloseBrace,
            Else,
            OpenBrace,
            Identifier,
            OpenParen,
            Minus,
            Identifier,
            CloseParen,
            CloseBrace,
        ]
    );

    assert_eq!(
        lex("while i <= 10 && j >= 0 { i += 1; j -= 1 }"),
        vec![
            While,
            Identifier,
            LessEqual,
            Int,
            And,
            Identifier,
            GreaterEqual,
            Int,
            OpenBrace,
            Identifier,
            PlusAssign,
            Int,
            Semicolon,
            Identifier,
            MinusAssign,
            Int,
            CloseBrace,
        ]
    );

    assert_eq!(
        lex(r#"message = "Hello, world!"; is_valid = true;"#),
        vec![Identifier, Assign, Str, Semicolon, Identifier, Assign, Bool, Semicolon,]
    );

    assert_eq!(
        lex(r#"_hello = "Hello, world!"; match _hello { _ => _hello };"#),
        vec![
            Identifier, Assign, Str, Semicolon, Match, Identifier, OpenBrace, Underscore,
            ArrowRight, Identifier, CloseBrace, Semicolon,
        ]
    );

    assert_eq!(
        lex("use std.collections.HashMap;"),
        vec![Use, Identifier, Dot, Identifier, Dot, Identifier, Semicolon,]
    );

    assert_eq!(
        lex(r#"
        add | Int, Int -> Int;
        add = fn(a, b) -> {
            a + b
        };
            "#),
        vec![
            Identifier, Pipe, Identifier, Comma, Identifier, Arrow, Identifier, Semicolon,
            Identifier, Assign, Function, OpenParen, Identifier, Comma, Identifier, CloseParen,
            Arrow, OpenBrace, Identifier, Plus, Identifier, CloseBrace, Semicolon,
        ]
    );
}
