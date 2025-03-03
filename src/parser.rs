use pest::iterators::Pair;
use pest_derive::Parser;

use crate::exp::*;

#[derive(Parser)]
#[grammar = "lispico.pest"]
pub struct LispicoParser;

fn construct_list(pair: Pair<Rule>) -> List {
    match pair.as_rule() {
        Rule::nil => List::Nil,
        Rule::list => {
            let mut pairs = pair.into_inner();
            List::Cons(
                Box::new(construct_exp(pairs.next().unwrap())),
                Box::new(construct_list(pairs.next().unwrap())),
            )
        }
        _ => unreachable!("unexpected rule: {:?}", pair.as_rule()),
    }
}

pub fn construct_exp(pair: Pair<Rule>) -> Exp {
    match pair.as_rule() {
        Rule::nil => Exp::List(List::Nil),
        Rule::identifier => Exp::Atom(Atom::Identifier(pair.as_str().to_string())),
        Rule::number => Exp::Atom(Atom::Number(pair.as_str().parse().unwrap())),
        Rule::string => Exp::Atom(Atom::String(pair.as_str().to_string())),
        Rule::list => Exp::List(construct_list(pair)),
        Rule::quote_exp => Exp::List(List::Cons(
            Box::new(Exp::Atom(Atom::Identifier("'".to_string()))),
            Box::new(List::Cons(
                Box::new(construct_exp(pair.into_inner().next().unwrap())),
                Box::new(List::Nil),
            )),
        )),
        _ => unreachable!("unexpected rule: {:?}", pair.as_rule()),
    }
}
