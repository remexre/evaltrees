use std::collections::HashSet;

use ast::{Expr, Literal, Op};
use cst::Expr as CstExpr;
use symbol::Symbol;

/// An expression type used only in lazy evaluation, to add the where-bound variables.
#[derive(Clone, Debug, DisplayAttr, PartialEq)]
pub enum LazyExpr<Aux> {
    /// A conditional expression.
    #[display(fmt = "If({}, {}, {})", _0, _1, _2)]
    If(
        Box<LazyExpr<Aux>>,
        Box<LazyExpr<Aux>>,
        Box<LazyExpr<Aux>>,
        Aux,
    ),

    /// A literal value.
    #[display(fmt = "{}", _0)]
    Literal(Literal, Aux),

    /// A binary operator.
    #[display(fmt = "{}({}, {})", _0, _1, _2)]
    Op(Op, Box<LazyExpr<Aux>>, Box<LazyExpr<Aux>>, Aux),

    /// A variable.
    #[display(fmt = "{}", _0)]
    Variable(Symbol, Aux),

    /// A where-bound variable.
    #[display(fmt = "${}", _0)]
    WhereVar(usize, Aux),
}

impl<Aux> LazyExpr<Aux> {
    /// Returns the indices of the where-bound variables.
    pub fn where_vars_indices(&self) -> HashSet<usize> {
        match *self {
            LazyExpr::If(ref c, ref t, ref e, _) => {
                let mut set = c.where_vars_indices();
                set.extend(t.where_vars_indices());
                set.extend(e.where_vars_indices());
                set
            }
            LazyExpr::Literal(_, _) => HashSet::new(),
            LazyExpr::Op(_, ref l, ref r, _) => {
                let mut set = l.where_vars_indices();
                set.extend(r.where_vars_indices());
                set
            }
            LazyExpr::Variable(_, _) => HashSet::new(),
            LazyExpr::WhereVar(num, _) => {
                let mut set = HashSet::new();
                set.insert(num);
                set
            }
        }
    }
}

impl<Aux: Clone> LazyExpr<Aux> {
    /// Applies a replacement to the expression.
    pub fn apply_replacement(&mut self, idx: usize, expr: &LazyExpr<Aux>) {
        match *self {
            LazyExpr::If(ref mut c, ref mut t, ref mut e, _) => {
                c.apply_replacement(idx, expr);
                t.apply_replacement(idx, expr);
                e.apply_replacement(idx, expr);
            }
            LazyExpr::Op(_, ref mut l, ref mut r, _) => {
                l.apply_replacement(idx, expr);
                r.apply_replacement(idx, expr);
            }
            LazyExpr::WhereVar(num, _) if num == idx => {
                *self = expr.clone();
            }
            LazyExpr::Literal(_, _) | LazyExpr::Variable(_, _) | LazyExpr::WhereVar(_, _) => {}
        }
    }

    /// Converts the LazyExpr into a CST approximation of itself, for printing.
    pub fn to_cst(&self) -> CstExpr {
        Expr::from(self.clone()).to_cst()
    }

    /// Gets the auxiliary data as a reference.
    pub fn aux_ref(&self) -> &Aux {
        match *self {
            LazyExpr::If(_, _, _, ref aux)
            | LazyExpr::Literal(_, ref aux)
            | LazyExpr::Op(_, _, _, ref aux)
            | LazyExpr::Variable(_, ref aux)
            | LazyExpr::WhereVar(_, ref aux) => aux,
        }
    }
}

impl<Aux> From<Expr<Aux>> for LazyExpr<Aux> {
    fn from(expr: Expr<Aux>) -> LazyExpr<Aux> {
        match expr {
            Expr::If(c, t, e, aux) => LazyExpr::If(
                Box::new((*c).into()),
                Box::new((*t).into()),
                Box::new((*e).into()),
                aux,
            ),
            Expr::Literal(lit, aux) => LazyExpr::Literal(lit, aux),
            Expr::Op(op, l, r, aux) => {
                LazyExpr::Op(op, Box::new((*l).into()), Box::new((*r).into()), aux)
            }
            Expr::Variable(name, aux) => LazyExpr::Variable(name, aux),
        }
    }
}

impl<Aux> From<LazyExpr<Aux>> for Expr<Aux> {
    fn from(expr: LazyExpr<Aux>) -> Expr<Aux> {
        match expr {
            LazyExpr::If(c, t, e, aux) => Expr::If(
                Box::new((*c).into()),
                Box::new((*t).into()),
                Box::new((*e).into()),
                aux,
            ),
            LazyExpr::Literal(lit, aux) => Expr::Literal(lit, aux),
            LazyExpr::Op(op, l, r, aux) => {
                Expr::Op(op, Box::new((*l).into()), Box::new((*r).into()), aux)
            }
            LazyExpr::Variable(name, aux) => Expr::Variable(name, aux),
            LazyExpr::WhereVar(num, aux) => Expr::Variable(format!("${}", num).into(), aux),
        }
    }
}
