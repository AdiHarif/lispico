use std::fmt::Display;

use crate::predefined::PREDEFINED_OPERATORS;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Identifier(String),
    Number(f64),
    String(String),
}

impl Atom {
    pub fn as_identifier(&self) -> Result<&str> {
        match self {
            Atom::Identifier(id) => Ok(id),
            _ => Err("Expected an identifier".into()),
        }
    }

    pub fn as_number(&self) -> Result<f64> {
        match self {
            Atom::Number(num) => Ok(*num),
            _ => Err("Expected a number".into()),
        }
    }

    pub fn as_string(&self) -> Result<&str> {
        match self {
            Atom::String(s) => Ok(s),
            _ => Err("Expected a string".into()),
        }
    }

    pub fn eval(&self, env: List) -> Result<(Exp, List)> {
        match self {
            Atom::Identifier(identifier) => {
                let res = env_lookup(identifier, &env);
                Ok((res, env))
            }
            Atom::Number(num) => Ok((Exp::Atom(Atom::Number(*num)), env)),
            Atom::String(s) => Ok((Exp::Atom(Atom::String(s.to_string())), env)),
        }
    }
}

impl Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::Identifier(identifier) => write!(f, "{identifier}"),
            Atom::Number(num) => write!(f, "{num}"),
            Atom::String(s) => write!(f, "\"{s}\""),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum List {
    Nil,
    Cons(Box<Exp>, Box<List>),
}

impl List {
    pub fn hd(&self) -> Result<&Exp> {
        match self {
            List::Cons(hd, _) => Ok(hd),
            List::Nil => Err("Expected a cons, but got an atom".into()),
        }
    }

    pub fn tl(&self) -> Result<&List> {
        match self {
            List::Cons(_, tl) => Ok(tl),
            List::Nil => Err("Expected a cons, but got an atom".into()),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            List::Nil => 0,
            List::Cons(_, tl) => 1 + tl.len(),
        }
    }

    pub fn nth(&self, n: usize) -> Result<&Exp> {
        self.slice(n)?.hd()
    }

    pub fn slice(&self, start: usize) -> Result<&List> {
        match start {
            0 => Ok(self),
            _ => self.tl()?.slice(start - 1),
        }
    }

    pub fn extend(&self, other: &List) -> List {
        match self {
            List::Nil => other.clone(),
            List::Cons(hd, tl) => List::Cons(hd.clone(), Box::new(tl.extend(other))),
        }
    }

