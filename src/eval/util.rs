//! Helpers for implementing evaluators.

use std::collections::HashMap;

use symbol::Symbol;

use ast::{Decl, Expr, Type};

/// Stores the types of the given declarations into a map.
pub fn collect_types(decls: &[Decl<Type>]) -> HashMap<Symbol, Type> {
    decls
        .iter()
        .map(|decl| (decl.name, decl.aux_ref().clone()))
        .collect()
}

/// Returns whether the given expression is reducible, given the decls in scope.
pub fn reducible(expr: &Expr<Type>, decls: &HashMap<Symbol, Type>) -> bool {
    unimplemented!()
}
