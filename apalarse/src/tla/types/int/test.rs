use crate::tla::{Expr, Int, IntExpr, Set};

#[test]
fn test_pos_int() {
    let m: Int = 1;
    assert_eq!(m.tla_expr_test(), "1");
}

#[test]
fn test_neg_int() {
    let m: Int = -1;
    assert_eq!(m.tla_expr_test(), "-1");
}

#[test]
fn test_int_add() {
    let m = 1.add(2);
    assert_eq!(m.tla_expr_test(), "(1 + 2)");
}

#[test]
fn test_int_sub() {
    let m = 1.sub(2);
    assert_eq!(m.tla_expr_test(), "(1 - 2)");
}

#[test]
fn test_int_mul() {
    let m = 1.mul(2);
    assert_eq!(m.tla_expr_test(), "(1 * 2)");
}

#[test]
fn test_int_div() {
    let m = 1.quo(2);
    assert_eq!(m.tla_expr_test(), "(1 \\div 2)");
}

#[test]
fn test_int_mod() {
    let m = 1.rem(2);
    assert_eq!(m.tla_expr_test(), "(1 % 2)");
}

#[test]
fn test_int_gt() {
    let m = 1.tla_gt(2);
    assert_eq!(m.tla_expr_test(), "(1 > 2)");
}

#[test]
fn test_int_ge() {
    let m = 1.tla_geq(2);
    assert_eq!(m.tla_expr_test(), "(1 >= 2)");
}

#[test]
fn test_int_lt() {
    let m = 1.tla_lt(2);
    assert_eq!(m.tla_expr_test(), "(1 < 2)");
}

#[test]
fn test_int_le() {
    let m = 1.tla_leq(2);
    assert_eq!(m.tla_expr_test(), "(1 <= 2)");
}

#[test]
fn test_int_eq() {
    let m = 1.equal(2);
    assert_eq!(m.tla_expr_test(), "(1 = 2)");
}

#[test]
fn test_int_neq() {
    let m = 1.nequal(2);
    assert_eq!(m.tla_expr_test(), "(1 /= 2)");
}

#[test]
fn test_int_in() {
    let m = 1.is_in(Set::from(vec![2]));
    assert_eq!(m.tla_expr_test(), "(1 \\in {2})");
}

#[test]
fn test_int_not_in() {
    let m = 1.not_in(Set::from(vec![2]));
    assert_eq!(m.tla_expr_test(), "(1 \\notin {2})");
}
