use ast::{Decl, Expr, Pattern};
use typeck::{ty::Ty, util::AnnotEnv};

impl Decl<()> {
    pub(in typeck) fn add_type_annotations(self, env: &mut AnnotEnv) -> Decl<Ty> {
        let ty = env.get(self.name);
        let args = self
            .args
            .into_iter()
            .map(|a| a.add_type_annotations(env))
            .collect();
        let body = self.body.add_type_annotations(env);
        Decl {
            name: self.name,
            args,
            body,
            aux: ty.clone(),
        }
    }
}

impl Expr<()> {
    pub(in typeck) fn add_type_annotations(self, env: &mut AnnotEnv) -> Expr<Ty> {
        match self {
            Expr::If(c, t, e, ()) => Expr::If(
                Box::new(c.add_type_annotations(env)),
                Box::new(t.add_type_annotations(env)),
                Box::new(e.add_type_annotations(env)),
                Ty::fresh(),
            ),
            Expr::Literal(l, ()) => Expr::Literal(l, Ty::fresh()),
            Expr::Op(op, l, r, ()) => Expr::Op(
                op,
                Box::new(l.add_type_annotations(env)),
                Box::new(r.add_type_annotations(env)),
                Ty::fresh(),
            ),
            Expr::Variable(n, ()) => Expr::Variable(n, env.get(n)),
        }
    }
}

impl Pattern<()> {
    fn add_type_annotations(self, env: &mut AnnotEnv) -> Pattern<Ty> {
        match self {
            Pattern::Binding(n, ()) => {
                let ty = Ty::fresh();
                env.put(n, ty.clone());
                Pattern::Binding(n, ty)
            }
            Pattern::Cons(l, r, ()) => Pattern::Cons(
                Box::new(l.add_type_annotations(env)),
                Box::new(r.add_type_annotations(env)),
                Ty::fresh(),
            ),
            Pattern::Literal(l, ()) => Pattern::Literal(l, Ty::fresh()),
        }
    }
}
