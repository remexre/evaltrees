use std::fs::File;
use std::io::Read;

use ast::{Decl, PrintStyle};
use cst::{parse_decls, Expr as CstExpr};
use eval::{CallByName, CallByValue, Evaluator, LazyEvaluation};
use typeck::typeck;

/// The "harness" for testing an evaluator.
pub fn test_evaluator<E: Evaluator>(
    mut evaluator: E,
    expected: Option<Result<&str, &str>>,
    name: &str,
) {
    evaluator.set_print_style(PrintStyle::CST);
    match evaluator.step_many(32) {
        Ok(()) => {
            if evaluator.normal_form() {
                let s = evaluator.to_string();
                assert_eq!(Some(Ok(&s as &str)), expected, "{}", name);
            } else {
                assert_eq!(None, expected, "{}", name);
            }
        }
        Err(e) => {
            let e = e.to_string();
            assert_eq!(Some(Err(&e as &str)), expected, "{}", name);
        }
    }
}

/// Creates a test for the given example(s).
macro_rules! example_test {
    (priv: $name:ident) => {
        {
            let cst = {
                let mut f = File::open(concat!("examples/", stringify!($name), ".etl"))
                    .expect("Failed to open example");
                let mut src = String::new();
                f.read_to_string(&mut src).expect("Failed to read example");
                parse_decls(&src).expect("Failed to parse example")
            };
            let ast = cst
                .into_iter()
                .map(|decl| decl.into_ast())
                .collect::<Result<Vec<_>, _>>()
                .expect("Failed to convert CST");
            typeck(ast, Vec::new()).expect("Failed to type-check decls")
        }
    };
    (priv: $name:ident, $expr:expr, $cbv:expr, $cbn:expr, $lazy:expr) => {
        {
            let typed_ast = example_test!(priv: $name);
            let expr = $expr.parse::<CstExpr>()
                .expect("Failed to parse expr")
                .into_ast()
                .expect("Failed to convert expr to AST");
            let expr_decl = Decl {
                name: "".into(),
                args: vec![],
                body: expr,
                aux: (),
            };

            let decls = typeck(vec![expr_decl], typed_ast)
                .expect("Failed to type-check expression")
                .into_iter()
                .map(|decl| decl.map_aux(|_| ()))
                .collect::<Vec<_>>();

            test_evaluator(CallByValue::new(decls.clone()), $cbv, concat!(stringify!(name), " cbv"));
            test_evaluator(CallByName::new(decls.clone()), $cbn, concat!(stringify!(name), " cbn"));
            test_evaluator(LazyEvaluation::new(decls), $lazy, concat!(stringify!(name), " lazy"));
        }
    };
    (priv impl: $(#[$attr:meta])* $name:ident ()) => {
        $(#[$attr])*
        #[test]
        fn $name() {
            example_test!(priv: $name);
        }
    };
    (priv impl: $(#[$attr:meta])* $name:ident ($expr:expr, $expected:expr)) => {
        $(#[$attr])*
        #[test]
        fn $name() {
            let expected = Some(Ok($expected));
            example_test!(priv: $name, $expr, expected, expected, expected);
        }
    };
    (priv impl: $(#[$attr:meta])* $name:ident ($expr:expr, $cbv:expr, $cbn:expr, $lazy:expr)) => {
        $(#[$attr])*
        #[test]
        fn $name() {
            example_test!(priv: $name, $expr, $cbv, $cbn, $lazy);
        }
    };
    (,$($rest:tt)*) => { example_test!($($rest)*); };
    ($($(#[$attr:meta])* $name:ident $args:tt ,)+) => {
        $(example_test!(priv impl: $(#[$attr])* $name $args);)+
    };
}

example_test! {
    and("andl [true; true; false; true]", "false"),
    div0("const 1 div0", Some(Err("division by zero")), Some(Ok("1")), Some(Ok("1"))),
    double("doubleApp double 4", "16"),
    higher_order("map (plus 3) [1; 2; 3]", "4 :: 5 :: 6 :: []"),
    id("id id 137", "137"),
    infinite("take 2 ones", None, Some(Ok("1 :: 1 :: []")), Some(Ok("1 :: 1 :: []"))),
    mutual("odd 5", "true"),
    need("triple 2 + double 3", "12"),
    #[should_panic]
    occurs_error(),
    poly("(I 40) + (S K K 2)", "42"),
    tail_recursion("(sum [1; 2; 3]) + (sumTR [4; 5; 6])", "21"),
    #[should_panic(expected = "Failed to type-check decls: CantUnify(Int, List(Int))")]
    type_error(),
}
