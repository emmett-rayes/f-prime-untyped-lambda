use std::ops::{Deref, Range};

pub mod combinators;

pub type ParserError<I> = (String, I, Range<usize>);
pub type ParserResult<I, O> = Result<(O, I), ParserError<I>>;
pub type BoxedParser<'a, I, O> = Box<dyn Parser<I, Output = O> + 'a>;

pub trait ParserInput
where
    Self: Sized,
{
    fn error(self, message: &str) -> ParserError<Self>;
}

pub trait DefaultParsable<I>
where
    I: ParserInput,
{
    fn parse(input: I) -> ParserResult<I, Self>
    where
        Self: Sized,
    {
        Self::parser().parse(input)
    }

    fn parser() -> impl Parser<I, Output = Self>
    where
        Self: Sized;
}

pub trait Parser<I>
where
    I: ParserInput,
{
    type Output;

    fn parse<'a>(&self, input: I) -> ParserResult<I, Self::Output>
    where
        I: 'a;

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
    fn at_least(self, minimum: u64) -> AtLeastParser<Self>
    where
        Self: Sized,
        I: Clone,
    {
        AtLeastParser::new(minimum, self)
    }
}

impl<'a, I, O> Parser<I> for BoxedParser<'a, I, O>
where
    I: ParserInput,
{
    type Output = O;

    fn parse<'b>(&self, input: I) -> ParserResult<I, Self::Output>
    where
        I: 'b,
    {
        self.deref().parse(input)
    }

    fn boxed<'b>(self) -> BoxedParser<'a, I, Self::Output>
    where
        Self: 'b,
    {
        self
    }
}

impl<F, I, O> Parser<I> for F
where
    I: ParserInput,
    F: Fn(I) -> ParserResult<I, O>,
{
    type Output = O;

    fn parse<'a>(&self, input: I) -> ParserResult<I, O>
    where
        I: 'a,
    {
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

    fn parse<'a>(&self, input: I) -> ParserResult<I, Self::Output>
    where
        I: 'a,
    {
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

    fn parse<'a>(&self, input: I) -> ParserResult<I, Self::Output>
    where
        I: 'a,
    {
        self.first_parser
            .parse(input)
            .and_then(|(output1, remaining1)| {
                self.second_parser
                    .parse(remaining1)
                    .map(|(output2, remaining2)| ((output1, output2), remaining2))
            })
    }
}

pub trait ThenParserExtensions<I, O1, O2, P>
where
    I: ParserInput,
{
    fn skip_then(self, parser: P) -> impl Parser<I, Output = O2>;

    fn then_skip(self, parser: P) -> impl Parser<I, Output = O1>;
}

impl<I, O1, O2, P1, P2> ThenParserExtensions<I, O1, O2, P2> for P1
where
    I: ParserInput,
    P1: Parser<I, Output = O1>,
    P2: Parser<I, Output = O2>,
{
    fn skip_then(self, parser: P2) -> impl Parser<I, Output = O2> {
        self.then(parser).right()
    }

    fn then_skip(self, parser: P2) -> impl Parser<I, Output = O1> {
        self.then(parser).left()
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

    fn parse<'a>(&self, input: I) -> ParserResult<I, Self::Output>
    where
        I: 'a,
    {
        let input_clone = input.clone();
        self.first_parser
            .parse(input)
            .or_else(|_| self.second_parser.parse(input_clone))
    }
}

pub struct AtLeastParser<P> {
    min: u64,
    parser: P,
}

impl<P> AtLeastParser<P> {
    fn new<I, O>(min: u64, parser: P) -> AtLeastParser<P>
    where
        I: ParserInput,
        P: Parser<I, Output = O>,
    {
        AtLeastParser { min, parser }
    }
}

impl<I, O, P> Parser<I> for AtLeastParser<P>
where
    I: ParserInput + Clone,
    P: Parser<I, Output = O>,
{
    type Output = Vec<O>;

    fn parse<'a>(&self, input: I) -> ParserResult<I, Self::Output>
    where
        I: 'a,
    {
        let mut total_output = Vec::new();
        let mut remaining_input = input;
        while let Ok((output, remaining)) = self.parser.parse(remaining_input.clone()) {
            total_output.push(output);
            remaining_input = remaining
        }
        if (total_output.len() as u64) < self.min {
            Err(remaining_input.error("Unexpected input at this position."))
        } else {
            Ok((total_output, remaining_input))
        }
    }
}
