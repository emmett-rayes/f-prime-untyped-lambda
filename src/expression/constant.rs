use std::marker::PhantomData;

use f_prime_parser::combinators::one_of;
use f_prime_parser::{DefaultParsable, Parser, ParserResult};

use crate::expression::buffer::PositionedBuffer;
use crate::expression::literal::LiteralParser;
use crate::expression::symbol::Symbol;

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

pub struct ConstantParser<C> {
    phantom: PhantomData<C>,
}

impl<C> Default for ConstantParser<C> {
    fn default() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

pub trait DefinedC {
    const CHOICES: &'static [&'static str];
}

impl<'a, C> Parser<PositionedBuffer<'a>> for ConstantParser<C>
where
    C: DefinedC,
{
    type Output = Constant<C>;

    fn parse<'b>(
        &self,
        input: PositionedBuffer<'a>,
    ) -> ParserResult<PositionedBuffer<'a>, Self::Output>
    where
        PositionedBuffer<'a>: 'b,
    {
        let parser = one_of(
            C::CHOICES
                .iter()
                .map(|constant| LiteralParser::new(constant).boxed())
                .collect(),
        )
        .map(Constant::from);

        parser.parse(input)
    }
}

impl<'a, C> DefaultParsable<PositionedBuffer<'a>> for Constant<C>
where
    C: DefinedC,
{
    fn parser() -> impl Parser<PositionedBuffer<'a>, Output = Self>
    where
        Self: Sized,
    {
        ConstantParser::default()
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use super::*;

    #[test]
    fn test_constant() {
        #[derive(Debug)]
        struct TestC;

        impl DefinedC for TestC {
            const CHOICES: &'static [&'static str] = &["fix", "top"];
        }

        let input = PositionedBuffer::new("fix");
        assert_matches!(
            Constant::<TestC>::parse(input),
            Ok((constant, _)) if constant.symbol == "fix",
        );

        let input = PositionedBuffer::new("top");
        assert_matches!(
            Constant::<TestC>::parse(input),
            Ok((constant, _)) if constant.symbol == "top",
        );

        let input = PositionedBuffer::new("else");
        assert_matches!(Constant::<TestC>::parse(input), Err(_),);
    }
}
