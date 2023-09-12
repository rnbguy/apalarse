use crate::tla::{Context, Expr, ListExpr};

#[derive(Debug, Clone)]
pub struct Concat<L, R>
where
    L: ListExpr,
    R: ListExpr<ElemType = L::ElemType>,
{
    lhs: L,
    rhs: R,
}

impl<L, R> Concat<L, R>
where
    L: ListExpr,
    R: ListExpr<ElemType = L::ElemType>,
{
    pub fn new(lhs: L, rhs: R) -> Concat<L, R> {
        Concat { lhs, rhs }
    }
}

impl<L, R> Expr for Concat<L, R>
where
    L: ListExpr,
    R: ListExpr<ElemType = L::ElemType>,
{
    type Output = L::Output;

    fn tla_expr(&self, cx: &mut Context) -> String {
        format!("({} \\o {})", self.lhs.tla_expr(cx), self.rhs.tla_expr(cx),)
    }

    fn evaluate(&self) -> Self::Output {
        todo!()
    }
}
