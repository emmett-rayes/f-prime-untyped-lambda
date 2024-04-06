use f_prime_parser::{Parser, ParserResult, ThenParserExtensions};

use crate::expression::buffer::{Parsable, PositionedBuffer};
use crate::expression::Expression;
use crate::expression::symbol::literal_parser;
use crate::expression::variable::Variable;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Abstraction {
    pub parameter: Variable,
    pub body: Expression,
}

impl Parsable for Abstraction {
    fn parse(input: PositionedBuffer) -> ParserResult<PositionedBuffer, Self> {
        let parser = literal_parser("λ")
            .or_else(literal_parser("@"))
            .or_else(literal_parser("\\"))
            .skip_then(Variable::parser().at_least(1))
            .then_skip(literal_parser("."))
            .then(Expression::parser())
            .map(|(parameters, body)| {
                parameters.into_iter().rfold(body, |body, parameter| {
                    Expression::from(Abstraction { parameter, body})
                })
            })
            .map(|expr| Abstraction::try_from(expr).unwrap());

        parser.parse(input)
    }
}

impl TryFrom<Expression> for Abstraction {
    type Error = ();

    fn try_from(value: Expression) -> Result<Self, Self::Error> {
        if let Expression::Abstraction(abstraction) = value {
            Ok(*abstraction)
        }
        else {
            Err(())
        }
    }
}
