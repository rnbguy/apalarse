use crate::tla::{BoolExpr, Context, Expr, SetExpr, Variable};

#[derive(Debug, Clone)]
pub struct Filter<S, P, U>
where
    S: SetExpr,
    P: Fn(Variable<S::ElemType>) -> U,
    U: BoolExpr,
{
    pub set: S,
    pub predicate: P,
}

impl<S, P, U> Filter<S, P, U>
where
    S: SetExpr,
    P: Fn(Variable<S::ElemType>) -> U,
    U: BoolExpr,
{
    pub fn new(set: S, predicate: P) -> Filter<S, P, U> {
        Filter { set, predicate }
    }
}

impl<S, P, U> Expr for Filter<S, P, U>
where
    S: SetExpr,
    P: Fn(Variable<S::ElemType>) -> U + Clone,
    U: BoolExpr,
{
    type Output = S::Output;

    fn tla_expr(&self, cx: &mut Context) -> String {
        cx.enter_scope();
        let new_var = cx.bound_var();
        let rt = format!(
            "{{{} \\in {}: {}}}",
            new_var.tla_expr(cx),
            self.set.tla_expr(cx),
            (self.predicate)(new_var.clone()).tla_expr(cx),
        );
        cx.exit_scope();
        rt
    }

    fn evaluate(&self) -> Self::Output {
        todo!()
    }
}
