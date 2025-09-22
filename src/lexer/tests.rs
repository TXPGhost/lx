use super::*;

fn lex_str(source: &str) -> Vec<Result<Token, UnrecognizedTokenError>> {
    let lexer = Lexer::new(source)
        .unwrap_or_else(|_| panic!("unable to construct lexer for source {source}"));
    let tokens: Vec<Result<Token, UnrecognizedTokenError>> = lexer.collect();
    tokens
}

fn lex_str_ok(source: &str) -> Vec<Token> {
    let lexer = Lexer::new(source)
        .unwrap_or_else(|_| panic!("unable to construct lexer for source {source}"));
    let tokens: Vec<Token> = lexer
        .into_iter()
        .map(|tok| tok.expect("illegal token"))
        .collect();
    tokens
}

#[test]
fn test_empty() {
    assert_eq!(lex_str("").len(), 0);
}

#[test]
fn test_repeated_tokens() {
    let tokens = [
        (",", Token::Comma, 4, false),
        ("\n", Token::Newline, 4, true),
        ("=", Token::Equals, 1, false),
        ("==", Token::EqualsEquals, 4, false),
        ("!=", Token::NotEquals, 4, false),
        ("..", Token::Dots, 4, false),
        (".", Token::Dot, 1, false),
        (":", Token::Colon, 1, false),
        ("::", Token::ColonColon, 4, false),
        (";", Token::Semicolon, 4, false),
        ("?", Token::QuestionMark, 4, false),
        ("->", Token::Arrow, 4, false),
        ("|", Token::Bar, 4, false),
        ("+", Token::Plus, 1, false),
        ("++", Token::PlusPlus, 4, false),
        ("-", Token::Minus, 1, false),
        ("*", Token::Times, 1, false),
        ("**", Token::TimesTimes, 4, false),
        ("/", Token::Divide, 1, false),
        ("//", Token::DivideDivide, 4, false),
        ("%", Token::Percent, 4, false),
        ("<<", Token::LShift, 4, false),
        (">>", Token::RShift, 4, false),
        ("(", Token::LParen, 4, false),
        (")", Token::RParen, 4, false),
        ("{", Token::LCurl, 4, false),
        ("}", Token::RCurl, 4, false),
        ("[", Token::LSquare, 4, false),
        ("]", Token::RSquare, 4, false),
        ("<", Token::LAngle, 1, false),
        (">", Token::RAngle, 1, false),
    ];
    for (tok_str, tok, count, condense) in tokens {
        for n in 1..count {
            let source = tok_str.repeat(n);
            let expected_count = if condense { n.min(1) } else { n };
            let actual = lex_str_ok(&source);
            if actual != vec![tok; expected_count] {
                panic!("tokens not equal: (\"{source}\", [{tok:?}; {n}]), found {actual:?}");
            }
        }
    }
}
