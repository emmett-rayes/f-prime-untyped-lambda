use f_prime_parser::{ParserError, ParserInput};

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
    fn error(self, message: &str) -> ParserError<Self> {
        let range = self.position..self.position;
        (String::from(message), self, range)
    }
}
