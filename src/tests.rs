#[cfg(test)]
mod tests {
    use pest::Parser;

    use crate::*;
    use exp::*;
    use parser::*;

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
            "(? 't 'a 'b)",
            "(? 't 'a)",
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
    fn test_exp() {
        let programs = vec![
            (
                "(a)",
                Exp::List(List::Cons(
                    Box::new(Exp::Identifier("a".to_string())),
                    Box::new(List::Nil),
                )),
            ),
            (
                "(a b)",
                Exp::List(List::Cons(
                    Box::new(Exp::Identifier("a".to_string())),
                    Box::new(List::Cons(
                        Box::new(Exp::Identifier("b".to_string())),
                        Box::new(List::Nil),
                    )),
                )),
            ),
            (
                "(a b c)",
                Exp::List(List::Cons(
                    Box::new(Exp::Identifier("a".to_string())),
                    Box::new(List::Cons(
                        Box::new(Exp::Identifier("b".to_string())),
                        Box::new(List::Cons(
                            Box::new(Exp::Identifier("c".to_string())),
                            Box::new(List::Nil),
                        )),
                    )),
                )),
            ),
            ("()", Exp::List(List::Nil)),
            (
                "(())",
                Exp::List(List::Cons(
                    Box::new(Exp::List(List::Nil)),
                    Box::new(List::Nil),
                )),
            ),
            (
                "((a))",
                Exp::List(List::Cons(
                    Box::new(Exp::List(List::Cons(
                        Box::new(Exp::Identifier("a".to_string())),
                        Box::new(List::Nil),
                    ))),
                    Box::new(List::Nil),
                )),
            ),
            (
                "(a (b))",
                Exp::List(List::Cons(
                    Box::new(Exp::Identifier("a".to_string())),
                    Box::new(List::Cons(
                        Box::new(Exp::List(List::Cons(
                            Box::new(Exp::Identifier("b".to_string())),
                            Box::new(List::Nil),
                        ))),
                        Box::new(List::Nil),
                    )),
                )),
            ),
            (
                "(a (b c))",
                Exp::List(List::Cons(
                    Box::new(Exp::Identifier("a".to_string())),
                    Box::new(List::Cons(
                        Box::new(Exp::List(List::Cons(
                            Box::new(Exp::Identifier("b".to_string())),
                            Box::new(List::Cons(
                                Box::new(Exp::Identifier("c".to_string())),
                                Box::new(List::Nil),
                            )),
                        ))),
                        Box::new(List::Nil),
                    )),
                )),
            ),
            (
                "('a)",
                Exp::List(List::Cons(
                    Box::new(Exp::List(List::Cons(
                        Box::new(Exp::Identifier("'".to_string())),
                        Box::new(List::Cons(
                            Box::new(Exp::Identifier("a".to_string())),
                            Box::new(List::Nil),
                        )),
                    ))),
                    Box::new(List::Nil),
                )),
            ),
            (
                "('a 'b)",
                Exp::List(List::Cons(
                    Box::new(Exp::List(List::Cons(
                        Box::new(Exp::Identifier("'".to_string())),
                        Box::new(List::Cons(
                            Box::new(Exp::Identifier("a".to_string())),
                            Box::new(List::Nil),
                        )),
                    ))),
                    Box::new(List::Cons(
                        Box::new(Exp::List(List::Cons(
                            Box::new(Exp::Identifier("'".to_string())),
                            Box::new(List::Cons(
                                Box::new(Exp::Identifier("b".to_string())),
                                Box::new(List::Nil),
                            )),
                        ))),
                        Box::new(List::Nil),
                    )),
                )),
            ),
            (
                "(a 'b c)",
                Exp::List(List::Cons(
                    Box::new(Exp::Identifier("a".to_string())),
                    Box::new(List::Cons(
                        Box::new(Exp::List(List::Cons(
                            Box::new(Exp::Identifier("'".to_string())),
                            Box::new(List::Cons(
                                Box::new(Exp::Identifier("b".to_string())),
                                Box::new(List::Nil),
                            )),
                        ))),
                        Box::new(List::Cons(
                            Box::new(Exp::Identifier("c".to_string())),
                            Box::new(List::Nil),
                        )),
                    )),
                )),
            ),
            (
                "( a )",
                Exp::List(List::Cons(
                    Box::new(Exp::Identifier("a".to_string())),
                    Box::new(List::Nil),
                )),
            ),
        ];

        for (program, expected) in programs {
            let pairs = LispicoParser::parse(Rule::program, program)
                .unwrap()
                .next()
                .unwrap();
            let exp = construct_exp(pairs);
            assert_eq!(exp, expected, "program: {}", program);
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
            ("()", Exp::List(List::Nil)),
            (
                "(. 'a '(b))",
                Exp::List(List::Cons(
                    Box::new(Exp::Identifier("a".to_string())),
                    Box::new(List::Cons(
                        Box::new(Exp::Identifier("b".to_string())),
                        Box::new(List::Nil),
                    )),
                )),
            ),
            ("(.< '(a b))", Exp::Identifier("a".to_string())),
            (
                "(.> '(a b))",
                Exp::List(List::Cons(
                    Box::new(Exp::Identifier("b".to_string())),
                    Box::new(List::Nil),
                )),
            ),
            (
                "(. 'a ())",
                Exp::List(List::Cons(
                    Box::new(Exp::Identifier("a".to_string())),
                    Box::new(List::Nil),
                )),
            ),
            ("(? 't 'a 'b)", Exp::Identifier("a".to_string())),
            ("(? () 'a 'b)", Exp::Identifier("b".to_string())),
            ("(? 't 'a)", Exp::Identifier("a".to_string())),
            ("(? () 'a)", Exp::List(List::Nil)),
        ];

        for (program, expected) in programs {
            let pairs = LispicoParser::parse(Rule::program, program)
                .unwrap()
                .next()
                .unwrap();
            let exp = construct_exp(pairs);
            assert_eq!(exp.eval(), expected, "program: {}", program);
        }
    }

