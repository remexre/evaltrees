use std::collections::BTreeMap;
use std::sync::atomic::{AtomicUsize, Ordering};

use typeck::Ty;

/// A variable present in a substitution.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SubstVar(usize);

impl SubstVar {
    /// Generates a fresh variable.
    pub fn gensym() -> SubstVar {
        lazy_static! {
            pub static ref next: AtomicUsize = AtomicUsize::default();
        }
        let n = next.fetch_add(1, Ordering::SeqCst);
        SubstVar(n)
    }
}

/// A substitution.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Substitution(BTreeMap<SubstVar, Ty>);

impl Substitution {
    /// Looks up the type corresponding to the given variable.
    pub fn lookup(&self, var: SubstVar) -> Option<Ty> {
        self.0.get(&var).map(|ty| ty.clone())
    }
}
