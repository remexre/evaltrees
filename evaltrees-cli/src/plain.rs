use evaltrees::ast::{Decl, PrintStyle, Type};
use failure::Error;
use symbol::Symbol;

use crate::options::Options;

pub fn run(mut decls: Vec<Decl<Type>>, options: &Options) -> Result<(), Error> {
    decls.sort_by_key(|decl| decl.name);

    print_decls(&decls, options.print_style());

    let make_evaluator = options.make_evaluator()?;
    let mut evaluator = make_evaluator(decls.into_iter().map(|d| d.map_aux(|_| ())).collect());
    evaluator.set_print_style(options.print_style());
    println!("{}", evaluator);
    while !evaluator.normal_form() {
        evaluator.step()?;
        println!("{}", evaluator);
    }
    Ok(())
}

fn print_decls(decls: &[Decl<Type>], print_style: PrintStyle) {
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
                println!();
            }

            println!("{} : {}", decl.name, decl.aux);
            last_name = decl.name;
        }
        println!("{};;", decl.display_as(print_style));
    }
}
