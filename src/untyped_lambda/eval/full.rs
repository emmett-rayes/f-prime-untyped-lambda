use crate::expression::variable::Variable;
use crate::untyped_lambda::eval::shift::DeBruijnShift;
use crate::untyped_lambda::eval::substitution::DeBruijnSubstitution;
use crate::untyped_lambda::eval::BetaReduction;
use crate::untyped_lambda::term::term_helpers::try_replace_term;
use crate::untyped_lambda::term::{UntypedAbstraction, UntypedApplication, UntypedTerm};
use crate::visitor::Visitor;

pub struct FullBetaEvaluator;

impl BetaReduction<UntypedTerm> for FullBetaEvaluator {
    fn reduce_once(term: UntypedTerm) -> Result<UntypedTerm, UntypedTerm> {
        let mut visitor = FullBetaEvaluator;
        visitor.visit(term)
    }

    fn reduce(mut term: UntypedTerm) -> Result<UntypedTerm, UntypedTerm> {
        let mut visitor = FullBetaEvaluator;
        let mut count = 0;
        loop {
            count += 1;
            match visitor.visit(term) {
                Ok(t) => {
                    term = t;
                }
                Err(t) => return if count > 1 { Ok(t) } else { Err(t) },
            }
        }
    }
}

impl Visitor<Variable> for FullBetaEvaluator {
    type Result = Result<UntypedTerm, UntypedTerm>;

    fn visit(&mut self, variable: Variable) -> Self::Result {
        Err(UntypedTerm::from(variable))
    }
}

impl Visitor<UntypedAbstraction> for FullBetaEvaluator {
    type Result = Result<UntypedTerm, UntypedTerm>;

    fn visit(&mut self, mut abstraction: UntypedAbstraction) -> Self::Result {
        if try_replace_term(&mut abstraction.body, |term| self.visit(term)) {
            Ok(UntypedTerm::from(abstraction))
        } else {
            Err(UntypedTerm::from(abstraction))
        }
    }
}

impl Visitor<UntypedApplication> for FullBetaEvaluator {
    type Result = Result<UntypedTerm, UntypedTerm>;

    fn visit(&mut self, mut application: UntypedApplication) -> Self::Result {
        if try_replace_term(&mut application.applicator, |term| self.visit(term)) {
            return Ok(UntypedTerm::from(application));
        }

        if try_replace_term(&mut application.argument, |term| self.visit(term)) {
            return Ok(UntypedTerm::from(application));
        }

        if let UntypedTerm::Abstraction(applicator) = application.applicator {
            let argument_shifted = DeBruijnShift::shift(1, application.argument);
            let target = 1;
            let substituted =
                DeBruijnSubstitution::substitute(target, argument_shifted, applicator.body);
            Ok(DeBruijnShift::shift(-1, substituted))
        } else {
            Err(UntypedTerm::from(application))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::buffer::PositionedBuffer;
    use crate::expression::Expression;
    use crate::untyped_lambda::term::de_bruijn::DeBruijnConverter;
    use crate::untyped_lambda::term::pretty_print::UntypedPrettyPrinter;

    #[test]
    fn test_call_by_value() {
        let input = PositionedBuffer::new("(λx. x) (λx. y)");
        dbg!(&input.buffer);
        let output = UntypedTerm::parse(input);
        let term = DeBruijnConverter::convert(output.unwrap().0);
        let value = FullBetaEvaluator::reduce(term);
        let result = UntypedPrettyPrinter::format(value.unwrap());
        assert_eq!(result, "λx. y");
    }
}
