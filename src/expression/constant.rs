use crate::expression::{literal, Expression, ExpressionParser};
use f_prime_parser::combinators::one_of;
use f_prime_parser::{Parser, PositionedBuffer};
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

impl<CONSTANTS: DefinedConstants> Expression for Constant<CONSTANTS>
where
    Self: Sized,
{
    fn parser<'a>() -> ExpressionParser<'a, Self> {
        let parser = move |input: PositionedBuffer<'a>| {
            one_of(
                CONSTANTS::CHOICES
                    .iter()
                    .map(|constant| literal(constant).boxed())
                    .collect(),
            )
            .map(|symbol| Constant {
                symbol,
                constants: PhantomData,
            })
            .parse(input)
        };
        parser.boxed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use f_prime_parser::PositionedBuffer;
    use std::assert_matches::assert_matches;

    #[test]
    fn test_constant() {
        #[derive(Debug)]
        struct TestConstants;

        impl DefinedConstants for TestConstants {
            const CHOICES: &'static [&'static str] = &["fix", "top"];
        }

        let input = PositionedBuffer::new("fix");
        assert_matches!(
            Constant::<TestConstants>::parse(input),
            Ok((constant, _)) if constant.symbol == "fix",
        );

        let input = PositionedBuffer::new("top");
        assert_matches!(
            Constant::<TestConstants>::parse(input),
            Ok((constant, _)) if constant.symbol == "top",
        );

        let input = PositionedBuffer::new("else");
        assert_matches!(Constant::<TestConstants>::parse(input), Err(_),);
    }
}
