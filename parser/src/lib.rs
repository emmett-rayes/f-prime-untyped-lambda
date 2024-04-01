#![feature(assert_matches)]

pub type ParserPosition = usize;

#[derive(Debug)]
pub struct ParserInput<'a> {
    pub string: &'a str,
    pub position: ParserPosition,
}

#[derive(Debug)]
pub struct ParserError {
    pub message: String,
    pub start_position: Option<ParserPosition>,
    pub end_position: Option<ParserPosition>,
}

impl<'a> ParserInput<'a> {
    pub fn new(str: &'a str) -> Self {
        ParserInput {
            string: str,
            position: 0,
        }
    }

    fn seek(self, length: usize) -> Self {
        ParserInput {
            string: &self.string[length..],
            position: self.position + length,
        }
    }
}

impl ParserError {
    fn new(message: String) -> Self {
        ParserError {
            message: message,
            start_position: None,
            end_position: None,
        }
    }

    fn new_at(message: String, position: ParserPosition) -> Self {
        ParserError {
            message: message,
            start_position: Some(position),
            end_position: Some(position),
        }
    }

    fn new_range(
        message: String,
        start_position: ParserPosition,
        end_position: ParserPosition,
    ) -> Self {
        ParserError {
            message: message,
            start_position: Some(start_position),
            end_position: Some(end_position),
        }
    }
}

pub type ParserResult<'a, Output> = Result<(ParserInput<'a>, Output), ParserError>;

pub trait Parser<'a, Output> {
    fn parse(&self, input: ParserInput<'a>) -> ParserResult<'a, Output>;

    fn map<F, MapOutput>(&self, op: F) -> impl Parser<'a, MapOutput>
    where
        F: Fn(Output) -> MapOutput,
    {
        move |input| {
            self.parse(input)
                .map(|(remaining, result)| (remaining, op(result)))
        }
    }
}

impl<'a, F, Output> Parser<'a, Output> for F
where
    F: Fn(ParserInput<'a>) -> ParserResult<'a, Output>,
{
    fn parse(&self, input: ParserInput<'a>) -> ParserResult<'a, Output> {
        self(input)
    }
}

pub fn literal<'a>(expected: &'static str) -> impl Parser<'a, String> {
    move |input: ParserInput<'a>| {
        if input.string.starts_with(expected) {
            Ok((input.seek(expected.len()), expected.to_string()))
        } else {
            Err(ParserError::new_at(
                format!("Failed to match literal {expected}."),
                input.position,
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches::assert_matches;

    #[test]
    fn test_literal() {
        let literal_parser = literal("hello");

        let input = ParserInput::new("hello, world!");
        assert_matches!(
            literal_parser.parse(input),
            Ok((remaining, string)) if string == "hello" && remaining.string == ", world!",
        );

        let input = ParserInput::new("goodbye, world!");
        assert_matches!(literal_parser.parse(input), Err(_),);
    }
}
