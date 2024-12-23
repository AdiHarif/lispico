use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum List {
    Nil,
    Cons(Box<Exp>, Box<List>),
}

impl List {
    pub fn hd(&self) -> &Exp {
        match self {
            List::Cons(car, _) => car,
            _ => panic!("Expected a cons, but got an atom"),
        }
    }

    pub fn tl(&self) -> &List {
        match self {
            List::Cons(_, cdr) => cdr,
            _ => panic!("Expected a cons, but got an atom"),
        }
    }

    pub fn eval(&self, env: List) -> (Exp, List) {
        match self {
            List::Nil => (Exp::List(List::Nil), env),
            List::Cons(hd, tl) => eval_function(hd, tl, env),
        }
    }
}

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            List::Nil => write!(f, ""),
            List::Cons(hd, tl) if matches!(**tl, List::Nil) => {
                write!(f, "{}", hd)
            }
            List::Cons(hd, tl) => {
                write!(f, "{} {}", hd, tl)
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Exp {
    Identifier(String),
    List(List),
}

impl Exp {
    pub fn eval(&self, env: List) -> (Exp, List) {
        match self {
            Exp::Identifier(_) => (Exp::List(List::Nil), env),
            Exp::List(list) => list.eval(env),
        }
    }
}

impl Display for Exp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Exp::Identifier(identifier) => write!(f, "{}", identifier),
            Exp::List(List::Cons(hd, tl)) if matches!(**hd, Exp::Identifier(ref id) if id == "'") =>
            {
                write!(f, "'{}", tl)
            }
            Exp::List(list) => write!(f, "({})", list),
        }
    }
}

fn eval_function(operator: &Exp, args: &List, env: List) -> (Exp, List) {
    match operator {
        Exp::Identifier(identifier) => match identifier.as_str() {
            "." => {
                let (new_hd, new_env) = args.hd().eval(env);
                let (new_tl, new_env) = args.tl().hd().eval(new_env);
                if let Exp::List(list) = new_tl {
                    return (
                        Exp::List(List::Cons(Box::new(new_hd), Box::new(list))),
                        new_env,
                    );
                }
                panic!("Expected a list, but got an atom");
            }
            ".<" => {
                let (arg, new_env) = args.hd().eval(env);
                if let Exp::List(list) = arg {
                    return (list.hd().clone(), new_env);
                }
                panic!("Expected a list, but got an atom");
            }
            ".>" => {
                let (arg, new_env) = args.hd().eval(env);
                if let Exp::List(list) = arg {
                    return (Exp::List(list.tl().clone()), new_env);
                }
                panic!("Expected a list, but got an atom");
            }
            "'" => (args.hd().clone(), env),
            "?" => {
                let (cond, new_env) = args.hd().eval(env);
                match cond {
                    Exp::List(List::Nil) if matches!(args.tl().tl(), List::Nil) => {
                        (Exp::List(List::Nil), new_env)
                    }
                    Exp::List(List::Nil) => args.tl().tl().hd().eval(new_env),
                    _ => args.tl().hd().eval(new_env),
                }
            }
            _ => panic!("Unknown operator: {}", identifier),
        },
        _ => panic!("Expected an identifier, but got a list"),
    }
}
