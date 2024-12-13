#[cfg(test)]
mod tests {
    use pest::Parser;

    use crate::*;
    use parser::*;
    use sexp::*;

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
            assert!(LispicoParser::parse(Rule::program, program).is_ok());
        }

        let faulty_programs = vec!["(", ")", "(a", "a)", "(a b", "(a b c", "(' a)"];
        for program in faulty_programs {
            assert!(LispicoParser::parse(Rule::program, program).is_err());
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
            let pairs = LispicoParser::parse(Rule::list, program)
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
            let pairs = LispicoParser::parse(Rule::identifier, identifier);
            assert!(
                pairs.is_ok() && pairs.unwrap().len() == 1,
                "identifier: {}",
                identifier
            );
        }

        let invalid_identifiers = vec!["", "(", "[", "\"", " ", "'"];
        for identifier in invalid_identifiers {
            let pairs = LispicoParser::parse(Rule::identifier, identifier);
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
            let pairs = LispicoParser::parse(Rule::list, program)
                .unwrap()
                .next()
                .unwrap();
            let sexp = construct_sexp(pairs);
            assert_eq!(sexp.eval(), expected, "program: {}", program);
        }
    }
}
