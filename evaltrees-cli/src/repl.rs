use std::io::Error as IoError;
use std::mem::replace;

use evaltrees::ast::{Decl, PrintStyle, Type};
use evaltrees::eval::Evaluator;
use evaltrees::repl::ReplCommand;
use evaltrees::typeck::typeck;
use failure::Error;
use linefeed::{reader::ReadResult, Interface, Terminal};
use symbol::Symbol;

pub fn run(
    mut decls: Vec<Decl<Type>>,
    mut print_style: PrintStyle,
    mut make_evaluator: fn(Vec<Decl<()>>) -> Box<dyn Evaluator>,
) -> Result<(), Error> {
    let iface = Interface::new("evaltrees")?;
    iface.set_prompt("> ")?;
    print_decls(&iface, &decls, print_style)?;
    loop {
        let line = match iface.read_line()? {
            ReadResult::Input(line) => line,
            _ => break Ok(()),
        };
        if line.trim() == "" {
            continue;
        }
        match repl_one(
            &iface,
            &line,
            &mut decls,
            &mut make_evaluator,
            &mut print_style,
        ) {
            Ok(true) => {}
            Ok(false) => break Ok(()),
            Err(err) => {
                error!("{}", err);
            }
        }
        iface.add_history_unique(line);
    }
}

fn repl_one<T: Terminal>(
    iface: &Interface<T>,
    line: &str,
    decls: &mut Vec<Decl<Type>>,
    make_evaluator: &mut fn(Vec<Decl<()>>) -> Box<dyn Evaluator>,
    print_style: &mut PrintStyle,
) -> Result<bool, Error> {
    match line.parse()? {
        ReplCommand::Decl(decl) => {
            let decl = decl.into_ast()?;
            let (diff_name, mut same_name) = split_vec(
                decls.clone(),
                |d| d.name == decl.name,
                |d| d.map_aux(|_ty| ()),
            );
            same_name.push(decl);
            replace(decls, typeck(same_name, diff_name)?);
            Ok(true)
        }
        ReplCommand::Evaluator(e) => {
            *make_evaluator = e;
            Ok(true)
        }
        ReplCommand::Expr(expr) => {
            let decls = typeck(
                vec![Decl {
                    name: "".into(),
                    args: vec![],
                    body: expr.into_ast()?,
                    aux: (),
                }],
                decls.clone(),
            )?;

            let mut evaluator =
                make_evaluator(decls.into_iter().map(|d| d.map_aux(|_| ())).collect());
            evaluator.set_print_style(*print_style);
            loop {
                if !evaluator.normal_form() {
                    evaluator.step()?;
                    writeln!(iface, "{}", evaluator)?;
                } else {
                    break Ok(true);
                }
            }
        }
        ReplCommand::Help => {
            writeln!(iface, "{}", ReplCommand::help())?;
            Ok(true)
        }
        ReplCommand::List => {
            print_decls(&iface, &decls, *print_style)?;
            Ok(true)
        }
        ReplCommand::PrintStyle(sty) => {
            *print_style = sty;
            Ok(true)
        }
        ReplCommand::Quit => Ok(false),
        ReplCommand::Reset => {
            decls.clear();
            Ok(true)
        }
        ReplCommand::Typeof(expr) => {
            let decls = typeck(
                vec![Decl {
                    name: "".into(),
                    args: vec![],
                    body: expr.into_ast()?,
                    aux: (),
                }],
                decls.clone(),
            )?;
            let decl = decls.iter().find(|decl| decl.name == "").unwrap();
            writeln!(
                iface,
                "{} : {}",
                decl.body.display_as(PrintStyle::CST),
                decl.aux
            )?;
            Ok(true)
        }
    }
}

fn print_decls<T: Terminal>(
    iface: &Interface<T>,
    decls: &[Decl<Type>],
    print_style: PrintStyle,
) -> Result<(), IoError> {
    let mut first = true;
    let mut last_name: Symbol = "".into();
    for decl in decls {
        if decl.name.as_str() == "" {
            continue;
        }
        if decl.name != last_name {
            if first {
                first = false;
            } else {
                writeln!(iface)?;
            }

            writeln!(iface, "{} : {}", decl.name, decl.aux)?;
            last_name = decl.name;
        }
        writeln!(iface, "{};;", decl.display_as(print_style))?;
    }
    Ok(())
}

// TODO: https://github.com/rust-lang/rust/issues/43244
fn split_vec<F, F2, T, T2>(mut vec: Vec<T>, mut is_right: F, mut transform: F2) -> (Vec<T>, Vec<T2>)
where
    F: FnMut(&mut T) -> bool,
    F2: FnMut(T) -> T2,
{
    let mut i = 0;
    let mut right = Vec::new();
    while i != vec.len() {
        if is_right(&mut vec[i]) {
            let val = vec.remove(i);
            right.push(transform(val));
        } else {
            i += 1;
        }
    }
    (vec, right)
}
