use ast::{Literal, Op};
use cst::Expr;

#[test]
fn calls() {
    let expr: Expr = "f x y".parse().unwrap();
    assert_eq!(
        expr,
        Expr::Op(
            Op::App,
            Box::new(Expr::Op(
                Op::App,
                Box::new(Expr::Variable("f".into())),
                Box::new(Expr::Variable("x".into())),
            )),
            Box::new(Expr::Variable("y".into())),
        )
    )
}

#[test]
fn lists() {
    let expr: Expr = "[1; 2; 3]".parse().unwrap();
    assert_eq!(
        expr,
        Expr::List(vec![
            Expr::Literal(Literal::Int(1)),
            Expr::Literal(Literal::Int(2)),
            Expr::Literal(Literal::Int(3)),
        ])
    )
}

#[test]
fn math_precedence() {
    let expr: Expr = "1 + 2 * 3 / 4 - 5 - 6".parse().unwrap();
    assert_eq!(
        expr,
        Expr::Op(
            Op::Sub,
            Box::new(Expr::Op(
                Op::Sub,
                Box::new(Expr::Op(
                    Op::Add,
                    Box::new(Expr::Literal(Literal::Int(1))),
                    Box::new(Expr::Op(
                        Op::Div,
                        Box::new(Expr::Op(
                            Op::Mul,
                            Box::new(Expr::Literal(Literal::Int(2))),
                            Box::new(Expr::Literal(Literal::Int(3))),
                        )),
                        Box::new(Expr::Literal(Literal::Int(4))),
                    )),
                )),
                Box::new(Expr::Literal(Literal::Int(5))),
            )),
            Box::new(Expr::Literal(Literal::Int(6))),
        )
    )
}
