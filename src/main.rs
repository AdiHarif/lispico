use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "mylisp.pest"]
struct MylispParser;

#[derive(Debug, PartialEq)]
enum Atom {
    Nil,
    Identifier(String),
}

#[derive(Debug, PartialEq)]
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
}
