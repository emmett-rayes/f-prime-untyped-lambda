use crate::expression::{symbol, Expression};
use f_prime_parser::{Parser, PositionedBuffer};

#[derive(Debug)]
pub struct Variable {
    symbol: String,
}

impl Expression for Variable {
    fn parser<'a>() -> impl Parser<PositionedBuffer<'a>, Output = Self> + 'a
    where
        Self: Sized,
    {
        symbol().map(|output| Variable { symbol: output })
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use f_prime_parser::PositionedBuffer;

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
