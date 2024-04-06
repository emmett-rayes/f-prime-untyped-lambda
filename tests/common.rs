use f_prime::expr::buffer::PositionedBuffer;
use f_prime::expr::Expression;
use f_prime::term::untyped::de_bruijn::DeBruijnConverter;
use f_prime::term::untyped::pretty_print::UntypedPrettyPrinter;
use f_prime::term::untyped::UntypedTerm;

pub fn parse_untyped_term(input: &str) -> UntypedTerm {
    let input = PositionedBuffer::new(input);
    let output = UntypedTerm::parse(input);
    output.unwrap().0
}

pub fn convert_de_bruijn(term: &mut UntypedTerm) {
    DeBruijnConverter::convert(term);
}

pub fn pretty_print_untyped_term(term: &mut UntypedTerm) -> String {
    UntypedPrettyPrinter::format_de_bruijn(term)
}

pub fn process_untyped(input: &str) -> String {
    let mut term = parse_untyped_term(input);
    convert_de_bruijn(&mut term);
    pretty_print_untyped_term(&mut term)
}
