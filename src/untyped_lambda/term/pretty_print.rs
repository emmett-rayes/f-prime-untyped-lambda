use crate::expression::variable::Variable;
use crate::untyped_lambda::term::{UntypedAbstraction, UntypedApplication, UntypedTerm};
use crate::visitor::Visitor;
use std::collections::HashSet;

#[derive(Default)]
pub struct UntypedPrettyPrinter {
    free_variables: HashSet<String>,
}

impl UntypedPrettyPrinter {
    pub fn format(term: UntypedTerm) -> String {
        let mut visitor = UntypedPrettyPrinter::default();
        visitor.visit(term)
    }
}

impl Visitor<Variable> for UntypedPrettyPrinter {
    type Result = String;

    fn visit(&mut self, variable: Variable) -> Self::Result {
        if variable.index == 0 {
            self.free_variables.insert(variable.symbol.clone());
            variable.symbol.clone()
        } else {
            variable.index.to_string()
        }
    }
}

impl Visitor<Box<UntypedAbstraction>> for UntypedPrettyPrinter {
    type Result = String;

    fn visit(&mut self, abstraction: Box<UntypedAbstraction>) -> Self::Result {
        let body = self.visit(abstraction.body);
        let body = if body.starts_with("(λ") {
            body.strip_prefix("(").unwrap().strip_suffix(')').unwrap()
        } else {
            body.as_str()
        };
        if self.free_variables.remove(&abstraction.parameter.symbol) {
            format!("(λ{}.{})", abstraction.parameter.symbol, body)
        } else {
            format!("(λ{})", body)
        }
    }
}

impl Visitor<Box<UntypedApplication>> for UntypedPrettyPrinter {
    type Result = String;

    fn visit(&mut self, application: Box<UntypedApplication>) -> Self::Result {
        let argument_is_application = matches!(application.argument, UntypedTerm::Application(_));
        let applicator = self.visit(application.applicator);
        let argument = self.visit(application.argument);
        if argument_is_application {
            format!("{} ({})", applicator, argument,)
        } else {
            format!("{} {}", applicator, argument,)
        }
    }
}

impl Visitor<UntypedTerm> for UntypedPrettyPrinter {
    type Result = String;

    fn visit(&mut self, term: UntypedTerm) -> Self::Result {
        match term {
            UntypedTerm::Variable(variable) => self.visit(variable),
            UntypedTerm::Abstraction(abstraction) => self.visit(abstraction),
            UntypedTerm::Application(application) => self.visit(application),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::Expression;
    use crate::untyped_lambda::term::de_bruijn::DeBruijnConverter;
    use f_prime_parser::PositionedBuffer;

    #[test]
    fn test_pretty_print_de_bruin() {
        let input = PositionedBuffer::new("(λx.λy.λz. w x y z)");
        let output = UntypedTerm::parse(input);
        let term = DeBruijnConverter::convert(output.unwrap().0);
        assert_eq!(UntypedPrettyPrinter::format(term), "(λλλw 3 2 1)");
    }

    #[test]
    fn test_pretty_print_symbolic() {
        let input = PositionedBuffer::new("(λx.λy.λz. w x y z)");
        let output = UntypedTerm::parse(input);
        let term = output.unwrap().0;
        assert_eq!(UntypedPrettyPrinter::format(term), "(λx.λy.λz.w x y z)");
    }

    #[test]
    fn test_associativity() {
        let input = PositionedBuffer::new("λx y z.x z (y z)");
        let output = UntypedTerm::parse(input);
        let term = DeBruijnConverter::convert(output.unwrap().0);
        assert_eq!(UntypedPrettyPrinter::format(term), "(λλλ3 1 (2 1))");
    }
}
