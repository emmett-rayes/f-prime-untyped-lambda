use crate::expression::abstraction::Abstraction;
use crate::expression::variable::{DeBruijnIndex, Variable};
use crate::lang::untyped::term::UntypedLambda;

enum PrinterMode {
    Named,
    Indexed,
    NamelessLocals,
}


trait PrettyPrint {
    fn pretty_print(&self) -> String;
}

impl<T> PrettyPrint for Variable<T> {
    fn pretty_print(&self) -> String {
        PrinterMode::Named => self.symbol.clone(),
        PrinterMode::Indexed => self.index.to_string(),
        PrinterMode::NamelessLocals => {
            if variable.index <= current_scope {
                variable.index.to_string()
            } else {
                variable.symbol.clone()
            }
        }
    }
}

impl<P, B> PrettyPrint for Abstraction<P, B> {

}

pub struct ExpressionPrettyPrinter {
    mode: PrinterMode,
}

impl ExpressionPrettyPrinter {
    pub fn format_named(expression: &UntypedLambda) -> String {
        Self::format_inner(expression, PrinterMode::Named)
    }

    pub fn format_indexed(expression: &UntypedLambda) -> String {
        Self::format_inner(expression, PrinterMode::Indexed)
    }

    pub fn format_nameless_locals(expression: &UntypedLambda) -> String {
        Self::format_inner(expression, PrinterMode::NamelessLocals)
    }

    pub fn format(expression: &UntypedLambda) -> String {
        Self::format_named(expression)
    }

    fn format_inner(expression: &UntypedLambda, mode: PrinterMode) -> String {
        let expression_is_abstraction = matches!(
            expression,
            UntypedLambda::Abstraction(_) | UntypedLambda::TypedAbstraction(_)
        );
        let mut printer = ExpressionPrettyPrinter { mode };
        let string = printer.traverse(expression, 0);
        if expression_is_abstraction {
            string
                .strip_prefix('(')
                .and_then(|s| s.strip_suffix(')'))
                .map(|s| s.to_string())
                .unwrap_or(string)
        } else {
            string
        }
    }

    fn traverse(&mut self, expression: &UntypedLambda, current_scope: DeBruijnIndex) -> String {
        let mut parameter_type = None;
        if let PrinterMode::Named = self.mode {
            if let UntypedLambda::TypedAbstraction(abstraction) = expression {
                parameter_type = Some(self.traverse(&abstraction.parameter_type, current_scope))
            }
        };
        let parameter_type = parameter_type;

        match expression {
            UntypedLambda::Variable(variable) => match self.mode {
                PrinterMode::Named => variable.symbol.clone(),
                PrinterMode::Indexed => variable.index.to_string(),
                PrinterMode::NamelessLocals => {
                    if variable.index <= current_scope {
                        variable.index.to_string()
                    } else {
                        variable.symbol.clone()
                    }
                }
            },
            UntypedLambda::Abstraction(box Abstraction { parameter, body })
            | UntypedLambda::TypedAbstraction(box TypedAbstraction {
                parameter, body, ..
            }) => {
                let body_is_abstraction = matches!(
                    body,
                    UntypedLambda::Abstraction(_) | UntypedLambda::TypedAbstraction(_)
                );
                let body = self.traverse(body, current_scope + 1);
                let body = if body_is_abstraction {
                    body.strip_prefix('(')
                        .and_then(|s| s.strip_suffix(')'))
                        .unwrap_or(body.as_str())
                } else {
                    body.as_str()
                };
                match self.mode {
                    PrinterMode::Named => {
                        if let Some(parameter_type) = parameter_type {
                            format!("(λ{}:{}. {})", parameter.symbol, parameter_type, body)
                        } else {
                            format!("(λ{}. {})", parameter.symbol, body)
                        }
                    }
                    PrinterMode::Indexed | PrinterMode::NamelessLocals => format!("(λ {})", body),
                }
            }
            UntypedLambda::Application(application) => {
                let argument_is_application =
                    matches!(application.argument, UntypedLambda::Application(_));
                let applicator = self.traverse(&application.applicator, current_scope);
                let argument = self.traverse(&application.argument, current_scope);
                if argument_is_application {
                    format!("{} ({})", applicator, argument,)
                } else {
                    format!("{} {}", applicator, argument,)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::expression::buffer::{Parsable, PositionedBuffer};
    use crate::traverse::de_bruijn::convert::DeBruijnConverter;

    use super::*;

    #[test]
    fn test_pretty_print_named() {
        let input = PositionedBuffer::new("(λx.λy.λz. w x y z)");
        let (mut expression, _) = UntypedLambda::parse(input).unwrap();
        DeBruijnConverter::convert(&mut expression);
        let pretty = ExpressionPrettyPrinter::format_named(&mut expression);
        assert_eq!(pretty, "λx. λy. λz. w x y z");
    }

    #[test]
    fn test_pretty_print_nameless() {
        let input = PositionedBuffer::new("(λx.λy.λz. w x y z)");
        let (mut expression, _) = UntypedLambda::parse(input).unwrap();
        DeBruijnConverter::convert(&mut expression);
        let pretty = ExpressionPrettyPrinter::format_nameless_locals(&mut expression);
        assert_eq!(pretty, "λ λ λ w 3 2 1");
    }

    #[test]
    fn test_pretty_print_indexed() {
        let input = PositionedBuffer::new("(λx.λy.λz. w x y z)");
        let (mut expression, _) = UntypedLambda::parse(input).unwrap();
        DeBruijnConverter::convert(&mut expression);
        let pretty = ExpressionPrettyPrinter::format_indexed(&mut expression);
        assert_eq!(pretty, "λ λ λ 4 3 2 1");
    }

    #[test]
    fn test_associativity() {
        let input = PositionedBuffer::new("λx y z.x z (y z)");
        let (mut expression, _) = UntypedLambda::parse(input).unwrap();
        DeBruijnConverter::convert(&mut expression);
        let pretty = ExpressionPrettyPrinter::format_indexed(&mut expression);
        assert_eq!(pretty, "λ λ λ 3 1 (2 1)");
    }

    #[test]
    fn test_typed_abstraction() {
        let input = PositionedBuffer::new("λx:T,y:U.x y z");
        let (mut expression, _) = UntypedLambda::parse(input).unwrap();
        DeBruijnConverter::convert(&mut expression);
        let pretty = ExpressionPrettyPrinter::format_named(&mut expression);
        assert_eq!(pretty, "λx:T. λy:U. x y z");
    }
}
