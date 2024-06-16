use crate::tla::{Context, Expr, Int, ListExpr, Variable};

#[derive(Debug, Clone)]
pub struct Replace<L, P, U>
where
    L: ListExpr,
    P: Fn(Variable<L::ElemType>) -> U,
    U: Expr<Output = L::ElemType>,
{
    inner: L,
    key: Int,
    value: P,
}

impl<L, P, U> Replace<L, P, U>
where
    L: ListExpr,
    P: Fn(Variable<L::ElemType>) -> U,
    U: Expr<Output = L::ElemType>,
{
    pub fn new(inner: L, key: Int, value: P) -> Self {
        Self { inner, key, value }
    }
}

impl<L, P, U> Expr for Replace<L, P, U>
where
    L: ListExpr,
    P: Fn(Variable<L::ElemType>) -> U + Clone,
    U: Expr<Output = L::ElemType>,
{
    type Output = L::Output;

    fn tla_expr(&self, cx: &mut Context) -> String {
        cx.enter_scope();
        let new_var = cx.map_var();
        let rt = format!(
            "[{} EXCEPT ![{}] = {}]",
            self.inner.tla_expr(cx),
            self.key.tla_expr(cx),
            (self.value)(new_var).tla_expr(cx),
        );
        cx.exit_scope();
        rt
    }

    fn evaluate(&self) -> Self::Output {
        todo!()
    }
}
