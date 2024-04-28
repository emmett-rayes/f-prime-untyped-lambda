use crate::expression::UntypedLambda;
use crate::term::Term;

pub struct SimplyTypedLambdaTerm {
    pub expression: UntypedLambda,
}

impl SimplyTypedLambdaTerm {
    fn validate(expression: &UntypedLambda) -> bool {
        match expression {
            UntypedLambda::Variable(variable) => variable.index != 0,
            UntypedLambda::TypedAbstraction(abstraction) => Self::validate(&abstraction.body),
            UntypedLambda::Application(application) => {
                Self::validate(&application.applicator) && Self::validate(&application.argument)
            }
            _ => false,
        }
    }
}

impl TryFrom<UntypedLambda> for SimplyTypedLambdaTerm {
    type Error = ();

    fn try_from(value: UntypedLambda) -> Result<Self, Self::Error> {
        let term = SimplyTypedLambdaTerm { expression: value };
        if Self::validate(term.as_expr()) {
            Ok(term)
        } else {
            Err(())
        }
    }
}

impl Term for SimplyTypedLambdaTerm {
    fn as_expr(&self) -> &UntypedLambda {
        &self.expression
    }

    fn as_expr_mut(&mut self) -> &mut UntypedLambda {
        &mut self.expression
    }

    fn validate(&self) -> bool {
        Self::validate(self.as_expr())
    }
}
