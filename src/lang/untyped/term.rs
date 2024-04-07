use crate::expression::Expression;
use crate::term::Term;

pub struct UntypedLambdaTerm {
    pub expression: Expression,
}

impl UntypedLambdaTerm {
    pub fn new(expression: Expression) -> Self {
        UntypedLambdaTerm { expression }
    }
}

impl Term for UntypedLambdaTerm {
    fn as_expr(&mut self) -> &mut Expression {
        &mut self.expression
    }
}
