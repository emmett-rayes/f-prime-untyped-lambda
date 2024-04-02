use f_prime_parser::combinators::one_of;
use f_prime_parser::{Parser, ParserResult, PositionedBuffer};

use crate::expression::variable::Variable;
use crate::expression::{literal, Expression};

#[derive(Debug)]
pub enum UntypedTerm {
    Variable(Variable),
    Abstraction(Box<UntypedAbstraction>),
    Application(Box<UntypedApplication>),
}

impl Expression for UntypedTerm {
    fn parse(input: PositionedBuffer) -> ParserResult<PositionedBuffer, Self> {
        let sub_parsers = vec![
            Variable::parser().map(UntypedTerm::Variable).boxed(),
            UntypedAbstraction::parser()
                .map(|out| UntypedTerm::Abstraction(Box::new(out)))
                .boxed(),
            UntypedApplication::parser()
                .map(|out| UntypedTerm::Application(Box::new(out)))
                .boxed(),
        ];
        let parser = one_of(sub_parsers);
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
        let parser = literal("\\")
            .then(Variable::parser())
            .right()
            .then(UntypedTerm::parser())
            .map(|(parameter, body)| UntypedAbstraction { parameter, body })
            .boxed();
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
        let parser = UntypedTerm::parser()
            .then(UntypedTerm::parser())
            .map(|(applicand, argument)| UntypedApplication {
                applicand,
                argument,
            })
            .boxed();
        parser.parse(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use f_prime_parser::PositionedBuffer;

    #[test]
    fn test_application() {
        let input = PositionedBuffer::new("x y");
        dbg!(UntypedTerm::parse(input));
    }
}
