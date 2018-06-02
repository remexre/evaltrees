use evaltrees::ast::{Decl, Expr, Type};
use failure::Error;

pub fn run(decls: Vec<Decl<Type>>, expr: Expr<Type>) -> Result<(), Error> {
    for decl in decls {
        println!("{} : {}", decl.name, decl.aux);
    }

    println!("\nmain expr : {}", expr.aux_ref());
    unimplemented!()
}
