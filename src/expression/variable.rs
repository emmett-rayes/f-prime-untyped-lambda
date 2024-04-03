use crate::expression::buffer::PositionedBuffer;
use crate::expression::{symbol, Expression};
use crate::visitor::Visitable;
use f_prime_parser::{Parser, ParserResult};

pub type VariableIndex = u64;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Variable {
    pub symbol: String,
    pub index: VariableIndex,
}

impl Variable {
    pub(crate) fn new(symbol: &str) -> Self {
        Variable {
            symbol: symbol.to_string(),
            index: 0,
        }
    }
}

impl Visitable for Variable {}

impl Expression for Variable {
    fn parse(input: PositionedBuffer) -> ParserResult<PositionedBuffer, Self> {
        let parser = symbol().map(Variable::new);
        parser.parse(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches::assert_matches;

    #[test]
    fn test_variable() {
        let input = PositionedBuffer::new("x y");
        assert_matches!(
            Variable::parse(input),
            Ok((variable, _)) if variable.symbol == "x",
        );

        let input = PositionedBuffer::new("->");
        assert_matches!(Variable::parse(input), Err(_),);
    }
}
