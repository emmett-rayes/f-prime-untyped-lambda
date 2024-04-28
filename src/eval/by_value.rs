use crate::eval::BetaReduction;
use crate::expression::abstraction::{Abstraction, TypedAbstraction};
use crate::expression::UntypedLambda;
use crate::expression::variable::Variable;
use crate::term::Term;
use crate::traverse::de_bruijn::shift::DeBruijnShift;
use crate::traverse::de_bruijn::substitution::DeBruijnSubstitution;

#[derive(Default)]
pub struct CallByValueEvaluator {
    normalize: bool,
}

impl CallByValueEvaluator {
    fn evaluate(expression: &mut UntypedLambda) -> bool {
        let mut evaluator = Self::default();
        evaluator.traverse(expression)
    }

    fn normalize(expression: &mut UntypedLambda) -> bool {
        let mut evaluator = Self { normalize: true };
        evaluator.traverse(expression)
    }

    fn traverse(&mut self, expression: &mut UntypedLambda) -> bool {
        match expression {
            UntypedLambda::Variable(_) => false,
            UntypedLambda::Abstraction(box Abstraction { parameter: _, body })
            | UntypedLambda::TypedAbstraction(box TypedAbstraction {
                parameter: _, body, ..
            }) => self.normalize && self.traverse(body),
            UntypedLambda::Application(application) => {
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
                if !matches!(
                    application.applicator,
                    UntypedLambda::Abstraction(_) | UntypedLambda::TypedAbstraction(_)
                ) {
                    return false;
                }
                let dummy = UntypedLambda::from(Variable::from(String::new()));
                let application = std::mem::replace(expression, dummy);
                if let UntypedLambda::Application(mut application) = application {
                    if self.traverse(&mut application.applicator) {
                        return true;
                    }
                    if self.traverse(&mut application.argument) {
                        return true;
                    }
                    if let UntypedLambda::Abstraction(box Abstraction {
                        parameter: _,
                        mut body,
                    })
                    | UntypedLambda::TypedAbstraction(box TypedAbstraction {
                        parameter: _,
                        mut body,
                        ..
                    }) = application.applicator
                    {
                        let target = 1;
                        DeBruijnShift::shift(1, &mut application.argument);
                        DeBruijnSubstitution::substitute(target, application.argument, &mut body);
                        DeBruijnShift::shift(-1, &mut body);
                        *expression = body;
                        return true;
                    } else {
                        unreachable!()
                    }
                } else {
                    unreachable!()
                }
            }
        }
    }
}

impl<T> BetaReduction<T> for CallByValueEvaluator
where
    T: Term,
{
    fn reduce_once(term: &mut T) -> bool {
        Self::evaluate(term.as_expr_mut())
    }
}

pub struct FullBetaEvaluator;

impl<T> BetaReduction<T> for FullBetaEvaluator
where
    T: Term,
{
    fn reduce_once(term: &mut T) -> bool {
        CallByValueEvaluator::normalize(term.as_expr_mut())
    }
}

#[cfg(test)]
mod tests {
    use crate::expression::buffer::{Parsable, PositionedBuffer};
    use crate::term::untyped::UntypedLambdaTerm;
    use crate::traverse::de_bruijn::convert::DeBruijnConverter;
    use crate::traverse::pretty_print::ExpressionPrettyPrinter;

    use super::*;

    #[test]
    fn test_full_beta() {
        let input = PositionedBuffer::new("(λn.λs.λz.s (n s z)) (λs.λz.z)");
        let output = UntypedLambda::parse(input);
        let mut expression = output.unwrap().0;
        DeBruijnConverter::convert(&mut expression);
        let mut term = UntypedLambdaTerm::try_from(expression).unwrap();
        let result = FullBetaEvaluator::reduce(&mut term);
        assert!(result);
        let format = ExpressionPrettyPrinter::format_named(term.as_expr_mut());
        assert_eq!(format, "λs. λz. s z");
    }
}
