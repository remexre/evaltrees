use ast::{Decl, Expr, Literal, Op};
use cst::{Decl as CstDecl, Expr as CstExpr};

impl From<CstDecl> for Decl {
    fn from(p: CstDecl) -> Decl {
        Decl {
            name: p.name,
            args: p.args,
            body: p.body.into(),
            aux: (),
        }
    }
}

impl From<CstExpr> for Expr {
    fn from(p: CstExpr) -> Expr {
        match p {
            CstExpr::List(mut es) => {
                let mut expr = Expr::Literal(Literal::Nil, ());
                while let Some(e) = es.pop() {
                    expr = Expr::Op(Op::Cons, Box::new(e.into()), Box::new(expr), ());
                }
                expr
            }
            CstExpr::Literal(lit) => Expr::Literal(lit, ()),
            CstExpr::Op(op, l, r) => {
                Expr::Op(op, Box::new(Expr::from(*l)), Box::new(Expr::from(*r)), ())
            }
            CstExpr::Variable(sym) => Expr::Variable(sym, ()),
        }
    }
}
