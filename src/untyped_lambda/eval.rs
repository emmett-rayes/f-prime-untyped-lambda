use crate::expression::Expression;

pub mod by_value;
pub mod full;
mod shift;
mod substitution;

pub trait BetaReduction<T>
where
    T: Expression,
{
    fn reduce_once(term: &mut T) -> bool;

    fn reduce(term: &mut T) -> bool;
}
