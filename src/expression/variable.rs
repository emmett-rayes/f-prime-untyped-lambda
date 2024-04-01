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
        symbol().map(|output| Variable { symbol: output })
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use f_prime_parser::ParserInput;

    use super::*;

    #[test]
    fn test_variable() {
        let input = ParserInput::new("x y");
        assert_matches!(
            Variable::parse(input),
            Ok((variable, _)) if variable.symbol == "x",
        );

        let input = ParserInput::new("->");
        assert_matches!(Variable::parse(input), Err(_),);
    }
}
