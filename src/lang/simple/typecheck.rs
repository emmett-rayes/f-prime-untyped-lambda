use std::collections::HashMap;

use crate::expression::UntypedLambda;
use crate::expression::variable::Variable;
use crate::term::simple::SimplyTypedLambdaTerm;
use crate::term::Term;
use crate::typecheck::TypeChecker;

pub struct SimpleType {
    pub expression: UntypedLambda,
}

impl SimpleType {
    fn validate(expression: &UntypedLambda) -> bool {
        match expression {
            UntypedLambda::Variable(variable) => variable.index != 0,
            UntypedLambda::Application(application) => {
                Self::validate(&application.applicator) && Self::validate(&application.argument)
            }
            _ => false,
        }
    }
}

impl TryFrom<UntypedLambda> for SimpleType {
    type Error = ();

    fn try_from(value: UntypedLambda) -> Result<Self, Self::Error> {
        let simple_type = SimpleType { expression: value };
        if Self::validate(simple_type.as_expr()) {
            Ok(simple_type)
        } else {
            Err(())
        }
    }
}

impl Term for SimpleType {
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

#[derive(Default)]
struct SimplyTypedLambdaTypeChecker<'a> {
    type_context: HashMap<&'a Variable, &'a UntypedLambda>,
}

impl<'a> SimplyTypedLambdaTypeChecker<'a> {
    fn check(expression: &UntypedLambda, term_type: &UntypedLambda) -> bool {
        let mut checker = Self::default();
        checker.traverse(expression, term_type)
    }

    fn traverse(&mut self, expression: &'a UntypedLambda, term_type: &UntypedLambda) -> bool {
        match expression {
            UntypedLambda::Variable(variable) => {
                if !self.type_context.contains_key(&variable) {
                    false
                } else {
                    self.type_context[&variable] == term_type
                }
            }
            UntypedLambda::TypedAbstraction(abstraction) => {
                self.type_context
                    .insert(&abstraction.parameter, &abstraction.parameter_type);
                let result = self.traverse(&abstraction.body, term_type);
                self.type_context.remove(&abstraction.parameter);
                result
            }
            UntypedLambda::Application(application) => {
                self.traverse(&application.applicator, term_type)
                    && self.traverse(&application.argument, term_type)
            }
            _ => unimplemented!(),
        }
    }
}

impl<'a> TypeChecker<SimplyTypedLambdaTerm, SimpleType> for SimplyTypedLambdaTypeChecker<'a> {
    fn check(term: &SimplyTypedLambdaTerm, term_type: &SimpleType) -> bool {
        Self::check(term.as_expr(), term_type.as_expr())
    }
}
