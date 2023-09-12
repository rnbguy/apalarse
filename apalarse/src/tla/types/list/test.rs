use crate::tla::{Expr, IntExpr, List, ListExpr};

#[test]
fn test_list() {
    let m = List::from(["1".to_owned(), "3".to_owned()]);
    assert_eq!(m.tla_expr_test(), "<<\"1\",\"3\">>");
}

#[test]
fn test_append() {
    let m = List::from([1, 3]).append(4);
    assert_eq!(m.tla_expr_test(), "Append(<<1,3>>, 4)");
}

#[test]
fn test_concat() {
    let m = List::from([1, 3]).concat(List::from([4, 5]));
    assert_eq!(m.tla_expr_test(), "(<<1,3>> \\o <<4,5>>)");
}

#[test]
fn test_head() {
    let m = List::from([1, 3]).head();
    assert_eq!(m.tla_expr_test(), "Head(<<1,3>>)");
}

#[test]
fn test_tail() {
    let m = List::from([1, 3]).tail();
    assert_eq!(m.tla_expr_test(), "Tail(<<1,3>>)");
}

#[test]
fn test_size() {
    let m = List::from([1, 3]).size();
    assert_eq!(m.tla_expr_test(), "Len(<<1,3>>)");
}

#[test]
fn test_list_get() {
    let m = List::from([1, 3]).nth(1);
    assert_eq!(m.tla_expr_test(), "<<1,3>>[1]");
}

#[test]
fn test_list_domain() {
    let m = List::from([1, 3]).domain();
    assert_eq!(m.tla_expr_test(), "(DOMAIN <<1,3>>)");
}

#[test]
fn test_list_replace() {
    let m = List::from([1, 3]).replace(1, |v| v.add(2));
    assert_eq!(m.tla_expr_test(), "[<<1,3>> EXCEPT ![1] = (@ + 2)]");
}
