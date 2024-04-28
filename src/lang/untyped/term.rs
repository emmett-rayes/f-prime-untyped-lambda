use std::cell::Cell;

use f_prime_parser::combinators::between;
use f_prime_parser::{DefaultParsable, Parser, ParserInput, ParserResult};

use crate::expression::abstraction::Abstraction;
use crate::expression::application::Application;
use crate::expression::buffer::PositionedBuffer;
use crate::expression::literal::LiteralParser;
use crate::expression::variable::Variable;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UntypedLambda {
    Variable(Variable<UntypedLambda>),
    Abstraction(Box<Abstraction<Variable<UntypedLambda>, UntypedLambda>>),
    Application(Box<Application<UntypedLambda, UntypedLambda>>),
}

impl UntypedLambda {
    pub fn is_value(&self) -> bool {
        matches!(self, UntypedLambda::Abstraction(_))
    }
}

impl<'a> DefaultParsable<PositionedBuffer<'a>> for UntypedLambda {
    fn parser() -> impl Parser<PositionedBuffer<'a>, Output = Self>
        where
            Self: Sized,
    {
        UntypedLambdaParser
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum UntypedLambdaCompanion {
    Variable,
    Abstraction,
    Application,
    Parens,
}

pub struct UntypedLambdaParser;

impl UntypedLambdaParser {
    fn disable_mutable_recursion<'b, I, O, P>(
        companion_tag: UntypedLambdaCompanion,
        parser: P,
    ) -> impl Parser<I, Output = O> + 'b
        where
            O: 'b,
            I: ParserInput + 'b,
            P: Parser<I, Output = O> + 'b,
    {
        std::thread_local! {
            static PENDING : Cell<Option<UntypedLambdaCompanion>> = const { Cell::new(None) };
        }

        move |input: I| {
            if PENDING.get().is_some_and(|tag| tag == companion_tag) {
                Err(input.error("Infinite recursion"))
            } else {
                let old_value = PENDING.get();
                PENDING.set(Some(companion_tag));
                let result = parser.parse(input);
                PENDING.set(old_value);
                result
            }
        }
    }

    fn variable_parser<'a>() -> impl Parser<PositionedBuffer<'a>, Output = UntypedLambda> + 'a {
        Self::disable_mutable_recursion(
            UntypedLambdaCompanion::Variable,
            Variable::parser().map(UntypedLambda::from),
        )
    }

    fn abstraction_parser<'a>() -> impl Parser<PositionedBuffer<'a>, Output = UntypedLambda> + 'a {
        Self::disable_mutable_recursion(
            UntypedLambdaCompanion::Abstraction,
            Abstraction::parser().map(UntypedLambda::from),
        )
    }

    fn application_parser<'a>() -> impl Parser<PositionedBuffer<'a>, Output = UntypedLambda> + 'a {
        Self::disable_mutable_recursion(
            UntypedLambdaCompanion::Application,
            Application::parser().map(UntypedLambda::from),
        )
    }

    fn parens_parser<'a>() -> impl Parser<PositionedBuffer<'a>, Output = UntypedLambda> + 'a {
        Self::disable_mutable_recursion(
            UntypedLambdaCompanion::Parens,
            between(
                LiteralParser::new("("),
                UntypedLambda::parser(),
                LiteralParser::new(")"),
            ),
        )
    }
}

impl<'a> Parser<PositionedBuffer<'a>> for UntypedLambdaParser {
    type Output = UntypedLambda;

    fn parse<'b>(
        &self,
        input: PositionedBuffer<'a>,
    ) -> ParserResult<PositionedBuffer<'a>, Self::Output>
        where
            PositionedBuffer<'a>: 'b,
            Self::Output: 'b,
    {
        let parser = Self::abstraction_parser()
            .or_else(Self::application_parser())
            .or_else(Self::variable_parser())
            .or_else(Self::parens_parser());

        parser.parse(input)
    }
}