    pub fn eval(&self, env: List) -> Result<(Exp, List)> {
        match self {
            List::Nil => Ok((Exp::List(List::Nil), env)),
            List::Cons(hd, tl) => eval_function(hd, tl, env),
        }
    }
}

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            List::Nil => write!(f, ""),
            List::Cons(hd, tl) if matches!(**tl, List::Nil) => {
                write!(f, "{hd}")
            }
            List::Cons(hd, tl) => {
                write!(f, "{hd} {tl}")
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Exp {
    Atom(Atom),
    List(List),
}

impl Exp {
    pub fn as_atom(&self) -> Result<&Atom> {
        match self {
            Exp::Atom(atom) => Ok(atom),
            _ => Err("Expected an identifier, but got a list".into()),
        }
    }

    pub fn as_list(&self) -> Result<&List> {
        match self {
            Exp::List(list) => Ok(list),
            _ => Err("Expected a list, but got an identifier".into()),
        }
    }
}

fn env_lookup(identifier: &str, env: &List) -> Exp {
    match env {
        List::Nil => Exp::List(List::Nil),
        List::Cons(hd, tl) => match **hd {
            Exp::List(List::Cons(ref name, ref value_list)) if matches!(**name, Exp::Atom(Atom::Identifier(ref id)) if id == identifier) =>
            {
                debug_assert!(value_list.hd().is_ok());
                return value_list.hd().unwrap().clone();
            }
            _ => env_lookup(identifier, tl),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::construct_exp;
    use crate::parser::LispicoParser;
    use crate::parser::Rule;
    use pest::Parser;

    #[test]
    fn lookup() {
        let envs = vec![
            ("()", "a", Exp::List(List::Nil)),
            ("((a x))", "a", Exp::Atom(Atom::Identifier("x".to_string()))),
            (
                "((a x) (b y))",
                "a",
                Exp::Atom(Atom::Identifier("x".to_string())),
            ),
            (
                "((a x) (b y))",
                "b",
                Exp::Atom(Atom::Identifier("y".to_string())),
            ),
            ("((a x) (b y))", "", Exp::List(List::Nil)),
        ];

        for (env_str, identifier, expected) in envs {
            let pair = LispicoParser::parse(Rule::program, env_str)
                .unwrap()
                .next()
                .unwrap();
            let env_exp = construct_exp(pair);
            let env;
            if let Exp::List(list) = env_exp {
                env = list;
            } else {
                panic!("Expected a list, but got an atom");
            }

            let res = env_lookup(identifier, &env);
            assert_eq!(res, expected, "env: {env_str}");
        }
    }

    #[test]
    fn list_methods() -> Result<()> {
        let list = List::Cons(
            Box::new(Exp::Atom(Atom::Identifier("a".to_string()))),
            Box::new(List::Cons(
                Box::new(Exp::Atom(Atom::Identifier("b".to_string()))),
                Box::new(List::Nil),
            )),
        );
        assert_eq!(list.len(), 2);
        assert_eq!(list.hd()?, &Exp::Atom(Atom::Identifier("a".to_string())));
        assert_eq!(
            list.tl()?.hd()?,
            &Exp::Atom(Atom::Identifier("b".to_string()))
        );
        assert_eq!(list.nth(1)?, &Exp::Atom(Atom::Identifier("b".to_string())));
        assert!(list.nth(2).is_err());
        assert_eq!(list.slice(0)?, &list);
        assert_eq!(
            list.slice(1)?.hd()?,
            &Exp::Atom(Atom::Identifier("b".to_string()))
        );
        assert_eq!(list.slice(2)?, &List::Nil);
        Ok(())
    }

    #[test]
    fn list_extend() {
        let list1 = List::Cons(
            Box::new(Exp::Atom(Atom::Identifier("a".to_string()))),
            Box::new(List::Cons(
                Box::new(Exp::Atom(Atom::Identifier("b".to_string()))),
                Box::new(List::Nil),
            )),
        );
        let list2 = List::Cons(
            Box::new(Exp::Atom(Atom::Identifier("c".to_string()))),
            Box::new(List::Cons(
                Box::new(Exp::Atom(Atom::Identifier("d".to_string()))),
                Box::new(List::Nil),
            )),
        );
        let expected = List::Cons(
            Box::new(Exp::Atom(Atom::Identifier("a".to_string()))),
            Box::new(List::Cons(
                Box::new(Exp::Atom(Atom::Identifier("b".to_string()))),
                Box::new(List::Cons(
                    Box::new(Exp::Atom(Atom::Identifier("c".to_string()))),
                    Box::new(List::Cons(
                        Box::new(Exp::Atom(Atom::Identifier("d".to_string()))),
                        Box::new(List::Nil),
                    )),
                )),
            )),
        );

        assert_eq!(list1.extend(&list2), expected);
        assert_eq!(List::Nil.extend(&list2), list2);
        assert_eq!(list1.extend(&List::Nil), list1);
        assert_eq!(List::Nil.extend(&List::Nil), List::Nil);
    }
}

impl Exp {
    pub fn eval(&self, env: List) -> Result<(Exp, List)> {
        match self {
            Exp::Atom(atom) => atom.eval(env),
            Exp::List(list) => list.eval(env),
        }
    }
}

impl Display for Exp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Exp::Atom(atom) => write!(f, "{atom}"),
            Exp::List(List::Cons(hd, tl)) if matches!(**hd, Exp::Atom(Atom::Identifier(ref id)) if id == "'") =>
            {
                write!(f, "'{tl}")
            }
            Exp::List(list) => write!(f, "({list})"),
        }
    }
}

fn eval_function(operator: &Exp, args: &List, env: List) -> Result<(Exp, List)> {
    match operator {
        Exp::Atom(Atom::Identifier(identifier)) => {
            for (op, func) in PREDEFINED_OPERATORS.iter() {
                if op == identifier {
                    return func(args, env);
                }
            }
            return Err("Unknown operator".into());
        }
        _ => Err("Expected an identifier, but got a list".into()),
    }
}
