use std::marker::PhantomData;

use f_prime_parser::{DefaultParsable, Parser, ParserResult};

use crate::expression::buffer::PositionedBuffer;
use crate::expression::symbol::{Symbol, SymbolParser};

pub type DeBruijnIndex = u64;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Variable<T> {
    pub symbol: Symbol,
    pub index: DeBruijnIndex,
    phantom: PhantomData<T>,
}

impl<T> From<Symbol> for Variable<T> {
    fn from(value: Symbol) -> Self {
        Variable {
            symbol: value,
            index: 0,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> DefaultParsable<PositionedBuffer<'a>> for Variable<T> {
    fn parser() -> impl Parser<PositionedBuffer<'a>, Output = Self> {
        VariableParser::default()
    }
}

pub struct VariableParser<T> {
    phantom: PhantomData<T>,
}

impl<T> Default for VariableParser<T> {
    fn default() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<'a, T> Parser<PositionedBuffer<'a>> for VariableParser<T> {
    type Output = Variable<T>;

    fn parse<'b>(
        &self,
        input: PositionedBuffer<'a>,
    ) -> ParserResult<PositionedBuffer<'a>, Self::Output>
    where
        PositionedBuffer<'a>: 'b,
    {
        let parser = SymbolParser.map(Variable::from);
        parser.parse(input)
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use super::*;

    #[test]
    fn test_variable() {
        #[derive(Debug)]
        struct Dummy;

        let input = PositionedBuffer::new("x y");
        assert_matches!(
            Variable::<Dummy>::parser().parse(input),
            Ok((variable, _)) if variable.symbol == "x",
        );

        let input = PositionedBuffer::new("->");
        assert_matches!(Variable::<Dummy>::parse(input), Err(_),);
    }
}
