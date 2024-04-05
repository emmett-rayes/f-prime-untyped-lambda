use crate::expression::variable::Variable;
use crate::untyped_lambda::term::{
    UntypedAbstraction, UntypedApplication, UntypedTerm, UntypedTermNonRewritingVisitor,
};
use crate::visitor::Visitor;
use std::collections::HashSet;

#[derive(Default)]
pub struct UntypedPrettyPrinter {
    free_variables: HashSet<String>,
    de_bruijn: bool,
}

impl UntypedPrettyPrinter {
    fn format_inner(term: &mut UntypedTerm, de_bruijn: bool) -> String {
        let term_is_abstraction = matches!(term, UntypedTerm::Abstraction(_));
        let mut visitor = UntypedPrettyPrinter {
            de_bruijn,
            ..Default::default()
        };
        let string = visitor.visit(term);
        if term_is_abstraction {
            string
                .strip_prefix('(')
                .and_then(|s| s.strip_suffix(')'))
                .map(|s| s.to_string())
                .unwrap_or(string)
        } else {
            string
        }
    }

    pub fn format(term: &mut UntypedTerm) -> String {
        Self::format_inner(term, false)
    }

    pub fn format_de_bruijn(term: &mut UntypedTerm) -> String {
        Self::format_inner(term, true)
    }
}

impl UntypedTermNonRewritingVisitor for UntypedPrettyPrinter {}

impl Visitor<Variable> for UntypedPrettyPrinter {
    type Result = String;

    fn visit(&mut self, variable: &mut Variable) -> Self::Result {
        if variable.index == 0 || !self.de_bruijn {
            self.free_variables.insert(variable.symbol.clone());
            variable.symbol.clone()
        } else {
            variable.index.to_string()
        }
    }
}

impl Visitor<UntypedAbstraction> for UntypedPrettyPrinter {
    type Result = String;

    fn visit(&mut self, abstraction: &mut UntypedAbstraction) -> Self::Result {
        let body_is_abstraction = matches!(abstraction.body, UntypedTerm::Abstraction(_));
        let body = self.visit(&mut abstraction.body);
        let body = if body_is_abstraction {
            body.strip_prefix('(')
                .and_then(|s| s.strip_suffix(')'))
                .unwrap_or(body.as_str())
        } else {
            body.as_str()
        };
        if !self.de_bruijn || self.free_variables.remove(&abstraction.parameter.symbol) {
            format!("(λ{}. {})", abstraction.parameter.symbol, body)
        } else {
            format!("(λ {})", body)
        }
    }
}

impl Visitor<UntypedApplication> for UntypedPrettyPrinter {
    type Result = String;

    fn visit(&mut self, application: &mut UntypedApplication) -> Self::Result {
        let argument_is_application = matches!(application.argument, UntypedTerm::Application(_));
        let applicator = self.visit(&mut application.applicator);
        let argument = self.visit(&mut application.argument);
        if argument_is_application {
            format!("{} ({})", applicator, argument,)
        } else {
            format!("{} {}", applicator, argument,)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::buffer::PositionedBuffer;
    use crate::expression::Expression;
    use crate::untyped_lambda::term::de_bruijn::DeBruijnConverter;

    #[test]
    fn test_pretty_print_de_bruin() {
        let input = PositionedBuffer::new("(λx.λy.λz. w x y z)");
        let output = UntypedTerm::parse(input);
        let mut term = output.unwrap().0;
        DeBruijnConverter::convert(&mut term);
        assert_eq!(
            UntypedPrettyPrinter::format_de_bruijn(&mut term),
            "λ λ λ w 3 2 1"
        );
    }

    #[test]
    fn test_pretty_print_symbolic() {
        let input = PositionedBuffer::new("(λx.λy.λz. w x y z)");
        let output = UntypedTerm::parse(input);
        let mut term = output.unwrap().0;
        assert_eq!(
            UntypedPrettyPrinter::format_de_bruijn(&mut term),
            "λx. λy. λz. w x y z"
        );
    }

    #[test]
    fn test_associativity() {
        let input = PositionedBuffer::new("λx y z.x z (y z)");
        let output = UntypedTerm::parse(input);
        let mut term = output.unwrap().0;
        DeBruijnConverter::convert(&mut term);
        assert_eq!(
            UntypedPrettyPrinter::format_de_bruijn(&mut term),
            "λ λ λ 3 1 (2 1)"
        );
    }
}
