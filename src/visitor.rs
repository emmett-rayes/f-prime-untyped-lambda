pub trait Visitor<T> {
    type Result;
    type Context;

    fn visit(&mut self, context: Self::Context, t: &mut T) -> Self::Result;
}
