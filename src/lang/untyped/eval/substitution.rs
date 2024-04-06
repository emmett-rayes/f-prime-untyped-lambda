use crate::eval::untyped::shift::DeBruijnShift;
use crate::lang::expr::variable::VariableIndex;
use crate::term::untyped::{UntypedAbstraction, UntypedApplication, UntypedTerm};
use crate::visitor::Visitor;
use std::ops::DerefMut;

pub struct DeBruijnSubstitution {
    replacement: UntypedTerm,
}

impl DeBruijnSubstitution {
    pub fn substitute(target: VariableIndex, replacement: UntypedTerm, term: &mut UntypedTerm) {
        let mut visitor = DeBruijnSubstitution { replacement };
        visitor.visit(target, term);
    }
}

impl Visitor<UntypedTerm> for DeBruijnSubstitution {
    type Result = ();
    type Context = VariableIndex;

    fn visit(&mut self, target: Self::Context, term: &mut UntypedTerm) -> Self::Result {
        match term {
            UntypedTerm::Variable(variable) => {
                if variable.index == target {
                    *term = self.replacement.clone();
                }
            }
            UntypedTerm::Abstraction(abstraction) => self.visit(target, abstraction.deref_mut()),
            UntypedTerm::Application(application) => self.visit(target, application.deref_mut()),
        }
    }
}

impl Visitor<UntypedAbstraction> for DeBruijnSubstitution {
    type Result = ();
    type Context = VariableIndex;

    fn visit(
        &mut self,
        target: Self::Context,
        abstraction: &mut UntypedAbstraction,
    ) -> Self::Result {
        let replacement = self.replacement.clone();
        DeBruijnShift::shift(1, &mut self.replacement);
        self.visit(target + 1, &mut abstraction.body);
        self.replacement = replacement;
    }
}

impl Visitor<UntypedApplication> for DeBruijnSubstitution {
    type Result = ();
    type Context = VariableIndex;

    fn visit(
        &mut self,
        target: Self::Context,
        application: &mut UntypedApplication,
    ) -> Self::Result {
        self.visit(target, &mut application.applicator);
        self.visit(target, &mut application.argument);
    }
}
