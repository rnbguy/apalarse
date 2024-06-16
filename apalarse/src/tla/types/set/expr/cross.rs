use crate::tla::{Context, Expr, Map, Set, SetExpr};

#[derive(Debug, Clone)]
pub struct Cross<S, U>
where
    S: SetExpr,
    U: SetExpr,
{
    pub lhs: S,
    pub rhs: U,
}

impl<S, U> Cross<S, U>
where
    S: SetExpr,
    U: SetExpr,
{
    pub const fn new(lhs: S, rhs: U) -> Self {
        Self { lhs, rhs }
    }
}

impl<S, U> Expr for Cross<S, U>
where
    S: SetExpr,
    U: SetExpr,
{
    type Output = Set<Map<S::ElemType, U::ElemType>>;

    fn tla_expr(&self, cx: &mut Context) -> String {
        format!("[{} -> {}]", self.lhs.tla_expr(cx), self.rhs.tla_expr(cx),)
    }

    fn evaluate(&self) -> Self::Output {
        todo!()
    }
}
