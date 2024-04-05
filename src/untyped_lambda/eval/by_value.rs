use crate::expression::variable::Variable;
use crate::untyped_lambda::eval::shift::DeBruijnShift;
use crate::untyped_lambda::eval::substitution::DeBruijnSubstitution;
use crate::untyped_lambda::eval::TracingBetaReduction;
use crate::untyped_lambda::term::pretty_print::UntypedPrettyPrinter;
use crate::untyped_lambda::term::{UntypedAbstraction, UntypedTerm};
use crate::visitor::Visitor;
use std::ops::DerefMut;

#[derive(Default)]
pub struct CallByValueEvaluator {
    normalize: bool,
}

impl CallByValueEvaluator {
    pub fn new(normalize: bool) -> Self {
        Self { normalize }
    }
}

impl TracingBetaReduction<UntypedTerm> for CallByValueEvaluator {
    fn trace_once(term: &mut UntypedTerm) -> Option<String> {
        let mut visitor = CallByValueEvaluator::default();
        if visitor.visit(term) {
            Some(UntypedPrettyPrinter::format(term))
        } else {
            None
        }
    }

    fn trace(term: &mut UntypedTerm) -> Vec<String> {
        let mut visitor = CallByValueEvaluator::default();
        let mut trace = Vec::new();
        while visitor.visit(term) {
            trace.push(UntypedPrettyPrinter::format(term));
        }
        trace
    }
}

impl Visitor<Variable> for CallByValueEvaluator {
    type Result = bool;

    fn visit(&mut self, _: &mut Variable) -> Self::Result {
        false
    }
}

impl Visitor<UntypedAbstraction> for CallByValueEvaluator {
    type Result = bool;

    fn visit(&mut self, abstraction: &mut UntypedAbstraction) -> Self::Result {
        self.normalize && self.visit(&mut abstraction.body)
    }
}

impl Visitor<UntypedTerm> for CallByValueEvaluator {
    type Result = bool;

    fn visit(&mut self, term: &mut UntypedTerm) -> Self::Result {
        match term {
            UntypedTerm::Variable(variable) => self.visit(variable),
            UntypedTerm::Abstraction(abstraction) => self.visit(abstraction.deref_mut()),
            UntypedTerm::Application(application) => {
                if (self.normalize || !application.applicator.is_value())
                    && self.visit(&mut application.applicator)
                {
                    return true;
                }
                if (self.normalize || !application.argument.is_value())
                    && self.visit(&mut application.argument)
                {
                    return true;
                }
                if !matches!(application.applicator, UntypedTerm::Abstraction(_)) {
                    return false;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::buffer::PositionedBuffer;
    use crate::expression::Expression;
    use crate::untyped_lambda::eval::BetaReduction;
    use crate::untyped_lambda::term::de_bruijn::DeBruijnConverter;
    use crate::untyped_lambda::term::pretty_print::UntypedPrettyPrinter;

    #[test]
    fn test_call_by_value() {
        let input = PositionedBuffer::new("(λx. x) (λx. y)");
        dbg!(&input.buffer);
        let output = UntypedTerm::parse(input);
        let mut term = output.unwrap().0;
        DeBruijnConverter::convert(&mut term);
        let result = CallByValueEvaluator::reduce(&mut term);
        assert!(result);
        let format = UntypedPrettyPrinter::format(&mut term);
        assert_eq!(format, "λx. y");
    }
}
