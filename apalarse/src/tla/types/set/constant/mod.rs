mod empty;
mod nat;
mod range_set;

pub use empty::Empty;
pub use nat::Nat;
pub use range_set::RangeSet;

use crate::tla::{Context, Expr, Set, TlaType};

impl<T> TlaType for Set<T>
where
    T: TlaType,
{
    fn get_type() -> String {
        format!("Set({})", T::get_type())
    }
}

impl<T> Expr for Set<T>
where
    T: Expr,
{
    type Output = Set<T::Output>;

    fn tla_expr(&self, cx: &mut Context) -> String {
        let s: Vec<_> = self.inner.iter().map(|e| e.tla_expr(cx)).collect();
        format!("{{{}}}", s.join(","))
    }

    fn evaluate(&self) -> Self::Output {
        todo!()
        // self.iter().map(|e| e.evaluate()).collect()
    }
}
