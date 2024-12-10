use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "mylisp.pest"]
struct MylispParser;

#[derive(Debug)]
enum Atom {
    Nil,
    Identifier(String),
}

#[derive(Debug)]
enum Sexp {
    Atom(Atom),
    Cons(Box<Sexp>, Box<Sexp>),
}

fn construct_sexp(pair: Pair<Rule>) -> Sexp {
    match pair.as_rule() {
        Rule::nil => Sexp::Atom(Atom::Nil),
        Rule::identifier => Sexp::Atom(Atom::Identifier(pair.as_str().to_string())),
        Rule::seq => {
            let mut seq = pair.into_inner();
            Sexp::Cons(
                Box::new(construct_sexp(seq.next().unwrap())),
                Box::new(construct_sexp(seq.next().unwrap())),
            )
        }
        _ => unreachable!("unexpected rule: {:?}", pair.as_rule()),
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let programs = vec![
            "(a)",
            "(a b)",
            "(a b c)",
            "()",
            "(())",
            "((a))",
            "(a (b))",
            "(a (b c))",
        ];
        for program in programs {
            assert!(MylispParser::parse(Rule::program, program).is_ok());
        }

        let faulty_programs = vec!["(", ")", "(a", "a)", "(a b", "(a b c"];
        for program in faulty_programs {
            assert!(MylispParser::parse(Rule::program, program).is_err());
        }
    }
}
