use f_prime_parser::{Parser, ParserResult};
use f_prime_parser::combinators::between;

use crate::expression::abstraction::Abstraction;
use crate::expression::application::Application;
use crate::expression::buffer::{Parsable, PositionedBuffer};
use crate::expression::symbol::literal_parser;
use crate::expression::variable::{IndexedVariable, NamedVariable, Variable};

pub mod buffer;
pub mod symbol;
pub mod constant;
pub mod variable;
pub mod abstraction;
pub mod application;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expression {
    NamedVariable(NamedVariable),
    IndexedVariable(IndexedVariable),
    Abstraction(Box<Abstraction>),
    Application(Box<Application>),
}

impl Expression {
    fn variable_parser<'a>() -> impl Parser<PositionedBuffer<'a>, Output = Self> + 'a {
        Variable::parser().map(Expression::from)
    }

    fn abstraction_parser<'a>() -> impl Parser<PositionedBuffer<'a>, Output = Self> + 'a {
        Abstraction::parser().map(Expression::from)
    }

    fn application_parser<'a>() -> impl Parser<PositionedBuffer<'a>, Output = Self> + 'a {
        Application::parser().map(Expression::from)
    }

    fn atom_parser<'a>() -> impl Parser<PositionedBuffer<'a>, Output = Self> + 'a {
        between(literal_parser("("), Expression::parser(), literal_parser(")"))
            .or_else(Expression::abstraction_parser())
            .or_else(Expression::variable_parser())
    }
}

impl Parsable for Expression {
    fn parse(input: PositionedBuffer) -> ParserResult<PositionedBuffer, Self> {
        let parser = Expression::abstraction_parser()
            .or_else(Expression::application_parser())
            .or_else(Expression::atom_parser());
        parser.parse(input)
    }
}

impl From<NamedVariable> for Expression {
    fn from(value: NamedVariable) -> Self {
        Expression::NamedVariable(value)
    }
}

impl From<IndexedVariable> for Expression {
    fn from(value: IndexedVariable) -> Self {
        Expression::IndexedVariable(value)
    }
}

impl From<Abstraction> for Expression {
    fn from(value: Abstraction) -> Self {
        Expression::Abstraction(Box::from(value))
    }
}

impl From<Application> for Expression {
    fn from(value: Application) -> Self {
        Expression::Application(Box::from(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let input = PositionedBuffer::new("λx. a (λt. b x t (f (λu. a u t z) λs. w)) w y");
        let output = Expression::parse(input);
        assert!(output.unwrap().1.buffer.is_empty())
    }

    #[test]
    fn test_multi_abstraction() {
        let input = PositionedBuffer::new("λx y.x y z");
        let output = Expression::parse(input);
        assert!(output.unwrap().1.buffer.is_empty())
    }
}
