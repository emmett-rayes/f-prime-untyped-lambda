use crate::expression::abstraction::{Abstraction, TypedAbstraction};
use crate::expression::variable::DeBruijnIndex;
use crate::expression::Expression;
use std::ops::Deref;

pub struct DeBruijnShift {
    place: i64,
}

impl DeBruijnShift {
    pub fn shift(place: i64, expression: &mut Expression) {
        let mut shifter = Self { place };
        shifter.traverse(1, expression)
    }

    fn traverse(&mut self, cutoff: DeBruijnIndex, expression: &mut Expression) {
        match expression {
            Expression::Variable(variable) => {
                if variable.index >= cutoff {
                    variable.index = variable.index.saturating_add_signed(self.place);
                }
            }
            Expression::Abstraction(box Abstraction { parameter, body })
            | Expression::TypedAbstraction(box TypedAbstraction {
                parameter, body, ..
            }) => {
                self.traverse(cutoff + 1, body);
            }
            Expression::Application(application) => {
                self.traverse(cutoff, &mut application.applicator);
                self.traverse(cutoff, &mut application.argument);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::expression::buffer::{Parsable, PositionedBuffer};
    use crate::traverse::de_bruijn::convert::DeBruijnConverter;
    use crate::traverse::pretty_print::ExpressionPrettyPrinter;

    use super::*;

    #[test]
    fn test_shift() {
        let input = PositionedBuffer::new("(λx.λy. x (y w))");
        let (mut expression, _) = Expression::parse(input).unwrap();
        DeBruijnConverter::convert(&mut expression);
        DeBruijnShift::shift(2, &mut expression);
        let pretty = ExpressionPrettyPrinter::format_indexed(&mut expression);
        assert_eq!(pretty, "λ λ 2 (1 5)");
    }

    #[test]
    fn test_shift_nested() {
        let input = PositionedBuffer::new("(λx. x w (λy. y x w))");
        let (mut expression, _) = Expression::parse(input).unwrap();
        DeBruijnConverter::convert(&mut expression);
        dbg!(ExpressionPrettyPrinter::format_indexed(&mut expression));
        DeBruijnShift::shift(2, &mut expression);
        let pretty = ExpressionPrettyPrinter::format_indexed(&mut expression);
        assert_eq!(pretty, "λ 1 4 (λ 1 2 5)");
    }
}
