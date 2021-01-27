use std::collections::BTreeSet;

use symbol::Symbol;

use crate::ast::{Decl, Expr, Literal, Op, Pattern};
use crate::cst::{Decl as CstDecl, Expr as CstExpr};

/// An error converting a concrete syntax tree to an abstract syntax tree.
/// These, generally, are errors which are valid syntax but have ill-defined
/// or invalid semantics.
#[derive(Clone, Debug, Fail, PartialEq)]
pub enum ASTConversionError {
    #[fail(display = "Found duplicate variable `{}' in arguments to `{}'.", _1, _0)]
    DuplicateArgVar(Symbol, Symbol),
}

/// Checks that the arguments contain no duplicate variables.
pub fn check_args<Aux>(args: &[Pattern<Aux>]) -> Result<(), Symbol> {
    let mut vars = BTreeSet::new();

    fn check_arg<Aux>(vars: &mut BTreeSet<Symbol>, arg: &Pattern<Aux>) -> Result<(), Symbol> {
        match *arg {
            Pattern::Binding(n, _) => if vars.insert(n) {
                Ok(())
            } else {
                Err(n)
            },
            Pattern::Cons(ref l, ref r, _) => {
                check_arg(vars, l)?;
                check_arg(vars, r)
            }
            Pattern::Literal(_, _) => Ok(()),
        }
    }

    for arg in args {
        check_arg(&mut vars, arg)?;
    }
    Ok(())
}

impl<Aux> Decl<Aux> {
    /// Converts the AST decl back to a CST decl.
    pub fn to_cst(&self) -> CstDecl {
        fn remove_aux_from_pattern<Aux>(pat: &Pattern<Aux>) -> Pattern<()> {
            match *pat {
                Pattern::Binding(var, _) => Pattern::Binding(var, ()),
                Pattern::Cons(ref l, ref r, _) => Pattern::Cons(
                    Box::new(remove_aux_from_pattern(l)),
                    Box::new(remove_aux_from_pattern(r)),
                    (),
                ),
                Pattern::Literal(lit, _) => Pattern::Literal(lit, ()),
            }
        }

        CstDecl {
            name: self.name,
            args: self.args.iter().map(remove_aux_from_pattern).collect(),
            body: self.body.to_cst(),
        }
    }
}

impl CstDecl {
    /// Converts a CST decl to an AST decl.
    pub fn into_ast(self) -> Result<Decl<()>, ASTConversionError> {
        check_args(&self.args).map_err(|n| ASTConversionError::DuplicateArgVar(self.name, n))?;
        Ok(Decl {
            name: self.name,
            args: self.args,
            body: self.body.into_ast()?,
            aux: (),
        })
    }
}

impl<Aux> Expr<Aux> {
    /// Converts the AST expression back to a CST expression.
    pub fn to_cst(&self) -> CstExpr {
        match *self {
            Expr::If(ref c, ref t, ref e, _) => CstExpr::If(
                Box::new(c.to_cst()),
                Box::new(t.to_cst()),
                Box::new(e.to_cst()),
            ),
            Expr::Literal(l, _) => CstExpr::Literal(l),
            Expr::Op(op, ref l, ref r, _) => {
                CstExpr::Op(op, Box::new(l.to_cst()), Box::new(r.to_cst()))
            }
            Expr::Variable(var, _) => CstExpr::Variable(var),
        }
    }
}

impl CstExpr {
    /// Converts a CST decl to an AST decl.
    pub fn into_ast(self) -> Result<Expr<()>, ASTConversionError> {
        match self {
            CstExpr::If(c, t, e) => Ok(Expr::If(
                Box::new(c.into_ast()?),
                Box::new(t.into_ast()?),
                Box::new(e.into_ast()?),
                (),
            )),
            CstExpr::List(mut es) => {
                let mut expr = Expr::Literal(Literal::Nil, ());
                while let Some(e) = es.pop() {
                    expr = Expr::Op(Op::Cons, Box::new(e.into_ast()?), Box::new(expr), ());
                }
                Ok(expr)
            }
            CstExpr::Literal(lit) => Ok(Expr::Literal(lit, ())),
            CstExpr::Op(op, l, r) => Ok(Expr::Op(
                op,
                Box::new(l.into_ast()?),
                Box::new(r.into_ast()?),
                (),
            )),
            CstExpr::Variable(sym) => Ok(Expr::Variable(sym, ())),
        }
    }
}
