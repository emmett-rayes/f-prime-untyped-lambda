use f_prime_parser::{BoxedParser, Parser, ParserInput, ParserResult, PositionedBuffer};

pub mod constant;
pub mod variable;

pub type ExpressionParser<'a, Exp> = BoxedParser<'a, PositionedBuffer<'a>, Exp>;

pub trait Expression
where
    Self: Sized,
{
    fn parse(input: PositionedBuffer) -> ParserResult<PositionedBuffer, Self>
    where
        Self: Sized,
    {
        Self::parser().parse(input)
    }

    fn parser<'a>() -> ExpressionParser<'a, Self>;
}

fn symbol<'a>() -> ExpressionParser<'a, String> {
    let parser = move |input: PositionedBuffer<'a>| {
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
    };
    parser.boxed()
}

pub fn literal<'a>(expected: &'static str) -> ExpressionParser<'a, String> {
    let parser = move |input: PositionedBuffer<'a>| {
        if input.buffer.starts_with(expected) {
            Ok((expected.to_string(), input.seek(expected.len())))
        } else {
            Err(input.error(format!("Failed to match literal {expected}.")))
        }
    };
    parser.boxed()
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
