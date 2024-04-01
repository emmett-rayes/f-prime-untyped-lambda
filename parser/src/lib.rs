use std::ops::Range;

pub type ParserResult<Input, Output> = Result<(Output, Input), (String, Input, Range<usize>)>;

#[derive(Clone, Debug)]
pub struct ParserInput<'a> {
    pub buffer: &'a str,
    pub position: usize,
}

impl<'a> ParserInput<'a> {
    pub fn new(input: &'a str) -> Self {
        ParserInput {
            buffer: input,
            position: 0,
        }
    }

    pub fn seek(self, length: usize) -> Self {
        ParserInput {
            buffer: &self.buffer[length..],
            position: self.position + length,
        }
    }

    pub fn error(self, message: String) -> (String, Self, Range<usize>) {
        let range = self.position..self.position;
        (message, self, range)
    }
}

pub trait Parser<Input>
where
    Input: Clone,
{
    type Output;

    fn parse(&self, input: Input) -> ParserResult<Input, Self::Output>;

    fn map<'a, F, MapOutput>(self, op: F) -> BoxedParser<'a, Input, MapOutput>
    where
        Self: Sized + 'a,
        F: Fn(Self::Output) -> MapOutput + 'a,
    {
        BoxedParser::new(move |input| {
            self.parse(input)
                .map(|(output, remaining)| (op(output), remaining))
        })
    }

    fn or_else<'a>(
        self,
        other: impl Parser<Input, Output = Self::Output> + 'a,
    ) -> BoxedParser<'a, Input, Self::Output>
    where
        Self: Sized + 'a,
    {
        BoxedParser::new(move |input: Input| {
            let input_clone = input.clone();
            self.parse(input).or_else(|_| other.parse(input_clone))
        })
    }
}

impl<F, Input, Output> Parser<Input> for F
where
    Input: Clone,
    F: Fn(Input) -> ParserResult<Input, Output>,
{
    type Output = Output;

    fn parse(&self, input: Input) -> ParserResult<Input, Output> {
        self(input)
    }
}

pub struct BoxedParser<'a, Input, Output> {
    parser: Box<dyn Parser<Input, Output = Output> + 'a>,
}

impl<'a, Input, Output> Parser<Input> for BoxedParser<'a, Input, Output>
where
    Input: Clone,
{
    type Output = Output;

    fn parse(&self, input: Input) -> ParserResult<Input, Output> {
        self.parser.parse(input)
    }
}

impl<'a, Input, Output> BoxedParser<'a, Input, Output> {
    pub fn new<P>(parser: P) -> Self
    where
        Input: Clone,
        P: Parser<Input, Output = Output> + 'a,
    {
        BoxedParser {
            parser: Box::new(parser),
        }
    }
}
