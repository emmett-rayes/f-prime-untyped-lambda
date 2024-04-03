use crate::expression::variable::{Variable, VariableIndex};
use crate::untyped_lambda::term::{UntypedAbstraction, UntypedApplication, UntypedTerm};
use crate::visitor::Visitor;
use std::collections::{HashMap, LinkedList};
use std::ops::DerefMut;

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
    type Result = Variable;

    fn visit(&mut self, mut variable: Variable) -> Self::Result {
        if let Some(scopes) = self.variable_map.get(&variable.symbol) {
            let binding_scope = *scopes.front().unwrap();
            variable.index = self.current_scope - binding_scope + 1;
        }
        variable
    }
}

impl Visitor<Box<UntypedAbstraction>> for DeBruijnConverter {
    type Result = Box<UntypedAbstraction>;

    fn visit(&mut self, mut abstraction: Box<UntypedAbstraction>) -> Self::Result {
        self.current_scope += 1;
        self.variable_map
            .entry(abstraction.parameter.symbol.clone())
            .or_default()
            .push_front(self.current_scope);
        let abstraction_ref = abstraction.deref_mut();
        replace_term(&mut abstraction_ref.body, |term| self.visit(term));
        abstraction
    }
}

impl Visitor<Box<UntypedApplication>> for DeBruijnConverter {
    type Result = Box<UntypedApplication>;

    fn visit(&mut self, mut application: Box<UntypedApplication>) -> Self::Result {
        let application_ref = application.deref_mut();
        replace_term(&mut application_ref.applicator, |term| self.visit(term));
        replace_term(&mut application_ref.argument, |term| self.visit(term));
        application
    }
}

fn replace_term(dst: &mut UntypedTerm, f: impl FnOnce(UntypedTerm) -> UntypedTerm) {
    let dummy_term = UntypedTerm::from(Variable::new(""));
    let term = std::mem::replace(dst, dummy_term);
    let _ = std::mem::replace(dst, f(term));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::Expression;
    use f_prime_parser::PositionedBuffer;

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
