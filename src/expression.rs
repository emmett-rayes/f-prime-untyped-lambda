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
        // Self::parse
        move |input: PositionedBuffer<'a>| Self::parse(input)
    }
}

fn parse_symbol(input: PositionedBuffer) -> ParserResult<PositionedBuffer, String> {
    let input = input.seek_whitespace();
    let mut matched = String::new();
    let mut chars = input.buffer.chars();

    match chars.next() {
        Some(c) if c.is_alphabetic() => matched.push(c),
        _ => return Err(input.error("Failed to match symbol.".to_string())),
    }

    for c in chars {
        if c.is_alphanumeric() || c == '-' || c == '_' {
            matched.push(c);
        } else {
            break;
        }
    }

    let matched_len = matched.len();
    Ok((matched, input.seek(matched_len)))
}

pub fn symbol<'a>() -> impl Parser<PositionedBuffer<'a>, Output = String> + 'a {
    parse_symbol
}

pub fn parse_literal<'a>(
    expected: &str,
    input: PositionedBuffer<'a>,
) -> ParserResult<PositionedBuffer<'a>, String> {
    let input = input.seek_whitespace();
    if input.buffer.starts_with(expected) {
        Ok((expected.to_string(), input.seek(expected.len())))
    } else {
        Err(input.error(format!("Failed to match literal {expected}.")))
    }
}

pub fn literal<'a>(
    expected: &'static str,
) -> impl Parser<PositionedBuffer<'a>, Output = String> + 'a {
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
