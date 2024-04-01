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

pub struct BoxedParser<'a, Output> {
    parser: Box<dyn Parser<'a, Output> + 'a>,
}

impl<'a, Output> BoxedParser<'a, Output> {
    fn new<P>(parser: P) -> Self
    where
        P: Parser<'a, Output> + 'a,
    {
        BoxedParser {
            parser: Box::new(parser),
        }
    }
}

pub trait Parser<'a, Output> {
    fn parse(&self, input: ParserInput<'a>) -> ParserResult<'a, Output>;

    fn map<F, MapOutput>(self, op: F) -> BoxedParser<'a, MapOutput>
    where
        Self: Sized + 'a,
        F: Fn(Output) -> MapOutput + 'a,
    {
        BoxedParser::new(move |input| {
            self.parse(input)
                .map(|(remaining, output)| (remaining, op(output)))
        })
    }

    fn and_then<F, ThenOutput>(self, op: F) -> BoxedParser<'a, ThenOutput>
    where
        Self: Sized + 'a,
        F: Fn((ParserInput<'a>, Output)) -> ParserResult<'a, ThenOutput> + 'a,
    {
        BoxedParser::new(move |input| {
            self.parse(input)
                .and_then(|parser_result| op(parser_result))
        })
    }
}

impl<'a, Output> Parser<'a, Output> for BoxedParser<'a, Output> {
    fn parse(&self, input: ParserInput<'a>) -> ParserResult<'a, Output> {
        self.parser.parse(input)
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

pub fn literal<'a>(expected: &'static str) -> BoxedParser<'a, String> {
    BoxedParser::new(move |input: ParserInput<'a>| {
        if input.string.starts_with(expected) {
            Ok((input.seek(expected.len()), expected.to_string()))
        } else {
            Err(ParserError::new_at(
                format!("Failed to match literal {expected}."),
                input.position,
            ))
        }
    })
}

fn symbol<'a>() -> BoxedParser<'a, String> {
    BoxedParser::new(move |input: ParserInput<'a>| {
        let mut matched = String::new();
        for c in input.string.chars() {
            if c == ' ' || c == '\\' || c == '.' {
                break;
            } else {
                matched.push(c);
            }
        }
        Ok((input.seek(matched.len()), matched))
    })
}

fn constant<'a>() -> BoxedParser<'a, String> {
    symbol().and_then(|(remaining, output)| match output.chars().next() {
        None => Err(ParserError::new_at(
            "Failed to match constant.".to_string(),
            remaining.position - output.len(),
        )),
        Some(c) if c.is_lowercase() => Err(ParserError::new_range(
            "Constants must start with either a symbol or an upper case letter.".to_string(),
            remaining.position - output.len(),
            output.len() - 1,
        )),
        Some(_) => Ok((remaining, output)),
    })
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

    #[test]
    fn test_constant() {
        use std::assert_matches::assert_matches;

        let constant_parser = constant();

        let input = ParserInput::new("Type ...");
        assert_matches!(
            constant_parser.parse(input),
            Ok((_, string)) if string == "Type",
        );

        let input = ParserInput::new("-> ...");
        assert_matches!(
            constant_parser.parse(input),
            Ok((_, string)) if string == "->",
        );

        let input = ParserInput::new("\\X");
        assert_matches!(constant_parser.parse(input), Err(_),);

        let input = ParserInput::new("symbol ...");
        assert_matches!(
            constant_parser.parse(input),
            Err(error) if error.start_position == Some(0) && error.end_position == Some("symbol".len() - 1),
        );
    }
}
