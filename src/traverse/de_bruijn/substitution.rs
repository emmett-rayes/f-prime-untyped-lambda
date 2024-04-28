use crate::expression::abstraction::{Abstraction, TypedAbstraction};
use crate::expression::variable::DeBruijnIndex;
use crate::expression::UntypedLambda;
use crate::traverse::de_bruijn::shift::DeBruijnShift;

pub struct DeBruijnSubstitution {
    replacement: UntypedLambda,
}

impl DeBruijnSubstitution {
    pub fn substitute(
        target: DeBruijnIndex,
        replacement: UntypedLambda,
        expression: &mut UntypedLambda,
    ) {
        let mut substitution = DeBruijnSubstitution { replacement };
        substitution.traverse(target, expression);
    }

    fn traverse(&mut self, target: DeBruijnIndex, expression: &mut UntypedLambda) {
        match expression {
            UntypedLambda::Variable(variable) => {
                if variable.index == target {
                    *expression = self.replacement.clone()
                }
            }
            UntypedLambda::Abstraction(box Abstraction { parameter: _, body })
            | UntypedLambda::TypedAbstraction(box TypedAbstraction {
                parameter: _, body, ..
            }) => {
                let replacement = self.replacement.clone();
                DeBruijnShift::shift(1, &mut self.replacement);
                self.traverse(target + 1, body);
                self.replacement = replacement;
            }
            UntypedLambda::Application(application) => {
                self.traverse(target, &mut application.applicator);
                self.traverse(target, &mut application.argument);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::expression::buffer::{Parsable, PositionedBuffer};
    use crate::expression::variable::Variable;
    use crate::traverse::de_bruijn::convert::DeBruijnConverter;
    use crate::traverse::pretty_print::ExpressionPrettyPrinter;

    use super::*;

    #[test]
    fn test_substitute() {
        let input = PositionedBuffer::new("(b (λx.λy.b))");
        let (mut expression, _) = UntypedLambda::parse(input).unwrap();
        DeBruijnConverter::convert(&mut expression);

        let replacement = UntypedLambda::from(Variable::from(String::from("a")));
        DeBruijnSubstitution::substitute(1, replacement, &mut expression);
        let pretty = ExpressionPrettyPrinter::format_named(&mut expression);
        assert_eq!(pretty, "a (λx. λy. a)");
    }

    #[test]
    fn test_substitute_2() {
        let input = PositionedBuffer::new("b (λx.b)");
        let (mut expression, _) = UntypedLambda::parse(input).unwrap();
        DeBruijnConverter::convert(&mut expression);

        let replacement_input = PositionedBuffer::new("a (λz.a)");
        let (mut replacement, _) = UntypedLambda::parse(replacement_input).unwrap();
        DeBruijnConverter::convert(&mut replacement);

        DeBruijnSubstitution::substitute(1, replacement, &mut expression);
        let pretty = ExpressionPrettyPrinter::format_nameless_locals(&mut expression);
        assert_eq!(pretty, "a (λ a) (λ a (λ a))");
    }
}
