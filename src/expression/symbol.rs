use f_prime_parser::{DefaultParsable, Parser, ParserInput, ParserResult};

use crate::expression::buffer::PositionedBuffer;

pub type Symbol = String;

pub struct SymbolParser;

impl<'a> Parser<PositionedBuffer<'a>> for SymbolParser {
    type Output = Symbol;

    fn parse<'b>(
        &self,
        input: PositionedBuffer<'a>,
    ) -> ParserResult<PositionedBuffer<'a>, Self::Output>
    where
        PositionedBuffer<'a>: 'b,
        Self::Output: 'b,
    {
        let input = input.seek_whitespace();
        let mut chars = input.buffer.chars();

        let mut matched = 0;
        match chars.next() {
            Some(c) if c.is_ascii_alphabetic() => matched += 1,
            _ => {
                return Err(input.error("Invalid symbol."));
            }
        }

        for c in chars {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                matched += 1;
            } else {
                break;
            }
        }

        Ok((input.buffer[0..matched].to_string(), input.seek(matched)))
    }
}

impl<'a> DefaultParsable<PositionedBuffer<'a>> for Symbol {
    fn parser() -> impl Parser<PositionedBuffer<'a>, Output = Self>
    where
        Self: Sized,
    {
        SymbolParser
    }
}
