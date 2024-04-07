use crate::eval::untyped::by_value::CallByValueEvaluator;
use crate::eval::BetaReduction;
use crate::term::untyped::UntypedLambdaTerm;
use crate::term::Term;

pub struct FullBetaEvaluator;

impl BetaReduction<UntypedLambdaTerm> for FullBetaEvaluator {
    fn reduce_once(term: &mut UntypedLambdaTerm) -> bool {
        CallByValueEvaluator::normalize(term.as_expr())
    }
}

#[cfg(test)]
mod tests {
    use crate::eval::BetaReduction;
    use crate::expression::buffer::{Parsable, PositionedBuffer};
    use crate::expression::Expression;
    use crate::term::Term;
    use crate::traverse::de_bruijn::convert::DeBruijnConverter;
    use crate::traverse::pretty_print::ExpressionPrettyPrinter;

    use super::*;

    #[test]
    fn test_full_beta() {
        let input = PositionedBuffer::new("(λn.λs.λz.s (n s z)) (λs.λz.z)");
        let output = Expression::parse(input);
        let mut expression = output.unwrap().0;
        DeBruijnConverter::convert(&mut expression);
        let mut term = UntypedLambdaTerm::new(expression);
        let result = FullBetaEvaluator::reduce(&mut term);
        assert!(result);
        let format = ExpressionPrettyPrinter::format_named(term.as_expr());
        assert_eq!(format, "λs. λz. s z");
    }
}
