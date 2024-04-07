use std::collections::HashSet;

use crate::expression::Expression;

#[derive(Default)]
pub struct ExpressionPrettyPrinter {
    de_bruijn: bool,
    free_variables: HashSet<String>,
}

impl ExpressionPrettyPrinter {
    pub fn format(expression: &mut Expression) -> String {
        Self::format_inner(expression, false)
    }

    pub fn format_de_bruijn(expression: &mut Expression) -> String {
        Self::format_inner(expression, true)
    }

    fn format_inner(expression: &mut Expression, de_bruijn: bool) -> String {
        let expression_is_abstraction = matches!(expression, Expression::Abstraction(_));
        let mut printer = ExpressionPrettyPrinter {
            de_bruijn,
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
                if variable.index == 0 || !self.de_bruijn {
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
                if !self.de_bruijn || self.free_variables.remove(&abstraction.parameter.symbol) {
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
    fn test_pretty_print_de_bruin() {
        let input = PositionedBuffer::new("(λx.λy.λz. w x y z)");
        let output = Expression::parse(input);
        let mut expression = output.unwrap().0;
        DeBruijnConverter::convert(&mut expression);
        assert_eq!(
            ExpressionPrettyPrinter::format_de_bruijn(&mut expression),
            "λ λ λ w 3 2 1"
        );
    }

    #[test]
    fn test_pretty_print_symbolic() {
        let input = PositionedBuffer::new("(λx.λy.λz. w x y z)");
        let output = Expression::parse(input);
        let mut expression = output.unwrap().0;
        assert_eq!(
            ExpressionPrettyPrinter::format_de_bruijn(&mut expression),
            "λx. λy. λz. w x y z"
        );
    }

    #[test]
    fn test_associativity() {
        let input = PositionedBuffer::new("λx y z.x z (y z)");
        let output = Expression::parse(input);
        let mut expression = output.unwrap().0;
        DeBruijnConverter::convert(&mut expression);
        assert_eq!(
            ExpressionPrettyPrinter::format_de_bruijn(&mut expression),
            "λ λ λ 3 1 (2 1)"
        );
    }
}
