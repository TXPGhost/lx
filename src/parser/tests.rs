use super::*;

fn make_lexer(source: &str) -> Lexer {
    Lexer::new(source).unwrap_or_else(|_| panic!("unable to construct lexer for source {source}"))
}

#[test]
fn test_parse_number() {
    let mut lexer = make_lexer("42");
    parse_number(&mut lexer).unwrap();
}
