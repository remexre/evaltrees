use evaltrees::ast::{Decl, PrintStyle, Type};
use failure::Error;
use symbol::Symbol;

use options::Options;

pub fn run(mut decls: Vec<Decl<Type>>, options: &Options) -> Result<(), Error> {
    decls.sort_by_key(|decl| decl.name);

    print_decls(&decls, options.print_style());

    let mut evaluator = options.make_evaluator(decls)?;
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
