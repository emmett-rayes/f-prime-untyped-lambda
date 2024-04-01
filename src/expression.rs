use f_prime_parser::{BoxedParser, Parser, ParserInput, ParserResult};

type ExpressionParser<'a, Exp> = BoxedParser<'a, ParserInput<'a>, Exp>;

pub trait Expression {
    fn parse(input: ParserInput) -> ParserResult<ParserInput, Self>
    where
        Self: Sized,
    {
        Self::parser().parse(input)
    }

    fn parser<'a>() -> ExpressionParser<'a, Self>
    where
        Self: Sized;
}

fn symbol<'a>() -> ExpressionParser<'a, String> {
    BoxedParser::new(move |input: ParserInput<'a>| {
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
    })
}

pub fn literal<'a>(expected: &'static str) -> ExpressionParser<'a, String> {
    BoxedParser::new(move |input: ParserInput<'a>| {
        if input.buffer.starts_with(expected) {
            Ok((expected.to_string(), input.seek(expected.len())))
        } else {
            Err(input.error(format!("Failed to match literal {expected}.")))
        }
    })
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use super::*;

    #[test]
    fn test_literal() {
        let literal_parser = literal("hello");

        let input = ParserInput::new("hello, world!");
        assert_matches!(
            literal_parser.parse(input),
            Ok((output, remaining)) if output == "hello" && remaining.buffer == ", world!",
        );

        let input = ParserInput::new("goodbye, world!");
        assert_matches!(literal_parser.parse(input), Err(_),);
    }
}
