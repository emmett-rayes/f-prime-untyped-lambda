use f_prime_parser::{Parser, ParserError, ParserInput, ParserResult};

pub trait Parsable
where
    Self: Sized,
{
    fn parse(input: PositionedBuffer) -> ParserResult<PositionedBuffer, Self>;

    fn parser<'a>() -> impl Parser<PositionedBuffer<'a>, Output = Self> + 'a
    where
        Self: 'a,
    {
        Self::parse
    }
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

    pub fn seek_whitespace(self) -> Self {
        let mut ws = 0;
        for c in self.buffer.chars() {
            if !c.is_whitespace() {
                break;
            }
            ws += 1;
        }
        self.seek(ws)
    }
}

impl<'a> ParserInput for PositionedBuffer<'a> {
    fn error(self, message: String) -> ParserError<Self> {
        let range = self.position..self.position;
        (message, self, range)
    }
}
