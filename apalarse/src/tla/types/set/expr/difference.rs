use crate::tla::{Context, Expr, SetExpr};

#[derive(Debug, Clone, Copy)]
pub struct Difference<L, R>
where
    L: SetExpr,
    R: SetExpr<ElemType = L::ElemType>,
{
    lhs: L,
    rhs: R,
}

impl<L, R> Difference<L, R>
where
    L: SetExpr,
    R: SetExpr<ElemType = L::ElemType>,
{
    pub fn new(lhs: L, rhs: R) -> Difference<L, R> {
        Difference { lhs, rhs }
    }
}

impl<L, R> Expr for Difference<L, R>
where
    L: SetExpr,
    R: SetExpr<ElemType = L::ElemType>,
{
    type Output = L::Output;

    fn tla_expr(&self, cx: &mut Context) -> String {
        format!("({} \\ {})", self.lhs.tla_expr(cx), self.rhs.tla_expr(cx))
    }

    fn evaluate(&self) -> Self::Output {
        todo!()
    }
}
