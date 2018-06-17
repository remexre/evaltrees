use super::*;
use ast::{Expr, Pattern};

#[test]
fn app() {
    let mut evaluator = LazyEvaluation::new(vec![
        Decl {
            name: "f".into(),
            args: vec![
                Pattern::Binding("x".into(), ()),
                Pattern::Binding("y".into(), ()),
            ],
            body: Expr::Op(
                Op::Add,
                Box::new(Expr::Variable("x".into(), ())),
                Box::new(Expr::Variable("y".into(), ())),
                (),
            ),
            aux: (),
        },
        Decl {
            name: "".into(),
            args: vec![],
            body: Expr::Op(
                Op::App,
                Box::new(Expr::Op(
                    Op::App,
                    Box::new(Expr::Variable("f".into(), ())),
                    Box::new(Expr::Op(
                        Op::Add,
                        Box::new(Expr::Literal(Literal::Int(1), ())),
                        Box::new(Expr::Literal(Literal::Int(2), ())),
                        (),
                    )),
                    (),
                )),
                Box::new(Expr::Op(
                    Op::Add,
                    Box::new(Expr::Literal(Literal::Int(3), ())),
                    Box::new(Expr::Literal(Literal::Int(4), ())),
                    (),
                )),
                (),
            ),
            aux: (),
        },
    ]);

    assert!(evaluator.step().is_ok());
    assert_eq!(
        format!("{}", evaluator),
        "App(App(f, Add(1, 2)), Add(3, 4))"
    );
    assert!(!evaluator.normal_form());

    assert!(evaluator.step().is_ok());
    assert_eq!(format!("{}", evaluator), "Add(Add(1, 2), Add(3, 4))");
    assert!(!evaluator.normal_form());

    assert!(evaluator.step().is_ok());
    assert_eq!(format!("{}", evaluator), "Add(3, Add(3, 4))");
    assert!(!evaluator.normal_form());

    assert!(evaluator.step().is_ok());
    assert_eq!(format!("{}", evaluator), "Add(3, 7)");
    assert!(!evaluator.normal_form());

    assert!(evaluator.step().is_ok());
    assert_eq!(format!("{}", evaluator), "10");
    assert!(evaluator.normal_form());
}

#[test]
fn sharing() {
    let mut evaluator = LazyEvaluation::new(vec![
        Decl {
            name: "f".into(),
            args: vec![Pattern::Binding("x".into(), ())],
            body: Expr::Op(
                Op::Add,
                Box::new(Expr::Variable("x".into(), ())),
                Box::new(Expr::Variable("x".into(), ())),
                (),
            ),
            aux: (),
        },
        Decl {
            name: "".into(),
            args: vec![],
            body: Expr::Op(
                Op::App,
                Box::new(Expr::Variable("f".into(), ())),
                Box::new(Expr::Op(
                    Op::App,
                    Box::new(Expr::Variable("f".into(), ())),
                    Box::new(Expr::Op(
                        Op::Mul,
                        Box::new(Expr::Literal(Literal::Int(2), ())),
                        Box::new(Expr::Literal(Literal::Int(3), ())),
                        (),
                    )),
                    (),
                )),
                (),
            ),
            aux: (),
        },
    ]);

    assert!(evaluator.step().is_ok());
    assert_eq!(format!("{}", evaluator), "App(f, App(f, Mul(2, 3)))");
    assert!(!evaluator.normal_form());

    assert!(evaluator.step().is_ok());
    assert_eq!(
        format!("{}", evaluator),
        "Add($0, $0)\n  where $0 = App(f, Mul(2, 3))"
    );
    assert!(!evaluator.normal_form());

    assert!(evaluator.step().is_ok());
    assert_eq!(
        format!("{}", evaluator),
        "Add($0, $0)\n  where $0 = Add($1, $1)\n        $1 = Mul(2, 3)"
    );
    assert!(!evaluator.normal_form());

    assert!(evaluator.step().is_ok());
    assert_eq!(
        format!("{}", evaluator),
        "Add($0, $0)\n  where $0 = Add($1, $1)\n        $1 = 6"
    );
    assert!(!evaluator.normal_form());

    assert!(evaluator.step().is_ok());
    assert_eq!(
        format!("{}", evaluator),
        "Add($0, $0)\n  where $0 = Add(6, 6)"
    );
    assert!(!evaluator.normal_form());

    assert!(evaluator.step().is_ok());
    assert_eq!(format!("{}", evaluator), "Add($0, $0)\n  where $0 = 12");
    assert!(!evaluator.normal_form());

    assert!(evaluator.step().is_ok());
    assert_eq!(format!("{}", evaluator), "Add(12, 12)");
    assert!(!evaluator.normal_form());

    assert!(evaluator.step().is_ok());
    assert_eq!(format!("{}", evaluator), "24");
    assert!(evaluator.normal_form());
}
