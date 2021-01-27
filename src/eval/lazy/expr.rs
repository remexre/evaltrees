use std::collections::HashSet;
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::ast::{Expr, Literal, Op, PrintStyle};
use crate::cst::Expr as CstExpr;
use symbol::Symbol;

/// An expression type used only in lazy evaluation, to add the where-bound variables.
#[derive(Clone, Debug, DisplayAttr, PartialEq)]
pub enum LazyExpr {
    /// A conditional expression.
    #[display(fmt = "If({}, {}, {})", _0, _1, _2)]
    If(Box<LazyExpr>, Box<LazyExpr>, Box<LazyExpr>),

    /// A literal value.
    #[display(fmt = "{}", _0)]
    Literal(Literal),

    /// A binary operator.
    #[display(fmt = "{}({}, {})", _0, _1, _2)]
    Op(Op, Box<LazyExpr>, Box<LazyExpr>),

    /// A variable.
    #[display(fmt = "{}", _0)]
    Variable(Symbol),

    /// A where-bound variable.
    #[display(fmt = "${}", _0)]
    WhereVar(usize),
}

impl LazyExpr {
    /// Returns the indices of the where-bound variables.
    pub fn where_vars_indices(&self) -> HashSet<usize> {
        match *self {
            LazyExpr::If(ref c, ref t, ref e) => {
                let mut set = c.where_vars_indices();
                set.extend(t.where_vars_indices());
                set.extend(e.where_vars_indices());
                set
            }
            LazyExpr::Literal(_) => HashSet::new(),
            LazyExpr::Op(_, ref l, ref r) => {
                let mut set = l.where_vars_indices();
                set.extend(r.where_vars_indices());
                set
            }
            LazyExpr::Variable(_) => HashSet::new(),
            LazyExpr::WhereVar(num) => {
                let mut set = HashSet::new();
                set.insert(num);
                set
            }
        }
    }
}

impl LazyExpr {
    /// Applies a replacement to the expression.
    pub fn apply_replacement(&mut self, idx: usize, expr: &LazyExpr) {
        match *self {
            LazyExpr::If(ref mut c, ref mut t, ref mut e) => {
                c.apply_replacement(idx, expr);
                t.apply_replacement(idx, expr);
                e.apply_replacement(idx, expr);
            }
            LazyExpr::Op(_, ref mut l, ref mut r) => {
                l.apply_replacement(idx, expr);
                r.apply_replacement(idx, expr);
            }
            LazyExpr::WhereVar(num) if num == idx => {
                *self = expr.clone();
            }
            LazyExpr::Literal(_) | LazyExpr::Variable(_) | LazyExpr::WhereVar(_) => {}
        }
    }

    /// Returns a Display that follows the given print style.
    pub fn display_as<'a>(&'a self, style: PrintStyle) -> impl 'a + Display {
        struct D<'a>(&'a LazyExpr, PrintStyle);
        impl<'a> Display for D<'a> {
            fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
                match self.1 {
                    PrintStyle::AST => Display::fmt(self.0, fmt),
                    PrintStyle::CST => Display::fmt(&self.0.to_cst(), fmt),
                }
            }
        }

        D(self, style)
    }

    /// Converts the LazyExpr into a CST approximation of itself, for printing.
    pub fn to_cst(&self) -> CstExpr {
        Expr::from(self.clone()).to_cst()
    }
}

impl<Aux> From<Expr<Aux>> for LazyExpr {
    fn from(expr: Expr<Aux>) -> LazyExpr {
        match expr {
            Expr::If(c, t, e, _) => LazyExpr::If(
                Box::new((*c).into()),
                Box::new((*t).into()),
                Box::new((*e).into()),
            ),
            Expr::Literal(lit, _) => LazyExpr::Literal(lit),
            Expr::Op(op, l, r, _) => LazyExpr::Op(op, Box::new((*l).into()), Box::new((*r).into())),
            Expr::Variable(name, _) => LazyExpr::Variable(name),
        }
    }
}

impl From<LazyExpr> for Expr<()> {
    fn from(expr: LazyExpr) -> Expr<()> {
        match expr {
            LazyExpr::If(c, t, e) => Expr::If(
                Box::new((*c).into()),
                Box::new((*t).into()),
                Box::new((*e).into()),
                (),
            ),
            LazyExpr::Literal(lit) => Expr::Literal(lit, ()),
            LazyExpr::Op(op, l, r) => {
                Expr::Op(op, Box::new((*l).into()), Box::new((*r).into()), ())
            }
            LazyExpr::Variable(name) => Expr::Variable(name, ()),
            LazyExpr::WhereVar(num) => Expr::Variable(format!("${}", num).into(), ()),
        }
    }
}
