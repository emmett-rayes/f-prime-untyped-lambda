use f_prime::expression::buffer::{Parsable, PositionedBuffer};
use f_prime::expression::UntypedLambda;
// use f_prime::traverse::de_bruijn::convert::DeBruijnConverter;
// use f_prime::traverse::pretty_print::ExpressionPrettyPrinter;

pub fn parse_expression(input: &str) -> UntypedLambda {
    let input = PositionedBuffer::new(input);
    let output = UntypedLambda::parse(input);
    assert!(output.clone().unwrap().1.buffer.is_empty());
    dbg!(output.clone().unwrap().0);
    output.unwrap().0
}

pub fn convert_de_bruijn(expression: &mut UntypedLambda) {
    todo!()
    // DeBruijnConverter::convert(expression);
}

pub fn pretty_print_expression(expression: &mut UntypedLambda) -> String {
    todo!()
    // ExpressionPrettyPrinter::format_nameless_locals(expression)
}

pub fn process_untyped(input: &str) -> String {
    let mut term = parse_expression(input);
    todo!()
    // convert_de_bruijn(&mut term);
    // pretty_print_expression(&mut term)
}
