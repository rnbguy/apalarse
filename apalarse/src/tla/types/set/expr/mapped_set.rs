use crate::tla::{Context, Expr, Set, SetExpr, Variable};

#[derive(Debug, Clone)]
pub struct MappedSet<S, P, U>
where
    S: SetExpr,
    P: Fn(Variable<S::ElemType>) -> U,
    U: Expr,
{
    pub set: S,
    pub predicate: P,
}

impl<S, P, U> MappedSet<S, P, U>
where
    S: SetExpr,
    P: Fn(Variable<S::ElemType>) -> U,
    U: Expr,
{
    pub fn new(set: S, predicate: P) -> Self {
        Self { set, predicate }
    }
}

impl<S, P, U> Expr for MappedSet<S, P, U>
where
    S: SetExpr,
    P: Fn(Variable<S::ElemType>) -> U + Clone,
    U: Expr,
{
    type Output = Set<U::Output>;

    fn tla_expr(&self, cx: &mut Context) -> String {
        cx.enter_scope();
        let new_var = cx.bound_var();
        let rt = format!(
            "{{{}: {} \\in {}}}",
            (self.predicate)(new_var.clone()).tla_expr(cx),
            new_var.tla_expr(cx),
            self.set.tla_expr(cx),
        );
        cx.exit_scope();
        rt
    }

    fn evaluate(&self) -> Self::Output {
        todo!()
    }
}
