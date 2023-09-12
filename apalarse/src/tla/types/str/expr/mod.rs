use crate::tla::{Expr, Str};

pub trait StrExpr: Expr {}

impl<U> StrExpr for U where U: Expr<Output = Str> {}
