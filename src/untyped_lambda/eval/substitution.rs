use crate::expression::variable::{Variable, VariableIndex};
use crate::untyped_lambda::eval::shift::DeBruijnShift;
use crate::untyped_lambda::term::term_helpers::replace_term;
use crate::untyped_lambda::term::{UntypedAbstraction, UntypedApplication, UntypedTerm};
use crate::visitor::Visitor;

pub struct DeBruijnSubstitution {
    target: VariableIndex,
    replacement: UntypedTerm,
}

impl DeBruijnSubstitution {
    pub fn substitute(
        target: VariableIndex,
        replacement: UntypedTerm,
        term: UntypedTerm,
    ) -> UntypedTerm {
        let mut visitor = DeBruijnSubstitution {
            target,
            replacement,
        };
        visitor.visit(term)
    }
}

impl Visitor<Variable> for DeBruijnSubstitution {
    type Result = UntypedTerm;

    fn visit(&mut self, variable: Variable) -> Self::Result {
        if variable.index == self.target {
            self.replacement.clone()
        } else {
            UntypedTerm::from(variable)
        }
    }
}

impl Visitor<UntypedAbstraction> for DeBruijnSubstitution {
    type Result = UntypedTerm;

    fn visit(&mut self, mut abstraction: UntypedAbstraction) -> Self::Result {
        replace_term(&mut abstraction.body, move |term| {
            let shifted = DeBruijnShift::shift(1, self.replacement.clone());
            self.target += 1;
            let replacement = std::mem::replace(&mut self.replacement, shifted);
            let body = self.visit(term);
            self.replacement = replacement;
            self.target -= 1;
            body
        });
        UntypedTerm::from(abstraction)
    }
}

impl Visitor<UntypedApplication> for DeBruijnSubstitution {
    type Result = UntypedTerm;

    fn visit(&mut self, mut application: UntypedApplication) -> Self::Result {
        replace_term(&mut application.applicator, |term| self.visit(term));
        replace_term(&mut application.argument, |term| self.visit(term));
        UntypedTerm::from(application)
    }
}

//
