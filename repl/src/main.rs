use std::io::{BufRead, Write};

use f_prime::eval::full::FullBetaEvaluator;
use f_prime::eval::TracingBetaReduction;
use f_prime::expression::buffer::{Parsable, PositionedBuffer};
use f_prime::expression::Expression;
use f_prime::term::untyped::UntypedLambdaTerm;
use f_prime::traverse::de_bruijn::convert::DeBruijnConverter;
use f_prime::traverse::pretty_print::ExpressionPrettyPrinter;

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
        let parsed = Expression::parse(buffer);
        if parsed.is_err() {
            print_error();
            continue;
        }
        let parsed = parsed.unwrap();
        if parsed.1.buffer.len() > 1 {
            print_error();
            continue;
        }
        let mut expression = parsed.0;
        DeBruijnConverter::convert(&mut expression);
        let format = ExpressionPrettyPrinter::format_named(&mut expression);
        let mut term = UntypedLambdaTerm::new(expression);
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
