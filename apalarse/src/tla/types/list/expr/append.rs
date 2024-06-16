use crate::tla::{Context, Expr, ListExpr};

#[derive(Debug, Clone)]
pub struct Append<L, E>
where
    E: Expr,
    L: ListExpr<ElemType = E::Output>,
{
    list: L,
    elem: E,
}

impl<L, E> Append<L, E>
where
    E: Expr,
    L: ListExpr<ElemType = E::Output>,
{
    pub const fn new(list: L, elem: E) -> Self {
        Self { list, elem }
    }
}

impl<L, E> Expr for Append<L, E>
where
    E: Expr,
    L: ListExpr<ElemType = E::Output>,
{
    type Output = L::Output;

    fn tla_expr(&self, cx: &mut Context) -> String {
        cx.add_module("Sequences");
        format!(
            "Append({}, {})",
            self.list.tla_expr(cx),
            self.elem.tla_expr(cx)
        )
    }

    fn evaluate(&self) -> Self::Output {
        todo!()
    }
}
