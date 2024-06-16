use core::marker::PhantomData;

use crate::tla::{Context, Expr, Set, TlaType};

#[derive(Debug, Clone, Default)]
pub struct Empty<T>
where
    T: TlaType,
{
    _phantom: PhantomData<T>,
}

impl<T> Empty<T>
where
    T: TlaType,
{
    #[must_use]
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<T> Expr for Empty<T>
where
    T: TlaType,
{
    type Output = Set<T>;

    fn tla_expr(&self, _cx: &mut Context) -> String {
        "{}".into()
    }

    fn evaluate(&self) -> Self::Output {
        todo!()
    }
}
