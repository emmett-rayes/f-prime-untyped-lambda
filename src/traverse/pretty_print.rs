use std::collections::HashSet;

use crate::expression::Expression;

#[derive(Default)]
pub struct ExpressionPrettyPrinter {
    indexed_variables: bool,
    free_variables: HashSet<String>,
}

impl ExpressionPrettyPrinter {
    pub fn format(expression: &mut Expression) -> String {
        Self::format_inner(expression, false)
    }

    pub fn format_indexed(expression: &mut Expression) -> String {
        Self::format_inner(expression, true)
    }

    fn format_inner(expression: &mut Expression, indexed_variables: bool) -> String {
        let expression_is_abstraction = matches!(expression, Expression::Abstraction(_));
        let mut printer = ExpressionPrettyPrinter {
            indexed_variables,
            ..Default::default()
        };
        let string = printer.traverse(expression);
        if expression_is_abstraction {
            string
                .strip_prefix('(')
                .and_then(|s| s.strip_suffix(')'))
                .map(|s| s.to_string())
                .unwrap_or(string)
        } else {
            string
        }
    }

    fn traverse(&mut self, expression: &mut Expression) -> String {
        match expression {
            Expression::Variable(variable) => {
                if variable.index == 0 || !self.indexed_variables {
                    self.free_variables.insert(variable.symbol.clone());
                    variable.symbol.clone()
                } else {
                    variable.index.to_string()
                }
            }
            Expression::Abstraction(abstraction) => {
                let body_is_abstraction = matches!(abstraction.body, Expression::Abstraction(_));
                let body = self.traverse(&mut abstraction.body);
                let body = if body_is_abstraction {
                    body.strip_prefix('(')
                        .and_then(|s| s.strip_suffix(')'))
                        .unwrap_or(body.as_str())
                } else {
                    body.as_str()
                };
                if !self.indexed_variables
                    || self.free_variables.remove(&abstraction.parameter.symbol)
                {
                    format!("(λ{}. {})", abstraction.parameter.symbol, body)
                } else {
                    format!("(λ {})", body)
                }
            }
            Expression::Application(application) => {
                let argument_is_application =
                    matches!(application.argument, Expression::Application(_));
                let applicator = self.traverse(&mut application.applicator);
                let argument = self.traverse(&mut application.argument);
                if argument_is_application {
                    format!("{} ({})", applicator, argument,)
                } else {
                    format!("{} {}", applicator, argument,)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::expression::buffer::{Parsable, PositionedBuffer};
    use crate::expression::Expression;
    use crate::traverse::de_bruijn::convert::DeBruijnConverter;

    use super::*;

    #[test]
    fn test_pretty_print() {
        let input = PositionedBuffer::new("(λx.λy.λz. w x y z)");
        let (mut expression, _) = Expression::parse(input).unwrap();
        DeBruijnConverter::convert(&mut expression);
        let pretty = ExpressionPrettyPrinter::format(&mut expression);
        assert_eq!(pretty, "λx. λy. λz. w x y z");
    }

    #[test]
    fn test_pretty_print_indexed() {
        let input = PositionedBuffer::new("(λx.λy.λz. w x y z)");
        let (mut expression, _) = Expression::parse(input).unwrap();
        DeBruijnConverter::convert(&mut expression);
        let pretty = ExpressionPrettyPrinter::format_indexed(&mut expression);
        assert_eq!(pretty, "λ λ λ w 3 2 1");
    }

    #[test]
    fn test_associativity() {
        let input = PositionedBuffer::new("λx y z.x z (y z)");
        let (mut expression, _) = Expression::parse(input).unwrap();
        DeBruijnConverter::convert(&mut expression);
        let pretty = ExpressionPrettyPrinter::format_indexed(&mut expression);
        assert_eq!(pretty, "λ λ λ 3 1 (2 1)");
    }
}
