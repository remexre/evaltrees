//! A Hindley-Milner type-checker (with inference).

mod annotations;
mod constraint;
mod subst;
mod ty;
mod util;

use ast::{Decl, Type};
use typeck::annotations::add_annotations_to_decls;

/// An error during typechecking.
#[derive(Clone, Debug, Fail, PartialEq)]
pub enum TypeError {
    #[fail(display = "d")]
    d,
}

/// Completely type-checks a series of declarations.
pub fn typeck_decls(decls: Vec<Decl<()>>) -> Result<Vec<Decl<Type>>, TypeError> {
    // First, annotate the decls with type variables.
    let decls = add_annotations_to_decls(decls);

    // Next, collect the type constraints.
    let constraints = decls
        .iter()
        .map(|decl| decl.collect_constraints())
        .collect::<Vec<_>>();

    unimplemented!("{:?}", (decls, constraints))
}
