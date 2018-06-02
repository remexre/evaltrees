use ast::Type;

#[test]
fn type_forall_precedence() {
    // (->) (forall a. (->) a a) Int
    let ty1 = Type::Func(
        Box::new(Type::Forall(Box::new(Type::Func(
            Box::new(Type::Var(0)),
            Box::new(Type::Var(0)),
        )))),
        Box::new(Type::Int),
    );

    // forall a. (->) ((->) a a) Int
    let ty2 = Type::Forall(Box::new(Type::Func(
        Box::new(Type::Func(Box::new(Type::Var(0)), Box::new(Type::Var(0)))),
        Box::new(Type::Int),
    )));

    assert_eq!(ty1.to_string(), "('a. 'a -> 'a) -> int");
    assert_eq!(ty2.to_string(), "'a. ('a -> 'a) -> int");
}

#[test]
fn type_func_precedence() {
    // (->) ((->) Int Int) Int
    let ty1 = Type::Func(
        Box::new(Type::Func(Box::new(Type::Int), Box::new(Type::Int))),
        Box::new(Type::Int),
    );

    // (->) Int ((->) Int Int)
    let ty2 = Type::Func(
        Box::new(Type::Int),
        Box::new(Type::Func(Box::new(Type::Int), Box::new(Type::Int))),
    );

    assert_eq!(ty1.to_string(), "(int -> int) -> int");
    assert_eq!(ty2.to_string(), "int -> int -> int");
}

#[test]
fn type_list_precedence() {
    // (->) Int (List Int)
    let ty1 = Type::Func(
        Box::new(Type::Int),
        Box::new(Type::List(Box::new(Type::Int))),
    );

    // List ((->) Int Int)
    let ty2 = Type::List(Box::new(Type::Func(
        Box::new(Type::Int),
        Box::new(Type::Int),
    )));

    // forall a. List a
    let ty3 = Type::Forall(Box::new(Type::List(Box::new(Type::Var(0)))));

    // List (forall a. a)
    let ty4 = Type::List(Box::new(Type::Forall(Box::new(Type::Var(0)))));

    assert_eq!(ty1.to_string(), "int -> int list");
    assert_eq!(ty2.to_string(), "(int -> int) list");
    assert_eq!(ty3.to_string(), "'a. 'a list");
    assert_eq!(ty4.to_string(), "('a. 'a) list");
}
