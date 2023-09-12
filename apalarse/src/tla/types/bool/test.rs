use crate::tla::{BoolExpr, Expr, Set};

#[test]
fn test_true() {
    let m: bool = true;
    assert_eq!(m.tla_expr_test(), "TRUE");
}

#[test]
fn test_false() {
    let m: bool = false;
    assert_eq!(m.tla_expr_test(), "FALSE");
}

#[test]
fn test_bool_not() {
    let m = true.not();
    assert_eq!(m.tla_expr_test(), "(~TRUE)");
}

#[test]
fn test_bool_and() {
    let m = true.and(false);
    assert_eq!(m.tla_expr_test(), "(TRUE /\\ FALSE)");
}

#[test]
fn test_bool_or() {
    let m = true.or(false);
    assert_eq!(m.tla_expr_test(), "(TRUE \\/ FALSE)");
}

#[test]
fn test_bool_implies() {
    let m = true.imply(false);
    assert_eq!(m.tla_expr_test(), "(TRUE => FALSE)");
}

#[test]
fn test_bool_equiv() {
    let m = true.equiv(false);
    assert_eq!(m.tla_expr_test(), "(TRUE <=> FALSE)");
}

#[test]
fn test_bool_eq() {
    let m = true.equal(false);
    assert_eq!(m.tla_expr_test(), "(TRUE = FALSE)");
}

#[test]
fn test_bool_neq() {
    let m = true.nequal(false);
    assert_eq!(m.tla_expr_test(), "(TRUE /= FALSE)");
}

#[test]
fn test_bool_in() {
    let m = true.is_in(Set::from(vec![false]));
    assert_eq!(m.tla_expr_test(), "(TRUE \\in {FALSE})");
}

#[test]
fn test_bool_not_in() {
    let m = true.not_in(Set::from(vec![false]));
    assert_eq!(m.tla_expr_test(), "(TRUE \\notin {FALSE})");
}
