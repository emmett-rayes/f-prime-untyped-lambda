use crate::{BoxedParser, Parser, ParserInput};

pub fn one_of<'a, I, O, P>(vec: Vec<P>) -> impl Parser<I, Output = O> + 'a
where
    I: ParserInput + Clone + 'a,
    O: 'a,
    P: Parser<I, Output = O> + 'a,
{
    fn or_else<'a, I, O>(
        this: BoxedParser<'a, I, O>,
        other: impl Parser<I, Output = O> + 'a,
    ) -> impl Parser<I, Output = O> + 'a
    where
        I: ParserInput + Clone + 'a,
        O: 'a,
    {
        this.or_else(other)
    }

    vec.into_iter()
        .map(|p| p.boxed())
        .reduce(|p1, p2| or_else(p1, p2).boxed())
        .expect("Parser list must not be empty")
}
