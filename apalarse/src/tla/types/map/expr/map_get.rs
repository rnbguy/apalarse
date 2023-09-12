use crate::tla::{Context, Expr, MapExpr};

#[derive(Debug, Clone)]
pub struct MapGet<FE, KE>
where
    FE: MapExpr,
    KE: Expr<Output = FE::KeyType>,
{
    func: FE,
    key: KE,
}

impl<FE, KE> MapGet<FE, KE>
where
    FE: MapExpr,
    KE: Expr<Output = FE::KeyType>,
{
    pub fn new(func: FE, key: KE) -> Self {
        Self { func, key }
    }
}

impl<FE, KE> Expr for MapGet<FE, KE>
where
    FE: MapExpr,
    KE: Expr<Output = FE::KeyType>,
{
    type Output = FE::ValueType;

    fn tla_expr(&self, cx: &mut Context) -> String {
        format!("{}[{}]", self.func.tla_expr(cx), self.key.tla_expr(cx),)
    }

    fn evaluate(&self) -> Self::Output {
        todo!()
    }
}
