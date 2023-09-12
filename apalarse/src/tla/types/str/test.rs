use serde::{Deserialize, Serialize};

use crate::tla::{Expr, Id};

#[test]
fn test_string() {
    let x = String::from("hello world");
    assert_eq!(x.tla_expr_test().as_str(), "\"hello world\"");
    // assert_eq!(x.concrete(), "hello world");
}

#[test]
fn test_serialize_deserialize() {
    #[derive(Deserialize, Serialize, Clone)]
    enum Hello {
        World,
        Mars,
    }

    let x: Id<Hello> = Hello::World.into();
    assert_eq!(x.tla_expr_test().as_str(), "\"World\"");

    let y: Id<Hello> = Hello::Mars.into();
    assert_eq!(y.tla_expr_test().as_str(), "\"Mars\"");

    let x_eq_y = x.equal(y);
    assert_eq!(x_eq_y.tla_expr_test().as_str(), "(\"World\" = \"Mars\")");
}
