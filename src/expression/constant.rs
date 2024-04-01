use crate::expression::{literal, Expression, ExpressionParser};
use f_prime_parser::{BoxedParser, Parser, ParserInput};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Constant<T>
where
    T: DefinedConstants,
{
    symbol: String,
    constants: PhantomData<T>,
}

pub trait DefinedConstants {
    const CHOICES: &'static [&'static str];
}

impl<CONSTANTS: DefinedConstants> Expression for Constant<CONSTANTS> {
    fn parser<'a>() -> ExpressionParser<'a, Self>
    where
        Self: Sized,
    {
        BoxedParser::new(move |input: ParserInput<'a>| {
            CONSTANTS::CHOICES
                .iter()
                .map(|constant| literal(constant))
                .reduce(Parser::or_else)
                .ok_or(input.clone().error("Failed to match constant.".to_string()))?
                .map(|symbol| Constant {
                    symbol,
                    constants: PhantomData,
                })
                .parse(input)
        })
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use f_prime_parser::ParserInput;

    use super::*;

    #[test]
    fn test_constant() {
        #[derive(Debug)]
        struct TestConstants;

        impl DefinedConstants for TestConstants {
            const CHOICES: &'static [&'static str] = &["fix", "top"];
        }

        let input = ParserInput::new("fix");
        assert_matches!(
            Constant::<TestConstants>::parse(input),
            Ok((constant, _)) if constant.symbol == "fix",
        );

        let input = ParserInput::new("top");
        assert_matches!(
            Constant::<TestConstants>::parse(input),
            Ok((constant, _)) if constant.symbol == "top",
        );

        let input = ParserInput::new("else");
        assert_matches!(Constant::<TestConstants>::parse(input), Err(_),);
    }
}
