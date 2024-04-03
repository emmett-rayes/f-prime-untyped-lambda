pub trait Visitor<T> {
    type Result;

    fn visit(&mut self, t: T) -> Self::Result;
}

pub trait Visitable
where
    Self: Sized,
{
    fn accept<V, R>(self, visitor: &mut V) -> R
    where
        V: Visitor<Self, Result = R>,
    {
        visitor.visit(self)
    }
}
