use crate::tla::{Context, Expr, Int, ListExpr};

#[derive(Debug, Clone)]
pub struct ListGet<L>
where
    L: ListExpr,
{
    inner: L,
    key: Int,
}

impl<L> ListGet<L>
where
    L: ListExpr,
{
    pub const fn new(inner: L, key: Int) -> Self {
        Self { inner, key }
    }
}

impl<L> Expr for ListGet<L>
where
    L: ListExpr,
{
    type Output = L::ElemType;

    fn tla_expr(&self, cx: &mut Context) -> String {
        format!("{}[{}]", self.inner.tla_expr(cx), self.key.tla_expr(cx),)
    }

    fn evaluate(&self) -> Self::Output {
        todo!()
    }
}
