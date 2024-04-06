use crate::expression::buffer::PositionedBuffer;
use crate::expression::variable::Variable;
use crate::expression::{literal, Expression};
use crate::visitor::Visitor;
use f_prime_parser::combinators::between;
use f_prime_parser::{Parser, ParserResult, ThenParserExtensions};
use std::ops::DerefMut;

pub mod de_bruijn;
pub mod pretty_print;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UntypedTerm {
    Variable(Variable),
    Abstraction(Box<UntypedAbstraction>),
    Application(Box<UntypedApplication>),
}

impl UntypedTerm {
    pub fn is_value(&self) -> bool {
        matches!(self, UntypedTerm::Abstraction(_))
    }

    fn variable_parser<'a>() -> impl Parser<PositionedBuffer<'a>, Output = Self> + 'a {
        Variable::parser().map(UntypedTerm::from)
    }

    fn abstraction_parser<'a>() -> impl Parser<PositionedBuffer<'a>, Output = Self> + 'a {
        UntypedAbstraction::parser().map(UntypedTerm::from)
    }

    fn application_parser<'a>() -> impl Parser<PositionedBuffer<'a>, Output = Self> + 'a {
        UntypedApplication::parser().map(UntypedTerm::from)
    }

    fn atom_parser<'a>() -> impl Parser<PositionedBuffer<'a>, Output = Self> + 'a {
        between(literal("("), UntypedTerm::parser(), literal(")"))
            .or_else(UntypedTerm::abstraction_parser())
            .or_else(UntypedTerm::variable_parser())
    }
}

impl From<Variable> for UntypedTerm {
    fn from(variable: Variable) -> Self {
        UntypedTerm::Variable(variable)
    }
}

impl From<UntypedAbstraction> for UntypedTerm {
    fn from(abstraction: UntypedAbstraction) -> Self {
        UntypedTerm::Abstraction(Box::new(abstraction))
    }
}

impl From<Box<UntypedAbstraction>> for UntypedTerm {
    fn from(abstraction: Box<UntypedAbstraction>) -> Self {
        UntypedTerm::Abstraction(abstraction)
    }
}

impl From<UntypedApplication> for UntypedTerm {
    fn from(application: UntypedApplication) -> Self {
        UntypedTerm::Application(Box::new(application))
    }
}

impl From<Box<UntypedApplication>> for UntypedTerm {
    fn from(application: Box<UntypedApplication>) -> Self {
        UntypedTerm::Application(application)
    }
}

impl TryFrom<UntypedTerm> for Variable {
    type Error = String;

    fn try_from(term: UntypedTerm) -> Result<Self, Self::Error> {
        if let UntypedTerm::Variable(variable) = term {
            Ok(variable)
        } else {
            Err(String::from("Term is not an variable."))
        }
    }
}

impl Expression for UntypedTerm {
    fn parse(input: PositionedBuffer) -> ParserResult<PositionedBuffer, Self> {
        let parser = UntypedTerm::abstraction_parser()
            .or_else(UntypedTerm::application_parser())
            .or_else(UntypedTerm::atom_parser());

        parser.parse(input)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UntypedAbstraction {
    pub parameter: Variable,
    pub body: UntypedTerm,
}

impl UntypedAbstraction {
    pub fn new(parameter: Variable, body: UntypedTerm) -> Self {
        UntypedAbstraction { parameter, body }
    }
}

impl TryFrom<UntypedTerm> for UntypedAbstraction {
    type Error = String;

    fn try_from(term: UntypedTerm) -> Result<Self, Self::Error> {
        if let UntypedTerm::Abstraction(abstraction) = term {
            Ok(*abstraction)
        } else {
            Err(String::from("Term is not an abstraction."))
        }
    }
}

impl Expression for UntypedAbstraction {
    fn parse(input: PositionedBuffer) -> ParserResult<PositionedBuffer, Self> {
        let parser = literal("λ")
            .or_else(literal("@"))
            .or_else(literal("\\"))
            .skip_then(Variable::parser().at_least(1))
            .then_skip(literal("."))
            .then(UntypedTerm::parser())
            .map(|(parameters, body)| {
                parameters.into_iter().rfold(body, |body, parameter| {
                    UntypedTerm::from(UntypedAbstraction::new(parameter, body))
                })
            })
            .map(|term| UntypedAbstraction::try_from(term).unwrap());

        parser.parse(input)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UntypedApplication {
    pub applicator: UntypedTerm,
    pub argument: UntypedTerm,
}

impl UntypedApplication {
    pub fn new(applicator: UntypedTerm, argument: UntypedTerm) -> Self {
        UntypedApplication {
            applicator,
            argument,
        }
    }
}

impl TryFrom<UntypedTerm> for UntypedApplication {
    type Error = String;

    fn try_from(term: UntypedTerm) -> Result<Self, Self::Error> {
        if let UntypedTerm::Application(application) = term {
            Ok(*application)
        } else {
            Err(String::from("Term is not an application."))
        }
    }
}

impl Expression for UntypedApplication {
    fn parse(input: PositionedBuffer) -> ParserResult<PositionedBuffer, Self> {
        let parser = UntypedTerm::atom_parser().at_least(2).map(|terms| {
            terms
                .into_iter()
                .reduce(|applicator, argument| {
                    UntypedTerm::from(UntypedApplication::new(applicator, argument))
                })
                .map(|term| UntypedApplication::try_from(term).unwrap())
                .unwrap()
        });

        parser.parse(input)
    }
}

pub trait StructurePreservingUntypedTermVisitor {}

impl<V, C, R> Visitor<UntypedTerm> for V
where
    V: StructurePreservingUntypedTermVisitor
        + Visitor<Variable, Context = C, Result = R>
        + Visitor<UntypedAbstraction, Context = C, Result = R>
        + Visitor<UntypedApplication, Context = C, Result = R>,
{
    type Result = R;
    type Context = C;

    fn visit(&mut self, context: Self::Context, term: &mut UntypedTerm) -> Self::Result {
        match term {
            UntypedTerm::Variable(variable) => self.visit(context, variable),
            UntypedTerm::Abstraction(abstraction) => self.visit(context, abstraction.deref_mut()),
            UntypedTerm::Application(application) => self.visit(context, application.deref_mut()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let input = PositionedBuffer::new("λx. a (λt. b x t (f (λu. a u t z) λs. w)) w y");
        let output = UntypedTerm::parse(input);
        assert!(output.unwrap().1.buffer.is_empty())
    }

    #[test]
    fn test_multi_abstraction() {
        let input = PositionedBuffer::new("λx y.x y z");
        let output = UntypedTerm::parse(input);
        assert!(output.unwrap().1.buffer.is_empty())
    }
}
