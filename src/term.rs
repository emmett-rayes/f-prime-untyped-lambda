use crate::expression::Expression;
pub use crate::lang::untyped::term as untyped;

pub trait Term {
    fn as_expr(&self) -> &Expression;

    fn as_expr_mut(&mut self) -> &mut Expression;

    fn validate(&self) -> bool;
}
