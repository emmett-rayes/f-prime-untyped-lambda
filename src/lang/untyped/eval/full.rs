use crate::eval::untyped::by_value::CallByValueEvaluator;
use crate::eval::untyped::TracingBetaReduction;
use crate::term::untyped::pretty_print::UntypedPrettyPrinter;
use crate::term::untyped::UntypedTerm;
use crate::visitor::Visitor;

pub struct FullBetaEvaluator;

impl TracingBetaReduction<UntypedTerm> for FullBetaEvaluator {
    fn trace_once(term: &mut UntypedTerm) -> Option<String> {
        let mut visitor = CallByValueEvaluator::new(true);
        if visitor.visit((), term) {
            Some(UntypedPrettyPrinter::format(term))
        } else {
            None
        }
    }

    fn trace(term: &mut UntypedTerm) -> Vec<String> {
        let mut visitor = CallByValueEvaluator::new(true);
        let mut trace = Vec::new();
        while visitor.visit((), term) {
            trace.push(UntypedPrettyPrinter::format(term));
        }
        trace
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eval::untyped::BetaReduction;
    use crate::expr::buffer::PositionedBuffer;
    use crate::expr::Expression;
    use crate::term::untyped::de_bruijn::DeBruijnConverter;

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
