use crate::tla::{Expr, IntExpr, Map, MapExpr};

#[test]
fn test_map() {
    let m = Map::from(vec![("1".to_owned(), 2), ("3".to_owned(), 4)]);
    assert!(matches!(
        m.tla_expr_test().as_str(),
        "SetAsFun({<<\"3\", 4>>,<<\"1\", 2>>})" | "SetAsFun({<<\"1\", 2>>,<<\"3\", 4>>})"
    ));
}

#[test]
fn test_map_replace() {
    let m = Map::from(vec![(1, 4)]).replace(1, |v| v.add(2));
    assert_eq!(
        m.tla_expr_test(),
        "[SetAsFun({<<1, 4>>}) EXCEPT ![1] = (@ + 2)]"
    );
}

#[test]
fn test_map_domain() {
    let m = Map::from(vec![(0, 2)]).domain();
    assert_eq!(m.tla_expr_test(), "(DOMAIN SetAsFun({<<0, 2>>}))");
}

#[test]
fn test_map_get() {
    let m = Map::from(vec![(1, 2)]).tla_get(1);
    assert_eq!(m.tla_expr_test(), "SetAsFun({<<1, 2>>})[1]");
}
