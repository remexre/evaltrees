use super::*;
use ast::Pattern;

#[test]
fn app() {
    let mut evaluator = CallByName::new(vec![
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
fn id_int() {
    let mut evaluator = CallByName::new(vec![
        Decl {
            name: "f".into(),
            args: vec![Pattern::Literal(Literal::Int(0), ())],
            body: Expr::Literal(Literal::Int(0), ()),
            aux: (),
        },
        Decl {
            name: "f".into(),
            args: vec![Pattern::Binding("x".into(), ())],
            body: Expr::Op(
                Op::Add,
                Box::new(Expr::Literal(Literal::Int(1), ())),
                Box::new(Expr::Op(
                    Op::App,
                    Box::new(Expr::Variable("f".into(), ())),
                    Box::new(Expr::Op(
                        Op::Sub,
                        Box::new(Expr::Variable("x".into(), ())),
                        Box::new(Expr::Literal(Literal::Int(1), ())),
                        (),
                    )),
                    (),
                )),
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
                Box::new(Expr::Literal(Literal::Int(1), ())),
                (),
            ),
            aux: (),
        },
    ]);

    assert!(evaluator.step().is_ok());
    assert_eq!(format!("{}", evaluator), "App(f, 1)");
    assert!(!evaluator.normal_form());

    assert!(evaluator.step().is_ok());
    assert_eq!(format!("{}", evaluator), "Add(1, App(f, Sub(1, 1)))");
    assert!(!evaluator.normal_form());

    assert!(evaluator.step().is_ok());
    assert_eq!(format!("{}", evaluator), "Add(1, App(f, 0))");
    assert!(!evaluator.normal_form());

    assert!(evaluator.step().is_ok());
    assert_eq!(format!("{}", evaluator), "Add(1, 0)");
    assert!(!evaluator.normal_form());

    assert!(evaluator.step().is_ok());
    assert_eq!(format!("{}", evaluator), "1");
    assert!(evaluator.normal_form());
}

#[test]
fn strictness() {
    let mut evaluator = CallByName::new(vec![
        Decl {
            name: "f".into(),
            args: vec![
                Pattern::Literal(Literal::Nil, ()),
                Pattern::Binding("y".into(), ()),
            ],
            body: Expr::Variable("y".into(), ()),
            aux: (),
        },
        Decl {
            name: "f".into(),
            args: vec![
                Pattern::Cons(
                    Box::new(Pattern::Binding("h".into(), ())),
                    Box::new(Pattern::Binding("t".into(), ())),
                    (),
                ),
                Pattern::Binding("y".into(), ()),
            ],
            body: Expr::Op(
                Op::Add,
                Box::new(Expr::Variable("h".into(), ())),
                Box::new(Expr::Op(
                    Op::App,
                    Box::new(Expr::Op(
                        Op::App,
                        Box::new(Expr::Variable("f".into(), ())),
                        Box::new(Expr::Variable("t".into(), ())),
                        (),
                    )),
                    Box::new(Expr::Variable("y".into(), ())),
                    (),
                )),
                (),
            ),
            aux: (),
        },
        Decl {
            name: "g".into(),
            args: vec![Pattern::Binding("x".into(), ())],
            body: Expr::Op(
                Op::Cons,
                Box::new(Expr::Variable("x".into(), ())),
                Box::new(Expr::Literal(Literal::Nil, ())),
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
                        Op::App,
                        Box::new(Expr::Variable("g".into(), ())),
                        Box::new(Expr::Op(
                            Op::Add,
                            Box::new(Expr::Literal(Literal::Int(1), ())),
                            Box::new(Expr::Literal(Literal::Int(2), ())),
                            (),
                        )),
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
        "App(App(f, App(g, Add(1, 2))), Add(3, 4))"
    );
    assert!(!evaluator.normal_form());

    assert!(evaluator.step().is_ok());
    assert_eq!(
        format!("{}", evaluator),
        "App(App(f, Cons(Add(1, 2), [])), Add(3, 4))"
    );
    assert!(!evaluator.normal_form());

    assert!(evaluator.step().is_ok());
    assert_eq!(
        format!("{}", evaluator),
        "Add(Add(1, 2), App(App(f, []), Add(3, 4)))"
    );
    assert!(!evaluator.normal_form());

    assert!(evaluator.step().is_ok());
    assert_eq!(
        format!("{}", evaluator),
        "Add(3, App(App(f, []), Add(3, 4)))"
    );
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
