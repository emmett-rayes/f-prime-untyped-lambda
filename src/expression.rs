use f_prime_parser::combinators::between;
use f_prime_parser::{Parser, ParserResult};

use crate::expression::abstraction::{Abstraction, TypedAbstraction};
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
    TypedAbstraction(Box<TypedAbstraction>),
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
        Abstraction::parser()
            .map(Expression::from)
            .or_else(TypedAbstraction::parser().map(Expression::from))
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

impl From<TypedAbstraction> for Expression {
    fn from(value: TypedAbstraction) -> Self {
        Expression::TypedAbstraction(Box::from(value))
    }
}

impl From<Application> for Expression {
    fn from(value: Application) -> Self {
        Expression::Application(Box::from(value))
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

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use super::*;

    #[test]
    fn test_variable() {
        let input = PositionedBuffer::new("x");
        let (expression, remaining) = Expression::parse(input).unwrap();
        assert!(remaining.buffer.is_empty());
        assert_matches!(expression, Expression::Variable(_));
        let variable = Variable::try_from(expression).unwrap();
        assert_eq!(variable.symbol, "x");
    }

    #[test]
    fn test_abstraction() {
        let input = PositionedBuffer::new("λx. y x");
        let (expression, remaining) = Expression::parse(input).unwrap();
        assert!(remaining.buffer.is_empty());
        assert_matches!(expression, Expression::Abstraction(_));
        let abstraction = Abstraction::try_from(expression).unwrap();
        assert_eq!(abstraction.parameter.symbol, "x");
        assert_matches!(abstraction.body, Expression::Application(_));
    }

    #[test]
    fn test_abstraction_nested() {
        let input = PositionedBuffer::new("λx y.x y z");
        let (expression, remaining) = Expression::parse(input).unwrap();
        assert!(remaining.buffer.is_empty());
        assert_matches!(expression, Expression::Abstraction(_));
        let abstraction = Abstraction::try_from(expression).unwrap();
        assert_matches!(abstraction.body, Expression::Abstraction(_));
    }

    #[test]
    fn test_typed_abstraction() {
        let input = PositionedBuffer::new("λx:T.y x");
        let (expression, remaining) = Expression::parse(input).unwrap();
        assert!(remaining.buffer.is_empty());
        assert_matches!(expression, Expression::TypedAbstraction(_));
        let abstraction = TypedAbstraction::try_from(expression).unwrap();
        assert_eq!(abstraction.parameter.symbol, "x");
        assert_matches!(abstraction.parameter_type, Expression::Variable(_));
        assert_matches!(abstraction.body, Expression::Application(_));
    }

    #[test]
    fn test_typed_abstraction_nested() {
        let input = PositionedBuffer::new("λx:T,y:U.x y z");
        let (expression, remaining) = Expression::parse(input).unwrap();
        assert!(remaining.buffer.is_empty());
        dbg!(&expression);
        assert_matches!(expression, Expression::TypedAbstraction(_));
        let abstraction = TypedAbstraction::try_from(expression).unwrap();
        assert_matches!(abstraction.body, Expression::TypedAbstraction(_));
    }

    #[test]
    fn test_application() {
        let input = PositionedBuffer::new("(λx. x) (λx. x)");
        let (expression, remaining) = Expression::parse(input).unwrap();
        assert!(remaining.buffer.is_empty());
        assert_matches!(expression, Expression::Application(_));
        let application = Application::try_from(expression).unwrap();
        assert_matches!(application.applicator, Expression::Abstraction(_));
        assert_matches!(application.argument, Expression::Abstraction(_));
    }

    #[test]
    fn test_expression() {
        let input = PositionedBuffer::new("λx. a (λt. b x t (f (λu. a u t z) λs. w)) w y");
        let (expression, remaining) = Expression::parse(input).unwrap();
        assert!(remaining.buffer.is_empty());
        assert_matches!(expression, Expression::Abstraction(_));
    }
}
