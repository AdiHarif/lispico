use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use crate::sexp::*;

#[derive(Parser)]
#[grammar = "lispico.pest"]
pub struct LispicoParser;

pub fn construct_sexp(pair: Pair<Rule>) -> Sexp {
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
        Rule::quote_exp => Sexp::Cons(
            Box::new(Sexp::Atom(Atom::Identifier("'".to_string()))),
            Box::new(Sexp::Cons(
                Box::new(construct_sexp(pair.into_inner().next().unwrap())),
                Box::new(Sexp::Atom(Atom::Nil)),
            )),
        ),
        _ => unreachable!("unexpected rule: {:?}", pair.as_rule()),
    }
}
