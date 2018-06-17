use ast::{Literal, Op};
use cst::Expr;

#[test]
fn call() {
    assert_eq!(
        "f (g h) i j",
        format!(
            "{}",
            Expr::Op(
                Op::App,
                Box::new(Expr::Op(
                    Op::App,
                    Box::new(Expr::Op(
                        Op::App,
                        Box::new(Expr::Variable("f".into())),
                        Box::new(Expr::Op(
                            Op::App,
                            Box::new(Expr::Variable("g".into())),
                            Box::new(Expr::Variable("h".into())),
                        )),
                    )),
                    Box::new(Expr::Variable("i".into())),
                )),
                Box::new(Expr::Variable("j".into())),
            )
        )
    );
}

#[test]
fn if_prec() {
    assert_eq!(
        "if b then 1 else 2 + 3",
        format!(
            "{}",
            Expr::If(
                Box::new(Expr::Variable("b".into())),
                Box::new(Expr::Literal(Literal::Int(1))),
                Box::new(Expr::Op(
                    Op::Add,
                    Box::new(Expr::Literal(Literal::Int(2))),
                    Box::new(Expr::Literal(Literal::Int(3))),
                )),
            )
        )
    );

    assert_eq!(
        "(if b then 1 else 2) + 3",
        format!(
            "{}",
            Expr::Op(
                Op::Add,
                Box::new(Expr::If(
                    Box::new(Expr::Variable("b".into())),
                    Box::new(Expr::Literal(Literal::Int(1))),
                    Box::new(Expr::Literal(Literal::Int(2))),
                )),
                Box::new(Expr::Literal(Literal::Int(3))),
            ),
        )
    );
}

#[test]
fn math_noparens() {
    assert_eq!(
        "1 * 2 + 3 * 4",
        format!(
            "{}",
            Expr::Op(
                Op::Add,
                Box::new(Expr::Op(
                    Op::Mul,
                    Box::new(Expr::Literal(Literal::Int(1))),
                    Box::new(Expr::Literal(Literal::Int(2))),
                )),
                Box::new(Expr::Op(
                    Op::Mul,
                    Box::new(Expr::Literal(Literal::Int(3))),
                    Box::new(Expr::Literal(Literal::Int(4))),
                )),
            )
        )
    );
}

#[test]
fn math_parens() {
    assert_eq!(
        "(1 + 2) * (3 + 4)",
        format!(
            "{}",
            Expr::Op(
                Op::Mul,
                Box::new(Expr::Op(
                    Op::Add,
                    Box::new(Expr::Literal(Literal::Int(1))),
                    Box::new(Expr::Literal(Literal::Int(2))),
                )),
                Box::new(Expr::Op(
                    Op::Add,
                    Box::new(Expr::Literal(Literal::Int(3))),
                    Box::new(Expr::Literal(Literal::Int(4))),
                )),
            )
        )
    );
}

#[test]
fn subtraction() {
    assert_eq!(
        "1 - 2 - 3 - 4",
        format!(
            "{}",
            Expr::Op(
                Op::Sub,
                Box::new(Expr::Op(
                    Op::Sub,
                    Box::new(Expr::Op(
                        Op::Sub,
                        Box::new(Expr::Literal(Literal::Int(1))),
                        Box::new(Expr::Literal(Literal::Int(2))),
                    )),
                    Box::new(Expr::Literal(Literal::Int(3))),
                )),
                Box::new(Expr::Literal(Literal::Int(4))),
            )
        )
    );

    assert_eq!(
        "1 - (2 - (3 - 4))",
        format!(
            "{}",
            Expr::Op(
                Op::Sub,
                Box::new(Expr::Literal(Literal::Int(1))),
                Box::new(Expr::Op(
                    Op::Sub,
                    Box::new(Expr::Literal(Literal::Int(2))),
                    Box::new(Expr::Op(
                        Op::Sub,
                        Box::new(Expr::Literal(Literal::Int(3))),
                        Box::new(Expr::Literal(Literal::Int(4))),
                    )),
                )),
            )
        )
    );
}
