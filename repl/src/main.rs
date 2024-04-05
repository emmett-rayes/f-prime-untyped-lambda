use f_prime::expression::buffer::PositionedBuffer;
use f_prime::expression::Expression;
use f_prime::untyped_lambda::eval::full::FullBetaEvaluator;
use f_prime::untyped_lambda::eval::TracingBetaReduction;
use f_prime::untyped_lambda::term::de_bruijn::DeBruijnConverter;
use f_prime::untyped_lambda::term::pretty_print::UntypedPrettyPrinter;
use f_prime::untyped_lambda::term::UntypedTerm;
use std::io::{BufRead, Write};

fn print_prompt() {
    print!(">> ");
    let _ = std::io::stdout().flush();
}

fn print_error() {
    println!("!!");
    print_prompt();
}

fn main() -> Result<(), std::io::Error> {
    print_prompt();
    for line in std::io::stdin().lock().lines() {
        if line.is_err() {
            print_error();
            continue;
        }
        let line = line.unwrap();
        let buffer = PositionedBuffer::new(line.as_str());
        let parsed = UntypedTerm::parse(buffer);
        if parsed.is_err() {
            print_error();
            continue;
        }
        let parsed = parsed.unwrap();
        if parsed.1.buffer.len() > 1 {
            print_error();
            continue;
        }
        let mut term = parsed.0;
        DeBruijnConverter::convert(&mut term);
        let format = UntypedPrettyPrinter::format(&mut term);
        let result = FullBetaEvaluator::trace(&mut term);
        if result.is_empty() {
            println!("stuck!");
        }
        println!("0. {}", format);
        for (i, step) in result.iter().enumerate() {
            println!("{}. {}", i + 1, step);
        }
        print_prompt();
    }
    Ok(())
}
