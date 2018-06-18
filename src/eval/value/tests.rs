use super::*;
use ast::Pattern;

#[test]
fn app() {
    let mut evaluator = CallByValue::new(vec![
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
    assert_eq!(format!("{}", evaluator), "App(App(f, 3), Add(3, 4))");
    assert!(!evaluator.normal_form());

    evaluator.step().unwrap();
    assert_eq!(format!("{}", evaluator), "App(App(f, 3), 7)");
    assert!(!evaluator.normal_form());

    evaluator.step().unwrap();
    assert_eq!(format!("{}", evaluator), "Add(3, 7)");
    assert!(!evaluator.normal_form());

    evaluator.step().unwrap();
    assert_eq!(format!("{}", evaluator), "10");
    assert!(evaluator.normal_form());
}
