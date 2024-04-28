use f_prime_parser::{Parser, ParserInput, ParserResult};

use crate::expression::buffer::PositionedBuffer;

pub type Literal = String;

pub struct LiteralParser<'a> {
    expected: &'a str,
}

impl<'a> LiteralParser<'a> {
    pub fn new(expected: &'a str) -> Self {
        LiteralParser { expected }
    }
}

impl<'a> Parser<PositionedBuffer<'a>> for LiteralParser<'a> {
    type Output = Literal;

    fn parse<'b>(
        &self,
        input: PositionedBuffer<'a>,
    ) -> ParserResult<PositionedBuffer<'a>, Self::Output>
    where
        PositionedBuffer<'a>: 'b,
        Self::Output: 'b,
    {
        let input = input.seek_whitespace();
        if input.buffer.starts_with(self.expected) {
            Ok((
                input.buffer[0..self.expected.len()].to_string(),
                input.seek(self.expected.len()),
            ))
        } else {
            Err(input.error(&format!("Expected '{}' at this position.", self.expected)))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use super::*;

    #[test]
    fn test_literal() {
        let literal_parser = LiteralParser::new("hello");

        let input = PositionedBuffer::new("hello, world!");
        assert_matches!(
            literal_parser.parse(input),
            Ok((output, remaining)) if output == "hello" && remaining.buffer == ", world!",
        );

        let input = PositionedBuffer::new("goodbye, world!");
        assert_matches!(literal_parser.parse(input), Err(_),);
    }
}
