use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

use display_attr::DisplayAttr;

use crate::ast::{Decl, Expr, Pattern};
use crate::typeck::ty::Ty;

/// A variable present in a substitution.
#[derive(Clone, Copy, Debug, DisplayAttr, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[display(fmt = "?{}", _0)]
pub struct SubstVar(usize);

impl SubstVar {
    /// Generates a fresh variable.
    pub fn fresh() -> SubstVar {
        lazy_static::lazy_static! {
            pub static ref NEXT: AtomicUsize = AtomicUsize::default();
        }
        let n = NEXT.fetch_add(1, Ordering::SeqCst);
        SubstVar(n)
    }
}

/// A substitution.
#[derive(Clone, Debug, Default, PartialEq)]
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
        assert!(self.0.insert(var, ty).is_none());
    }

    /// Looks up the type corresponding to the given variable.
    pub fn get(&self, var: SubstVar) -> Option<Ty> {
        self.0.get(&var).cloned()
    }

    /// Creates a new, empty substitution.
    pub fn new() -> Substitution {
        Substitution(HashMap::new())
    }
}

impl Ty {
    /// Applies a substitution to the type.
    pub fn apply_subst(&mut self, subst: &Substitution) {
        let new_self = match *self {
            Ty::Bool | Ty::Int => None,
            Ty::Func(ref mut l, ref mut r) => {
                l.apply_subst(subst);
                r.apply_subst(subst);
                None
            }
            Ty::List(ref mut t) => {
                t.apply_subst(subst);
                None
            }
            Ty::Var(v) => subst.get(v),
        };
        if let Some(new_self) = new_self {
            *self = new_self;
        }
    }

    /// Applies a single replacement to the type.
    pub fn sub(&mut self, var: SubstVar, ty: &Ty) {
        let new_self = match self {
            Ty::Bool | Ty::Int => None,
            Ty::Func(l, r) => {
                l.sub(var, ty);
                r.sub(var, ty);
                None
            }
            Ty::List(t) => {
                t.sub(var, ty);
                None
            }
            Ty::Var(v) if *v == var => Some(ty.clone()),
            Ty::Var(_) => None,
        };
        if let Some(new_self) = new_self {
            *self = new_self;
        }
    }
}

impl Decl<Ty> {
    /// Applies a substitution to the declaration.
    pub(in crate::typeck) fn apply_subst(&mut self, subst: &Substitution) {
        for arg in &mut self.args {
            arg.apply_subst(subst);
        }
        self.body.apply_subst(subst);
        self.aux.apply_subst(subst);
    }
}

impl Expr<Ty> {
    /// Applies a substitution to the expression.
    pub(in crate::typeck) fn apply_subst(&mut self, subst: &Substitution) {
        match self {
            Expr::If(c, t, e, ty) => {
                c.apply_subst(subst);
                t.apply_subst(subst);
                e.apply_subst(subst);
                ty.apply_subst(subst);
            }
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
    fn apply_subst(&mut self, subst: &Substitution) {
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
