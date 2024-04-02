use std::ops::{Deref, Range};

pub mod combinators;

pub type ParserError<I> = (String, I, Range<usize>);
pub type ParserResult<I, O> = Result<(O, I), ParserError<I>>;
pub type BoxedParser<'a, I, O> = Box<dyn Parser<I, Output = O> + 'a>;

pub trait ParserInput
where
    Self: Sized,
{
    fn error(self, message: String) -> ParserError<Self>;
}

#[derive(Clone, Debug)]
pub struct PositionedBuffer<'a> {
    pub buffer: &'a str,
    pub position: usize,
}

impl<'a> PositionedBuffer<'a> {
    pub fn new(input: &'a str) -> Self {
        PositionedBuffer {
            buffer: input,
            position: 0,
        }
    }

    pub fn seek(self, length: usize) -> Self {
        PositionedBuffer {
            buffer: &self.buffer[length..],
            position: self.position + length,
        }
    }
}

impl<'a> ParserInput for PositionedBuffer<'a> {
    fn error(self, message: String) -> ParserError<Self> {
        let range = self.position..self.position;
        (message, self, range)
    }
}

pub trait Parser<I>
where
    I: ParserInput,
{
    type Output;

    fn parse(&self, input: I) -> ParserResult<I, Self::Output>;

    fn boxed<'a>(self) -> BoxedParser<'a, I, Self::Output>
    where
        Self: Sized + 'a,
    {
        Box::new(self)
    }

    fn map<F, A>(self, f: F) -> MapParser<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Output) -> A,
    {
        MapParser::new(self, f)
    }

    fn then<O, P>(self, parser: P) -> ThenParser<Self, P>
    where
        Self: Sized,
        P: Parser<I, Output = O>,
    {
        ThenParser::new(self, parser)
    }

    fn or_else<P>(self, parser: P) -> OrElseParser<Self, P>
    where
        Self: Sized,
        I: Clone,
        P: Parser<I, Output = Self::Output>,
    {
        OrElseParser::new(self, parser)
    }
}

impl<'a, I, O> Parser<I> for BoxedParser<'a, I, O>
where
    I: ParserInput,
{
    type Output = O;

    fn parse(&self, input: I) -> ParserResult<I, Self::Output> {
        self.deref().parse(input)
    }
}

impl<F, I, O> Parser<I> for F
where
    I: ParserInput,
    F: Fn(I) -> ParserResult<I, O>,
{
    type Output = O;

    fn parse(&self, input: I) -> ParserResult<I, O> {
        self(input)
    }
}

pub struct MapParser<P, F> {
    parser: P,
    function: F,
}

impl<P, F> MapParser<P, F> {
    fn new<I, A, B>(parser: P, f: F) -> MapParser<P, F>
    where
        I: ParserInput,
        P: Parser<I, Output = A>,
        F: Fn(A) -> B,
    {
        MapParser {
            parser,
            function: f,
        }
    }
}

impl<I, A, B, P, F> Parser<I> for MapParser<P, F>
where
    I: ParserInput,
    P: Parser<I, Output = A>,
    F: Fn(A) -> B,
{
    type Output = B;

    fn parse(&self, input: I) -> ParserResult<I, Self::Output> {
        self.parser
            .parse(input)
            .map(|(output, remaining)| ((self.function)(output), remaining))
    }
}

pub struct ThenParser<P1, P2> {
    first_parser: P1,
    second_parser: P2,
}

impl<P1, P2> ThenParser<P1, P2> {
    fn new<I, O1, O2>(first: P1, second: P2) -> ThenParser<P1, P2>
    where
        I: ParserInput,
        P1: Parser<I, Output = O1>,
        P2: Parser<I, Output = O2>,
    {
        ThenParser {
            first_parser: first,
            second_parser: second,
        }
    }

    pub fn left<I, O1, O2>(self) -> impl Parser<I, Output = O1>
    where
        I: ParserInput,
        Self: Parser<I, Output = (O1, O2)>,
    {
        self.map(|(a1, _)| a1)
    }

    pub fn right<I, O1, O2>(self) -> impl Parser<I, Output = O2>
    where
        I: ParserInput,
        Self: Parser<I, Output = (O1, O2)>,
    {
        self.map(|(_, a2)| a2)
    }
}

impl<I, O1, O2, P1, P2> Parser<I> for ThenParser<P1, P2>
where
    I: ParserInput,
    P1: Parser<I, Output = O1>,
    P2: Parser<I, Output = O2>,
{
    type Output = (O1, O2);

    fn parse(&self, input: I) -> ParserResult<I, Self::Output> {
        self.first_parser
            .parse(input)
            .and_then(|(output1, remaining1)| {
                self.second_parser
                    .parse(remaining1)
                    .map(|(output2, remaining2)| ((output1, output2), remaining2))
            })
    }
}

pub struct OrElseParser<P1, P2> {
    first_parser: P1,
    second_parser: P2,
}

impl<P1, P2> OrElseParser<P1, P2> {
    fn new<I, O>(first: P1, second: P2) -> OrElseParser<P1, P2>
    where
        I: ParserInput,
        P1: Parser<I, Output = O>,
        P2: Parser<I, Output = O>,
    {
        OrElseParser {
            first_parser: first,
            second_parser: second,
        }
    }
}

impl<I, O, P1, P2> Parser<I> for OrElseParser<P1, P2>
where
    I: ParserInput + Clone,
    P1: Parser<I, Output = O>,
    P2: Parser<I, Output = O>,
{
    type Output = O;

    fn parse(&self, input: I) -> ParserResult<I, Self::Output> {
        let input_clone = input.clone();
        self.first_parser
            .parse(input)
            .or_else(|_| self.second_parser.parse(input_clone))
    }
}
