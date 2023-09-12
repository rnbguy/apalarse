use crate::tla::macros::{binary_expr_func, binary_expr_struct};
use crate::tla::{Bool, Context, Expr, Int};

binary_expr_struct!(Add, IntExpr, Int, '+', Some("Integers"));
binary_expr_struct!(Sub, IntExpr, Int, '-', Some("Integers"));
binary_expr_struct!(Mul, IntExpr, Int, '*', Some("Integers"));
binary_expr_struct!(Quo, IntExpr, Int, "\\div", Some("Integers"));
binary_expr_struct!(Rem, IntExpr, Int, '%', Some("Integers"));
binary_expr_struct!(Lt, IntExpr, Bool, '<', Some("Integers"));
binary_expr_struct!(Leq, IntExpr, Bool, "<=", Some("Integers"));
binary_expr_struct!(Gt, IntExpr, Bool, '>', Some("Integers"));
binary_expr_struct!(Geq, IntExpr, Bool, ">=", Some("Integers"));

pub trait IntExpr: Expr<Output = Int> {
    binary_expr_func!(add, Add);
    binary_expr_func!(sub, Sub);
    binary_expr_func!(mul, Mul);
    binary_expr_func!(quo, Quo);
    binary_expr_func!(rem, Rem);
    binary_expr_func!(tla_lt, Lt);
    binary_expr_func!(tla_leq, Leq);
    binary_expr_func!(tla_gt, Gt);
    binary_expr_func!(tla_geq, Geq);
}

impl<T> IntExpr for T where Self: Expr<Output = Int> {}

// can't do following
// because
// https://github.com/rust-lang/rfcs/issues/1124
// https://www.reddit.com/r/rust/comments/3709tl/implementing_external_trait_for_types/
// this allows `a + b`, `a * b` etc.

// impl<T, Rhs> std::ops::Add<Rhs> for T where T: Expr<Output = Int>, Rhs: Expr<Output = Int> {
//     type Output = Add<Self, Rhs>;

//     fn add(self, rhs: Rhs) -> Self::Output {
//         Add {
//             lhs: self,
//             rhs,
//         }
//     }
// }
