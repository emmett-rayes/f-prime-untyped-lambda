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

impl Parsable for crate::term::untyped::UntypedAbstraction {
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
            .map(|term| crate::term::untyped::UntypedAbstraction::try_from(term).unwrap());

        parser.parse(input)
    }
}
