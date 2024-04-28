use std::fmt::Debug;
use std::marker::PhantomData;

use f_prime_parser::{DefaultParsable, Parser, ParserResult, ThenParserExtensions};

use crate::expression::buffer::PositionedBuffer;
use crate::expression::literal::LiteralParser;
use crate::expression::symbol::Symbol;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Abstraction<P, B> {
    pub parameter: P,
    pub body: B,
}

impl<'a, P, B> DefaultParsable<PositionedBuffer<'a>> for Abstraction<P, B>
where
    Self: TryFrom<B>,
    <Self as TryFrom<B>>::Error: Debug,
    P: 'a + DefaultParsable<PositionedBuffer<'a>>,
    B: DefaultParsable<PositionedBuffer<'a>> + From<Abstraction<P, B>>,
{
    fn parser() -> impl Parser<PositionedBuffer<'a>, Output = Self> {
        AbstractionParser::default()
    }
}

pub struct AbstractionParser<P, B> {
    parameter_parser: PhantomData<P>,
    body_parser: PhantomData<B>,
}

impl<'a, P, B> AbstractionParser<P, B> {
    fn lambda_parser() -> impl Parser<PositionedBuffer<'a>, Output = Symbol> + 'a {
        LiteralParser::new("λ")
            .or_else(LiteralParser::new("@"))
            .or_else(LiteralParser::new("\\"))
    }

    fn parameters_parser() -> impl Parser<PositionedBuffer<'a>, Output = Vec<P>> + 'a
    where
        P: 'a + DefaultParsable<PositionedBuffer<'a>>,
    {
        P::parser().at_least(1).then_skip(LiteralParser::new("."))
    }
}

impl<P, B> Default for AbstractionParser<P, B> {
    fn default() -> Self {
        Self {
            parameter_parser: PhantomData,
            body_parser: PhantomData,
        }
    }
}

impl<'a, P, B> Parser<PositionedBuffer<'a>> for AbstractionParser<P, B>
where
    Abstraction<P, B>: TryFrom<B>,
    <Abstraction<P, B> as TryFrom<B>>::Error: Debug,
    P: 'a + DefaultParsable<PositionedBuffer<'a>>,
    B: DefaultParsable<PositionedBuffer<'a>> + From<Abstraction<P, B>>,
{
    type Output = Abstraction<P, B>;

    fn parse<'b>(
        &self,
        input: PositionedBuffer<'a>,
    ) -> ParserResult<PositionedBuffer<'a>, Self::Output>
    where
        PositionedBuffer<'a>: 'b,
    {
        let parser = Self::lambda_parser()
            .skip_then(Self::parameters_parser())
            .then(B::parser())
            .map(|(parameters, body)| {
                parameters.into_iter().rfold(body, |body, parameter| {
                    B::from(Abstraction { parameter, body })
                })
            })
            .map(|expr| Abstraction::<P, B>::try_from(expr).unwrap());

        parser.parse(input)
    }
}
