use std::io::{self, Write};

use parser::LispicoParser;
use pest::Parser;

mod exp;
mod parser;
mod tests;

use exp::{Exp, List};

fn main() -> io::Result<()> {
    let mut env = List::Nil;
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input)? == 0 {
            return Ok(());
        }

        input = input.trim().to_string();
        if input.is_empty() {
            continue;
        }

        let pair = LispicoParser::parse(parser::Rule::program, &input)
            .expect("failed to parse input")
            .next()
            .unwrap();

        let exp = parser::construct_exp(pair);
        let (res, new_env) = exp.eval(env).unwrap();
        env = new_env;

        if let Exp::List(List::Nil) = res {
            continue;
        }
        println!("{res}");
    }
}
