use f_prime::expression::buffer::PositionedBuffer;
use f_prime::expression::Expression;
use f_prime::untyped_lambda::term::de_bruijn::DeBruijnConverter;
use f_prime::untyped_lambda::term::pretty_print::UntypedPrettyPrinter;
use f_prime::untyped_lambda::term::UntypedTerm;

pub fn parse_untyped_term(input: &str) -> UntypedTerm {
    let input = PositionedBuffer::new(input);
    let output = UntypedTerm::parse(input);
    output.unwrap().0
}

pub fn convert_de_bruijn(term: UntypedTerm) -> UntypedTerm {
    DeBruijnConverter::convert(term)
}

pub fn pretty_print_untyped_term(term: UntypedTerm) -> String {
    UntypedPrettyPrinter::format_de_bruijn(term)
}

pub fn process_untyped(input: &str) -> String {
    pretty_print_untyped_term(convert_de_bruijn(parse_untyped_term(input)))
}
