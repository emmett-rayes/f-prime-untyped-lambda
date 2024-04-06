use crate::eval::untyped::shift::DeBruijnShift;
use crate::eval::untyped::substitution::DeBruijnSubstitution;
use crate::eval::TracingBetaReduction;
use crate::lang::expr::variable::Variable;
use crate::term::untyped::pretty_print::UntypedPrettyPrinter;
use crate::term::untyped::{UntypedAbstraction, UntypedTerm};
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
        if visitor.visit((), term) {
            Some(UntypedPrettyPrinter::format(term))
        } else {
            None
        }
    }

    fn trace(term: &mut UntypedTerm) -> Vec<String> {
        let mut visitor = CallByValueEvaluator::default();
        let mut trace = Vec::new();
        while visitor.visit((), term) {
            trace.push(UntypedPrettyPrinter::format(term));
        }
        trace
    }
}

impl Visitor<Variable> for CallByValueEvaluator {
    type Result = bool;
    type Context = ();

    fn visit(&mut self, _: Self::Context, _: &mut Variable) -> Self::Result {
        false
    }
}

impl Visitor<UntypedAbstraction> for CallByValueEvaluator {
    type Result = bool;
    type Context = ();

    fn visit(
        &mut self,
        empty_context: Self::Context,
        abstraction: &mut UntypedAbstraction,
    ) -> Self::Result {
        self.normalize && self.visit(empty_context, &mut abstraction.body)
    }
}

impl Visitor<UntypedTerm> for CallByValueEvaluator {
    type Result = bool;
    type Context = ();

    fn visit(&mut self, empty_context: Self::Context, term: &mut UntypedTerm) -> Self::Result {
        match term {
            UntypedTerm::Variable(variable) => self.visit(empty_context, variable),
            UntypedTerm::Abstraction(abstraction) => {
                self.visit(empty_context, abstraction.deref_mut())
            }
            UntypedTerm::Application(application) => {
                if (self.normalize || !application.applicator.is_value())
                    && self.visit(empty_context, &mut application.applicator)
                {
                    return true;
                }
                if (self.normalize || !application.argument.is_value())
                    && self.visit(empty_context, &mut application.argument)
                {
                    return true;
                }
                if !matches!(application.applicator, UntypedTerm::Abstraction(_)) {
                    return false;
                }
                let dummy = UntypedTerm::Variable(Variable::new(""));
                let application = std::mem::replace(term, dummy);
                if let UntypedTerm::Application(mut application) = application {
                    if self.visit(empty_context, &mut application.applicator) {
                        return true;
                    }
                    if self.visit(empty_context, &mut application.argument) {
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
    use crate::eval::BetaReduction;
    use crate::expr::buffer::PositionedBuffer;
    use crate::expr::Expression;
    use crate::term::untyped::de_bruijn::DeBruijnConverter;

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
