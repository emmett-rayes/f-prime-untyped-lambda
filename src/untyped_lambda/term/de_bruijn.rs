use crate::expression::variable::{Variable, VariableIndex};
use crate::untyped_lambda::term::term_helpers::replace_term;
use crate::untyped_lambda::term::{UntypedAbstraction, UntypedApplication, UntypedTerm};
use crate::visitor::Visitor;
use std::collections::{HashMap, LinkedList};

#[derive(Default)]
pub struct DeBruijnConverter {
    current_scope: VariableIndex,
    variable_map: HashMap<String, LinkedList<VariableIndex>>,
}

impl DeBruijnConverter {
    pub fn convert(term: UntypedTerm) -> UntypedTerm {
        let mut visitor = DeBruijnConverter::default();
        visitor.visit(term)
    }
}

impl Visitor<Variable> for DeBruijnConverter {
    type Result = UntypedTerm;

    fn visit(&mut self, mut variable: Variable) -> Self::Result {
        if let Some(scopes) = self.variable_map.get(&variable.symbol) {
            let binding_scope = *scopes.front().unwrap();
            variable.index = self.current_scope - binding_scope + 1;
        }
        UntypedTerm::from(variable)
    }
}

impl Visitor<UntypedAbstraction> for DeBruijnConverter {
    type Result = UntypedTerm;

    fn visit(&mut self, mut abstraction: UntypedAbstraction) -> Self::Result {
        self.current_scope += 1;
        self.variable_map
            .entry(abstraction.parameter.symbol.clone())
            .or_default()
            .push_front(self.current_scope);
        replace_term(&mut abstraction.body, |term| self.visit(term));
        self.current_scope -= 1;
        UntypedTerm::from(abstraction)
    }
}

impl Visitor<UntypedApplication> for DeBruijnConverter {
    type Result = UntypedTerm;

    fn visit(&mut self, mut application: UntypedApplication) -> Self::Result {
        replace_term(&mut application.applicator, |term| self.visit(term));
        replace_term(&mut application.argument, |term| self.visit(term));
        UntypedTerm::from(application)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::buffer::PositionedBuffer;
    use crate::expression::Expression;

    #[test]
    fn test_de_bruijn() {
        let input = PositionedBuffer::new("(λx.λy.λz. w x y z)");
        let output = UntypedTerm::parse(input);
        let term = output.unwrap().0;
        let should = UntypedTerm::from(UntypedAbstraction::new(
            Variable::new("x"),
            UntypedTerm::from(UntypedAbstraction::new(
                Variable::new("y"),
                UntypedTerm::from(UntypedAbstraction::new(
                    Variable::new("z"),
                    UntypedTerm::from(UntypedApplication::new(
                        UntypedTerm::from(UntypedApplication::new(
                            UntypedTerm::from(UntypedApplication::new(
                                UntypedTerm::from(Variable::new("w")),
                                UntypedTerm::from(Variable {
                                    symbol: String::from("x"),
                                    index: 3,
                                }),
                            )),
                            UntypedTerm::from(Variable {
                                symbol: String::from("y"),
                                index: 2,
                            }),
                        )),
                        UntypedTerm::from(Variable {
                            symbol: String::from("z"),
                            index: 1,
                        }),
                    )),
                )),
            )),
        ));
        assert_eq!(DeBruijnConverter::convert(term), should);
    }
}
