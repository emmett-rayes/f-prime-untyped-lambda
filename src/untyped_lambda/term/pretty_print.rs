use crate::expression::variable::Variable;
use crate::untyped_lambda::term::{UntypedAbstraction, UntypedApplication, UntypedTerm};
use crate::visitor::Visitor;

pub struct UntypedPrettyPrinter;

impl UntypedPrettyPrinter {
    pub fn format(term: UntypedTerm) -> String {
        let mut visitor = UntypedPrettyPrinter;
        visitor.visit(term)
    }
}

impl Visitor<Variable> for UntypedPrettyPrinter {
    type Result = String;

    fn visit(&mut self, variable: Variable) -> Self::Result {
        if variable.index == 0 {
            variable.symbol.to_string()
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
            body.strip_prefix('(').unwrap().strip_suffix(')').unwrap()
        } else {
            body.as_str()
        };
        format!("(λ{})", body)
    }
}

impl Visitor<Box<UntypedApplication>> for UntypedPrettyPrinter {
    type Result = String;

    fn visit(&mut self, application: Box<UntypedApplication>) -> Self::Result {
        format!(
            "{} {}",
            self.visit(application.applicator),
            self.visit(application.argument)
        )
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
    use crate::untyped_lambda::term::debruijn::DeBruijnConverter;
    use f_prime_parser::PositionedBuffer;

    #[test]
    fn test_pretty_print() {
        let input = PositionedBuffer::new("(λx.λy.λz. w x y z)");
        let result = UntypedTerm::parse(input);
        let term = DeBruijnConverter::convert(result.unwrap().0);
        dbg!(UntypedPrettyPrinter::format(term));
    }
}
