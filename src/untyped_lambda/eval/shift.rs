use crate::expression::variable::Variable;
use crate::untyped_lambda::term::{
    UntypedAbstraction, UntypedApplication, UntypedTerm, UntypedTermNonRewritingVisitor,
};
use crate::visitor::Visitor;

pub struct DeBruijnShift {
    place: i64,
    cutoff: u64,
}

impl DeBruijnShift {
    pub fn shift(place: i64, term: &mut UntypedTerm) {
        let mut visitor = DeBruijnShift { place, cutoff: 1 };
        visitor.visit(term)
    }
}

impl UntypedTermNonRewritingVisitor for DeBruijnShift {}

impl Visitor<Variable> for DeBruijnShift {
    type Result = ();

    fn visit(&mut self, variable: &mut Variable) -> Self::Result {
        if variable.index >= self.cutoff {
            variable.index = variable.index.saturating_add_signed(self.place);
        }
    }
}

impl Visitor<UntypedAbstraction> for DeBruijnShift {
    type Result = ();

    fn visit(&mut self, abstraction: &mut UntypedAbstraction) -> Self::Result {
        self.cutoff += 1;
        self.visit(&mut abstraction.body);
        self.cutoff -= 1;
    }
}

impl Visitor<UntypedApplication> for DeBruijnShift {
    type Result = ();

    fn visit(&mut self, application: &mut UntypedApplication) -> Self::Result {
        self.visit(&mut application.applicator);
        self.visit(&mut application.argument);
    }
}
