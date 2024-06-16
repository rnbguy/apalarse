use crate::tla::{Bool, Context, Expr, TlaType, Variable};

#[derive(Debug, Clone)]
pub struct Unchanged<T>
where
    T: TlaType,
{
    pub inner: Variable<T>,
}

impl<T> Unchanged<T>
where
    T: TlaType,
{
    #[must_use]
    pub fn new(inner: Variable<T>) -> Self {
        Self { inner }
    }
}

impl<T> Expr for Unchanged<T>
where
    T: TlaType,
    Variable<T>: Expr,
{
    type Output = Bool;

    fn tla_expr(&self, cx: &mut Context) -> String {
        format!("(UNCHANGED {})", self.inner.tla_expr(cx))
    }

    fn evaluate(&self) -> Self::Output {
        todo!()
    }
}
