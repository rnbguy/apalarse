use crate::tla::{Empty, Expr, Int, IntExpr, Nat, RangeSet, Set, SetExpr};

// test each of the constant types - call one SetExpr method, one Expr method
// then test SetExpr methods
// then test Expr methods

#[test]
fn test_set_empty() {
    let s: Set<Int> = Set::empty();
    assert_eq!(s.tla_expr_test(), "{}");
}

#[test]
fn test_set_int() {
    let s: Set<Int> = Set::from(vec![1]);
    assert_eq!(s.tla_expr_test(), "{1}");
}

#[test]
fn test_set_repeat() {
    let s: Set<Int> = Set::from(vec![1, 1, 1]);
    assert_eq!(s.tla_expr_test(), "{1,1,1}");
}

#[test]
fn test_set_mult() {
    let s: Set<Int> = Set::from(vec![1, 2]);
    matches!(s.tla_expr_test().as_str(), "{1, 2}" | "{2, 1}");
}

#[test]
fn test_empty() {
    let s = Empty::<Int>::new();
    assert_eq!(s.tla_expr_test(), "{}");
    assert_eq!(s.cardinality().tla_expr_test(), "Cardinality({})");
}

#[test]
fn test_nat() {
    let s = Nat;
    assert_eq!(s.tla_expr_test(), "Nat");
    assert_eq!(s.cardinality().tla_expr_test(), "Cardinality(Nat)");
}

#[test]
fn test_range() {
    let s = RangeSet::new(1, 10);
    assert_eq!(s.tla_expr_test(), "1..10");
    assert_eq!(s.cardinality().tla_expr_test(), "Cardinality(1..10)");
}

#[test]
fn test_set_expr() {
    let s: Set<Int> = Set::from(vec![1]);
    assert_eq!(s.tla_contains(1).tla_expr_test(), "(1 \\in {1})");
    assert_eq!(s.tla_not_contains(1).tla_expr_test(), "(1 \\notin {1})");
    assert_eq!(s.tla_insert(2).tla_expr_test(), "({1} \\union {2})");
    assert_eq!(s.tla_remove(1).tla_expr_test(), "({1} \\ {1})");
    assert_eq!(
        s.tla_union(Set::<Int>::empty().tla_insert(2))
            .tla_expr_test(),
        "({1} \\union ({} \\union {2}))"
    );
    assert_eq!(
        s.tla_difference(Set::<Int>::empty().tla_insert(2))
            .tla_expr_test(),
        "({1} \\ ({} \\union {2}))"
    );
    assert_eq!(
        s.tla_intersect(Set::<Int>::empty().tla_insert(2))
            .tla_expr_test(),
        "({1} \\intersect ({} \\union {2}))"
    );
    assert_eq!(
        s.map_to_set(|x| x.add(1)).tla_expr_test(),
        "{(bound_1 + 1): bound_1 \\in {1}}"
    );
    assert_eq!(
        s.filter(|x| x.tla_gt(0)).tla_expr_test(),
        "{bound_1 \\in {1}: (bound_1 > 0)}"
    );
}

#[test]
fn test_expr() {
    let s: Set<Int> = Set::from(vec![1]);
    assert_eq!(s.equal(s.clone()).tla_expr_test(), "({1} = {1})");
    assert_eq!(s.nequal(s.clone()).tla_expr_test(), "({1} /= {1})");
    assert_eq!(1.is_in(s.clone()).tla_expr_test(), "(1 \\in {1})");
    assert_eq!(1.not_in(s).tla_expr_test(), "(1 \\notin {1})");
}
