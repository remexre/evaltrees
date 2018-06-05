use evaltrees::ast::{Decl, Expr, Type};
use failure::Error;
use symbol::Symbol;

pub fn run(mut decls: Vec<Decl<Type>>) -> Result<(), Error> {
    decls.sort_by_key(|decl| decl.name);

    // Yeah, this is a hack...
    let mut expr_ty = None;
    let mut first = true;
    let mut last_name: Symbol = "".into();
    for decl in decls {
        if decl.name.as_str() == "" {
            expr_ty = Some(decl.aux);
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

    let expr = Expr::Variable("".into(), expr_ty.unwrap());
    println!("\nexpr : {}\n", expr.aux_ref());
    unimplemented!()
}
