use crate::tla::{Context, Expr, Int, SetExpr};

#[derive(Debug, Clone)]
pub struct Cardinality<S>
where
    S: SetExpr,
{
    pub inner: S,
}

impl<S> Cardinality<S>
where
    S: SetExpr,
{
    pub const fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S> Expr for Cardinality<S>
where
    S: SetExpr,
{
    type Output = Int;

    fn tla_expr(&self, cx: &mut Context) -> String {
        format!("Cardinality({})", self.inner.tla_expr(cx))
    }

    fn evaluate(&self) -> Self::Output {
        todo!()
    }
}
