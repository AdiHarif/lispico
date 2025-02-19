#[cfg(test)]
mod tests {
    use pest::Parser;

    use crate::*;
    use exp::*;
    use parser::*;

    #[test]
    fn parser() {
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
    fn exp() {
        let programs = vec![
            (
                "(a)",
                Exp::List(List::Cons(
                    Box::new(Exp::Atom(Atom::Identifier("a".to_string()))),
                    Box::new(List::Nil),
                )),
            ),
            (
                "(a b)",
                Exp::List(List::Cons(
                    Box::new(Exp::Atom(Atom::Identifier("a".to_string()))),
                    Box::new(List::Cons(
                        Box::new(Exp::Atom(Atom::Identifier("b".to_string()))),
                        Box::new(List::Nil),
                    )),
                )),
            ),
            (
                "(a b c)",
                Exp::List(List::Cons(
                    Box::new(Exp::Atom(Atom::Identifier("a".to_string()))),
                    Box::new(List::Cons(
                        Box::new(Exp::Atom(Atom::Identifier("b".to_string()))),
                        Box::new(List::Cons(
                            Box::new(Exp::Atom(Atom::Identifier("c".to_string()))),
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
                        Box::new(Exp::Atom(Atom::Identifier("a".to_string()))),
                        Box::new(List::Nil),
                    ))),
                    Box::new(List::Nil),
                )),
            ),
            (
                "(a (b))",
                Exp::List(List::Cons(
                    Box::new(Exp::Atom(Atom::Identifier("a".to_string()))),
                    Box::new(List::Cons(
                        Box::new(Exp::List(List::Cons(
                            Box::new(Exp::Atom(Atom::Identifier("b".to_string()))),
                            Box::new(List::Nil),
                        ))),
                        Box::new(List::Nil),
                    )),
                )),
            ),
            (
                "(a (b c))",
                Exp::List(List::Cons(
                    Box::new(Exp::Atom(Atom::Identifier("a".to_string()))),
                    Box::new(List::Cons(
                        Box::new(Exp::List(List::Cons(
                            Box::new(Exp::Atom(Atom::Identifier("b".to_string()))),
                            Box::new(List::Cons(
                                Box::new(Exp::Atom(Atom::Identifier("c".to_string()))),
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
                        Box::new(Exp::Atom(Atom::Identifier("'".to_string()))),
                        Box::new(List::Cons(
                            Box::new(Exp::Atom(Atom::Identifier("a".to_string()))),
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
                        Box::new(Exp::Atom(Atom::Identifier("'".to_string()))),
                        Box::new(List::Cons(
                            Box::new(Exp::Atom(Atom::Identifier("a".to_string()))),
                            Box::new(List::Nil),
                        )),
                    ))),
                    Box::new(List::Cons(
                        Box::new(Exp::List(List::Cons(
                            Box::new(Exp::Atom(Atom::Identifier("'".to_string()))),
                            Box::new(List::Cons(
                                Box::new(Exp::Atom(Atom::Identifier("b".to_string()))),
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
                    Box::new(Exp::Atom(Atom::Identifier("a".to_string()))),
                    Box::new(List::Cons(
                        Box::new(Exp::List(List::Cons(
                            Box::new(Exp::Atom(Atom::Identifier("'".to_string()))),
                            Box::new(List::Cons(
                                Box::new(Exp::Atom(Atom::Identifier("b".to_string()))),
                                Box::new(List::Nil),
                            )),
                        ))),
                        Box::new(List::Cons(
                            Box::new(Exp::Atom(Atom::Identifier("c".to_string()))),
                            Box::new(List::Nil),
                        )),
                    )),
                )),
            ),
            (
                "( a )",
                Exp::List(List::Cons(
                    Box::new(Exp::Atom(Atom::Identifier("a".to_string()))),
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
    fn parse_identifier() {
        let identifiers = vec![
            "a", "aa", ".", ".<", ".>", "$", "@", "<<", ">>", "=", "_a", "a_", "a_a-a_",
        ];

        for identifier in identifiers {
            let pairs = LispicoParser::parse(Rule::identifier, identifier);
            assert!(
                pairs.is_ok() && pairs.unwrap().len() == 1,
                "identifier: {identifier}"
            );
        }

        let invalid_identifiers = vec!["", "(", "[", "\"", " ", "'"];
        for identifier in invalid_identifiers {
            let pairs = LispicoParser::parse(Rule::identifier, identifier);
            assert!(pairs.is_err(), "identifier: {identifier}");
        }
    }

    #[test]
    fn eval() {
        let programs = vec![
            ("()", Exp::List(List::Nil)),
            (
                "(. 'a '(b))",
                Exp::List(List::Cons(
                    Box::new(Exp::Atom(Atom::Identifier("a".to_string()))),
                    Box::new(List::Cons(
                        Box::new(Exp::Atom(Atom::Identifier("b".to_string()))),
                        Box::new(List::Nil),
                    )),
                )),
            ),
            ("(.< '(a b))", Exp::Atom(Atom::Identifier("a".to_string()))),
            (
                "(.> '(a b))",
                Exp::List(List::Cons(
                    Box::new(Exp::Atom(Atom::Identifier("b".to_string()))),
                    Box::new(List::Nil),
                )),
            ),
            (
                "(. 'a ())",
                Exp::List(List::Cons(
                    Box::new(Exp::Atom(Atom::Identifier("a".to_string()))),
                    Box::new(List::Nil),
                )),
            ),
            ("(? 't 'a 'b)", Exp::Atom(Atom::Identifier("a".to_string()))),
            ("(? () 'a 'b)", Exp::Atom(Atom::Identifier("b".to_string()))),
            ("(? 't 'a)", Exp::Atom(Atom::Identifier("a".to_string()))),
            ("(? () 'a)", Exp::List(List::Nil)),
            (
                "({} ((x 'a) (y 'b)) (. x (. y ())))",
                Exp::List(List::Cons(
                    Box::new(Exp::Atom(Atom::Identifier("a".to_string()))),
                    Box::new(List::Cons(
                        Box::new(Exp::Atom(Atom::Identifier("b".to_string()))),
                        Box::new(List::Nil),
                    )),
                )),
            ),
            (
                "({} ((x 'a) (y x)) (. x (. y ())))",
                Exp::List(List::Cons(
                    Box::new(Exp::Atom(Atom::Identifier("a".to_string()))),
                    Box::new(List::Cons(
                        Box::new(Exp::Atom(Atom::Identifier("a".to_string()))),
                        Box::new(List::Nil),
                    )),
                )),
            ),
            ("(+ 1 2)", Exp::Atom(Atom::Number(3.0))),
            ("(+ -1 2)", Exp::Atom(Atom::Number(1.0))),
            ("(+ 1 (+ 2 3))", Exp::Atom(Atom::Number(6.0))),
            ("(- 2 1)", Exp::Atom(Atom::Number(1.0))),
            ("(* 2 3)", Exp::Atom(Atom::Number(6.0))),
            ("(/ 6 2)", Exp::Atom(Atom::Number(3.0))),
            ("(^ 2 3)", Exp::Atom(Atom::Number(8.0))),
        ];

        for (program, expected) in programs {
            let pairs = LispicoParser::parse(Rule::program, program)
                .unwrap()
                .next()
                .unwrap();
            let exp = construct_exp(pairs);
            let (res, _) = exp.eval(List::Nil).unwrap();
            assert_eq!(res, expected, "program: {program}");
        }
    }

    #[test]
    fn display() {
        let programs = vec![
            (
                "(a)",
                Exp::List(List::Cons(
                    Box::new(Exp::Atom(Atom::Identifier("a".to_string()))),
                    Box::new(List::Nil),
                )),
            ),
            (
                "(a b)",
                Exp::List(List::Cons(
                    Box::new(Exp::Atom(Atom::Identifier("a".to_string()))),
                    Box::new(List::Cons(
                        Box::new(Exp::Atom(Atom::Identifier("b".to_string()))),
                        Box::new(List::Nil),
                    )),
                )),
            ),
            (
                "(a b c)",
                Exp::List(List::Cons(
                    Box::new(Exp::Atom(Atom::Identifier("a".to_string()))),
                    Box::new(List::Cons(
                        Box::new(Exp::Atom(Atom::Identifier("b".to_string()))),
                        Box::new(List::Cons(
                            Box::new(Exp::Atom(Atom::Identifier("c".to_string()))),
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
                        Box::new(Exp::Atom(Atom::Identifier("a".to_string()))),
                        Box::new(List::Nil),
                    ))),
                    Box::new(List::Nil),
                )),
            ),
            (
                "(a (b))",
                Exp::List(List::Cons(
                    Box::new(Exp::Atom(Atom::Identifier("a".to_string()))),
                    Box::new(List::Cons(
                        Box::new(Exp::List(List::Cons(
                            Box::new(Exp::Atom(Atom::Identifier("b".to_string()))),
                            Box::new(List::Nil),
                        ))),
                        Box::new(List::Nil),
                    )),
                )),
            ),
            (
                "(a (b c))",
                Exp::List(List::Cons(
                    Box::new(Exp::Atom(Atom::Identifier("a".to_string()))),
                    Box::new(List::Cons(
                        Box::new(Exp::List(List::Cons(
                            Box::new(Exp::Atom(Atom::Identifier("b".to_string()))),
                            Box::new(List::Cons(
                                Box::new(Exp::Atom(Atom::Identifier("c".to_string()))),
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
                        Box::new(Exp::Atom(Atom::Identifier("'".to_string()))),
                        Box::new(List::Cons(
                            Box::new(Exp::Atom(Atom::Identifier("a".to_string()))),
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
                        Box::new(Exp::Atom(Atom::Identifier("'".to_string()))),
                        Box::new(List::Cons(
                            Box::new(Exp::Atom(Atom::Identifier("a".to_string()))),
                            Box::new(List::Nil),
                        )),
                    ))),
                    Box::new(List::Cons(
                        Box::new(Exp::List(List::Cons(
                            Box::new(Exp::Atom(Atom::Identifier("'".to_string()))),
                            Box::new(List::Cons(
                                Box::new(Exp::Atom(Atom::Identifier("b".to_string()))),
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
                    Box::new(Exp::Atom(Atom::Identifier("a".to_string()))),
                    Box::new(List::Cons(
                        Box::new(Exp::List(List::Cons(
                            Box::new(Exp::Atom(Atom::Identifier("'".to_string()))),
                            Box::new(List::Cons(
                                Box::new(Exp::Atom(Atom::Identifier("b".to_string()))),
                                Box::new(List::Nil),
                            )),
                        ))),
                        Box::new(List::Cons(
                            Box::new(Exp::Atom(Atom::Identifier("c".to_string()))),
                            Box::new(List::Nil),
                        )),
                    )),
                )),
            ),
        ];

        for (program, exp) in programs {
            assert_eq!(program, exp.to_string(), "program: {program}");
        }
    }

    #[test]
    fn env() {
        let cases = vec![
            (
                "(? a 'b 'c)",
                "((a x))",
                Exp::Atom(Atom::Identifier("b".to_string())),
            ),
            (
                "(? a 'b 'c)",
                "((b x))",
                Exp::Atom(Atom::Identifier("c".to_string())),
            ),
            (
                "(. a b)",
                "((a x) (b (y z)))",
                Exp::List(List::Cons(
                    Box::new(Exp::Atom(Atom::Identifier("x".to_string()))),
                    Box::new(List::Cons(
                        Box::new(Exp::Atom(Atom::Identifier("y".to_string()))),
                        Box::new(List::Cons(
                            Box::new(Exp::Atom(Atom::Identifier("z".to_string()))),
                            Box::new(List::Nil),
                        )),
                    )),
                )),
            ),
            (
                "(.< a)",
                "((a (x y z)))",
                Exp::Atom(Atom::Identifier("x".to_string())),
            ),
            (
                "(.> a)",
                "((a (x y z)))",
                Exp::List(List::Cons(
                    Box::new(Exp::Atom(Atom::Identifier("y".to_string()))),
                    Box::new(List::Cons(
                        Box::new(Exp::Atom(Atom::Identifier("z".to_string()))),
                        Box::new(List::Nil),
                    )),
                )),
            ),
        ];

        for (program_str, env_str, expected) in cases {
            let program_pair = LispicoParser::parse(Rule::program, program_str)
                .unwrap()
                .next()
                .unwrap();
            let env_pair = LispicoParser::parse(Rule::program, env_str)
                .unwrap()
                .next()
                .unwrap();
            let program = construct_exp(program_pair);
            let env_exp = construct_exp(env_pair);
            let env;
            if let Exp::List(list) = env_exp {
                env = list;
            } else {
                panic!("Expected a list, but got an atom");
            }
            let (res, _) = program.eval(env).unwrap();
            assert_eq!(
                res, expected,
                "program: {program_str}, env: {env_str}, res: {res}"
            );
        }
    }
}
