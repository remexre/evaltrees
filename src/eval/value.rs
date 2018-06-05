use failure::Error;

use ast::{Decl, Expr, Literal, Op, Type};
use eval::util::reducible;
use eval::Evaluator;

/// Call-by-value evaluation.
#[derive(Debug, DisplayAttr)]
#[display(fmt = "{}", arg = "expr.as_ref().unwrap()")]
pub struct CallByValue {
    decls: Vec<Decl<Type>>,
    expr: Option<Expr<Type>>,
}

impl CallByValue {
    /// Creates a call-by-value interpreter from a list of declarations.
    /// The nameless declaration will be the expression.
    pub fn new(decls: Vec<Decl<Type>>) -> CallByValue {
        let expr = {
            let nameless = decls
                .iter()
                .find(|decl| decl.name == "".into())
                .expect("Nameless declaration missing");
            Expr::Variable("".into(), nameless.aux_ref().clone())
        };
        CallByValue {
            decls,
            expr: Some(expr),
        }
    }
}

impl Evaluator for CallByValue {
    fn normal_form(&self) -> bool {
        !reducible(self.expr.as_ref().unwrap(), &self.decls)
    }

    fn step(&mut self) -> Result<(), Error> {
        let expr = step(self.expr.take().unwrap(), &self.decls)?;
        self.expr = Some(expr);
        Ok(())
    }
}

fn step(expr: Expr<Type>, decls: &[Decl<Type>]) -> Result<Expr<Type>, Error> {
    let expr = match expr {
        Expr::If(c, t, e, ty) => match *c {
            Expr::Literal(Literal::True, _) => *t,
            Expr::Literal(Literal::False, _) => *e,
            c => Expr::If(Box::new(step(c, decls)?), t, e, ty),
        },
        Expr::Literal(l, ty) => Expr::Literal(l, ty),
        Expr::Op(Op::App, l, r, ty) => unimplemented!(),
        Expr::Op(Op::Cons, l, r, ty) => if reducible(&l, decls) {
            Expr::Op(Op::Cons, Box::new(step(*l, decls)?), r, ty)
        } else {
            Expr::Op(Op::Cons, l, Box::new(step(*r, decls)?), ty)
        },
        Expr::Op(Op::Add, l, r, ty) => math_op(|l, r| Ok(l + r), Op::Add, l, r, ty, decls)?,
        Expr::Op(Op::Sub, l, r, ty) => math_op(|l, r| Ok(l - r), Op::Sub, l, r, ty, decls)?,
        Expr::Op(Op::Mul, l, r, ty) => math_op(|l, r| Ok(l * r), Op::Mul, l, r, ty, decls)?,
        Expr::Op(Op::Div, l, r, ty) => math_op(|l, r| Ok(l / r), Op::Div, l, r, ty, decls)?,
        Expr::Op(Op::Mod, l, r, ty) => math_op(|l, r| Ok(l % r), Op::Mod, l, r, ty, decls)?,
        Expr::Variable(var, ty) => {
            let decl = decls
                .iter()
                .find(|decl| decl.name == var)
                .unwrap_or_else(|| panic!("Unknown variable {}", var));
            if decl.args.is_empty() {
                decl.body.clone()
            } else {
                Expr::Variable(var, ty)
            }
        }
    };
    Ok(expr)
}

fn math_op<F: Fn(usize, usize) -> Result<usize, Error>>(
    f: F,
    op: Op,
    l: Box<Expr<Type>>,
    r: Box<Expr<Type>>,
    ty: Type,
    decls: &[Decl<Type>],
) -> Result<Expr<Type>, Error> {
    if let Expr::Literal(Literal::Int(ln), _) = *l {
        if let Expr::Literal(Literal::Int(rn), _) = *r {
            f(ln, rn).map(|n| Expr::Literal(Literal::Int(n), ty))
        } else {
            Ok(Expr::Op(op, l, Box::new(step(*r, decls)?), ty))
        }
    } else {
        Ok(Expr::Op(op, Box::new(step(*l, decls)?), r, ty))
    }
}
