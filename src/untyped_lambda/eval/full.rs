use crate::expression::variable::Variable;
use crate::untyped_lambda::eval::shift::DeBruijnShift;
use crate::untyped_lambda::eval::substitution::DeBruijnSubstitution;
use crate::untyped_lambda::eval::BetaReduction;
use crate::untyped_lambda::term::{UntypedAbstraction, UntypedTerm};
use crate::visitor::Visitor;
use std::ops::DerefMut;

pub struct FullBetaEvaluator;

impl BetaReduction<UntypedTerm> for FullBetaEvaluator {
    fn reduce_once(term: &mut UntypedTerm) -> bool {
        let mut visitor = FullBetaEvaluator;
        visitor.visit(term)
    }

    fn reduce(term: &mut UntypedTerm) -> bool {
        let mut visitor = FullBetaEvaluator;
        let mut count = 0;
        while visitor.visit(term) {
            count += 1;
        }
        count >= 1
    }
}

impl Visitor<Variable> for FullBetaEvaluator {
    type Result = bool;

    fn visit(&mut self, _: &mut Variable) -> Self::Result {
        false
    }
}

impl Visitor<UntypedAbstraction> for FullBetaEvaluator {
    type Result = bool;

    fn visit(&mut self, abstraction: &mut UntypedAbstraction) -> Self::Result {
        self.visit(&mut abstraction.body)
    }
}

impl Visitor<UntypedTerm> for FullBetaEvaluator {
    type Result = bool;

    fn visit(&mut self, term: &mut UntypedTerm) -> Self::Result {
        match term {
            UntypedTerm::Variable(variable) => self.visit(variable),
            UntypedTerm::Abstraction(abstraction) => self.visit(abstraction.deref_mut()),
            _ => {
                if matches!(term, UntypedTerm::Application(_)) {
                    if let UntypedTerm::Application(application) = term {
                        if self.visit(&mut application.applicator) {
                            return true;
                        }
                        if self.visit(&mut application.argument) {
                            return true;
                        }
                        if !matches!(application.applicator, UntypedTerm::Abstraction(_)) {
                            return false;
                        }
                    }
                    let dummy = UntypedTerm::Variable(Variable::new(""));
                    let application = std::mem::replace(term, dummy);
                    if let UntypedTerm::Application(mut application) = application {
                        if self.visit(&mut application.applicator) {
                            return true;
                        }
                        if self.visit(&mut application.argument) {
                            return true;
                        }
                        if let UntypedTerm::Abstraction(mut applicator) = application.applicator {
                            let target = 1;
                            DeBruijnShift::shift(1, &mut application.argument);
                            DeBruijnSubstitution::substitute(
                                target,
                                application.argument,
                                &mut applicator.body,
                            );
                            DeBruijnShift::shift(-1, &mut applicator.body);
                            *term = applicator.body;
                            return true;
                        }
                    }
                }
                unreachable!()
            }
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
    fn test_full_beta() {
        let input = PositionedBuffer::new("(λn.λs.λz.s (n s z)) (λs.λz.z)");
        dbg!(&input.buffer);
        let output = UntypedTerm::parse(input);
        let mut term = output.unwrap().0;
        DeBruijnConverter::convert(&mut term);
        let result = FullBetaEvaluator::reduce(&mut term);
        assert!(result);
        let format = UntypedPrettyPrinter::format(&mut term);
        assert_eq!(format, "λs. λz. s z");
    }
}
