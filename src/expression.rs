use f_prime_parser::combinators::between;
use f_prime_parser::{Parser, ParserResult};

use crate::expression::abstraction::Abstraction;
use crate::expression::application::Application;
use crate::expression::buffer::{Parsable, PositionedBuffer};
use crate::expression::symbol::literal_parser;
use crate::expression::variable::Variable;

pub mod abstraction;
pub mod application;
pub mod buffer;
pub mod constant;
pub mod symbol;
pub mod variable;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expression {
    Variable(Variable),
    Abstraction(Box<Abstraction>),
    Application(Box<Application>),
}

impl Expression {
    pub fn is_value(&self) -> bool {
        matches!(self, Expression::Abstraction(_))
    }
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
        between(
            literal_parser("("),
            Expression::parser(),
            literal_parser(")"),
        )
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

impl From<Variable> for Expression {
    fn from(value: Variable) -> Self {
        Expression::Variable(value)
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
