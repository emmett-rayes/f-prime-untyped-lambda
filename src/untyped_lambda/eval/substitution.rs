use crate::expression::variable::VariableIndex;
use crate::untyped_lambda::eval::shift::DeBruijnShift;
use crate::untyped_lambda::term::{UntypedAbstraction, UntypedApplication, UntypedTerm};
use crate::visitor::Visitor;
use std::ops::DerefMut;

pub struct DeBruijnSubstitution {
    target: VariableIndex,
    replacement: UntypedTerm,
}

impl DeBruijnSubstitution {
    pub fn substitute(target: VariableIndex, replacement: UntypedTerm, term: &mut UntypedTerm) {
        let mut visitor = DeBruijnSubstitution {
            target,
            replacement,
        };
        visitor.visit(term);
    }
}

impl Visitor<UntypedTerm> for DeBruijnSubstitution {
    type Result = ();

    fn visit(&mut self, term: &mut UntypedTerm) -> Self::Result {
        match term {
            UntypedTerm::Variable(variable) => {
                if variable.index == self.target {
                    *term = self.replacement.clone();
                }
            }
            UntypedTerm::Abstraction(abstraction) => self.visit(abstraction.deref_mut()),
            UntypedTerm::Application(application) => self.visit(application.deref_mut()),
        }
    }
}

impl Visitor<UntypedAbstraction> for DeBruijnSubstitution {
    type Result = ();

    fn visit(&mut self, abstraction: &mut UntypedAbstraction) -> Self::Result {
        let replacement = self.replacement.clone();
        DeBruijnShift::shift(1, &mut self.replacement);
        self.target += 1;
        self.visit(&mut abstraction.body);
        self.replacement = replacement;
        self.target -= 1;
    }
}

impl Visitor<UntypedApplication> for DeBruijnSubstitution {
    type Result = ();

    fn visit(&mut self, application: &mut UntypedApplication) -> Self::Result {
        self.visit(&mut application.applicator);
        self.visit(&mut application.argument);
    }
}
