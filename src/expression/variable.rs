use crate::expression::{symbol, Expression, ExpressionParser};
use f_prime_parser::Parser;

#[derive(Debug)]
pub struct Variable {
    symbol: String,
}

impl Expression for Variable {
    fn parser<'a>() -> ExpressionParser<'a, Self>
    where
        Self: Sized,
    {
        symbol().map(|output| Variable { symbol: output }).boxed()
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