impl From<Variable<UntypedLambda>> for UntypedLambda {
    fn from(value: Variable<UntypedLambda>) -> Self {
        UntypedLambda::Variable(value)
    }
}

impl TryFrom<UntypedLambda> for Variable<UntypedLambda> {
    type Error = ();

    fn try_from(value: UntypedLambda) -> Result<Self, Self::Error> {
        if let UntypedLambda::Variable(variable) = value {
            Ok(variable)
        } else {
            Err(())
        }
    }
}

impl From<Abstraction<Variable<UntypedLambda>, UntypedLambda>> for UntypedLambda {
    fn from(value: Abstraction<Variable<UntypedLambda>, UntypedLambda>) -> Self {
        UntypedLambda::Abstraction(Box::from(value))
    }
}

impl TryFrom<UntypedLambda> for Abstraction<Variable<UntypedLambda>, UntypedLambda> {
    type Error = ();

    fn try_from(value: UntypedLambda) -> Result<Self, Self::Error> {
        if let UntypedLambda::Abstraction(abstraction) = value {
            Ok(*abstraction)
        } else {
            Err(())
        }
    }
}

impl From<Application<UntypedLambda, UntypedLambda>> for UntypedLambda {
    fn from(value: Application<UntypedLambda, UntypedLambda>) -> Self {
        UntypedLambda::Application(Box::from(value))
    }
}

impl TryFrom<UntypedLambda> for Application<UntypedLambda, UntypedLambda> {
    type Error = ();

    fn try_from(value: UntypedLambda) -> Result<Self, Self::Error> {
        if let UntypedLambda::Application(application) = value {
            Ok(*application)
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use super::*;

    fn parse_expression(input: &str) -> UntypedLambda {
        let input = PositionedBuffer::new(input);
        let (expression, remaining) = UntypedLambda::parse(input).unwrap();
        assert!(
            remaining.buffer.is_empty(),
            "Expression was not fully parsed. Remaining: {}",
            remaining.buffer
        );
        expression
    }

    #[test]
    fn test_variable() {
        let expression = parse_expression("x");
        assert_matches!(expression, UntypedLambda::Variable(_));
        let variable = Variable::try_from(expression).unwrap();
        assert_eq!(variable.symbol, "x");
    }

    #[test]
    fn test_abstraction() {
        let expression = parse_expression("λx. y x");
        assert_matches!(expression, UntypedLambda::Abstraction(_));
        let abstraction = Abstraction::try_from(expression).unwrap();
        assert_eq!(abstraction.parameter.symbol, "x");
        assert_matches!(abstraction.body, UntypedLambda::Application(_));
    }

    #[test]
    fn test_abstraction_nested() {
        let expression = parse_expression("λx y.x y z");
        assert_matches!(expression, UntypedLambda::Abstraction(_));
        let abstraction = Abstraction::try_from(expression).unwrap();
        assert_matches!(abstraction.body, UntypedLambda::Abstraction(_));
    }

    #[test]
    fn test_application() {
        let expression = parse_expression("(λx. x) (λx. x)");
        let application = Application::try_from(expression).unwrap();
        assert_matches!(application.applicator, UntypedLambda::Abstraction(_));
        assert_matches!(application.argument, UntypedLambda::Abstraction(_));
    }

    #[test]
    fn test_application_associativity() {
        let expression = parse_expression("x y z");
        assert_matches!(expression, UntypedLambda::Application(_));
        let application = Application::try_from(expression).unwrap();
        assert_matches!(application.applicator, UntypedLambda::Application(_));
        assert_matches!(application.argument, UntypedLambda::Variable(_));
    }

    #[test]
    fn test_expression() {
        let expression = parse_expression("λx. a (λt. b x t (f (λu. a u t z) λs. w)) w y");
        assert_matches!(expression, UntypedLambda::Abstraction(_));
    }
}
