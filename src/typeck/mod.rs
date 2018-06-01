//! A Hindley-Milner type-checker (with inference).

mod constraint;
mod subst;
mod ty;
mod util;

use std::collections::BTreeSet;

use hamt_rs::HamtMap;
use symbol::Symbol;

use ast::{Decl, Expr, Pattern};

use self::constraint::Constraint;
use self::subst::{SubstVar, Substitution};
use self::ty::Ty;

fn get_or_fresh(env: HamtMap<Symbol, Ty>, name: Symbol) -> (HamtMap<Symbol, Ty>, Ty) {
    // Borrowck gets mad here too...
    let tmp = env.find(&name).map(|ty| ty.clone());
    if let Some(ty) = tmp {
        let ty = ty.clone();
        (env, ty)
    } else {
        let ty = Ty::fresh();
        let env = env.plus(name, ty.clone());
        (env, ty)
    }
}

fn add_annotations_to_decls(decls: Vec<Decl<()>>) -> Vec<Decl<Ty>> {
    let mut env = HamtMap::new();

    // I can't use a .map() call here; borrowck gets mad...
    let mut out = Vec::with_capacity(decls.len());
    for decl in decls {
        let decl = add_annotations_to_decl(env.clone(), decl);
        env = env.plus(decl.name, decl.aux.clone());
        out.push(decl)
    }
    out
}

fn add_annotations_to_decl(env: HamtMap<Symbol, Ty>, decl: Decl<()>) -> Decl<Ty> {
    let (env, ty) = get_or_fresh(env, decl.name);
    Decl {
        name: decl.name,
        args: decl.args
            .into_iter()
            .map(|a| a.add_type_annotations())
            .collect(),
        body: decl.body.add_type_annotations(),
        aux: ty.clone(),
    }
}

impl Expr<()> {
    /// Converts the expression to one that has type annotations. All type
    /// annotations refer initially to fresh variables.
    fn add_type_annotations(self) -> Expr<Ty> {
        match self {
            Expr::Literal(l, ()) => Expr::Literal(l, Ty::fresh()),
            Expr::Op(op, l, r, ()) => Expr::Op(
                op,
                Box::new(l.add_type_annotations()),
                Box::new(r.add_type_annotations()),
                Ty::fresh(),
            ),
            Expr::Variable(n, ()) => Expr::Variable(n, Ty::fresh()),
        }
    }
}

impl Pattern<()> {
    /// Converts the pattern to one that has type annotations. All type
    /// annotations refer initially to fresh variables.
    fn add_type_annotations(self) -> Pattern<Ty> {
        match self {
            Pattern::Binding(n, ()) => Pattern::Binding(n, Ty::fresh()),
            Pattern::Cons(l, r, ()) => Pattern::Cons(
                Box::new(l.add_type_annotations()),
                Box::new(r.add_type_annotations()),
                Ty::fresh(),
            ),
            Pattern::Literal(l, ()) => Pattern::Literal(l, Ty::fresh()),
        }
    }
}
