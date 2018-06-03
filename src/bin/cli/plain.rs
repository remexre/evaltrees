use evaltrees::ast::{Decl, Expr, Type};
use failure::Error;
use symbol::Symbol;

pub fn run(mut decls: Vec<Decl<Type>>, expr: Expr<Type>) -> Result<(), Error> {
    decls.sort_by_key(|decl| decl.name);

    // Yeah, this is a hack...
    let mut first = true;
    let mut last_name: Symbol = "".into();
    for decl in decls {
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

    println!("\n{} : {}\n", expr, expr.aux_ref());
    unimplemented!()
}
