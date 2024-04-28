use crate::term::Term;
use crate::traverse::pretty_print::ExpressionPrettyPrinter;

pub mod by_value;

pub trait BetaReduction<T>
where
    T: Term,
{
    fn reduce_once(term: &mut T) -> bool;

    fn reduce(term: &mut T) -> bool {
        let mut reduced = false;
        while Self::reduce_once(term) {
            reduced = true;
        }
        reduced
    }
}

pub trait TracingBetaReduction<T>
where
    T: Term,
{
    fn trace_once(term: &mut T) -> Option<String>;

    fn trace(term: &mut T) -> Vec<String>;
}

impl<T, E> TracingBetaReduction<T> for E
where
    T: Term,
    E: BetaReduction<T>,
{
    fn trace_once(term: &mut T) -> Option<String> {
        if Self::reduce_once(term) {
            Some(ExpressionPrettyPrinter::format_named(term.as_expr_mut()))
        } else {
            None
        }
    }

    fn trace(term: &mut T) -> Vec<String> {
        let mut trace = Vec::new();
        while let Some(string) = Self::trace_once(term) {
            trace.push(string);
        }
        trace
    }
}
