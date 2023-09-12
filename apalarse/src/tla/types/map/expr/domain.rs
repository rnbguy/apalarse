use crate::tla::{Context, Expr, MapExpr, Set};

#[derive(Debug, Clone)]
pub struct Domain<FE>
where
    FE: MapExpr,
{
    inner: FE,
}

impl<FE> Domain<FE>
where
    FE: MapExpr,
{
    pub fn new(inner: FE) -> Self {
        Self { inner }
    }
}

impl<FE> Expr for Domain<FE>
where
    FE: MapExpr,
{
    type Output = Set<FE::KeyType>;

    fn tla_expr(&self, cx: &mut Context) -> String {
        format!("(DOMAIN {})", self.inner.tla_expr(cx),)
    }

    fn evaluate(&self) -> Self::Output {
        todo!()
    }
}
