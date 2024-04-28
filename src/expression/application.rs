use std::fmt::Debug;
use std::marker::PhantomData;

use f_prime_parser::{DefaultParsable, Parser, ParserResult};

use crate::expression::buffer::PositionedBuffer;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Application<L, A> {
    pub applicator: L,
    pub argument: A,
}

impl<'a, L, A> DefaultParsable<PositionedBuffer<'a>> for Application<L, A>
where
    Self: TryFrom<L>,
    <Self as TryFrom<L>>::Error: Debug,
    L: 'a + DefaultParsable<PositionedBuffer<'a>> + From<Application<L, A>>,
    A: 'a + DefaultParsable<PositionedBuffer<'a>>,
{
    fn parser() -> impl Parser<PositionedBuffer<'a>, Output = Self>
    where
        Self: Sized,
    {
        ApplicationParser::default()
    }
}

pub struct ApplicationParser<L, A> {
    phantom_1: PhantomData<L>,
    phantom_2: PhantomData<A>,
}

impl<L, A> Default for ApplicationParser<L, A> {
    fn default() -> Self {
        Self {
            phantom_1: PhantomData,
            phantom_2: PhantomData,
        }
    }
}

impl<'a, L, A> Parser<PositionedBuffer<'a>> for ApplicationParser<L, A>
where
    Application<L, A>: TryFrom<L>,
    <Application<L, A> as TryFrom<L>>::Error: Debug,
    L: 'a + DefaultParsable<PositionedBuffer<'a>> + From<Application<L, A>>,
    A: 'a + DefaultParsable<PositionedBuffer<'a>>,
{
    type Output = Application<L, A>;

    fn parse<'b>(
        &self,
        input: PositionedBuffer<'a>,
    ) -> ParserResult<PositionedBuffer<'a>, Self::Output>
    where
        PositionedBuffer<'a>: 'b,
        Self::Output: 'b,
    {
        let parser = L::parser()
            .then(A::parser().at_least(1))
            .map(|expressions| {
                Application::try_from(expressions.1.into_iter().fold(
                    expressions.0,
                    |accumulator, current| {
                        L::from(Application {
                            applicator: accumulator,
                            argument: current,
                        })
                    },
                ))
                .unwrap()
            });

        parser.parse(input)
    }
}
