use std::io::Error as IoError;

use evaltrees::ast::{Decl, Type};
use evaltrees::cst::Expr as CstExpr;
use evaltrees::eval::{CallByValue, Evaluator};
use evaltrees::typeck::typeck;
use failure::Error;
use linefeed::{reader::ReadResult, Interface, Terminal};
use symbol::Symbol;

pub fn run(mut decls: Vec<Decl<Type>>) -> Result<(), Error> {
    let reader = Interface::new("evaltrees")?;
    reader.set_prompt("> ");
    print_decls(&reader, &decls)?;
    loop {
        let line = match reader.read_line()? {
            ReadResult::Input(line) => line,
            _ => break Ok(()),
        };
        if let Err(err) = repl_one(&reader, line, &mut decls) {
            error!("{}", err);
        }
    }
}

fn repl_one<T: Terminal>(
    iface: &Interface<T>,
    line: String,
    decls: &mut Vec<Decl<Type>>,
) -> Result<(), Error> {
    let expr: CstExpr = line.parse()?;
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
    loop {
        writeln!(iface, "{}", evaluator)?;
        if !evaluator.normal_form() {
            evaluator.step()?;
        } else {
            break;
        }
    }
    Ok(())
}

fn print_decls<T: Terminal>(iface: &Interface<T>, decls: &[Decl<Type>]) -> Result<(), IoError> {
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
        writeln!(iface, "{};;", decl)?;
    }
    Ok(())
}
