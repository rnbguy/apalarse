use crate::tla::{Bool, Context, Expr};

#[derive(Debug, Clone)]
pub struct IfThenElse<C, P, N>
where
    C: Expr<Output = Bool>,
    P: Expr,
    N: Expr<Output = P::Output>,
{
    cond: C,
    pos: P,
    neg: N,
}

impl<C, P, N> IfThenElse<C, P, N>
where
    C: Expr<Output = Bool>,
    P: Expr,
    N: Expr<Output = P::Output>,
{
    pub fn new(cond: C, lhs: P, rhs: N) -> Self {
        Self {
            cond,
            pos: lhs,
            neg: rhs,
        }
    }
}

impl<C, P, N> Expr for IfThenElse<C, P, N>
where
    C: Expr<Output = Bool>,
    P: Expr,
    N: Expr<Output = P::Output>,
{
    type Output = P::Output;
    fn tla_expr(&self, cx: &mut Context) -> String {
        format!(
            "(IF {} THEN {} ELSE {})",
            self.cond.tla_expr(cx),
            self.pos.tla_expr(cx),
            self.neg.tla_expr(cx)
        )
    }

    fn evaluate(&self) -> Self::Output {
        if self.cond.evaluate() {
            self.pos.evaluate()
        } else {
            self.neg.evaluate()
        }
    }
}
