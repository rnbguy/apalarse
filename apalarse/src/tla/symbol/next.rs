use crate::tla::{Context, Expr, TlaType, Variable};

#[derive(Debug, Clone)]
pub struct Next<T>
where
    T: TlaType,
{
    pub inner: Variable<T>,
}

impl<T> Next<T>
where
    T: TlaType,
{
    pub fn new(inner: Variable<T>) -> Next<T> {
        Next { inner }
    }
}

impl<T> Expr for Next<T>
where
    T: TlaType,
    Variable<T>: Expr,
{
    type Output = T;

    fn tla_expr(&self, cx: &mut Context) -> String {
        format!("{}'", self.inner.tla_expr(cx))
    }

    fn evaluate(&self) -> Self::Output {
        todo!()
    }
}
