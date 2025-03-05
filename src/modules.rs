use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

use crate::exp::*;
use crate::parser;

use crate::parser::LispicoParser;
use pest::Parser;

pub fn execute_file(path: &str, env: List) -> Result<List> {
    let f = File::open(path)?;
    let f = BufReader::new(f);
    execute_stream(f, env, false)
}

pub fn execute_stream(stream: impl BufRead, env: List, prompt: bool) -> Result<List> {
    let mut env = env;
    let mut lines = stream.lines();
    loop {
        if prompt {
            print!("$ ");
            io::stdout().flush().unwrap();
        }

        let line = match lines.next() {
            None => break,
            Some(Ok(line)) => line,
            Some(Err(e)) => return Err(e.into()),
        };

        if line.is_empty() {
            continue;
        }

        let pair = LispicoParser::parse(parser::Rule::program, line.as_str())
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

    Ok(env)
}
