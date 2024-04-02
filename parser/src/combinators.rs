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

pub fn between<'a, OL, PL, I, O, P, OR, PR>(l: PL, p: P, r: PR) -> impl Parser<I, Output = O> + 'a
where
    I: ParserInput + 'a,
    O: 'a,
    P: Parser<I, Output = O> + 'a,
    OL: 'a,
    PL: Parser<I, Output = OL> + 'a,
    OR: 'a,
    PR: Parser<I, Output = OR> + 'a,
{
    l.then(p).right().then(r).left()
}
