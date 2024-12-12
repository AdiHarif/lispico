use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "mylisp.pest"]
struct MylispParser;

#[derive(Debug, PartialEq, Clone)]
enum Atom {
    Nil,
    Identifier(String),
}

impl Atom {
    fn eval(&self) -> Sexp {
        match self {
            Atom::Nil => Sexp::Atom(Atom::Nil),
            Atom::Identifier(_) => Sexp::Atom(Atom::Nil),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Sexp {
    Atom(Atom),
    Cons(Box<Sexp>, Box<Sexp>),
}

impl Sexp {
    fn car(&self) -> &Sexp {
        match self {
            Sexp::Cons(car, _) => car,
            _ => panic!("Expected a cons, but got an atom"),
        }
    }

    fn cdr(&self) -> &Sexp {
        match self {
            Sexp::Cons(_, cdr) => cdr,
            _ => panic!("Expected a cons, but got an atom"),
        }
    }

    fn eval(self) -> Sexp {
        match self {
            Sexp::Atom(atom) => atom.eval(),
            Sexp::Cons(car, cdr) => {
                if let Sexp::Atom(Atom::Identifier(identifier)) = *car {
                    return evalFunction(Sexp::Atom(Atom::Identifier(identifier)), *cdr);
                } else {
                    panic!("Expected an identifier, but got an atom");
                }
            }
        }
    }
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

fn evalFunction(operator: Sexp, args: Sexp) -> Sexp {
    match operator {
        Sexp::Atom(Atom::Identifier(identifier)) => match identifier.as_str() {
            "." => Sexp::Cons(
                Box::new(args.car().clone().eval()),
                Box::new(args.cdr().car().clone().eval()),
            ),
            ".<" => args.car().clone().eval().car().clone(),
            ".>" => args.car().clone().eval().cdr().clone(),
            "'" => args.car().clone(),
            _ => panic!("Unknown operator: {}", identifier),
        },
        Sexp::Atom(_) => panic!("Expected an identifier, but got an atom"),
        Sexp::Cons(_, _) => panic!("Expected an identifier, but got a list"),
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
            "('a)",
            "('a 'b)",
            "(a 'b c)",
            "( a )",
        ];
        for program in programs {
            assert!(MylispParser::parse(Rule::program, program).is_ok());
        }

        let faulty_programs = vec!["(", ")", "(a", "a)", "(a b", "(a b c", "(' a)"];
        for program in faulty_programs {
            assert!(MylispParser::parse(Rule::program, program).is_err());
        }
    }

    #[test]
    fn test_sexp() {
        let programs = vec![
            (
                "(a)",
                Sexp::Cons(
                    Box::new(Sexp::Atom(Atom::Identifier("a".to_string()))),
                    Box::new(Sexp::Atom(Atom::Nil)),
                ),
            ),
            (
                "(a b)",
                Sexp::Cons(
                    Box::new(Sexp::Atom(Atom::Identifier("a".to_string()))),
                    Box::new(Sexp::Cons(
                        Box::new(Sexp::Atom(Atom::Identifier("b".to_string()))),
                        Box::new(Sexp::Atom(Atom::Nil)),
                    )),
                ),
            ),
            (
                "(a b c)",
                Sexp::Cons(
                    Box::new(Sexp::Atom(Atom::Identifier("a".to_string()))),
                    Box::new(Sexp::Cons(
                        Box::new(Sexp::Atom(Atom::Identifier("b".to_string()))),
                        Box::new(Sexp::Cons(
                            Box::new(Sexp::Atom(Atom::Identifier("c".to_string()))),
                            Box::new(Sexp::Atom(Atom::Nil)),
                        )),
                    )),
                ),
            ),
            ("()", Sexp::Atom(Atom::Nil)),
            (
                "(())",
                Sexp::Cons(
                    Box::new(Sexp::Atom(Atom::Nil)),
                    Box::new(Sexp::Atom(Atom::Nil)),
                ),
            ),
            (
                "((a))",
                Sexp::Cons(
                    Box::new(Sexp::Cons(
                        Box::new(Sexp::Atom(Atom::Identifier("a".to_string()))),
                        Box::new(Sexp::Atom(Atom::Nil)),
                    )),
                    Box::new(Sexp::Atom(Atom::Nil)),
                ),
            ),
            (
                "(a (b))",
                Sexp::Cons(
                    Box::new(Sexp::Atom(Atom::Identifier("a".to_string()))),
                    Box::new(Sexp::Cons(
                        Box::new(Sexp::Cons(
                            Box::new(Sexp::Atom(Atom::Identifier("b".to_string()))),
                            Box::new(Sexp::Atom(Atom::Nil)),
                        )),
                        Box::new(Sexp::Atom(Atom::Nil)),
                    )),
                ),
            ),
            (
                "(a (b c))",
                Sexp::Cons(
                    Box::new(Sexp::Atom(Atom::Identifier("a".to_string()))),
                    Box::new(Sexp::Cons(
                        Box::new(Sexp::Cons(
                            Box::new(Sexp::Atom(Atom::Identifier("b".to_string()))),
                            Box::new(Sexp::Cons(
                                Box::new(Sexp::Atom(Atom::Identifier("c".to_string()))),
                                Box::new(Sexp::Atom(Atom::Nil)),
                            )),
                        )),
                        Box::new(Sexp::Atom(Atom::Nil)),
                    )),
                ),
            ),
            (
                "('a)",
                Sexp::Cons(
                    Box::new(Sexp::Cons(
                        Box::new(Sexp::Atom(Atom::Identifier("'".to_string()))),
                        Box::new(Sexp::Cons(
                            Box::new(Sexp::Atom(Atom::Identifier("a".to_string()))),
                            Box::new(Sexp::Atom(Atom::Nil)),
                        )),
                    )),
                    Box::new(Sexp::Atom(Atom::Nil)),
                ),
            ),
            (
                "('a 'b)",
                Sexp::Cons(
                    Box::new(Sexp::Cons(
                        Box::new(Sexp::Atom(Atom::Identifier("'".to_string()))),
                        Box::new(Sexp::Cons(
                            Box::new(Sexp::Atom(Atom::Identifier("a".to_string()))),
                            Box::new(Sexp::Atom(Atom::Nil)),
                        )),
                    )),
                    Box::new(Sexp::Cons(
                        Box::new(Sexp::Cons(
                            Box::new(Sexp::Atom(Atom::Identifier("'".to_string()))),
                            Box::new(Sexp::Cons(
                                Box::new(Sexp::Atom(Atom::Identifier("b".to_string()))),
                                Box::new(Sexp::Atom(Atom::Nil)),
                            )),
                        )),
                        Box::new(Sexp::Atom(Atom::Nil)),
                    )),
                ),
            ),
            (
                "(a 'b c)",
                Sexp::Cons(
                    Box::new(Sexp::Atom(Atom::Identifier("a".to_string()))),
                    Box::new(Sexp::Cons(
                        Box::new(Sexp::Cons(
                            Box::new(Sexp::Atom(Atom::Identifier("'".to_string()))),
                            Box::new(Sexp::Cons(
                                Box::new(Sexp::Atom(Atom::Identifier("b".to_string()))),
                                Box::new(Sexp::Atom(Atom::Nil)),
                            )),
                        )),
                        Box::new(Sexp::Cons(
                            Box::new(Sexp::Atom(Atom::Identifier("c".to_string()))),
                            Box::new(Sexp::Atom(Atom::Nil)),
                        )),
                    )),
                ),
            ),
            (
                "( a )",
                Sexp::Cons(
                    Box::new(Sexp::Atom(Atom::Identifier("a".to_string()))),
                    Box::new(Sexp::Atom(Atom::Nil)),
                ),
            ),
        ];

        for (program, expected) in programs {
            let pairs = MylispParser::parse(Rule::list, program)
                .unwrap()
                .next()
                .unwrap();
            let sexp = construct_sexp(pairs);
            assert_eq!(sexp, expected, "program: {}", program);
        }
    }

    #[test]
    fn test_parse_identifier() {
        let identifiers = vec![
            "a", "aa", ".", ".<", ".>", "$", "@", "<<", ">>", "=", "_a", "a_", "a_a-a_",
        ];

        for identifier in identifiers {
            let pairs = MylispParser::parse(Rule::identifier, identifier);
            assert!(
                pairs.is_ok() && pairs.unwrap().len() == 1,
                "identifier: {}",
                identifier
            );
        }

        let invalid_identifiers = vec!["", "(", "[", "\"", " ", "'"];
        for identifier in invalid_identifiers {
            let pairs = MylispParser::parse(Rule::identifier, identifier);
            assert!(pairs.is_err(), "identifier: {}", identifier);
        }
    }

    #[test]
    fn test_eval() {
        let programs = vec![
            ("()", Sexp::Atom(Atom::Nil)),
            (
                "(. 'a 'b)",
                Sexp::Cons(
                    Box::new(Sexp::Atom(Atom::Identifier("a".to_string()))),
                    Box::new(Sexp::Atom(Atom::Identifier("b".to_string()))),
                ),
            ),
            ("(.< '(a b))", Sexp::Atom(Atom::Identifier("a".to_string()))),
            (
                "(.> '(a b))",
                Sexp::Cons(
                    Box::new(Sexp::Atom(Atom::Identifier("b".to_string()))),
                    Box::new(Sexp::Atom(Atom::Nil)),
                ),
            ),
        ];

        for (program, expected) in programs {
            let pairs = MylispParser::parse(Rule::list, program)
                .unwrap()
                .next()
                .unwrap();
            let sexp = construct_sexp(pairs);
            assert_eq!(sexp.eval(), expected, "program: {}", program);
        }
    }
}
