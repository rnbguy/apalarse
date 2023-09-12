use crate::tla::{Context, Expr, Int, ListExpr, Set};

#[derive(Debug, Clone)]
pub struct ListDomain<L>
where
    L: ListExpr,
{
    inner: L,
}

impl<L> ListDomain<L>
where
    L: ListExpr,
{
    pub fn new(inner: L) -> ListDomain<L> {
        ListDomain { inner }
    }
}

impl<L> Expr for ListDomain<L>
where
    L: ListExpr,
{
    type Output = Set<Int>;

    fn tla_expr(&self, cx: &mut Context) -> String {
        format!("(DOMAIN {})", self.inner.tla_expr(cx),)
    }

    fn evaluate(&self) -> Self::Output {
        todo!()
    }
}
