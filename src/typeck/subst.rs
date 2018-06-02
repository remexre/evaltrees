use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

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
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Substitution(HashMap<SubstVar, Ty>);

impl Substitution {
    /// Adds a substitution.
    pub fn add(&mut self, var: SubstVar, ty: Ty) {
        let mut subst = HashMap::new();
        subst.insert(var, ty.clone());
        let subst = Substitution(subst);
        for ty_ in self.0.values_mut() {
            let ty = ty_.clone().apply_subst(&subst);
            *ty_ = ty;
        }
        if let Some(prev) = self.0.insert(var, ty.clone()) {
            warn!("Replacing {:?} with {:?}; this may be a bug?", prev, ty);
        }
    }

    /// Looks up the type corresponding to the given variable.
    pub fn get(&self, var: SubstVar) -> Option<Ty> {
        self.0.get(&var).map(|ty| ty.clone())
    }
}
