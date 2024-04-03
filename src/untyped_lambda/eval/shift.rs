use crate::expression::variable::Variable;
use crate::untyped_lambda::term::term_helpers::replace_term;
use crate::untyped_lambda::term::{UntypedAbstraction, UntypedApplication, UntypedTerm};
use crate::visitor::Visitor;

pub struct DeBruijnShift {
    place: i64,
    cutoff: u64,
}

impl DeBruijnShift {
    pub fn shift(place: i64, term: UntypedTerm) -> UntypedTerm {
        let mut visitor = DeBruijnShift { place, cutoff: 0 };
        visitor.visit(term)
    }
}

impl Visitor<Variable> for DeBruijnShift {
    type Result = UntypedTerm;

    fn visit(&mut self, mut variable: Variable) -> Self::Result {
        if variable.index >= self.cutoff {
            variable.index = variable.index.saturating_add_signed(self.place);
        }

        UntypedTerm::from(variable)
    }
}

impl Visitor<UntypedAbstraction> for DeBruijnShift {
    type Result = UntypedTerm;

    fn visit(&mut self, mut abstraction: UntypedAbstraction) -> Self::Result {
        self.cutoff += 1;
        replace_term(&mut abstraction.body, |term| self.visit(term));
        self.cutoff -= 1;
        UntypedTerm::from(abstraction)
    }
}

impl Visitor<UntypedApplication> for DeBruijnShift {
    type Result = UntypedTerm;

    fn visit(&mut self, mut application: UntypedApplication) -> Self::Result {
        replace_term(&mut application.applicator, |term| self.visit(term));
        replace_term(&mut application.argument, |term| self.visit(term));
        UntypedTerm::from(application)
    }
}
