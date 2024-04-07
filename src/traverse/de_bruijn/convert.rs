use std::collections::{HashMap, LinkedList};

use crate::expression::variable::DeBruijnIndex;
use crate::expression::Expression;

#[derive(Default)]
pub struct DeBruijnConverter {
    variable_map: HashMap<String, LinkedList<DeBruijnIndex>>,
}

impl DeBruijnConverter {
    pub fn convert(expression: &mut Expression) {
        let mut converter = DeBruijnConverter::default();
        converter.traverse(expression, 0)
    }

    fn traverse(&mut self, expression: &mut Expression, mut current_scope: u64) {
        match expression {
            Expression::Variable(variable) => {
                if let Some(scopes) = self.variable_map.get(&variable.symbol) {
                    if let Some(binding_scope) = scopes.front() {
                        variable.index = current_scope - binding_scope + 1;
                    }
                }
            }
            Expression::Abstraction(abstraction) => {
                current_scope += 1;
                self.variable_map
                    .entry(abstraction.parameter.symbol.clone())
                    .or_default()
                    .push_front(current_scope);
                self.traverse(&mut abstraction.body, current_scope);
                self.variable_map
                    .get_mut(&abstraction.parameter.symbol.clone())
                    .unwrap()
                    .pop_front();
            }
            Expression::Application(application) => {
                self.traverse(&mut application.applicator, current_scope);
                self.traverse(&mut application.argument, current_scope);
            }
        }
    }
}

mod tests {
    use crate::expression::buffer::{Parsable, PositionedBuffer};
    use crate::traverse::pretty_print::ExpressionPrettyPrinter;

    use super::*;

    #[test]
    fn test_free_variable() {
        let input = PositionedBuffer::new("a");
        let (mut expression, _) = Expression::parse(input).unwrap();
        DeBruijnConverter::convert(&mut expression);
        let pretty = ExpressionPrettyPrinter::format(&mut expression);
        assert_eq!(pretty, "a");
    }

    #[test]
    fn test_scopes() {
        let input = PositionedBuffer::new("(λx.λy.λz. w x y z)");
        let (mut expression, _) = Expression::parse(input).unwrap();
        DeBruijnConverter::convert(&mut expression);
        let pretty = ExpressionPrettyPrinter::format_indexed(&mut expression);
        assert_eq!(pretty, "λ λ λ w 3 2 1");
    }
}
