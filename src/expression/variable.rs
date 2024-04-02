use crate::expression::{symbol, Expression};
use f_prime_parser::{Parser, ParserResult, PositionedBuffer};

pub type VariableIndex = u64;

#[derive(Debug)]
pub struct Variable {
    symbol: String,
    index: VariableIndex,
}

impl Variable {
    fn new(symbol: String, index: VariableIndex) -> Self {
        Variable { symbol, index }
    }
}

impl Expression for Variable {
    fn parse(input: PositionedBuffer) -> ParserResult<PositionedBuffer, Self> {
        let parser = symbol().map(|output| Variable::new(output, 0));
        parser.parse(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use f_prime_parser::PositionedBuffer;
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
