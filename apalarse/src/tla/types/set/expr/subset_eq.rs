use crate::tla::{Bool, Context, Expr, SetExpr};

#[derive(Debug, Clone)]
pub struct SubsetEq<S, U>
where
    S: SetExpr,
    U: SetExpr<ElemType = S::ElemType>,
{
    pub lhs: S,
    pub rhs: U,
}

impl<S, U> SubsetEq<S, U>
where
    S: SetExpr,
    U: SetExpr<ElemType = S::ElemType>,
{
    pub const fn new(lhs: S, rhs: U) -> Self {
        Self { lhs, rhs }
    }
}

impl<S, U> Expr for SubsetEq<S, U>
where
    S: SetExpr,
    U: SetExpr<ElemType = S::ElemType>,
{
    type Output = Bool;

    fn tla_expr(&self, cx: &mut Context) -> String {
        format!(
            "({} \\subseteq {})",
            self.lhs.tla_expr(cx),
            self.rhs.tla_expr(cx),
        )
    }

    fn evaluate(&self) -> Self::Output {
        todo!()
    }
}
