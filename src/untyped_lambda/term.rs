use f_prime_parser::combinators::between;
use f_prime_parser::{Parser, ParserResult, PositionedBuffer};

use crate::expression::variable::Variable;
use crate::expression::{literal, Expression};

#[derive(Debug)]
pub enum UntypedTerm {
    Variable(Variable),
    Abstraction(Box<UntypedAbstraction>),
    Application(Box<UntypedApplication>),
}

impl UntypedTerm {
    fn variable_parser<'a>() -> impl Parser<PositionedBuffer<'a>, Output = Self> + 'a
    where
        Self: Expression,
    {
        Variable::parser().map(UntypedTerm::Variable)
    }

    fn abstraction_parser<'a>() -> impl Parser<PositionedBuffer<'a>, Output = Self> + 'a
    where
        Self: Expression,
    {
        UntypedAbstraction::parser().map(|out| UntypedTerm::Abstraction(Box::new(out)))
    }

    fn application_parser<'a>() -> impl Parser<PositionedBuffer<'a>, Output = Self> + 'a
    where
        Self: Expression,
    {
        UntypedApplication::parser().map(|out| UntypedTerm::Application(Box::new(out)))
    }
}

impl Expression for UntypedTerm {
    fn parse(input: PositionedBuffer) -> ParserResult<PositionedBuffer, Self> {
        let parser = UntypedTerm::variable_parser()
            .or_else(UntypedTerm::abstraction_parser())
            .or_else(UntypedTerm::application_parser());
        parser.parse(input)
    }
}

#[derive(Debug)]
pub struct UntypedAbstraction {
    parameter: Variable,
    body: UntypedTerm,
}

impl Expression for UntypedAbstraction {
    fn parse(input: PositionedBuffer) -> ParserResult<PositionedBuffer, Self> {
        let parser = literal("@")
            .then(Variable::parser())
            .right()
            .then(literal("."))
            .left()
            .then(UntypedTerm::parser())
            .map(|(parameter, body)| UntypedAbstraction { parameter, body });
        parser.parse(input)
    }
}

#[derive(Debug)]
pub struct UntypedApplication {
    applicand: UntypedTerm,
    argument: UntypedTerm,
}

impl Expression for UntypedApplication {
    fn parse(input: PositionedBuffer) -> ParserResult<PositionedBuffer, Self> {
        let parser = between(
            literal("("),
            UntypedTerm::parser().then(UntypedTerm::parser()),
            literal(")"),
        )
        .map(|(applicand, argument)| UntypedApplication {
            applicand,
            argument,
        });
        parser.parse(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use f_prime_parser::PositionedBuffer;

    #[test]
    fn test_application() {
        let input = PositionedBuffer::new("(@x.x @y.y)");
        let (output, remaining) = UntypedTerm::parse(input).unwrap();
        dbg!(output);
        assert!(remaining.buffer.is_empty())
    }
}
