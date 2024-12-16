#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
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
pub enum Sexp {
    Atom(Atom),
    Cons(Box<Sexp>, Box<Sexp>),
}

impl Sexp {
    pub fn car(&self) -> &Sexp {
        match self {
            Sexp::Cons(car, _) => car,
            _ => panic!("Expected a cons, but got an atom"),
        }
    }

    pub fn cdr(&self) -> &Sexp {
        match self {
            Sexp::Cons(_, cdr) => cdr,
            _ => panic!("Expected a cons, but got an atom"),
        }
    }

    pub fn eval(self) -> Sexp {
        match self {
            Sexp::Atom(atom) => atom.eval(),
            Sexp::Cons(car, cdr) => {
                if let Sexp::Atom(Atom::Identifier(identifier)) = *car {
                    return eval_function(Sexp::Atom(Atom::Identifier(identifier)), *cdr);
                } else {
                    panic!("Expected an identifier, but got an atom");
                }
            }
        }
    }
}

fn eval_function(operator: Sexp, args: Sexp) -> Sexp {
    match operator {
        Sexp::Atom(Atom::Identifier(identifier)) => match identifier.as_str() {
            "." => {
                let tl = args.cdr().car().clone().eval();
                if let Sexp::Atom(Atom::Nil) | Sexp::Cons(_, _) = tl {
                    return Sexp::Cons(Box::new(args.car().clone().eval()), Box::new(tl));
                }
                panic!("Expected a list, but got an atom");
            }
            ".<" => args.car().clone().eval().car().clone(),
            ".>" => args.car().clone().eval().cdr().clone(),
            "'" => args.car().clone(),
            _ => panic!("Unknown operator: {}", identifier),
        },
        Sexp::Atom(_) => panic!("Expected an identifier, but got an atom"),
        Sexp::Cons(_, _) => panic!("Expected an identifier, but got a list"),
    }
}
