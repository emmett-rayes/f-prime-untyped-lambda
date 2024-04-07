use std::collections::HashMap;

use crate::expression::abstraction::{Abstraction, TypedAbstraction};
use crate::expression::symbol::Symbol;
use crate::expression::variable::DeBruijnIndex;
use crate::expression::Expression;

#[derive(Default)]
pub struct DeBruijnConverter {
    variable_context: HashMap<Symbol, Vec<i64>>,
    free_variables: u64,
}

impl DeBruijnConverter {
    pub fn convert(expression: &mut Expression) {
        let mut converter = DeBruijnConverter::default();
        converter.traverse(expression, 0);
    }

    fn traverse(&mut self, expression: &mut Expression, current_scope: DeBruijnIndex) {
        match expression {
            Expression::Variable(variable) => {
                let binding_scope = if let Some(&scope) = self
                    .variable_context
                    .get(&variable.symbol)
                    .map(|scopes| scopes.last().unwrap())
                {
                    scope
                } else {
                    self.free_variables += 1;
                    let scope = -(self.free_variables as i64);
                    self.variable_context
                        .entry(variable.symbol.clone())
                        .or_default()
                        .push(scope);
                    scope
                };
                variable.index = (current_scope as i64 - binding_scope) as DeBruijnIndex;
            }
            Expression::Abstraction(box Abstraction { parameter, body })
            | Expression::TypedAbstraction(box TypedAbstraction {
                parameter, body, ..
            }) => {
                self.variable_context
                    .entry(parameter.symbol.clone())
                    .or_default()
                    .push(current_scope as i64);
                self.traverse(body, current_scope + 1);
                self.variable_context
                    .get_mut(&parameter.symbol.clone())
                    .unwrap()
                    .pop();
            }
            Expression::Application(application) => {
                self.traverse(&mut application.applicator, current_scope);
                self.traverse(&mut application.argument, current_scope);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::expression::buffer::{Parsable, PositionedBuffer};
    use crate::traverse::pretty_print::ExpressionPrettyPrinter;

    use super::*;

    #[test]
    fn test_free_variables() {
        let input = PositionedBuffer::new("a b c");
        let (mut expression, _) = Expression::parse(input).unwrap();
        DeBruijnConverter::convert(&mut expression);
        let pretty = ExpressionPrettyPrinter::format_indexed(&mut expression);
        assert_eq!(pretty, "1 2 3");
    }

    #[test]
    fn test_free_variables_nested() {
        let input = PositionedBuffer::new("b (λx.λy.b)");
        let (mut expression, _) = Expression::parse(input).unwrap();
        DeBruijnConverter::convert(&mut expression);
        let pretty = ExpressionPrettyPrinter::format_indexed(&mut expression);
        assert_eq!(pretty, "1 (λ λ 3)");
    }

    #[test]
    fn test_scopes() {
        let input = PositionedBuffer::new("(λx.λy.λz. w x y z)");
        let (mut expression, _) = Expression::parse(input).unwrap();
        DeBruijnConverter::convert(&mut expression);
        let pretty = ExpressionPrettyPrinter::format_indexed(&mut expression);
        assert_eq!(pretty, "λ λ λ 4 3 2 1");
    }

    #[test]
    fn test_scopes_nested() {
        let input = PositionedBuffer::new("(λw. (λx. w x y) (λx. x))");
        let (mut expression, _) = Expression::parse(input).unwrap();
        DeBruijnConverter::convert(&mut expression);
        let pretty = ExpressionPrettyPrinter::format_indexed(&mut expression);
        assert_eq!(pretty, "λ (λ 2 1 3) (λ 1)");
    }
}
