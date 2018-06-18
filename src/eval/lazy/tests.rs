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

    evaluator.step().unwrap();
    assert_eq!(
        format!("{}", evaluator),
        "App(App(f, Add(1, 2)), Add(3, 4))"
    );
    assert!(!evaluator.normal_form());

    evaluator.step().unwrap();
    assert_eq!(format!("{}", evaluator), "Add(Add(1, 2), Add(3, 4))");
    assert!(!evaluator.normal_form());

    evaluator.step().unwrap();
    assert_eq!(format!("{}", evaluator), "Add(3, Add(3, 4))");
    assert!(!evaluator.normal_form());

    evaluator.step().unwrap();
    assert_eq!(format!("{}", evaluator), "Add(3, 7)");
    assert!(!evaluator.normal_form());

    evaluator.step().unwrap();
    assert_eq!(format!("{}", evaluator), "10");
    assert!(evaluator.normal_form());
}

#[test]
fn id_int() {
    let mut evaluator = LazyEvaluation::new(vec![
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

    evaluator.step().unwrap();
    assert_eq!(format!("{}", evaluator), "App(f, 1)");
    assert!(!evaluator.normal_form());

    evaluator.step().unwrap();
    assert_eq!(format!("{}", evaluator), "Add(1, App(f, Sub(1, 1)))");
    assert!(!evaluator.normal_form());

    evaluator.step().unwrap();
    assert_eq!(format!("{}", evaluator), "Add(1, App(f, 0))");
    assert!(!evaluator.normal_form());

    evaluator.step().unwrap();
    assert_eq!(format!("{}", evaluator), "Add(1, 0)");
    assert!(!evaluator.normal_form());

    evaluator.step().unwrap();
    assert_eq!(format!("{}", evaluator), "1");
    assert!(evaluator.normal_form());
}

#[test]
fn infinite() {
    let mut evaluator = LazyEvaluation::new(vec![
        Decl {
            name: "ones".into(),
            args: vec![],
            body: Expr::Op(
                Op::Cons,
                Box::new(Expr::Literal(Literal::Int(1), ())),
                Box::new(Expr::Variable("ones".into(), ())),
                (),
            ),
            aux: (),
        },
        Decl {
            name: "take".into(),
            args: vec![
                Pattern::Literal(Literal::Int(0), ()),
                Pattern::Binding("l".into(), ()),
            ],
            body: Expr::Literal(Literal::Nil, ()),
            aux: (),
        },
        Decl {
            name: "take".into(),
            args: vec![
                Pattern::Binding("n".into(), ()),
                Pattern::Cons(
                    Box::new(Pattern::Binding("h".into(), ())),
                    Box::new(Pattern::Binding("t".into(), ())),
                    (),
                ),
            ],
            body: Expr::Op(
                Op::Cons,
                Box::new(Expr::Variable("h".into(), ())),
                Box::new(Expr::Op(
                    Op::App,
                    Box::new(Expr::Op(
                        Op::App,
                        Box::new(Expr::Variable("take".into(), ())),
                        Box::new(Expr::Op(
                            Op::Sub,
                            Box::new(Expr::Variable("n".into(), ())),
                            Box::new(Expr::Literal(Literal::Int(1), ())),
                            (),
                        )),
                        (),
                    )),
                    Box::new(Expr::Variable("t".into(), ())),
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
                Box::new(Expr::Op(
                    Op::App,
                    Box::new(Expr::Variable("take".into(), ())),
                    Box::new(Expr::Literal(Literal::Int(1), ())),
                    (),
                )),
                Box::new(Expr::Variable("ones".into(), ())),
                (),
            ),
            aux: (),
        },
    ]);

    evaluator.step().unwrap();
    assert_eq!(format!("{}", evaluator), "App(App(take, 1), ones)");
    assert!(!evaluator.normal_form());

    evaluator.step().unwrap();
    assert_eq!(format!("{}", evaluator), "App(App(take, 1), Cons(1, ones))");
    assert!(!evaluator.normal_form());

    evaluator.step().unwrap();
    assert_eq!(
        format!("{}", evaluator),
        "Cons(1, App(App(take, Sub(1, 1)), ones))"
    );
    assert!(!evaluator.normal_form());

    evaluator.step().unwrap();
    assert_eq!(format!("{}", evaluator), "Cons(1, App(App(take, 0), ones))");
    assert!(!evaluator.normal_form());

    evaluator.step().unwrap();
    assert_eq!(format!("{}", evaluator), "Cons(1, [])");
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

    evaluator.step().unwrap();
    assert_eq!(format!("{}", evaluator), "App(f, App(f, Mul(2, 3)))");
    assert!(!evaluator.normal_form());

    evaluator.step().unwrap();
    assert_eq!(
        format!("{}", evaluator),
        "Add($0, $0)\n  where $0 = App(f, Mul(2, 3))"
    );
    assert!(!evaluator.normal_form());

    evaluator.step().unwrap();
    assert_eq!(
        format!("{}", evaluator),
        "Add($0, $0)\n  where $0 = Add($1, $1)\n        $1 = Mul(2, 3)"
    );
    assert!(!evaluator.normal_form());

    evaluator.step().unwrap();
    assert_eq!(
        format!("{}", evaluator),
        "Add($0, $0)\n  where $0 = Add($1, $1)\n        $1 = 6"
    );
    assert!(!evaluator.normal_form());

    evaluator.step().unwrap();
    assert_eq!(
        format!("{}", evaluator),
        "Add($0, $0)\n  where $0 = Add(6, 6)"
    );
    assert!(!evaluator.normal_form());

    evaluator.step().unwrap();
    assert_eq!(format!("{}", evaluator), "Add($0, $0)\n  where $0 = 12");
    assert!(!evaluator.normal_form());

    evaluator.step().unwrap();
    assert_eq!(format!("{}", evaluator), "Add(12, 12)");
    assert!(!evaluator.normal_form());

    evaluator.step().unwrap();
    assert_eq!(format!("{}", evaluator), "24");
    assert!(evaluator.normal_form());
}

#[test]
fn strictness() {
    let mut evaluator = LazyEvaluation::new(vec![
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

    evaluator.step().unwrap();
    assert_eq!(
        format!("{}", evaluator),
        "App(App(f, App(g, Add(1, 2))), Add(3, 4))"
    );
    assert!(!evaluator.normal_form());

    evaluator.step().unwrap();
    assert_eq!(
        format!("{}", evaluator),
        "App(App(f, Cons(Add(1, 2), [])), Add(3, 4))"
    );
    assert!(!evaluator.normal_form());

    evaluator.step().unwrap();
    assert_eq!(
        format!("{}", evaluator),
        "Add(Add(1, 2), App(App(f, []), Add(3, 4)))"
    );
    assert!(!evaluator.normal_form());

    evaluator.step().unwrap();
    assert_eq!(
        format!("{}", evaluator),
        "Add(3, App(App(f, []), Add(3, 4)))"
    );
    assert!(!evaluator.normal_form());

    evaluator.step().unwrap();
    assert_eq!(format!("{}", evaluator), "Add(3, Add(3, 4))");
    assert!(!evaluator.normal_form());

    evaluator.step().unwrap();
    assert_eq!(format!("{}", evaluator), "Add(3, 7)");
    assert!(!evaluator.normal_form());

    evaluator.step().unwrap();
    assert_eq!(format!("{}", evaluator), "10");
    assert!(evaluator.normal_form());
}
