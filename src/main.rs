use std::io::{self, Write};

use parser::LispicoParser;
use pest::Parser;

mod exp;
mod parser;
mod tests;

use exp::List;

fn main() -> io::Result<()> {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input)? == 0 {
            return Ok(());
        }

        let pair = LispicoParser::parse(parser::Rule::program, &input)
            .expect("failed to parse input")
            .next()
            .unwrap();

        let exp = parser::construct_exp(pair);
        let (res, _) = exp.eval(List::Nil);
        println!("{}", res);
    }
}
