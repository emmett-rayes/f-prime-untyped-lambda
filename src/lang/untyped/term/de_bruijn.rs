use crate::lang::expr::variable::{Variable, VariableIndex};
use crate::term::untyped::{
    StructurePreservingUntypedTermVisitor, UntypedAbstraction, UntypedApplication, UntypedTerm,
};
use crate::visitor::Visitor;
use std::collections::{HashMap, LinkedList};

#[derive(Default)]
pub struct DeBruijnConverter {
    variable_map: HashMap<String, LinkedList<VariableIndex>>,
}

impl DeBruijnConverter {
    pub fn convert(term: &mut UntypedTerm) {
        let mut visitor = DeBruijnConverter::default();
        visitor.visit(0, term);
    }
}

impl StructurePreservingUntypedTermVisitor for DeBruijnConverter {}

impl Visitor<Variable> for DeBruijnConverter {
    type Result = ();
    type Context = VariableIndex;

    fn visit(&mut self, current_scope: Self::Context, variable: &mut Variable) -> Self::Result {
        if let Some(scopes) = self.variable_map.get(&variable.symbol) {
            if let Some(binding_scope) = scopes.front() {
                variable.index = current_scope - binding_scope + 1;
            }
        }
    }
}

impl Visitor<UntypedAbstraction> for DeBruijnConverter {
    type Result = ();
    type Context = VariableIndex;

    fn visit(
        &mut self,
        mut current_scope: Self::Context,
        abstraction: &mut UntypedAbstraction,
    ) -> Self::Result {
        current_scope += 1;
        self.variable_map
            .entry(abstraction.parameter.symbol.clone())
            .or_default()
            .push_front(current_scope);
        self.visit(current_scope, &mut abstraction.body);
        self.variable_map
            .get_mut(&abstraction.parameter.symbol.clone())
            .unwrap()
            .pop_front();
    }
}

impl Visitor<UntypedApplication> for DeBruijnConverter {
    type Result = ();
    type Context = VariableIndex;

    fn visit(
        &mut self,
        current_scope: Self::Context,
        application: &mut UntypedApplication,
    ) -> Self::Result {
        self.visit(current_scope, &mut application.applicator);
        self.visit(current_scope, &mut application.argument);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::buffer::PositionedBuffer;
    use crate::expr::Expression;
    use crate::term::untyped::UntypedAbstraction;

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
