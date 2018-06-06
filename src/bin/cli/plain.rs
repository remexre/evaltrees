use evaltrees::ast::{Decl, Type};
use failure::Error;
use symbol::Symbol;

use options::Options;

pub fn run(mut decls: Vec<Decl<Type>>, options: &Options) -> Result<(), Error> {
    decls.sort_by_key(|decl| decl.name);

    print_decls(&decls);

    let mut evaluator = options.make_evaluator(decls)?;
    println!("{}", evaluator);
    while !evaluator.normal_form() {
        evaluator.step()?;
        println!("{}", evaluator);
    }
    Ok(())
}

// Yeah, this is a hack...
fn print_decls(decls: &[Decl<Type>]) {
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
        println!("{};;", decl);
    }
}
