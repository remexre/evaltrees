use symbol::Symbol;

use ast::{Decl, Expr, Literal, Op, Pattern};
use cst::{Decl as CstDecl, Expr as CstExpr};

/// An error converting a concrete syntax tree to an abstract syntax tree.
/// These, generally, are errors which are valid syntax but have ill-defined
/// or invalid semantics.
#[derive(Clone, Debug, Fail, PartialEq)]
pub enum ASTConversionError {
    #[fail(display = "Found duplicate variable `{}' in arguments to `{}'.", _1, _0)]
    DuplicateArgVar(Symbol, Symbol),
}

/// Checks that the arguments contain no duplicate variables.
pub fn check_args(args: Vec<Pattern>) -> Result<Vec<Pattern>, ASTConversionError> {
    unimplemented!()
}

impl CstDecl {
    /// Converts a CST decl to an AST decl.
    pub fn to_ast(self) -> Result<Decl, ASTConversionError> {
        let args = check_args(self.args)?;
        Ok(Decl {
            name: self.name,
            args,
            body: self.body.to_ast()?,
            aux: (),
        })
    }
}

impl CstExpr {
    /// Converts a CST decl to an AST decl.
    pub fn to_ast(self) -> Result<Expr, ASTConversionError> {
        match self {
            CstExpr::List(mut es) => {
                let mut expr = Expr::Literal(Literal::Nil, ());
                while let Some(e) = es.pop() {
                    expr = Expr::Op(Op::Cons, Box::new(e.to_ast()?), Box::new(expr), ());
                }
                Ok(expr)
            }
            CstExpr::Literal(lit) => Ok(Expr::Literal(lit, ())),
            CstExpr::Op(op, l, r) => Ok(Expr::Op(
                op,
                Box::new(l.to_ast()?),
                Box::new(r.to_ast()?),
                (),
            )),
            CstExpr::Variable(sym) => Ok(Expr::Variable(sym, ())),
        }
    }
}
