use crate::expression::variable::Variable;
use crate::untyped_lambda::eval::shift::DeBruijnShift;
use crate::untyped_lambda::eval::substitution::DeBruijnSubstitution;
use crate::untyped_lambda::eval::BetaReduction;
use crate::untyped_lambda::term::term_helpers::try_replace_term;
use crate::untyped_lambda::term::{UntypedAbstraction, UntypedApplication, UntypedTerm};
use crate::visitor::Visitor;

pub struct CallByValueEvaluator;

impl BetaReduction<UntypedTerm> for CallByValueEvaluator {
    fn reduce_once(term: UntypedTerm) -> Result<UntypedTerm, UntypedTerm> {
        let mut visitor = CallByValueEvaluator;
        visitor.visit(term)
    }

    fn reduce(mut term: UntypedTerm) -> Result<UntypedTerm, UntypedTerm> {
        let mut visitor = CallByValueEvaluator;
        while !term.is_value() {
            term = visitor.visit(term)?
        }
        Ok(term)
    }
}

impl Visitor<Variable> for CallByValueEvaluator {
    type Result = Result<UntypedTerm, UntypedTerm>;

    fn visit(&mut self, variable: Variable) -> Self::Result {
        Err(UntypedTerm::from(variable))
    }
}

impl Visitor<UntypedAbstraction> for CallByValueEvaluator {
    type Result = Result<UntypedTerm, UntypedTerm>;

    fn visit(&mut self, abstraction: UntypedAbstraction) -> Self::Result {
        Err(UntypedTerm::from(abstraction))
    }
}

impl Visitor<UntypedApplication> for CallByValueEvaluator {
    type Result = Result<UntypedTerm, UntypedTerm>;

    fn visit(&mut self, mut application: UntypedApplication) -> Self::Result {
        if !application.applicator.is_value() {
            try_replace_term(&mut application.applicator, |term| self.visit(term))?;
            Ok(UntypedTerm::from(application))
        } else if !application.argument.is_value() {
            try_replace_term(&mut application.argument, |term| self.visit(term))?;
            Ok(UntypedTerm::from(application))
        } else {
            let argument_shifted = DeBruijnShift::shift(1, application.argument);
            if let UntypedTerm::Abstraction(applicator) = application.applicator {
                let target = 1;
                let substituted =
                    DeBruijnSubstitution::substitute(target, argument_shifted, applicator.body);
                Ok(DeBruijnShift::shift(-1, substituted))
            } else {
                panic!("Applicator has to be an abstraction for call by value evaluation.")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::Expression;
    use crate::untyped_lambda::term::de_bruijn::DeBruijnConverter;
    use crate::untyped_lambda::term::pretty_print::UntypedPrettyPrinter;
    use f_prime_parser::PositionedBuffer;

    #[test]
    fn test_parser() {
        let input = PositionedBuffer::new("(λx. x) (λx. y)");
        dbg!(&input.buffer);
        let output = UntypedTerm::parse(input);
        let term = DeBruijnConverter::convert(output.unwrap().0);
        let value = CallByValueEvaluator::reduce(term);
        let result = UntypedPrettyPrinter::format(value.unwrap());
        assert_eq!(result, "λ y");
    }
}
