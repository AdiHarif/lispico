use std::env;
use std::io;

mod exp;
mod modules;
mod parser;
mod predefined;
mod tests;

use exp::List;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Error - Too many arguments");
        println!("Usage: lispico [file]");
    } else if args.len() == 2 {
        let path = &args[1];
        modules::execute_file(path, List::Nil).expect("failed to execute file");
    } else {
        modules::execute_stream(io::stdin().lock(), List::Nil, true)
            .expect("failed to execute stream");
    }

    return Ok(());
}
