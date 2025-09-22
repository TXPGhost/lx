#[cfg(test)]
mod tests;

use logos::Logos;

#[allow(missing_docs)]
#[derive(Logos, Clone, Copy, Debug, PartialEq)]
#[logos(skip r"[ \t]+")]
pub enum Token {
    #[regex(r"[\n\f\r]+")]
    Newline,

    #[token(r",")]
    Comma,

    #[token(r"=")]
    Equals,

    #[token(r"==")]
    EqualsEquals,

    #[token(r"!=")]
    NotEquals,

    #[token(r"..")]
    Dots,

    #[token(r".")]
    Dot,

    #[token(r":")]
    Colon,

    #[token(r"::")]
    ColonColon,

    #[token(r";")]
    Semicolon,

    #[token(r"?")]
    QuestionMark,

    #[token(r"->")]
    Arrow,

    #[token(r"|")]
    Bar,

    #[token(r"+")]
    Plus,

    #[token(r"++")]
    PlusPlus,

    #[token(r"-")]
    Minus,

    #[token(r"*")]
    Times,

    #[token(r"**")]
    TimesTimes,

    #[token(r"/")]
    Divide,

    #[token(r"//")]
    DivideDivide,

    #[token(r"<<")]
    LShift,

    #[token(r">>")]
    RShift,

    #[token(r"%")]
    Percent,

    #[token(r"(")]
    LParen,

    #[token(r")")]
    RParen,

    #[token(r"{")]
    LCurl,

    #[token(r"}")]
    RCurl,

    #[token(r"[")]
    LSquare,

    #[token(r"]")]
    RSquare,

    #[token(r"<")]
    LAngle,

    #[token(r">")]
    RAngle,

    #[regex(r"[\d]+(\.[\d]+)?")]
    Number,

    #[regex(r"[_a-zA-Z]+[_a-zA-Z0-9]*")]
    Ident,

    #[regex(r#"["]([^"\\\n]|\\.|\\\n)*["]"#)]
    String,

    #[regex(r#"[']([^'\\\n]|\\.|\\\n)*[']"#)]
    Character,
}

/// A lexer wrapping the [logos::Lexer] type
pub struct Lexer<'source> {
    lexer: logos::Lexer<'source, Token>,
    next: Option<Token>,
    slice: &'source str,
}

/// An unrecognized token was encountered
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnrecognizedTokenError;

impl<'source> Lexer<'source> {
    /// Constructs a new [Lexer] from the given source code
    pub fn new(source: &'source str) -> Result<Self, UnrecognizedTokenError> {
        let mut lexer = Token::lexer(source);

        Ok(Self {
            next: match lexer.next() {
                Some(Ok(next)) => Some(next),
                Some(Err(_)) => return Err(UnrecognizedTokenError),
                None => None,
            },
            slice: lexer.slice(),
            lexer,
        })
    }

    /// Returns the source code's string slice
    pub fn slice(&self) -> &'source str {
        self.slice
    }

    /// Peeks the next [Token] without consuming it
    pub fn peek(&self) -> Option<Token> {
        self.next
    }
}

impl Iterator for Lexer<'_> {
    type Item = Result<Token, UnrecognizedTokenError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.slice = self.lexer.slice();

        let next = self.next;
        self.next = match self.lexer.next() {
            Some(Ok(next)) => Some(next),
            Some(Err(_)) => return Some(Err(UnrecognizedTokenError)),
            None => None,
        };
        next.map(Ok)
    }
}
