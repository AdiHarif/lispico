use std::fmt::Display;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

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
    Identifier(String),
    List(List),
}

fn env_lookup(identifier: &str, env: &List) -> Exp {
    match env {
        List::Nil => Exp::List(List::Nil),
        List::Cons(hd, tl) => match **hd {
            Exp::List(List::Cons(ref name, ref value_list)) if matches!(**name, Exp::Identifier(ref id) if id == identifier) =>
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
            ("((a x))", "a", Exp::Identifier("x".to_string())),
            ("((a x) (b y))", "a", Exp::Identifier("x".to_string())),
            ("((a x) (b y))", "b", Exp::Identifier("y".to_string())),
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
}

impl Exp {
    pub fn eval(&self, env: List) -> Result<(Exp, List)> {
        match self {
            Exp::Identifier(id) => Ok((env_lookup(id, &env), env)),
            Exp::List(list) => list.eval(env),
        }
    }
}

impl Display for Exp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Exp::Identifier(identifier) => write!(f, "{identifier}"),
            Exp::List(List::Cons(hd, tl)) if matches!(**hd, Exp::Identifier(ref id) if id == "'") =>
            {
                write!(f, "'{tl}")
            }
            Exp::List(list) => write!(f, "({list})"),
        }
    }
}

fn eval_function(operator: &Exp, args: &List, env: List) -> Result<(Exp, List)> {
    match operator {
        Exp::Identifier(identifier) => match identifier.as_str() {
            "." => {
                let (new_hd, new_env) = args.hd()?.eval(env)?;
                let (new_tl, new_env) = args.tl()?.hd()?.eval(new_env)?;
                if let Exp::List(list) = new_tl {
                    return Ok((
                        Exp::List(List::Cons(Box::new(new_hd), Box::new(list))),
                        new_env,
                    ));
                }
                return Err("Expected a list, but got an atom".into());
            }
            ".<" => {
                let (arg, new_env) = args.hd()?.eval(env)?;
                if let Exp::List(list) = arg {
                    return Ok((list.hd()?.clone(), new_env));
                }
                return Err("Expected a list, but got an atom".into());
            }
            ".>" => {
                let (arg, new_env) = args.hd()?.eval(env)?;
                if let Exp::List(list) = arg {
                    return Ok((Exp::List(list.tl()?.clone()), new_env));
                }
                panic!("Expected a list, but got an atom");
            }
            "'" => Ok((args.hd()?.clone(), env)),
            "?" => {
                let (cond, new_env) = args.hd()?.eval(env)?;
                match cond {
                    Exp::List(List::Nil) if matches!(args.tl()?.tl()?, List::Nil) => {
                        Ok((Exp::List(List::Nil), new_env))
                    }
                    Exp::List(List::Nil) => args.tl()?.tl()?.hd()?.eval(new_env),
                    _ => args.tl()?.hd()?.eval(new_env),
                }
            }
            ":=" => {
                let name = args.hd()?;
                if let Exp::List(_) = name {
                    return Err("Expected an identifier, but got a list".into());
                }
                let (value, new_env) = args.tl()?.hd()?.eval(env)?;
                let new_binding = List::Cons(
                    Box::new(Exp::Identifier(name.to_string())),
                    Box::new(List::Cons(Box::new(value), Box::new(List::Nil))),
                );
                let new_env = List::Cons(Box::new(Exp::List(new_binding)), Box::new(new_env));
                Ok((Exp::List(List::Nil), new_env))
            }
            _ => Err(format!("Unknown operator: {identifier}").into()),
        },
        _ => Err("Expected an identifier, but got a list".into()),
    }
}
