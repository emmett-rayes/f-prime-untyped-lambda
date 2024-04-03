use f_prime_parser::{Parser, ParserInput, ParserResult, PositionedBuffer};

pub mod constant;
pub mod variable;

pub trait Expression
where
    Self: Sized,
{
    fn parse(input: PositionedBuffer) -> ParserResult<PositionedBuffer, Self>;

    fn parser<'a>() -> impl Parser<PositionedBuffer<'a>, Output = Self> + 'a
    where
        Self: 'a,
    {
        Self::parse
    }
}

fn parse_symbol(input: PositionedBuffer) -> ParserResult<PositionedBuffer, &str> {
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

    Ok((&input.buffer[0..matched], input.seek(matched)))
}

pub fn symbol<'a>() -> impl Parser<PositionedBuffer<'a>, Output = &'a str> + 'a {
    parse_symbol
}

pub fn parse_literal<'a>(
    expected: &str,
    input: PositionedBuffer<'a>,
) -> ParserResult<PositionedBuffer<'a>, &'a str> {
    let input = input.seek_whitespace();
    if input.buffer.starts_with(expected) {
        Ok((&input.buffer[0..expected.len()], input.seek(expected.len())))
    } else {
        Err(input.error(format!("Expected '{expected}' at this position.")))
    }
}

pub fn literal<'a>(
    expected: &'static str,
) -> impl Parser<PositionedBuffer<'a>, Output = &'a str> + 'a {
    move |input: PositionedBuffer<'a>| parse_literal(expected, input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches::assert_matches;

    #[test]
    fn test_literal() {
        let literal_parser = literal("hello");

        let input = PositionedBuffer::new("hello, world!");
        assert_matches!(
            literal_parser.parse(input),
            Ok((output, remaining)) if output == "hello" && remaining.buffer == ", world!",
        );

        let input = PositionedBuffer::new("goodbye, world!");
        assert_matches!(literal_parser.parse(input), Err(_),);
    }
}
