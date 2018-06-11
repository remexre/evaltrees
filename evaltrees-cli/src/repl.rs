use std::io::Error as IoError;
use std::mem::replace;

use evaltrees::ast::{Decl, PrintStyle, Type};
use evaltrees::eval::{CallByValue, Evaluator};
use evaltrees::repl::ReplCommand;
use evaltrees::typeck::typeck;
use failure::Error;
use linefeed::{reader::ReadResult, Interface, Terminal};
use symbol::Symbol;

pub fn run(mut decls: Vec<Decl<Type>>, mut print_style: PrintStyle) -> Result<(), Error> {
    let iface = Interface::new("evaltrees")?;
    iface.set_prompt("> ");
    print_decls(&iface, &decls, print_style)?;
    loop {
        let line = match iface.read_line()? {
            ReadResult::Input(line) => line,
            _ => break Ok(()),
        };
        match repl_one(&iface, &line, &mut decls, &mut print_style) {
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
    print_style: &mut PrintStyle,
) -> Result<bool, Error> {
    let cmd = line.parse()?;
    info!("Running command {:?}", cmd);
    match cmd {
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

            let mut evaluator = CallByValue::new(decls);
            evaluator.set_print_style(*print_style);
            loop {
                writeln!(iface, "{}", evaluator)?;
                if !evaluator.normal_form() {
                    evaluator.step()?;
                } else {
                    break Ok(true);
                }
            }
        }
        ReplCommand::Help => {
            writeln!(iface, "{}", ReplCommand::help())?;
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
        ReplCommand::Typeof(name) => {
            let ty = decls
                .iter()
                .find(|decl| decl.name == name)
                .map(|decl| &decl.aux)
                .ok_or_else(|| format_err!("Unknown variable {}", name))?;
            writeln!(iface, "{}", ty)?;
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
