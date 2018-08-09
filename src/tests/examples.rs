use std::fs::File;
use std::io::Read;

use ast::{Decl, PrintStyle};
use cst::{parse_decls, Expr as CstExpr};
use eval::{CallByName, CallByValue, Evaluator, LazyEvaluation};
use typeck::typeck;

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

            let mut cbv = CallByValue::new(decls.clone());
            let mut cbn = CallByName::new(decls.clone());
            let mut lazy = LazyEvaluation::new(decls);

            cbv.set_print_style(PrintStyle::CST);
            cbv.step_many(100).expect("Evaluation error in CBV");
            cbn.set_print_style(PrintStyle::CST);
            cbn.step_many(100).expect("Evaluation error in CBN");
            lazy.set_print_style(PrintStyle::CST);
            lazy.step_many(100).expect("Evaluation error in Lazy");
    
            let cbv_str = cbv.to_string();
            let cbn_str = cbn.to_string();
            let lazy_str = lazy.to_string();

            let cbv = if cbv.normal_form() { Some(&cbv_str as &str) } else { None };
            let cbn = if cbn.normal_form() { Some(&cbn_str as &str) } else { None };
            let lazy = if lazy.normal_form() { Some(&lazy_str as &str) } else { None };
            
            assert_eq!(cbv, $cbv, "cbv");
            assert_eq!(cbn, $cbn, "cbn");
            assert_eq!(lazy, $lazy, "lazy");
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
            let expected = Some($expected);
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
    double("double addOne 4", "6"),
    higher_order("map (plus 3) [1; 2; 3]", "4 :: 5 :: 6 :: []"),
    id("id id 137", "137"),
    infinite("take 2 ones", None, Some("1 :: 1 :: []"), Some("1 :: 1 :: []")),
    mutual("odd 5", "true"),
    need("triple 2 + double 3", "12"),
    #[should_panic]
    occurs_error(),
    poly("(I 40) + (S K K 2)", "42"),
    #[should_panic(expected = "Failed to type-check decls: CantUnify(Int, List(Int))")]
    type_error(),
}
