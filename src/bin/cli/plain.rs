use evaltrees::ast::{Decl, Expr};
use failure::Error;

pub fn run(decls: Vec<Decl>, expr: Expr) -> Result<(), Error> {
    bail!("{:#?}", (decls, expr))
}
