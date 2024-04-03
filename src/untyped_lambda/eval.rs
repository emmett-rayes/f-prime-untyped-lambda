use crate::expression::Expression;

mod by_value;
mod shift;
mod substitution;

pub trait BetaReduction<T>
where
    T: Expression,
{
    fn reduce_once(term: T) -> Result<T, T>;

    fn reduce(term: T) -> Result<T, T>;
}
