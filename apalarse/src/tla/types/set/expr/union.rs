use crate::tla::{Context, Expr, SetExpr};

#[derive(Debug, Clone, Copy)]
pub struct Union<L, R>
where
    L: SetExpr,
    R: SetExpr<ElemType = L::ElemType>,
{
    lhs: L,
    rhs: R,
}

impl<L, R> Union<L, R>
where
    L: SetExpr,
    R: SetExpr<ElemType = L::ElemType>,
{
    pub fn new(lhs: L, rhs: R) -> Union<L, R> {
        Union { lhs, rhs }
    }
}

impl<L, R> Expr for Union<L, R>
where
    L: SetExpr,
    R: SetExpr<ElemType = L::ElemType>,
{
    type Output = L::Output;

    fn tla_expr(&self, cx: &mut Context) -> String {
        format!(
            "({} \\union {})",
            self.lhs.tla_expr(cx),
            self.rhs.tla_expr(cx)
        )
    }

    fn evaluate(&self) -> Self::Output {
        todo!()
    }
}
