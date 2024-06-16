use crate::tla::{Context, Expr, TlaType};

// derives Vec
pub type List<T> = Vec<T>;

impl<T> TlaType for List<T>
where
    T: TlaType,
{
    fn get_type() -> String {
        format!("Seq({})", T::get_type())
    }
}

impl<T> Expr for List<T>
where
    T: Expr,
{
    type Output = List<T::Output>;

    fn tla_expr(&self, cx: &mut Context) -> String {
        let v: Vec<_> = self.iter().map(|x| x.tla_expr(cx)).collect();
        format!("<<{}>>", v.join(","))
    }

    fn evaluate(&self) -> Self::Output {
        self.iter().map(crate::tla::expr::Expr::evaluate).collect()
    }
}
