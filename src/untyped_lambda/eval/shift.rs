use crate::expression::variable::Variable;
use crate::untyped_lambda::term::{
    UntypedAbstraction, UntypedApplication, UntypedTerm, UntypedTermNonRewritingVisitor,
};
use crate::visitor::Visitor;

pub struct DeBruijnShift {
    place: i64,
}

impl DeBruijnShift {
    pub fn shift(place: i64, term: &mut UntypedTerm) {
        let mut visitor = DeBruijnShift { place };
        visitor.visit(1, term)
    }
}

impl UntypedTermNonRewritingVisitor for DeBruijnShift {}

impl Visitor<Variable> for DeBruijnShift {
    type Result = ();
    type Context = u64;

    fn visit(&mut self, cutoff: Self::Context, variable: &mut Variable) -> Self::Result {
        if variable.index >= cutoff {
            variable.index = variable.index.saturating_add_signed(self.place);
        }
    }
}

impl Visitor<UntypedAbstraction> for DeBruijnShift {
    type Result = ();
    type Context = u64;

    fn visit(
        &mut self,
        cutoff: Self::Context,
        abstraction: &mut UntypedAbstraction,
    ) -> Self::Result {
        self.visit(cutoff + 1, &mut abstraction.body);
    }
}

impl Visitor<UntypedApplication> for DeBruijnShift {
    type Result = ();
    type Context = u64;

    fn visit(
        &mut self,
        cutoff: Self::Context,
        application: &mut UntypedApplication,
    ) -> Self::Result {
        self.visit(cutoff, &mut application.applicator);
        self.visit(cutoff, &mut application.argument);
    }
}
