use crate::eval::BetaReduction;
use crate::expression::variable::Variable;
use crate::expression::Expression;
use crate::term::untyped::UntypedLambdaTerm;
use crate::term::Term;
use crate::traverse::de_bruijn::shift::DeBruijnShift;
use crate::traverse::de_bruijn::substitution::DeBruijnSubstitution;

#[derive(Default)]
pub struct CallByValueEvaluator {
    normalize: bool,
}

impl CallByValueEvaluator {
    pub fn evaluate(expression: &mut Expression) -> bool {
        let mut evaluator = Self::default();
        evaluator.traverse(expression)
    }

    pub fn normalize(expression: &mut Expression) -> bool {
        let mut evaluator = Self { normalize: true };
        evaluator.traverse(expression)
    }

    fn traverse(&mut self, expression: &mut Expression) -> bool {
        match expression {
            Expression::Variable(_) => false,
            Expression::Abstraction(abstraction) => {
                self.normalize && self.traverse(&mut abstraction.body)
            }
            Expression::Application(application) => {
                if (self.normalize || !application.applicator.is_value())
                    && self.traverse(&mut application.applicator)
                {
                    return true;
                }
                if (self.normalize || !application.argument.is_value())
                    && self.traverse(&mut application.argument)
                {
                    return true;
                }
                if !matches!(application.applicator, Expression::Abstraction(_)) {
                    return false;
                }
                let dummy = Expression::from(Variable::from(String::new()));
                let application = std::mem::replace(expression, dummy);
                if let Expression::Application(mut application) = application {
                    if self.traverse(&mut application.applicator) {
                        return true;
                    }
                    if self.traverse(&mut application.argument) {
                        return true;
                    }
                    if let Expression::Abstraction(mut applicator) = application.applicator {
                        let target = 1;
                        DeBruijnShift::shift(1, &mut application.argument);
                        DeBruijnSubstitution::substitute(
                            target,
                            application.argument,
                            &mut applicator.body,
                        );
                        DeBruijnShift::shift(-1, &mut applicator.body);
                        *expression = applicator.body;
                        return true;
                    } else {
                        unreachable!()
                    }
                } else {
                    unreachable!()
                }
            }
            _ => unimplemented!(),
        }
    }
}

impl BetaReduction<UntypedLambdaTerm> for CallByValueEvaluator {
    fn reduce_once(term: &mut UntypedLambdaTerm) -> bool {
        Self::evaluate(term.as_expr())
    }
}
