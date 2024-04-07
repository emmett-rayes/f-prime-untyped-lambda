use crate::expression::Expression;
pub use crate::lang::untyped::term as untyped;

pub trait Term {
    fn as_expr(&mut self) -> &mut Expression;
}
