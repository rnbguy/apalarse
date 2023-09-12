#[cfg(test)]
mod test;

use crate::tla::{Context, Eq, InSet, Neq, NotInSet, SetExpr};

/// Expr is general symbolic expr
pub trait Expr {
    type Output: TlaType;
    fn tla_expr(&self, cx: &mut Context) -> String;
    fn evaluate(&self) -> Self::Output;
    fn get_expr_type(&self) -> String {
        Self::Output::get_type()
    }
    fn equal<Rhs>(&self, other: Rhs) -> Eq<Self, Rhs>
    where
        Self: Expr + Clone,
        Rhs: Expr<Output = Self::Output>,
    {
        Eq::new(self.clone(), other)
    }

    fn nequal<Rhs>(&self, other: Rhs) -> Neq<Self, Rhs>
    where
        Self: Expr + Clone,
        Rhs: Expr<Output = Self::Output>,
    {
        Neq::new(self.clone(), other)
    }

    fn is_in<S>(&self, other: S) -> InSet<Self, S>
    where
        Self: Expr + Clone,
        S: SetExpr<ElemType = Self::Output>,
    {
        InSet::new(self.clone(), other)
    }

    fn not_in<S>(&self, other: S) -> NotInSet<Self, S>
    where
        Self: Expr + Clone,
        S: SetExpr<ElemType = Self::Output>,
    {
        NotInSet::new(self.clone(), other)
    }
    #[cfg(test)]
    fn tla_expr_test(&self) -> String {
        let mut cx = Context::default();
        self.tla_expr(&mut cx)
    }
}

// Expr is concrete Rust struct
// The valeus are computed
// Like, Int, Bool, Set, Map
pub trait TlaType: Clone {
    fn get_type() -> String;
}

impl<T> Expr for Box<T>
where
    T: Expr + ?Sized,
{
    type Output = T::Output;

    fn tla_expr(&self, cx: &mut Context) -> String {
        self.as_ref().tla_expr(cx)
    }

    fn evaluate(&self) -> Self::Output {
        self.as_ref().evaluate()
    }
}
