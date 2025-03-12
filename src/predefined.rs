use crate::exp::{Atom, Exp, List, Result};
use crate::modules::execute_file;

type LispicoOperator = fn(&List, List) -> Result<(Exp, List)>;

pub static PREDEFINED_OPERATORS: [(&str, LispicoOperator); 14] = [
    (".", |args, env| -> Result<(Exp, List)> {
        let (new_hd, new_env) = args.hd()?.eval(env)?;
        let (new_tl, new_env) = args.tl()?.hd()?.eval(new_env)?;
        if let Exp::List(list) = new_tl {
            return Ok((
                Exp::List(List::Cons(Box::new(new_hd), Box::new(list))),
                new_env,
            ));
        }
        return Err("Expected a list, but got an atom".into());
    }),
    (".<", |args, env| -> Result<(Exp, List)> {
        let (arg, new_env) = args.hd()?.eval(env)?;
        if let Exp::List(list) = arg {
            return Ok((list.hd()?.clone(), new_env));
        }
        return Err("Expected a list, but got an atom".into());
    }),
    (".>", |args, env| -> Result<(Exp, List)> {
        let (arg, new_env) = args.hd()?.eval(env)?;
        if let Exp::List(list) = arg {
            return Ok((Exp::List(list.tl()?.clone()), new_env));
        }
        panic!("Expected a list, but got an atom");
    }),
    ("'", |args, env| -> Result<(Exp, List)> {
        Ok((args.hd()?.clone(), env))
    }),
    ("=", |args, env| -> Result<(Exp, List)> {
        let (lhs, env) = args.nth(0)?.eval(env)?;
        let (rhs, env) = args.nth(1)?.eval(env)?;
        if lhs != rhs {
            return Ok((Exp::List(List::Nil), env));
        } else {
            return Ok((Exp::Atom(Atom::Identifier("t".to_string())), env));
        }
    }),
    ("?", |args, env| -> Result<(Exp, List)> {
        let (cond, new_env) = args.hd()?.eval(env)?;
        match cond {
            Exp::List(List::Nil) if matches!(args.tl()?.tl()?, List::Nil) => {
                Ok((Exp::List(List::Nil), new_env))
            }
            Exp::List(List::Nil) => args.tl()?.tl()?.hd()?.eval(new_env),
            _ => args.tl()?.hd()?.eval(new_env),
        }
    }),
    (":=", |args, env| -> Result<(Exp, List)> {
        let name = args.hd()?;
        if let Exp::List(_) = name {
            return Err("Expected an identifier, but got a list".into());
        }
        let (value, new_env) = args.tl()?.hd()?.eval(env)?;
        let new_binding = List::Cons(
            Box::new(Exp::Atom(Atom::Identifier(name.to_string()))),
            Box::new(List::Cons(Box::new(value), Box::new(List::Nil))),
        );
        let new_env = List::Cons(Box::new(Exp::List(new_binding)), Box::new(new_env));
        Ok((Exp::List(List::Nil), new_env))
    }),
    ("{}", |args, env| -> Result<(Exp, List)> {
        let inner_env = construct_let_env(args.nth(0)?.as_list()?, env.clone())?;
        let body = args.nth(1)?;
        let (res, _) = body.eval(inner_env)?;
        Ok((res, env))
    }),
    ("#", |args, env| -> Result<(Exp, List)> {
        let (filename, env) = args.hd()?.eval(env)?;
        let path = filename.as_atom()?.as_string()?;
        let new_env = execute_file(path, env)?;
        Ok((Exp::List(List::Nil), new_env))
    }),
    ("+", |args, env| eval_numeric_operator("+", args, env)),
    ("-", |args, env| eval_numeric_operator("-", args, env)),
    ("*", |args, env| eval_numeric_operator("*", args, env)),
    ("/", |args, env| eval_numeric_operator("/", args, env)),
    ("^", |args, env| eval_numeric_operator("^", args, env)),
];

pub fn construct_let_env(bindings: &List, env: List) -> Result<List> {
    if let List::Nil = bindings {
        return Ok(env);
    }
    let hd = bindings.hd()?.as_list()?;
    let name = hd.nth(0)?;
    let (value, new_env) = hd.nth(1)?.eval(env)?;
    let new_binding = List::Cons(
        Box::new(name.clone()),
        Box::new(List::Cons(Box::new(value), Box::new(List::Nil))),
    );
    let next_env = List::Cons(Box::new(Exp::List(new_binding)), Box::new(new_env));

    return construct_let_env(bindings.tl()?, next_env);
}

fn eval_numeric_operator(op: &str, args: &List, env: List) -> Result<(Exp, List)> {
    let (lhs, env) = args.nth(0)?.eval(env)?;
    let (rhs, env) = args.nth(1)?.eval(env)?;

    let x = lhs.as_atom()?.as_number()?;
    let y = rhs.as_atom()?.as_number()?;

    let result = match op {
        "+" => x + y,
        "-" => x - y,
        "*" => x * y,
        "/" => {
            if y == 0.0 {
                return Err("Division by zero".into());
            }
            x / y
        }
        "^" => x.powf(y),
        _ => unreachable!(),
    };

    Ok((Exp::Atom(Atom::Number(result)), env))
}
