#[cfg(test)]
mod tests;

use crate::{ast::*, lexer::*, node::*};

#[derive(Debug)]
pub enum ParseError {
    ExpectFailed { expected: Token, found: Token },
    OutOfTokens,
    UnrecognizedToken(UnrecognizedTokenError),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Loc {}

impl<'source> Lexer<'source> {
    fn expect(&mut self, expected: Token) -> Result<&'source str, ParseError> {
        let found = self.next();
        let found = match found {
            None => return Err(ParseError::OutOfTokens),
            Some(Err(e)) => return Err(ParseError::UnrecognizedToken(e)),
            Some(Ok(found)) => found,
        };
        if found != expected {
            return Err(ParseError::ExpectFailed { expected, found });
        }
        Ok(self.slice())
    }
}

pub fn parse_number<'a>(lexer: &mut Lexer<'_>) -> Result<Node<Expr<'a, Loc>, Loc>, ParseError> {
    let slice = lexer.expect(Token::Number)?;
    panic!("{slice}");
}
