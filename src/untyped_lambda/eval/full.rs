use crate::untyped_lambda::eval::by_value::CallByValueEvaluator;
use crate::untyped_lambda::eval::TracingBetaReduction;
use crate::untyped_lambda::term::pretty_print::UntypedPrettyPrinter;
use crate::untyped_lambda::term::UntypedTerm;
use crate::visitor::Visitor;

pub struct FullBetaEvaluator;

impl TracingBetaReduction<UntypedTerm> for FullBetaEvaluator {
    fn trace_once(term: &mut UntypedTerm) -> Option<String> {
        let mut visitor = CallByValueEvaluator::new(true);
        if visitor.visit(term) {
            Some(UntypedPrettyPrinter::format(term))
        } else {
            None
        }
    }

    fn trace(term: &mut UntypedTerm) -> Vec<String> {
        let mut visitor = CallByValueEvaluator::new(true);
        let mut trace = Vec::new();
        while visitor.visit(term) {
            trace.push(UntypedPrettyPrinter::format(term));
        }
        trace
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
