use f_prime::expression::buffer::PositionedBuffer;
use f_prime::expression::Expression;
use f_prime::untyped_lambda::eval::by_value::CallByValueEvaluator;
use f_prime::untyped_lambda::eval::BetaReduction;
use f_prime::untyped_lambda::term::de_bruijn::DeBruijnConverter;
use f_prime::untyped_lambda::term::pretty_print::UntypedPrettyPrinter;
use f_prime::untyped_lambda::term::UntypedTerm;
use std::io::{BufRead, Write};

fn print_prompt() {
    print!(">> ");
    let _ = std::io::stdout().flush();
}

fn main() -> Result<(), std::io::Error> {
    print_prompt();
    for line in std::io::stdin().lock().lines() {
        let input = line?;
        let buffer = PositionedBuffer::new(input.as_str());
        let output = UntypedTerm::parse(buffer);
        let term = DeBruijnConverter::convert(output.unwrap().0);
        let value = CallByValueEvaluator::reduce(term);
        let result = UntypedPrettyPrinter::format(value.unwrap());
        println!("{}", result);
        print_prompt();
    }
    Ok(())
}
