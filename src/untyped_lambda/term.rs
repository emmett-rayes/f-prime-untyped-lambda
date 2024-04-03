use crate::expression::variable::Variable;
use crate::expression::{literal, Expression};
use crate::visitor::{Visitable, Visitor};
use f_prime_parser::combinators::between;
use f_prime_parser::{Parser, ParserResult, PositionedBuffer, ThenParserExtensions};

pub mod debruijn;
pub mod pretty_print;

#[derive(Debug, Eq, PartialEq)]
pub enum UntypedTerm {
    Variable(Variable),
    Abstraction(Box<UntypedAbstraction>),
    Application(Box<UntypedApplication>),
}

impl UntypedTerm {
    fn variable_parser<'a>() -> impl Parser<PositionedBuffer<'a>, Output = Self> + 'a {
        Variable::parser().map(UntypedTerm::Variable)
    }

    fn abstraction_parser<'a>() -> impl Parser<PositionedBuffer<'a>, Output = Self> + 'a {
        UntypedAbstraction::parser().map(|out| UntypedTerm::Abstraction(Box::new(out)))
    }

    fn application_parser<'a>() -> impl Parser<PositionedBuffer<'a>, Output = Self> + 'a {
        UntypedApplication::parser().map(|out| UntypedTerm::Application(Box::new(out)))
    }

    fn atom_parser<'a>() -> impl Parser<PositionedBuffer<'a>, Output = Self> + 'a {
        between(literal("("), UntypedTerm::parser(), literal(")"))
            .or_else(UntypedTerm::abstraction_parser())
            .or_else(UntypedTerm::variable_parser())
    }
}

impl Visitable for UntypedTerm {}

impl Expression for UntypedTerm {
    fn parse(input: PositionedBuffer) -> ParserResult<PositionedBuffer, Self> {
        let parser = UntypedTerm::abstraction_parser()
            .or_else(UntypedTerm::application_parser())
            .or_else(UntypedTerm::atom_parser());

        parser.parse(input)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct UntypedAbstraction {
    parameter: Variable,
    body: UntypedTerm,
}

impl Visitable for UntypedAbstraction {}

impl Expression for UntypedAbstraction {
    fn parse(input: PositionedBuffer) -> ParserResult<PositionedBuffer, Self> {
        let parser = literal("@")
            .or_else(literal("λ"))
            .skip_then(Variable::parser())
            .then_skip(literal("."))
            .then(UntypedTerm::parser())
            .map(|(parameter, body)| UntypedAbstraction { parameter, body });

        parser.parse(input)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct UntypedApplication {
    applicator: UntypedTerm,
    argument: UntypedTerm,
}

impl Visitable for UntypedApplication {}

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
                .unwrap()
        });

        parser.parse(input)
    }
}

pub trait UntypedTermVisitor
where
    Self: Visitor<Variable>
        + Visitor<Box<UntypedAbstraction>>
        + Visitor<Box<UntypedApplication>>
        + Visitor<UntypedTerm>,
{
}

impl<T> UntypedTermVisitor for T where
    T: Visitor<Variable>
        + Visitor<Box<UntypedAbstraction>>
        + Visitor<Box<UntypedApplication>>
        + Visitor<UntypedTerm>
{
}

impl<T> Visitor<UntypedTerm> for T
where
    T: Visitor<Variable, Result = Variable>
        + Visitor<Box<UntypedAbstraction>, Result = Box<UntypedAbstraction>>
        + Visitor<Box<UntypedApplication>, Result = Box<UntypedApplication>>,
{
    type Result = UntypedTerm;

    fn visit(&mut self, term: UntypedTerm) -> Self::Result {
        match term {
            UntypedTerm::Variable(variable) => UntypedTerm::Variable(self.visit(variable)),
            UntypedTerm::Abstraction(abstraction) => {
                UntypedTerm::Abstraction(self.visit(abstraction))
            }
            UntypedTerm::Application(application) => {
                UntypedTerm::Application(self.visit(application))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use f_prime_parser::PositionedBuffer;

    #[test]
    fn test_parser() {
        let input = PositionedBuffer::new("λs. λz. s z");
        let result = UntypedTerm::parse(input);
        dbg!(&result);
        assert!(result.unwrap().1.buffer.is_empty())
    }
}
