use crate::expression::{literal, Expression};
use f_prime_parser::{BoxedParser, Parser, PositionedBuffer};
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
    fn parser<'a>() -> impl Parser<PositionedBuffer<'a>, Output = Self> + 'a {
        fn or_else<'a>(
            this: BoxedParser<'a, PositionedBuffer<'a>, String>,
            other: impl Parser<PositionedBuffer<'a>, Output = String> + 'a,
        ) -> BoxedParser<'a, PositionedBuffer<'a>, String> {
            this.or_else(other).boxed()
        }

        move |input: PositionedBuffer<'a>| {
            CONSTANTS::CHOICES
                .iter()
                .map(|constant| literal(constant).boxed())
                .reduce(or_else)
                .ok_or(input.clone().error("Failed to match constant.".to_string()))?
                .map(|symbol| Constant {
                    symbol,
                    constants: PhantomData,
                })
                .parse(input)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use f_prime_parser::PositionedBuffer;

    use super::*;

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
