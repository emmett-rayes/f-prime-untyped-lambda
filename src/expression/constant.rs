use std::marker::PhantomData;

use f_prime_parser::combinators::one_of;
use f_prime_parser::{Parser, ParserResult};

use crate::expression::buffer::Parsable;
use crate::expression::buffer::PositionedBuffer;
use crate::expression::symbol::{literal_parser, Symbol};

#[derive(Debug)]
pub struct Constant<T> {
    symbol: Symbol,
    constants: PhantomData<T>,
}

impl<T> From<Symbol> for Constant<T> {
    fn from(value: Symbol) -> Self {
        Constant {
            symbol: value,
            constants: PhantomData,
        }
    }
}

pub trait DefinedConstants {
    const CHOICES: &'static [&'static str];
}

impl<CONSTANTS> Parsable for Constant<CONSTANTS>
where
    CONSTANTS: DefinedConstants,
{
    fn parse(input: PositionedBuffer) -> ParserResult<PositionedBuffer, Self> {
        let parser = one_of(
            CONSTANTS::CHOICES
                .iter()
                .map(|constant| literal_parser(constant).boxed())
                .collect(),
        )
        .map(Constant::from);

        parser.parse(input)
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

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
