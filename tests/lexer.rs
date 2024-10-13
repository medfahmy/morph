use morph::{Lexer, TokenKind, TokenKind::*};

fn lex<'a>(input: &'a str) -> Vec<TokenKind> {
    Lexer::new(input).map(|token| token.kind).collect()
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

    assert_eq!(
        lex("let result = (10 - 5) * (3 + 2) / 2.5"),
        vec![
            Let,
            Identifier("result"),
            Assign,
            OpenParen,
            Int("10"),
            Minus,
            Int("5"),
            CloseParen,
            Multiply,
            OpenParen,
            Int("3"),
            Plus,
            Int("2"),
            CloseParen,
            Divide,
            Float("2.5"),
        ]
    );

    assert_eq!(
        lex("if x > 0 { print(x) } else { print(-x) }"),
        vec![
            If,
            Identifier("x"),
            GreaterThan,
            Int("0"),
            OpenBrace,
            Identifier("print"),
            OpenParen,
            Identifier("x"),
            CloseParen,
            CloseBrace,
            Else,
            OpenBrace,
            Identifier("print"),
            OpenParen,
            Minus,
            Identifier("x"),
            CloseParen,
            CloseBrace,
        ]
    );

    assert_eq!(
        lex("while i <= 10 && j >= 0 { i += 1; j -= 1 }"),
        vec![
            While,
            Identifier("i"),
            LessEqual,
            Int("10"),
            And,
            Identifier("j"),
            GreaterEqual,
            Int("0"),
            OpenBrace,
            Identifier("i"),
            PlusAssign,
            Int("1"),
            Semicolon,
            Identifier("j"),
            MinusAssign,
            Int("1"),
            CloseBrace,
        ]
    );

    assert_eq!(
        lex(r#"let message = "Hello, world!"; let is_valid = true;"#),
        vec![
            Let,
            Identifier("message"),
            Assign,
            Str("Hello, world!"),
            Semicolon,
            Let,
            Identifier("is_valid"),
            Assign,
            Bool(true),
            Semicolon,
        ]
    );

    assert_eq!(
        lex(r#"let _hello = "Hello, world!"; match _hello { _ => _hello };"#),
        vec![
            Let,
            Identifier("_hello"),
            Assign,
            Str("Hello, world!"),
            Semicolon,
            Match,
            Identifier("_hello"),
            OpenBrace,
            Underscore,
            ArrowRight,
            Identifier("_hello"),
            CloseBrace,
            Semicolon,
        ]
    );

    assert_eq!(
        lex("use std.collections.HashMap;"),
        vec![
            Use,
            Identifier("std"),
            Dot,
            Identifier("collections"),
            Dot,
            Identifier("HashMap"),
            Semicolon,
        ]
    );

    assert_eq!(
        lex(r#"
        add | Int, Int -> Int;
        add = fn(a, b) -> {
            a + b
        };
            "#),
        vec![
            Identifier("add"),
            Pipe,
            Identifier("Int"),
            Comma,
            Identifier("Int"),
            Arrow,
            Identifier("Int"),
            Semicolon,
            Identifier("add"),
            Assign,
            Function,
            OpenParen,
            Identifier("a"),
            Comma,
            Identifier("b"),
            CloseParen,
            Arrow,
            OpenBrace,
            Identifier("a"),
            Plus,
            Identifier("b"),
            CloseBrace,
            Semicolon,
        ]
    );
}
