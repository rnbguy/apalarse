mod if_then_else;
mod in_set;
mod not_in_set;
mod quant;

pub use if_then_else::IfThenElse;
pub use in_set::InSet;
pub use not_in_set::NotInSet;
pub use quant::Quant;

use crate::tla::macros::{
    binary_expr_func, binary_expr_struct, generic_binary_expr_struct, unary_expr_func,
    unary_expr_struct,
};
use crate::tla::{Bool, Context, Expr};

// can we add Eq, Neq, InSet, NotInSet here?

unary_expr_struct!(Not, BoolExpr, Bool, "~", None);
binary_expr_struct!(And, BoolExpr, Bool, "/\\", None);
binary_expr_struct!(Or, BoolExpr, Bool, "\\/", None);
binary_expr_struct!(Imply, BoolExpr, Bool, "=>", None);
binary_expr_struct!(Equiv, BoolExpr, Bool, "<=>", None);

generic_binary_expr_struct!(Eq, Bool, "=", None);
generic_binary_expr_struct!(Neq, Bool, "/=", None);

pub trait BoolExpr: Expr<Output = Bool> {
    unary_expr_func!(not, Not);
    binary_expr_func!(and, And);
    binary_expr_func!(or, Or);
    binary_expr_func!(imply, Imply);
    binary_expr_func!(equiv, Equiv);

    fn if_then_else<L, R>(self, pos: L, neg: R) -> IfThenElse<Self, L, R>
    where
        Self: Clone,
        L: Expr,
        R: Expr<Output = L::Output>,
    {
        IfThenElse::new(self, pos, neg)
    }
}

impl<T> BoolExpr for T where Self: Expr<Output = Bool> {}
