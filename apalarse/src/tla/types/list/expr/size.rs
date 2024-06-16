use crate::tla::{Context, Expr, Int, ListExpr};

#[derive(Debug, Clone)]
pub struct Size<L>
where
    L: ListExpr,
{
    inner: L,
}

impl<L> Size<L>
where
    L: ListExpr,
{
    pub const fn new(inner: L) -> Self {
        Self { inner }
    }
}

impl<L> Expr for Size<L>
where
    L: ListExpr,
{
    type Output = Int;

    fn tla_expr(&self, cx: &mut Context) -> String {
        format!("Len({})", self.inner.tla_expr(cx))
    }

    fn evaluate(&self) -> Self::Output {
        todo!()
    }
}
