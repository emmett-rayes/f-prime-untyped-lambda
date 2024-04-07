use crate::expression::Expression;
use crate::term::Term;

pub struct UntypedLambdaTerm {
    pub expression: Expression,
}

impl UntypedLambdaTerm {
    pub fn new(expression: Expression) -> Self {
        UntypedLambdaTerm { expression }
    }

    fn validate(expression: &Expression) -> bool {
        match expression {
            Expression::Variable(variable) => variable.index != 0,
            Expression::Abstraction(abstraction) => Self::validate(&abstraction.body),
            Expression::Application(application) => {
                Self::validate(&application.applicator) && Self::validate(&application.argument)
            }
            _ => false,
        }
    }
}

impl Term for UntypedLambdaTerm {
    fn as_expr(&self) -> &Expression {
        &self.expression
    }

    fn as_expr_mut(&mut self) -> &mut Expression {
        &mut self.expression
    }

    fn validate(&self) -> bool {
        Self::validate(&self.expression)
    }
}
