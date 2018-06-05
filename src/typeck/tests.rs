use ast::{Decl, Expr, Literal, Op, Pattern, Type};
use typeck::typeck_decls;

#[test]
fn map_map() {
    let decls = vec![
        Decl {
            name: "map".into(),
            args: vec![
                Pattern::Binding("f".into(), ()),
                Pattern::Literal(Literal::Nil, ()),
            ],
            body: Expr::Literal(Literal::Nil, ()),
            aux: (),
        },
        Decl {
            name: "map".into(),
            args: vec![
                Pattern::Binding("f".into(), ()),
                Pattern::Cons(
                    Box::new(Pattern::Binding("h".into(), ())),
                    Box::new(Pattern::Binding("t".into(), ())),
                    (),
                ),
            ],
            body: Expr::Op(
                Op::Cons,
                Box::new(Expr::Op(
                    Op::App,
                    Box::new(Expr::Variable("f".into(), ())),
                    Box::new(Expr::Variable("h".into(), ())),
                    (),
                )),
                Box::new(Expr::Op(
                    Op::App,
                    Box::new(Expr::Op(
                        Op::App,
                        Box::new(Expr::Variable("map".into(), ())),
                        Box::new(Expr::Variable("f".into(), ())),
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
            name: "mapmap".into(),
            args: vec![],
            body: Expr::Op(
                Op::App,
                Box::new(Expr::Variable("map".into(), ())),
                Box::new(Expr::Variable("map".into(), ())),
                (),
            ),
            aux: (),
        },
    ];

    // 'a 'b. ('a -> 'b) -> 'a list -> 'b list
    let map_ty = Type::Forall(Box::new(Type::Forall(Box::new(Type::Func(
        Box::new(Type::Func(Box::new(Type::Var(1)), Box::new(Type::Var(0)))),
        Box::new(Type::Func(
            Box::new(Type::List(Box::new(Type::Var(1)))),
            Box::new(Type::List(Box::new(Type::Var(0)))),
        )),
    )))));

    let decls = typeck_decls(decls, Vec::new()).unwrap();
    assert_eq!(
        decls.into_iter().map(|decl| decl.aux).collect::<Vec<_>>(),
        vec![
            map_ty.clone(),
            map_ty,
            // 'a 'b. ('a -> 'b) list -> ('a list -> 'b list) list
            Type::Forall(Box::new(Type::Forall(Box::new(Type::Func(
                Box::new(Type::List(Box::new(Type::Func(
                    Box::new(Type::Var(1)),
                    Box::new(Type::Var(0)),
                )))),
                Box::new(Type::List(Box::new(Type::Func(
                    Box::new(Type::List(Box::new(Type::Var(1)))),
                    Box::new(Type::List(Box::new(Type::Var(0)))),
                )))),
            ))))),
        ]
    );
}

#[test]
fn poly_id() {
    let decls = vec![
        Decl {
            name: "id".into(),
            args: vec![Pattern::Binding("x".into(), ())],
            body: Expr::Variable("x".into(), ()),
            aux: (),
        },
        Decl {
            name: "foo".into(),
            args: vec![],
            body: Expr::Op(
                Op::Cons,
                Box::new(Expr::Op(
                    Op::App,
                    Box::new(Expr::Variable("id".into(), ())),
                    Box::new(Expr::Literal(Literal::Int(1), ())),
                    (),
                )),
                Box::new(Expr::Op(
                    Op::App,
                    Box::new(Expr::Variable("id".into(), ())),
                    Box::new(Expr::Op(
                        Op::Cons,
                        Box::new(Expr::Literal(Literal::Int(2), ())),
                        Box::new(Expr::Literal(Literal::Nil, ())),
                        (),
                    )),
                    (),
                )),
                (),
            ),
            aux: (),
        },
    ];

    let decls = typeck_decls(decls, Vec::new()).unwrap();
    assert_eq!(
        decls.into_iter().map(|decl| decl.aux).collect::<Vec<_>>(),
        vec![
            Type::Forall(Box::new(Type::Func(
                Box::new(Type::Var(0)),
                Box::new(Type::Var(0)),
            ))),
            Type::List(Box::new(Type::Int)),
        ]
    );
}
