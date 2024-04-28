pub use crate::lang::simple::typecheck as simple;

pub trait TypeChecker<E, T> {
    fn check(term: &E, term_type: &T) -> bool;
}
