use crate::tla::{Bool, Context, Expr, SetExpr};

#[derive(Debug, Clone)]
pub struct InSet<E, S>
where
    E: Expr,
    S: SetExpr<ElemType = E::Output>,
{
    pub elem: E,
    pub set: S,
}

impl<E, S> InSet<E, S>
where
    E: Expr,
    S: SetExpr<ElemType = E::Output>,
{
    pub const fn new(elem: E, set: S) -> Self {
        Self { elem, set }
    }
}

impl<E, S> Expr for InSet<E, S>
where
    E: Expr,
    S: SetExpr<ElemType = E::Output>,
{
    type Output = Bool;

    fn tla_expr(&self, cx: &mut Context) -> String {
        format!(
            "({} \\in {})",
            self.elem.tla_expr(cx),
            self.set.tla_expr(cx)
        )
    }

    fn evaluate(&self) -> Self::Output {
        todo!()
    }
}
