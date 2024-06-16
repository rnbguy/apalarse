use serde::Deserialize;

use crate::tla::{Context, Expr, Int, Map, Str, Symbol, Variable};

#[test]
fn test_global_variable() {
    let m = Variable::<Int>::global(1);
    assert_eq!(m.name(), "global_1");
}

#[test]
fn test_bound_variable() {
    let m = Variable::<Int>::bound(1);
    assert_eq!(m.name(), "bound_1");
}

#[test]
fn test_map_variable() {
    let m = Variable::<Int>::map(1);
    assert_eq!(m.name(), "@");
}

#[test]
fn test_unchanged() {
    let m = Variable::<Int>::global(1).unchanged();
    assert_eq!(m.tla_expr_test(), "(UNCHANGED global_1)");
}

#[test]
fn test_symbol() {
    // TODO(rano): impl TlaType for ItfMap, ItfSet, ItfList, ItfBigInt
    // TODO(rano): need to create this derive to convert to bottom
    // #[derive(TlaState)]
    // struct State {
    //     foo: Int,
    //     bar: Map<Str, Int>,
    // }

    trait TlaState {
        fn variable(cx: &mut Context) -> Self;
    }

    #[derive(Clone, Debug, Deserialize)]
    struct State {
        foo: Symbol<Int>,
        bar: Symbol<Map<Str, Int>>,
    }

    impl TlaState for State {
        fn variable(cx: &mut Context) -> Self {
            Self {
                foo: Symbol::new(cx.var_with_name("foo")),
                bar: Symbol::new(cx.var_with_name("bar")),
            }
        }
    }

    let state_variable = State::variable(&mut Context::default());
    assert_eq!(state_variable.foo.variable().name(), "foo");
    assert_eq!(state_variable.bar.variable().name(), "bar");

    let json_string = r##"{"foo": 1, "bar": {"#map": [["1", 2], ["3", 4]]}}"##;

    let state: State = serde_json::from_str(json_string).unwrap();

    assert_eq!(state.foo.concrete(), &1);
    assert_eq!(
        state.bar.concrete().inner,
        vec![("1".into(), 2), ("3".into(), 4)]
    );
}
