use f_prime_parser::{Parser, ParserResult, ThenParserExtensions};

use crate::expression::buffer::{Parsable, PositionedBuffer};
use crate::expression::symbol::{literal_parser, Symbol};
use crate::expression::variable::Variable;
use crate::expression::Expression;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Abstraction {
    pub parameter: Variable,
    pub body: Expression,
}

impl Abstraction {
    fn lambda_parser<'a>() -> impl Parser<PositionedBuffer<'a>, Output = Symbol> + 'a {
        literal_parser("λ")
            .or_else(literal_parser("@"))
            .or_else(literal_parser("\\"))
    }

    fn parameters_parser<'a>() -> impl Parser<PositionedBuffer<'a>, Output = Vec<Variable>> + 'a {
        Variable::parser()
            .at_least(1)
            .then_skip(literal_parser("."))
    }
}

impl TryFrom<Expression> for Abstraction {
    type Error = ();

    fn try_from(value: Expression) -> Result<Self, Self::Error> {
        if let Expression::Abstraction(abstraction) = value {
            Ok(*abstraction)
        } else {
            Err(())
        }
    }
}

impl Parsable for Abstraction {
    fn parse(input: PositionedBuffer) -> ParserResult<PositionedBuffer, Self> {
        let parser = Abstraction::lambda_parser()
            .skip_then(Abstraction::parameters_parser())
            .then(Expression::parser())
            .map(|(parameters, body)| {
                parameters.into_iter().rfold(body, |body, parameter| {
                    Expression::from(Abstraction { parameter, body })
                })
            })
            .map(|expr| Abstraction::try_from(expr).unwrap());

        parser.parse(input)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypedAbstraction {
    pub parameter: Variable,
    pub parameter_type: Expression,
    pub body: Expression,
}

impl TypedAbstraction {
    pub fn typed_parameters_parser<'a>(
    ) -> impl Parser<PositionedBuffer<'a>, Output = Vec<(Variable, Expression)>> + 'a {
        let one = Variable::parser()
            .then_skip(literal_parser(":"))
            .then(Expression::parser());

        let more = literal_parser(",")
            .skip_then(Variable::parser())
            .then_skip(literal_parser(":"))
            .then(Expression::parser())
            .at_least(0);

        one.then(more)
            .map(|(first, mut rest)| {
                rest.insert(0, first);
                rest
            })
            .then_skip(literal_parser("."))
    }
}

impl TryFrom<Expression> for TypedAbstraction {
    type Error = ();

    fn try_from(value: Expression) -> Result<Self, Self::Error> {
        if let Expression::TypedAbstraction(abstraction) = value {
            Ok(*abstraction)
        } else {
            Err(())
        }
    }
}

impl Parsable for TypedAbstraction {
    fn parse(input: PositionedBuffer) -> ParserResult<PositionedBuffer, Self> {
        let parser = Abstraction::lambda_parser()
            .skip_then(TypedAbstraction::typed_parameters_parser())
            .then(Expression::parser())
            .map(|(parameters, body)| {
                parameters
                    .into_iter()
                    .rfold(body, |body, (parameter, parameter_type)| {
                        Expression::from(TypedAbstraction {
                            parameter,
                            parameter_type,
                            body,
                        })
                    })
            })
            .map(|expr| TypedAbstraction::try_from(expr).unwrap());

        parser.parse(input)
    }
}
