use crate::expression::variable::{Variable, VariableIndex};
use crate::untyped_lambda::term::{
    UntypedAbstraction, UntypedApplication, UntypedTerm, UntypedTermNonRewritingVisitor,
};
use crate::visitor::Visitor;
use std::collections::{HashMap, LinkedList};
use std::ops::DerefMut;

#[derive(Default)]
pub struct DeBruijnConverter {
    current_scope: VariableIndex,
    variable_map: HashMap<String, LinkedList<VariableIndex>>,
}

impl DeBruijnConverter {
    pub fn convert(term: &mut UntypedTerm) {
        let mut visitor = DeBruijnConverter::default();
        visitor.visit(term);
    }
}

impl UntypedTermNonRewritingVisitor for DeBruijnConverter {}

impl Visitor<Variable> for DeBruijnConverter {
    type Result = ();

    fn visit(&mut self, variable: &mut Variable) -> Self::Result {
        if let Some(scopes) = self.variable_map.get(&variable.symbol) {
            if let Some(binding_scope) = scopes.front() {
                variable.index = self.current_scope - binding_scope + 1;
            }
        }
    }
}

impl Visitor<UntypedAbstraction> for DeBruijnConverter {
    type Result = ();

    fn visit(&mut self, abstraction: &mut UntypedAbstraction) -> Self::Result {
        self.current_scope += 1;
        self.variable_map
            .entry(abstraction.parameter.symbol.clone())
            .or_default()
            .push_front(self.current_scope);
        self.visit(&mut abstraction.body);
        self.variable_map
            .get_mut(&abstraction.parameter.symbol.clone())
            .unwrap()
            .pop_front();
        self.current_scope -= 1;
    }
}

impl Visitor<UntypedApplication> for DeBruijnConverter {
    type Result = ();

    fn visit(&mut self, application: &mut UntypedApplication) -> Self::Result {
        self.visit(&mut application.applicator);
        self.visit(&mut application.argument);
    }
}

#[cfg(test)]
mod tests {
    use crate::expression::buffer::PositionedBuffer;
    use crate::expression::Expression;

    use super::*;

    #[test]
    fn test_de_bruijn() {
        let input = PositionedBuffer::new("(λx.λy.λz. w x y z)");
        let output = UntypedTerm::parse(input);
        let mut term = output.unwrap().0;
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
        DeBruijnConverter::convert(&mut term);
        assert_eq!(term, should);
    }
}
