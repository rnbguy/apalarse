use crate::tla::{Context, Expr, ListExpr};

#[derive(Debug, Clone)]
pub struct Tail<E>
where
    E: ListExpr,
{
    inner: E,
}

impl<E> Tail<E>
where
    E: ListExpr,
{
    pub fn new(inner: E) -> Self {
        Self { inner }
    }
}

impl<E> Expr for Tail<E>
where
    E: ListExpr,
{
    type Output = E::ElemType;

    fn tla_expr(&self, cx: &mut Context) -> String {
        format!("Tail({})", self.inner.tla_expr(cx))
    }

    fn evaluate(&self) -> Self::Output {
        todo!()
    }
}
