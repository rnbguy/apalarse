use crate::tla::{Bool, BoolExpr, Expr, Int, IntExpr, List, Map, Set, Str, TlaType};

#[test]
fn test_type_name() {
    let m = Int::get_type();
    assert_eq!(m, "Int");
}

#[test]
fn test_type_bool() {
    let m = Bool::get_type();
    assert_eq!(m, "Bool");
}

#[test]
fn test_type_set() {
    let m = Set::<Int>::get_type();
    assert_eq!(m, "Set(Int)");
}

#[test]
fn test_type_map() {
    let m = Map::<Int, Int>::get_type();
    assert_eq!(m, "(Int -> Int)");
}

#[test]
fn test_type_list() {
    let m = List::<Int>::get_type();
    assert_eq!(m, "Seq(Int)");
}

#[test]
fn test_type_str() {
    let m = Str::get_type();
    assert_eq!(m, "Str");
}

#[test]
fn test_type_all() {
    let m = Map::<Bool, Map<List<Int>, Set<Str>>>::get_type();
    assert_eq!(m, "(Bool -> (Seq(Int) -> Set(Str)))");
}

#[test]
fn test_boxed_bool_expr() {
    let m: Box<dyn Expr<Output = Bool>> = Box::new(1.equal(2));
    assert_eq!(m.tla_expr_test(), "(1 = 2)");
}

#[test]
fn test_boxed_int_expr() {
    let m: Box<dyn Expr<Output = i64>> = Box::new(1.add(2));
    assert_eq!(m.tla_expr_test(), "(1 + 2)");
}

#[test]
fn test_list_add() {
    let m = List::from([1.add(2), 3.add(4)]);
    assert_eq!(m.tla_expr_test(), "<<(1 + 2),(3 + 4)>>");
}

#[test]
fn test_list_boxed_empty() {
    let m: List<Box<dyn IntExpr>> = vec![];
    assert_eq!(m.tla_expr_test(), "<<>>");
}

#[test]
fn test_boxed_types() {
    let m: List<Box<dyn BoolExpr>> = vec![Box::new(1.equal(2)), Box::new(true.equal(false))];
    assert_eq!(m.tla_expr_test(), "<<(1 = 2),(TRUE = FALSE)>>");
}
