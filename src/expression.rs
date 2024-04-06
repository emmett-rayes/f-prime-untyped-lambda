use f_prime_parser::ParserResult;
use crate::expression::abstraction::Abstraction;
use crate::expression::buffer::{Parsable, PositionedBuffer};
use crate::expression::variable::{IndexedVariable, NamedVariable};

pub mod buffer;
pub mod symbol;
pub mod constant;
pub mod variable;
pub mod abstraction;


#[derive(Clone, Debug, Eq, PartialEq)]
enum Expression {
    NamedVariable(NamedVariable),
    IndexedVariable(IndexedVariable),
    Abstraction(Box<Abstraction>),
}

impl Parsable for Expression {
    fn parse(input: PositionedBuffer) -> ParserResult<PositionedBuffer, Self> {
        todo!()
    }
}

impl From<Abstraction> for Expression {
    fn from(value: Abstraction) -> Self {
        Expression::Abstraction(Box::from(value))
    }
}
