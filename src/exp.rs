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

    pub fn eval(&self) -> Exp {
        match self {
            List::Nil => Exp::List(List::Nil),
            List::Cons(hd, tl) => {
                return eval_function(*hd.clone(), *tl.clone());
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
    pub fn eval(self) -> Exp {
        match self {
            Exp::Identifier(_) => Exp::List(List::Nil),
            Exp::List(list) => list.eval(),
        }
    }
}

fn eval_function(operator: Exp, args: List) -> Exp {
    match operator {
        Exp::Identifier(identifier) => match identifier.as_str() {
            "." => {
                let new_hd = args.hd().clone().eval();
                let new_tl = args.tl().hd().clone().eval();
                if let Exp::List(list) = new_tl {
                    return Exp::List(List::Cons(Box::new(new_hd), Box::new(list)));
                }
                panic!("Expected a list, but got an atom");
            }
            ".<" => {
                let arg = args.hd().clone().eval();
                if let Exp::List(list) = arg {
                    return list.hd().clone();
                }
                panic!("Expected a list, but got an atom");
            }
            ".>" => {
                let arg = args.hd().clone().eval();
                if let Exp::List(list) = arg {
                    return Exp::List(list.tl().clone());
                }
                panic!("Expected a list, but got an atom");
            }
            "'" => args.hd().clone(),
            _ => panic!("Unknown operator: {}", identifier),
        },
        _ => panic!("Expected an identifier, but got a list"),
    }
}