    #[test]
    fn test_display() {
        let programs = vec![
            (
                "(a)",
                Exp::List(List::Cons(
                    Box::new(Exp::Identifier("a".to_string())),
                    Box::new(List::Nil),
                )),
            ),
            (
                "(a b)",
                Exp::List(List::Cons(
                    Box::new(Exp::Identifier("a".to_string())),
                    Box::new(List::Cons(
                        Box::new(Exp::Identifier("b".to_string())),
                        Box::new(List::Nil),
                    )),
                )),
            ),
            (
                "(a b c)",
                Exp::List(List::Cons(
                    Box::new(Exp::Identifier("a".to_string())),
                    Box::new(List::Cons(
                        Box::new(Exp::Identifier("b".to_string())),
                        Box::new(List::Cons(
                            Box::new(Exp::Identifier("c".to_string())),
                            Box::new(List::Nil),
                        )),
                    )),
                )),
            ),
            ("()", Exp::List(List::Nil)),
            (
                "(())",
                Exp::List(List::Cons(
                    Box::new(Exp::List(List::Nil)),
                    Box::new(List::Nil),
                )),
            ),
            (
                "((a))",
                Exp::List(List::Cons(
                    Box::new(Exp::List(List::Cons(
                        Box::new(Exp::Identifier("a".to_string())),
                        Box::new(List::Nil),
                    ))),
                    Box::new(List::Nil),
                )),
            ),
            (
                "(a (b))",
                Exp::List(List::Cons(
                    Box::new(Exp::Identifier("a".to_string())),
                    Box::new(List::Cons(
                        Box::new(Exp::List(List::Cons(
                            Box::new(Exp::Identifier("b".to_string())),
                            Box::new(List::Nil),
                        ))),
                        Box::new(List::Nil),
                    )),
                )),
            ),
            (
                "(a (b c))",
                Exp::List(List::Cons(
                    Box::new(Exp::Identifier("a".to_string())),
                    Box::new(List::Cons(
                        Box::new(Exp::List(List::Cons(
                            Box::new(Exp::Identifier("b".to_string())),
                            Box::new(List::Cons(
                                Box::new(Exp::Identifier("c".to_string())),
                                Box::new(List::Nil),
                            )),
                        ))),
                        Box::new(List::Nil),
                    )),
                )),
            ),
            (
                "('a)",
                Exp::List(List::Cons(
                    Box::new(Exp::List(List::Cons(
                        Box::new(Exp::Identifier("'".to_string())),
                        Box::new(List::Cons(
                            Box::new(Exp::Identifier("a".to_string())),
                            Box::new(List::Nil),
                        )),
                    ))),
                    Box::new(List::Nil),
                )),
            ),
            (
                "('a 'b)",
                Exp::List(List::Cons(
                    Box::new(Exp::List(List::Cons(
                        Box::new(Exp::Identifier("'".to_string())),
                        Box::new(List::Cons(
                            Box::new(Exp::Identifier("a".to_string())),
                            Box::new(List::Nil),
                        )),
                    ))),
                    Box::new(List::Cons(
                        Box::new(Exp::List(List::Cons(
                            Box::new(Exp::Identifier("'".to_string())),
                            Box::new(List::Cons(
                                Box::new(Exp::Identifier("b".to_string())),
                                Box::new(List::Nil),
                            )),
                        ))),
                        Box::new(List::Nil),
                    )),
                )),
            ),
            (
                "(a 'b c)",
                Exp::List(List::Cons(
                    Box::new(Exp::Identifier("a".to_string())),
                    Box::new(List::Cons(
                        Box::new(Exp::List(List::Cons(
                            Box::new(Exp::Identifier("'".to_string())),
                            Box::new(List::Cons(
                                Box::new(Exp::Identifier("b".to_string())),
                                Box::new(List::Nil),
                            )),
                        ))),
                        Box::new(List::Cons(
                            Box::new(Exp::Identifier("c".to_string())),
                            Box::new(List::Nil),
                        )),
                    )),
                )),
            ),
        ];

        for (program, exp) in programs {
            assert_eq!(program, exp.to_string(), "program: {}", program);
        }
    }
}
