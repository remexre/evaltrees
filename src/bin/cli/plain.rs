use evaltrees::ast::{Decl, Expr, Type};
use failure::Error;

pub fn run(decls: Vec<Decl<Type>>, expr: Expr<Type>) -> Result<(), Error> {
    bail!("{:#?}", (decls, expr))
}
