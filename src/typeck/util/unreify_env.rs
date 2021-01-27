use std::rc::Rc;

use crate::typeck::subst::SubstVar;

/// A lexical environment used when unreifying types.
///
/// Supports O(1) cloning.
#[derive(Clone, Debug)]
pub struct UnreifyEnv {
    // The option allows taking from inner easier.
    inner: Option<Rc<EnvInner>>,
}

#[derive(Clone, Debug)]
pub enum EnvInner {
    Cons(SubstVar, Rc<EnvInner>),
    Nil,
}

impl EnvInner {
    fn head(&self) -> SubstVar {
        match *self {
            EnvInner::Cons(v, _) => v,
            EnvInner::Nil => panic!("Took head of nil!"),
        }
    }

    fn tail(&self) -> &EnvInner {
        match *self {
            EnvInner::Cons(_, ref tl) => &**tl,
            EnvInner::Nil => panic!("Took tail of nil!"),
        }
    }
}

impl UnreifyEnv {
    /// Creates a new, empty UnreifyEnv.
    pub fn new() -> UnreifyEnv {
        UnreifyEnv {
            inner: Some(Rc::new(EnvInner::Nil)),
        }
    }

    /// Gets the variable associated with a given index. Panics if
    /// out-of-bounds.
    pub fn get(&self, n: usize) -> SubstVar {
        let mut cur: &EnvInner = self.inner.as_ref().unwrap();
        for _ in 0..n {
            cur = cur.tail();
        }
        cur.head()
    }

    /// Pushes a new variable onto the UnreifyEnv.
    pub fn push(&mut self, val: SubstVar) {
        let inner = self.inner.take().unwrap();
        self.inner = Some(Rc::new(EnvInner::Cons(val, inner)));
    }
}
