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

pub trait TracingBetaReduction<T>
where
    T: Expression,
{
    fn trace_once(term: &mut T) -> Option<String>;

    fn trace(term: &mut T) -> Vec<String>;
}

impl<T, E> BetaReduction<T> for E
where
    T: Expression,
    E: TracingBetaReduction<T>,
{
    fn reduce_once(term: &mut T) -> bool {
        E::trace_once(term).is_some()
    }

    fn reduce(term: &mut T) -> bool {
        !E::trace(term).is_empty()
    }
}
