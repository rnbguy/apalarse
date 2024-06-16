use crate::tla::{Context, Expr, ListExpr};

#[derive(Debug, Clone)]
pub struct Head<E>
where
    E: ListExpr,
{
    inner: E,
}

impl<E> Head<E>
where
    E: ListExpr,
{
    pub const fn new(inner: E) -> Self {
        Self { inner }
    }
}

impl<E> Expr for Head<E>
where
    E: ListExpr,
{
    type Output = E::Output;

    fn tla_expr(&self, cx: &mut Context) -> String {
        format!("Head({})", self.inner.tla_expr(cx))
    }

    fn evaluate(&self) -> Self::Output {
        todo!()
    }
}
