use f_prime::expression::buffer::{Parsable, PositionedBuffer};
use f_prime::expression::Expression;
use f_prime::traverse::de_bruijn::convert::DeBruijnConverter;
use f_prime::traverse::pretty_print::ExpressionPrettyPrinter;

pub fn parse_expression(input: &str) -> Expression {
    let input = PositionedBuffer::new(input);
    let output = Expression::parse(input);
    output.unwrap().0
}

pub fn convert_de_bruijn(expression: &mut Expression) {
    DeBruijnConverter::convert(expression);
}

pub fn pretty_print_expression(expression: &mut Expression) -> String {
    ExpressionPrettyPrinter::format_nameless_locals(expression)
}

pub fn process_untyped(input: &str) -> String {
    let mut term = parse_expression(input);
    convert_de_bruijn(&mut term);
    pretty_print_expression(&mut term)
}
