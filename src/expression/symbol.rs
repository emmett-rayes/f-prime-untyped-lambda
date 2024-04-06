use f_prime_parser::{Parser, ParserInput, ParserResult};

use crate::expression::buffer::PositionedBuffer;

pub type Symbol = String;

fn parse_symbol(input: PositionedBuffer) -> ParserResult<PositionedBuffer, Symbol> {
    let input = input.seek_whitespace();
    let mut chars = input.buffer.chars();

    let mut matched = 0;
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() => matched += 1,
        _ => return Err(input.error("Invalid symbol.".to_string())),
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

pub fn symbol_parser<'a>() -> impl Parser<PositionedBuffer<'a>, Output = Symbol> + 'a {
    parse_symbol
}

pub fn parse_literal<'a>(
    expected: &str,
    input: PositionedBuffer<'a>,
) -> ParserResult<PositionedBuffer<'a>, Symbol> {
    let input = input.seek_whitespace();
    if input.buffer.starts_with(expected) {
        Ok((
            input.buffer[0..expected.len()].to_string(),
            input.seek(expected.len()),
        ))
    } else {
        Err(input.error(format!("Expected '{expected}' at this position.")))
    }
}

pub fn literal_parser<'a>(
    expected: &'static str,
) -> impl Parser<PositionedBuffer<'a>, Output = Symbol> + 'a {
    move |input: PositionedBuffer<'a>| parse_literal(expected, input)
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use super::*;

    #[test]
    fn test_literal() {
        let literal_parser = literal_parser("hello");

        let input = PositionedBuffer::new("hello, world!");
        assert_matches!(
            literal_parser.parse(input),
            Ok((output, remaining)) if output == "hello" && remaining.buffer == ", world!",
        );

        let input = PositionedBuffer::new("goodbye, world!");
        assert_matches!(literal_parser.parse(input), Err(_),);
    }
}
