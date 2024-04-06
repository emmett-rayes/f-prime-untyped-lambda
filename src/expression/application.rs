use f_prime_parser::{Parser, ParserResult};

use crate::expression::buffer::{Parsable, PositionedBuffer};
use crate::expression::Expression;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Application {
    pub applicator: Expression,
    pub argument: Expression,
}

impl Parsable for Application {
    fn parse(input: PositionedBuffer) -> ParserResult<PositionedBuffer, Self> {
        let parser = Expression::atom_parser().at_least(2).map(|expressions| {
            expressions
                .into_iter()
                .reduce(|applicator, argument| {
                    Expression::from(Application {
                        applicator,
                        argument,
                    })
                })
                .map(|expr| Application::try_from(expr).unwrap())
                .unwrap()
        });

        parser.parse(input)
    }
}

impl TryFrom<Expression> for Application {
    type Error = ();

    fn try_from(value: Expression) -> Result<Self, Self::Error> {
        if let Expression::Application(application) = value {
            Ok(*application)
        } else {
            Err(())
        }
    }
}
