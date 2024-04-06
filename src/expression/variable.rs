use f_prime_parser::{Parser, ParserResult};

use crate::expression::buffer::PositionedBuffer;
use crate::expression::symbol::{symbol_parser, Symbol};
use crate::expression::buffer::Parsable;

pub type DeBruijnIndex = u64;
#[derive(Copy,Clone, Debug, Eq, PartialEq)]
pub struct IndexedVariable {
    pub index: DeBruijnIndex,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NamedVariable {
    pub symbol: Symbol,
}

impl From<String> for NamedVariable {
    fn from(value: String) -> Self {
        NamedVariable { symbol: value }
    }
}

impl Parsable for NamedVariable {
    fn parse(input: PositionedBuffer) -> ParserResult<PositionedBuffer, Self> {
        let parser = symbol_parser().map(NamedVariable::from);
        parser.parse(input)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Variable {
    Indexed(IndexedVariable),
    Named(NamedVariable),
}

impl Parsable for Variable {
    fn parse(input: PositionedBuffer) -> ParserResult<PositionedBuffer, Self> {
        let parser = NamedVariable::parser().map(Variable::Named);
        parser.parse(input)
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use super::*;

    #[test]
    fn test_variable() {
        let input = PositionedBuffer::new("x y");
        assert_matches!(
            NamedVariable::parse(input),
            Ok((variable, _)) if variable.symbol == "x",
        );

        let input = PositionedBuffer::new("->");
        assert_matches!(NamedVariable::parse(input), Err(_),);
    }
}
