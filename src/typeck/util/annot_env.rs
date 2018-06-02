use std::rc::Rc;

use symbol::Symbol;

use typeck::ty::Ty;

/// A lexical environment used when assigning type variables.
///
/// Supports O(1) cloning.
#[derive(Clone, Debug)]
pub struct AnnotEnv {
    // The option allows taking from inner easier.
    inner: Option<Rc<EnvInner>>,
}

#[derive(Clone, Debug)]
pub enum EnvInner {
    Cons(Symbol, Ty, Rc<EnvInner>),
    Nil,
}

impl AnnotEnv {
    /// Returns the value associated with a name, unless it does not exist.
    /// In that case, a fresh type variable is created and added to the
    /// environment.
    pub fn get(&mut self, name: Symbol) -> Ty {
        let mut cur: Rc<EnvInner> = self.inner.clone().unwrap();
        loop {
            let next = match *cur {
                EnvInner::Cons(n, ref ty, ref tl) => {
                    if n == name {
                        return ty.clone();
                    } else {
                        tl.clone()
                    }
                }
                EnvInner::Nil => {
                    let ty = Ty::fresh();
                    self.put(name, ty.clone());
                    return ty;
                }
            };
            cur = next;
        }
    }

    /// Creates a new AnnotEnv.
    pub fn new() -> AnnotEnv {
        AnnotEnv {
            inner: Some(Rc::new(EnvInner::Nil)),
        }
    }

    /// Creates a new binding.
    pub fn put(&mut self, name: Symbol, ty: Ty) {
        let inner = self.inner.take().unwrap();
        self.inner = Some(Rc::new(EnvInner::Cons(name, ty, inner)));
    }
}
