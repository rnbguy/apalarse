use crate::tla::{Context, Expr, Set};

#[derive(Debug, Clone, Copy)]
pub struct Singleton<T>
where
    T: Expr,
{
    elem: T,
}

impl<T> Singleton<T>
where
    T: Expr,
{
    pub const fn new(elem: T) -> Self {
        Self { elem }
    }
}

impl<T> Expr for Singleton<T>
where
    T: Expr,
{
    type Output = Set<T::Output>;

    fn tla_expr(&self, cx: &mut Context) -> String {
        format!("{{{}}}", self.elem.tla_expr(cx))
    }

    fn evaluate(&self) -> Self::Output {
        todo!()
    }
}
