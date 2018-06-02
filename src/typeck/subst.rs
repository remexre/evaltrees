use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

use ast::{Decl, Expr, Pattern};
use typeck::ty::Ty;

/// A variable present in a substitution.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SubstVar(usize);

impl SubstVar {
    /// Generates a fresh variable.
    pub fn gensym() -> SubstVar {
        lazy_static! {
            pub static ref NEXT: AtomicUsize = AtomicUsize::default();
        }
        let n = NEXT.fetch_add(1, Ordering::SeqCst);
        SubstVar(n)
    }
}

/// A substitution.
#[derive(Clone, Debug, PartialEq)]
pub struct Substitution(HashMap<SubstVar, Ty>);

impl Substitution {
    /// Adds a substitution.
    pub fn add(&mut self, var: SubstVar, ty: Ty) {
        let mut subst = HashMap::new();
        subst.insert(var, ty.clone());
        let subst = Substitution(subst);
        for ty in self.0.values_mut() {
            ty.apply_subst(&subst);
        }
        assert!(self.0.insert(var, ty.clone()).is_none());
    }

    /// Looks up the type corresponding to the given variable.
    pub fn get(&self, var: SubstVar) -> Option<Ty> {
        self.0.get(&var).map(|ty| ty.clone())
    }

    /// Creates a new, empty substitution.
    pub fn new() -> Substitution {
        Substitution(HashMap::new())
    }
}

impl Ty {
    /// Applies a substitution to the type.
    pub fn apply_subst(&mut self, subst: &Substitution) {
        match self {
            Ty::Func(l, r) => {
                l.apply_subst(subst);
                r.apply_subst(subst);
            }
            Ty::Int => {}
            Ty::List(t) => t.apply_subst(subst),
            Ty::Var(v) => if let Some(ty) = subst.get(*v) {
                *self = ty;
            },
        }
    }

    /// Applies a single replacement to the type.
    pub fn sub(&mut self, var: SubstVar, ty: &Ty) {
        match self {
            Ty::Func(l, r) => {
                l.sub(var, ty);
                r.sub(var, ty);
            }
            Ty::Int => {}
            Ty::List(t) => t.sub(var, ty),
            Ty::Var(v) if *v == var => {
                *self = ty.clone();
            }
            Ty::Var(_) => {}
        }
    }
}

impl Decl<Ty> {
    /// Applies a substitution to the Decl.
    pub fn apply_subst(&mut self, subst: &Substitution) {
        for arg in self.args.iter_mut() {
            arg.apply_subst(subst);
        }
        self.body.apply_subst(subst);
        self.aux.apply_subst(subst);
    }
}

impl Expr<Ty> {
    /// Applies a substitution to the Expr.
    pub fn apply_subst(&mut self, subst: &Substitution) {
        match self {
            Expr::Literal(_, ty) => ty.apply_subst(subst),
            Expr::Op(_, l, r, ty) => {
                l.apply_subst(subst);
                r.apply_subst(subst);
                ty.apply_subst(subst);
            }
            Expr::Variable(_, ty) => ty.apply_subst(subst),
        }
    }
}

impl Pattern<Ty> {
    /// Applies a substitution to the pattern.
    pub fn apply_subst(&mut self, subst: &Substitution) {
        match self {
            Pattern::Binding(_, ty) => ty.apply_subst(subst),
            Pattern::Cons(h, t, ty) => {
                h.apply_subst(subst);
                t.apply_subst(subst);
                ty.apply_subst(subst);
            }
            Pattern::Literal(_, ty) => ty.apply_subst(subst),
        }
    }
}
