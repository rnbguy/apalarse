use crate::tla::{Bool, Context, Expr, SetExpr};

#[derive(Debug, Clone)]
pub struct NotInSet<E, S>
where
    E: Expr,
    S: SetExpr<ElemType = E::Output>,
{
    elem: E,
    set: S,
}

impl<E, S> NotInSet<E, S>
where
    E: Expr,
    S: SetExpr<ElemType = E::Output>,
{
    pub const fn new(elem: E, set: S) -> Self {
        Self { elem, set }
    }
}

impl<E, S> Expr for NotInSet<E, S>
where
    E: Expr,
    S: SetExpr<ElemType = E::Output>,
{
    type Output = Bool;

    fn tla_expr(&self, cx: &mut Context) -> String {
        format!(
            "({} \\notin {})",
            self.elem.tla_expr(cx),
            self.set.tla_expr(cx)
        )
    }

    fn evaluate(&self) -> Self::Output {
        todo!()
    }
}
