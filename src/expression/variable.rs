use f_prime_parser::{Parser, ParserResult};

use crate::expression::buffer::Parsable;
use crate::expression::buffer::PositionedBuffer;
use crate::expression::symbol::{symbol_parser, Symbol};
use crate::expression::Expression;

pub type DeBruijnIndex = u64;
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Variable {
    pub symbol: Symbol,
    pub index: DeBruijnIndex,
}

impl From<Symbol> for Variable {
    fn from(value: Symbol) -> Self {
        Variable {
            symbol: value,
            index: 0,
        }
    }
}

impl TryFrom<Expression> for Variable {
    type Error = ();

    fn try_from(value: Expression) -> Result<Self, Self::Error> {
        if let Expression::Variable(variable) = value {
            Ok(variable)
        } else {
            Err(())
        }
    }
}

impl Parsable for Variable {
    fn parse(input: PositionedBuffer) -> ParserResult<PositionedBuffer, Self> {
        let parser = symbol_parser().map(Variable::from);
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
            Variable::parse(input),
            Ok((variable, _)) if variable.symbol == "x",
        );

        let input = PositionedBuffer::new("->");
        assert_matches!(Variable::parse(input), Err(_),);
    }
}
