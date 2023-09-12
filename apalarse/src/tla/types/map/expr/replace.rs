use crate::tla::{Context, Expr, MapExpr, Variable};

#[derive(Debug, Clone)]
pub struct Replace<FE, KE, VE, U>
where
    FE: MapExpr,
    KE: Expr<Output = FE::KeyType>,
    VE: Fn(Variable<FE::ValueType>) -> U,
    U: Expr<Output = FE::ValueType>,
{
    func: FE,
    key: KE,
    value: VE,
}

impl<FE, KE, VE, U> Replace<FE, KE, VE, U>
where
    FE: MapExpr,
    KE: Expr<Output = FE::KeyType>,
    VE: Fn(Variable<FE::ValueType>) -> U,
    U: Expr<Output = FE::ValueType>,
{
    pub fn new(func: FE, key: KE, value: VE) -> Self {
        Self { func, key, value }
    }
}

impl<FE, KE, VE, U> Expr for Replace<FE, KE, VE, U>
where
    FE: MapExpr,
    KE: Expr<Output = FE::KeyType>,
    VE: Fn(Variable<FE::ValueType>) -> U + Clone,
    U: Expr<Output = FE::ValueType>,
{
    type Output = FE::Output;

    fn tla_expr(&self, cx: &mut Context) -> String {
        cx.enter_scope();
        let new_var = cx.map_var();
        let rt = format!(
            "[{} EXCEPT ![{}] = {}]",
            self.func.tla_expr(cx),
            self.key.tla_expr(cx),
            (self.value)(new_var.clone()).tla_expr(cx),
        );
        cx.exit_scope();
        rt
    }

    fn evaluate(&self) -> Self::Output {
        todo!()
    }
}
