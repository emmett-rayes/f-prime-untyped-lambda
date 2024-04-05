use crate::untyped_lambda::eval::by_value::CallByValueEvaluator;
use crate::untyped_lambda::eval::BetaReduction;
use crate::untyped_lambda::term::UntypedTerm;
use crate::visitor::Visitor;

pub struct FullBetaEvaluator;

impl BetaReduction<UntypedTerm> for FullBetaEvaluator {
    fn reduce_once(term: &mut UntypedTerm) -> bool {
        let mut visitor = CallByValueEvaluator::new(true);
        visitor.visit(term)
    }

    fn reduce(term: &mut UntypedTerm) -> bool {
        let mut visitor = CallByValueEvaluator::new(true);
        let mut count = 0;
        while visitor.visit(term) {
            count += 1;
        }
        count >= 1
    }
}

#[cfg(test)]
mod tests {
    use crate::expression::buffer::PositionedBuffer;
    use crate::expression::Expression;
    use crate::untyped_lambda::term::de_bruijn::DeBruijnConverter;
    use crate::untyped_lambda::term::pretty_print::UntypedPrettyPrinter;

    use super::*;

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
