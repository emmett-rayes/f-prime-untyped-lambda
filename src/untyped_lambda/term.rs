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

    fn atom_parser<'a>() -> impl Parser<PositionedBuffer<'a>, Output = Self> + 'a
    where
        Self: Expression,
    {
        between(literal("("), UntypedTerm::parser(), literal(")"))
            .or_else(UntypedTerm::abstraction_parser())
            .or_else(UntypedTerm::variable_parser())
    }
}

impl Expression for UntypedTerm {
    fn parse(input: PositionedBuffer) -> ParserResult<PositionedBuffer, Self> {
        let parser = UntypedTerm::abstraction_parser()
            .or_else(UntypedTerm::application_parser())
            .or_else(UntypedTerm::atom_parser());
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
            .or_else(literal("λ"))
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
    applicator: UntypedTerm,
    argument: UntypedTerm,
}

impl Expression for UntypedApplication {
    fn parse(input: PositionedBuffer) -> ParserResult<PositionedBuffer, Self> {
        let parser = UntypedTerm::atom_parser().at_least(2).map(|terms| {
            terms
                .into_iter()
                .reduce(|applicator, argument| {
                    UntypedTerm::Application(Box::new(UntypedApplication {
                        applicator,
                        argument,
                    }))
                })
                .map(|application| {
                    if let UntypedTerm::Application(bx) = application {
                        *bx
                    } else {
                        unreachable!()
                    }
                })
                .unwrap_or_else(|| unreachable!())
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
        let input = PositionedBuffer::new("λx. a (λt. b x t (f (λu. a u t z) λs. w)) w y");
        let result = UntypedTerm::parse(input);
        dbg!(&result);
        assert!(result.unwrap().1.buffer.is_empty())
    }
}
